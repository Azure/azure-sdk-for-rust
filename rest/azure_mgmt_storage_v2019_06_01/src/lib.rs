mod models;
mod client;

pub use self::{models::*, client::*};

// these are all TODO
pub type UnknownType = serde_json::Value;

pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
}
