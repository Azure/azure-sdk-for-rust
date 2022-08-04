use azure_core::headers;
use azure_core::Method;
use serde::Serialize;
use std::collections::HashMap;

use crate::service::resources::{ConfigurationContent, ConfigurationMetrics};
use crate::service::responses::ConfigurationResponse;
use crate::service::{ServiceClient, API_VERSION};

azure_core::operation! {
    /// The CreateOrUpdateConfigurationBuilder is used to construct a new configuration
    /// or update an existing one.
    CreateOrUpdateConfiguration,
    client: ServiceClient,
    configuration_id: String,
    priority: u64,
    target_condition: String,
    etag: Option<String>,
    ?content: ConfigurationContent,
    ?metrics: HashMap<String, String>,
    ?labels: HashMap<String, String>
}

impl CreateOrUpdateConfigurationBuilder {
    /// Sets the device content for the configuration
    /// The content cannot be updated once it has been created.
    pub fn device_content(mut self, device_content: serde_json::Value) -> Self {
        let content = self.content.get_or_insert(Default::default());
        content.device_content = Some(device_content);
        self
    }

    /// Sets the module content for the configuration.
    /// The content cannot be updated once it has been created.
    pub fn module_content(mut self, module_content: serde_json::Value) -> Self {
        let content = self.content.get_or_insert(Default::default());
        content.module_content = Some(module_content);
        self
    }

    /// Sets the module content for the configuration
    /// The content cannot be updated once it has been created.
    pub fn modules_content(mut self, modules_content: serde_json::Value) -> Self {
        let content = self.content.get_or_insert(Default::default());
        content.modules_content = Some(modules_content);
        self
    }

    /// Add a metric to the configuration
    pub fn metric<S, T>(mut self, key: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let metrics = self.metrics.get_or_insert(Default::default());
        metrics.insert(key.into(), value.into());
        self
    }

    /// Add a label to the configuration.
    pub fn label<S, T>(mut self, key: S, value: T) -> Self
    where
        S: Into<String>,
        T: Into<String>,
    {
        let labels = self.labels.get_or_insert(Default::default());
        labels.insert(key.into(), value.into());
        self
    }

    /// Performs the create or update request on the device identity
    pub fn into_future(mut self) -> CreateOrUpdateConfiguration {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/configurations/{}?api-version={}",
                self.client.iot_hub_name, &self.configuration_id, API_VERSION
            );

            let mut request = self.client.finalize_request(&uri, Method::Put)?;

            match &self.etag {
                Some(etag) => {
                    request.insert_header(headers::IF_MATCH, format!("\"{}\"", etag));
                }
                None => (),
            }

            let body = CreateOrUpdateConfigurationBody {
                content: self.content.unwrap_or_default(),
                etag: self.etag,
                id: &self.configuration_id,
                labels: self.labels.unwrap_or_default(),
                metrics: ConfigurationMetrics {
                    queries: self.metrics.unwrap_or_default(),
                    results: HashMap::new(),
                },
                priority: self.priority,
                target_condition: &self.target_condition,
            };

            let body = azure_core::to_json(&body)?;
            request.set_body(body);

            let response = self.client.send(&mut self.context, &mut request).await?;

            CreateOrUpdateConfigurationResponse::try_from(response).await
        })
    }
}

pub type CreateOrUpdateConfigurationResponse = ConfigurationResponse;

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
