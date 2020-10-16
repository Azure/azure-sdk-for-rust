#[cfg(feature = "2020-06-30")]
mod v2020_06_30;
#[cfg(feature = "2020-06-30")]
pub use v2020_06_30::{models, operations, API_VERSION};
#[cfg(feature = "2020-09-30")]
mod v2020_09_30;
#[cfg(feature = "2020-09-30")]
pub use v2020_09_30::{models, operations, API_VERSION};
#[cfg(feature = "2020-06-01")]
mod v2020_06_01;
#[cfg(feature = "2020-06-01")]
pub use v2020_06_01::{models, operations, API_VERSION};
#[cfg(feature = "2020-05-01")]
mod v2020_05_01;
#[cfg(feature = "2020-05-01")]
pub use v2020_05_01::{models, operations, API_VERSION};
#[cfg(feature = "2019-12-01")]
mod v2019_12_01;
#[cfg(feature = "2019-12-01")]
pub use v2019_12_01::{models, operations, API_VERSION};
#[cfg(feature = "2019-11-01")]
mod v2019_11_01;
#[cfg(feature = "2019-11-01")]
pub use v2019_11_01::{models, operations, API_VERSION};
#[cfg(feature = "2019-07-01")]
mod v2019_07_01;
#[cfg(feature = "2019-07-01")]
pub use v2019_07_01::{models, operations, API_VERSION};
#[cfg(feature = "2019-03-01")]
mod v2019_03_01;
#[cfg(feature = "2019-03-01")]
pub use v2019_03_01::{models, operations, API_VERSION};
#[cfg(feature = "2018-10-01")]
mod v2018_10_01;
#[cfg(feature = "2018-10-01")]
pub use v2018_10_01::{models, operations, API_VERSION};
#[cfg(feature = "2018-06-01")]
mod v2018_06_01;
#[cfg(feature = "2018-06-01")]
pub use v2018_06_01::{models, operations, API_VERSION};
#[cfg(feature = "2018-04-01")]
mod v2018_04_01;
#[cfg(feature = "2018-04-01")]
pub use v2018_04_01::{models, operations, API_VERSION};
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
