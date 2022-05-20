use http::Response;
use serde::{Deserialize, Serialize};

use crate::service::resources::{ConfigurationContent, ConfigurationMetrics};

/// The representation of a configuration.
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationResponse {
    /// The content of the configuration.
    content: ConfigurationContent,
    /// The creation date and time of the configuration.
    created_time_utc: String,
    /// The ETag of the configuration.
    etag: String,
    /// The unique identifier of the configuration.
    id: String,
    /// The key-value pairs used to describe the configuration
    labels: serde_json::Value,
    /// The update date and time of the configuration
    last_updated_time_utc: String,
    /// The custom metrics specified by the developer as queries against twin reported properties
    metrics: ConfigurationMetrics,
    /// The priority number assigned to the configuration
    priority: u64,
    /// The schema version of the configuration
    schema_version: Option<String>,
    /// The system metrics computed by the IoT Hub that cannot be customized
    system_metrics: ConfigurationMetrics,
    /// The query used to define the targeted devices or modules.
    target_condition: String,
}

impl std::convert::TryFrom<Response<bytes::Bytes>> for ConfigurationResponse {
    type Error = crate::Error;

    fn try_from(response: Response<bytes::Bytes>) -> Result<Self, Self::Error> {
        let body = response.body();

        let configuration_response: ConfigurationResponse = serde_json::from_slice(body)?;

        Ok(configuration_response)
    }
}
