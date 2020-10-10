mod client;
mod models;

pub use self::{client::*, models::*};

pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
    pub bearer_access_token: Option<String>,
}
