mod client;
mod models;

pub use self::{client::*, models::*};

pub struct Configuration {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        {
            let api_version = "2020-08-01".to_owned();
            let client = reqwest::Client::new();
            let base_path = "https://management.azure.com".to_owned();
            Self {
                api_version,
                client,
                base_path,
                bearer_access_token: None,
            }
        }
    }
}
