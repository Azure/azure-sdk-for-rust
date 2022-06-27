use azure_core::headers;
use azure_core::Method;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryInto;

use crate::service::resources::{ConfigurationContent, ConfigurationMetrics};
use crate::service::responses::ConfigurationResponse;
use crate::service::{ServiceClient, API_VERSION};

/// The CreateOrUpdateConfigurationBuilder is used to construct a new configuration
/// or update an existing one.
pub struct CreateOrUpdateConfigurationBuilder<'a> {
    service_client: &'a ServiceClient,
    configuration_id: String,
    priority: u64,
    target_condition: String,
    etag: Option<String>,
    content: ConfigurationContent,
    metrics: HashMap<String, String>,
    labels: HashMap<String, String>,
}

impl<'a> CreateOrUpdateConfigurationBuilder<'a> {
    pub(crate) fn new(
        service_client: &'a ServiceClient,
        configuration_id: String,
        priority: u64,
        target_condition: String,
        etag: Option<String>,
    ) -> Self {
        Self {
            service_client,
            configuration_id,
            priority,
            target_condition,
            etag,
            content: ConfigurationContent {
                device_content: None,
                module_content: None,
                modules_content: None,
            },
            metrics: HashMap::new(),
            labels: HashMap::new(),
        }
    }

    /// Sets the device content for the configuration
    /// The content cannot be updated once it has been created.
    pub fn device_content(mut self, device_content: serde_json::Value) -> Self {
        self.content.device_content = Some(device_content);
        self
    }

    /// Sets the module content for the configuration.
    /// The content cannot be updated once it has been created.
    pub fn module_content(mut self, module_content: serde_json::Value) -> Self {
        self.content.module_content = Some(module_content);
        self
    }

    /// Sets the module content for the configuration
    /// The content cannot be updated once it has been created.
    pub fn modules_content(mut self, modules_content: serde_json::Value) -> Self {
        self.content.modules_content = Some(modules_content);
        self
    }

    /// Add a metric to the configuration
    pub fn metric<S, T>(mut self, key: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.metrics.insert(key.into(), value.into());
        self
    }

    /// Add multiple metrics to the configuration.
    pub fn metrics(mut self, metrics: HashMap<String, String>) -> Self {
        self.metrics = metrics;
        self
    }

    /// Add a label to the configuration.
    pub fn label<S, T>(mut self, key: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        self.labels.insert(key.into(), value.into());
        self
    }

    /// Add multiple labels to the configuration.
    pub fn labels(mut self, labels: HashMap<String, String>) -> Self {
        self.labels = labels;
        self
    }

    /// Performs the create or update request on the device identity
    pub async fn execute(self) -> azure_core::Result<ConfigurationResponse> {
        let uri = format!(
            "https://{}.azure-devices.net/configurations/{}?api-version={}",
            self.service_client.iot_hub_name, &self.configuration_id, API_VERSION
        );

        let mut request = self.service_client.prepare_request(&uri, Method::PUT)?;

        match &self.etag {
            Some(etag) => {
                request.insert_header(headers::IF_MATCH, format!("\"{}\"", etag));
            }
            None => (),
        }

        let body = CreateOrUpdateConfigurationBody {
            content: self.content,
            etag: self.etag,
            id: &self.configuration_id,
            labels: self.labels,
            metrics: ConfigurationMetrics {
                queries: self.metrics,
                results: HashMap::new(),
            },
            priority: self.priority,
            target_condition: &self.target_condition,
        };

        let body = azure_core::to_json(&body)?;
        request.set_body(body);

        self.service_client
            .http_client()
            .execute_request_check_status(&request)
            .await?
            .try_into()
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateOrUpdateConfigurationBody<'a, 'b> {
    content: ConfigurationContent,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<String>,
    id: &'a str,
    labels: HashMap<String, String>,
    metrics: ConfigurationMetrics,
    priority: u64,
    target_condition: &'b str,
}
