use serde::{Deserialize, Serialize};

/// The configuration content for devices or modules on edge devices.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationContent {
    /// The device configuration content.
    device_content: Option<serde_json::Value>,
    /// The module configuration content.
    module_content: Option<serde_json::Value>,
    /// The modules configuration content.
    modules_content: Option<serde_json::Value>,
}

/// The configuration metrics for Iot hub devices and modules.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationMetrics {
    /// The key-value pairs with queries and their identifier.
    queries: serde_json::Value,
    /// The results of the metrics collection queries.
    results: serde_json::Value,
}
