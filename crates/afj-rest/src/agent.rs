use agent::error::{Error, Result};
use reqwest::Url;

/// AFJ REST agent
#[derive(Debug)]
pub struct CloudAgentAfjRest {
    /// base url of the cloudagent
    pub endpoint: String,

    /// admin Api key for the cloudagent
    pub api_key: Option<String>,

    /// Authorization token for a multi tenancy agent
    pub auth_token: Option<String>,

    /// Agent version
    pub version: CloudAgentAfjRestVersion,
}

/// AFJ REST supported versions
/// TODO: How do we want to deal with mulitple versions? Architecture wise.
#[derive(Debug)]
pub enum CloudAgentAfjRestVersion {
    /// ~0.8.0
    ZeroEightZero,
}

impl std::fmt::Display for CloudAgentAfjRestVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            CloudAgentAfjRestVersion::ZeroEightZero => "0.8.0",
        };
        write!(f, "{}", v)
    }
}

impl std::fmt::Display for CloudAgentAfjRest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AFJ {}", self.version)
    }
}

impl CloudAgentAfjRest {
    /// Create a new instance of an AFJ REST agent
    pub fn new(
        endpoint: String,
        version: CloudAgentAfjRestVersion,
        api_key: Option<String>,
        auth_token: Option<String>,
    ) -> Self {
        Self {
            endpoint,
            version,
            api_key,
            auth_token,
        }
    }

    /// Create a url based on the base url and a list of paths
    pub fn create_url(&self, paths: Vec<&str>) -> Result<Url> {
        let mut url = Url::parse(&self.endpoint)
            .map_err(|_| Box::new(Error::UnreachableUrl) as Box<dyn std::error::Error>)?;
        url.set_path(&paths.join("/"));
        Ok(url)
    }
}
