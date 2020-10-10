mod models;
mod client;

pub use self::{models::*, client::*};

pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
    pub bearer_access_token: Option<String>,
}
