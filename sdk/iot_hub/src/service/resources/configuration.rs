use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The configuration content for devices or modules on edge devices.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationContent {
    /// The device configuration content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_content: Option<serde_json::Value>,
    /// The module configuration content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub module_content: Option<serde_json::Value>,
    /// The modules configuration content.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modules_content: Option<serde_json::Value>,
}

/// The configuration metrics for Iot hub devices and modules.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationMetrics {
    /// The key-value pairs with queries and their identifier.
    pub queries: HashMap<String, String>,
    /// The results of the metrics collection queries.
    pub results: HashMap<String, serde_json::Value>,
}

/// The configuration metrics for Iot hub devices and modules.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    /// The content of the configuration.
    pub content: ConfigurationContent,
    /// The creation date and time of the configuration.
    pub created_time_utc: String,
    /// The ETag of the configuration.
    pub etag: String,
    /// The unique identifier of the configuration.
    pub id: String,
    /// The key-value pairs used to describe the configuration
    pub labels: HashMap<String, String>,
    /// The update date and time of the configuration
    pub last_updated_time_utc: String,
    /// The custom metrics specified by the developer as queries against twin reported properties
    pub metrics: ConfigurationMetrics,
    /// The priority number assigned to the configuration
    pub priority: u64,
    /// The schema version of the configuration
    pub schema_version: Option<String>,
    /// The system metrics computed by the IoT Hub that cannot be customized
    pub system_metrics: ConfigurationMetrics,
    /// The query used to define the targeted devices or modules.
    pub target_condition: String,
}
