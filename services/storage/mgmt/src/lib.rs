#[cfg(feature = "2019-06-01")]
pub mod models_2019_06_01;
#[cfg(feature = "2019-06-01")]
pub use models_2019_06_01 as models;
#[cfg(feature = "2019-06-01")]
pub mod operations_2019_06_01;
#[cfg(feature = "2019-06-01")]
pub use operations_2019_06_01 as operations;
#[cfg(feature = "2019-06-01")]
pub const API_VERSION: &str = "2019-06-01";

#[cfg(feature = "2019-08-01-preview")]
pub mod models_2019_08_01_preview;
#[cfg(feature = "2019-08-01-preview")]
pub use models_2019_08_01_preview as models;
#[cfg(feature = "2019-08-01-preview")]
pub mod operations_2019_08_01_preview;
#[cfg(feature = "2019-08-01-preview")]
pub use operations_2019_08_01_preview as operations;
#[cfg(feature = "2019-08-01-preview")]
pub const API_VERSION: &str = "2019-08-01-preview";

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

pub struct Configuration {
    pub api_version: String,
    pub client: reqwest::Client,
    pub base_path: String,
    pub bearer_access_token: Option<String>,
}

impl Configuration {
    pub fn new(bearer_access_token: &str) -> Self {
        Self {
            bearer_access_token: Some(bearer_access_token.to_owned()),
            ..Default::default()
        }
    }
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
