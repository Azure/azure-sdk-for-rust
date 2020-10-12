mod client;
mod models;

pub use self::{client::*, models::*};

pub const API_VERSION: &str = "2019-06-01";

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Configuration {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        {
            Self {
                api_version: API_VERSION.to_owned(),
                client: reqwest::Client::new(),
                base_path: "https://management.azure.com".to_owned(),
                bearer_access_token: None,
            }
        }
    }
}
