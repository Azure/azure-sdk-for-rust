use serde::Deserialize;
/// Representation of the twin properties.
#[derive(Deserialize, Debug)]
pub struct TwinProperties {
    /// The desired properties.
    pub desired: serde_json::Value,
    /// The reported properties.
    pub reported: serde_json::Value,
}
