use crate::agent::CloudAgentPython;
use agent::error::{Error, Result};
use logger::pretty_stringify_obj;
use reqwest::{Client, RequestBuilder, Url};
use serde::de::DeserializeOwned;
use serde_json::Value;
use std::fmt::Debug;

/// Call logic for http calls
impl CloudAgentPython {
    /// Builds a get request and calls the sender
    pub async fn get<T: DeserializeOwned + Debug>(
        &self,
        url: Url,
        query: Option<Vec<(&str, String)>>,
    ) -> Result<T> {
        let client = match &query {
            Some(q) => Client::new().get(url).query(&q),
            None => Client::new().get(url),
        };

        log_trace!("Get request query:");
        log_trace!("{:#?}", query);

        self.send::<T>(client).await
    }

    /// Builds a post request and calls the sender
    pub async fn post<T: DeserializeOwned + Debug>(
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

        log_trace!("Post request body: {:#?}", body);
        log_trace!("Post request query: {:#?}", query);

        self.send::<T>(client).await
    }

    /// Sends any request
    pub async fn send<T: DeserializeOwned + Debug>(&self, client: RequestBuilder) -> Result<T> {
        let client = match &self.api_key {
            Some(a) => client.header("X-API-KEY", a),
            None => client,
        };

        let client = match &self.auth_token {
            Some(t) => client.header("Authorization", format!("Bearer {}", t)),
            None => client,
        };

        log_trace!("About to send request:");
        log_trace!("{:#?}", client);
        match client.send().await {
            Ok(res) => {
                let status_code = res.status().as_u16();
                log_debug!("Got response code {}", status_code);
                log_trace!("{:#?}", res);
                match status_code {
                    200..=299 => {
                        let parsed: Value = res.json().await.unwrap();
                        log_debug!("Response: {}", pretty_stringify_obj(&parsed));
                        serde_json::from_value(parsed).map_err(|e| {
                            log_warn!("{}", e);
                            Error::UnableToParseResponse.into()
                        })
                    }
                    // Issue credential message when attributes are not correct
                    400 => Err(res.text().await?.into()),
                    401 => Err(Error::AuthorizationFailed.into()),
                    404 => Err(Error::UrlDoesNotExist.into()),
                    422 => Err(res.text().await?.into()),
                    503 => Err(Error::HttpServiceUnavailable.into()),
                    500..=599 => Err(Error::InternalServerError(
                        res.status().as_u16(),
                        res.text().await?,
                    )
                    .into()),
                    _ => Err(Error::UnknownResponseStatusCode(res.text().await?).into()),
                }
            }
            Err(e) => {
                log_warn!("Request failed {}", e);
                Err(Error::UnreachableUrl.into())
            }
        }
    }
}
