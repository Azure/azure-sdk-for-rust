mod models;
mod client;

pub use self::{models::*, client::*};

// these are all TODO
pub type UnknownType = serde_json::Value;
pub type SkuName = serde_json::Value;
pub type Tier = serde_json::Value;
pub type SKUCapability = serde_json::Value;
pub type Sku = serde_json::Value;
pub type IPRule = serde_json::Value;
// in client
pub type PrivateLinkResourceListResult = serde_json::Value;
pub type PrivateEndpointConnection = serde_json::Value;
pub type PrivateEndpointConnectionListResult = serde_json::Value;

pub struct Configuration {
    pub base_path: String,
    pub client: reqwest::Client,
}
