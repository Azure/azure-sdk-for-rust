use azure_core::headers;
use azure_core::Method;
use serde::Serialize;
use std::collections::HashMap;
use std::convert::TryInto;

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
    /// Performs the create or update request on the device identity
    pub async fn into_future(self) -> CreateOrUpdateConfiguration {
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

            self.client
                .http_client()
                .execute_request_check_status(&request)
                .await?
                .try_into()
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
