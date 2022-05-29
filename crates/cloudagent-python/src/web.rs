use crate::cloud_agent::CloudAgent;
use agent::error::{Error, Result};
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;

/// Call logic for http calls
impl CloudAgent {
    /// Builds a get request and calls the sender
    pub async fn get<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
        let client = match &query {
            Some(q) => Client::new().get(url).query(&q),
            None => Client::new().get(url),
        };

        log_trace!("Get request query:\n{:#?}", query);

        self.send::<T>(client).await
    }

    /// Builds a post request and calls the sender
    pub async fn post<T: DeserializeOwned>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
        body: Option<Value>,
    ) -> Result<T> {
        let client = Client::new().post(url).query(&query);

        let client = match &body {
            Some(b) => client.json(&b),
            None => client,
        };

        log_trace!("Post request body:\n{:#?}", body);
        log_trace!("Post request query:\n{:#?}", query);

        self.send::<T>(client).await
    }

    /// Sends any request
    pub async fn send<T: DeserializeOwned>(&self, client: RequestBuilder) -> Result<T> {
        let client = match &self.api_key {
            Some(a) => client.header("X-API-KEY", a),
            None => client,
        };

        let client = match &self.auth_token {
            Some(t) => client.header("Authorization", format!("Bearer {}", t)),
            None => client,
        };

        log_trace!("About to send request:\n{:#?}", client);
        match client.send().await {
            Ok(res) => {
                let status_code = res.status().as_u16();
                log_trace!("Got {} response:\n{:#?}", status_code, res);
                match status_code {
                    200..=299 => res.json().await.map_err(|e| {
                        log!("{}", e);
                        Error::UnableToParseResponse.into()
                    }),
                    // Issue credential message when attributes are not correct
                    400 => Err(res.text().await?.into()),
                    401 => Err(Error::AuthorizationFailed.into()),
                    404 => Err(Error::UrlDoesNotExist.into()),
                    // TODO: This response is quite ugly. Is it only used at cred_def_id?
                    422 => Err(res.text().await?.into()),
                    503 => Err(Error::HttpServiceUnavailable.into()),
                    500..=599 => Err(Error::InternalServerError(res.status().as_u16()).into()),
                    _ => Err(Error::UnknownResponseStatusCode(res.text().await?).into()),
                }
            }
            Err(e) => {
                log_trace!("Request failed {}", e);
                Err(Error::UnreachableUrl.into())
            }
        }
    }
}
