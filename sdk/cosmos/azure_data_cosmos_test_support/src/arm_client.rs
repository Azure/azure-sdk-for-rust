// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Typed Azure Resource Manager client helpers for Cosmos live tests.
//!
//! This test-only client fills the gap until a generated Cosmos ARM crate is available.

use azure_core::{
    credentials::TokenCredential,
    http::{
        headers::HeaderName,
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientOptions, Method, Pipeline, PipelineSendOptions, RawResponse, Request, StatusCode,
        Url,
    },
    Result,
};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};

const DEFAULT_RESOURCE_MANAGER_ENDPOINT: &str = "https://management.azure.com/";
const RESOURCE_API_VERSION: &str = "2026-03-15";
const PARTITION_MERGE_API_VERSION: &str = "2026-04-01-preview";
const LRO_POLL_INTERVAL: Duration = Duration::from_secs(2);
const LRO_TIMEOUT: Duration = Duration::from_secs(60 * 60);

const SUBSCRIPTION_ID_ENV_VAR: &str = "COSMOS_SUBSCRIPTION_ID";
const RESOURCE_GROUP_ENV_VAR: &str = "COSMOS_RESOURCE_GROUP";
const ACCOUNT_NAME_ENV_VAR: &str = "COSMOS_ACCOUNT_NAME";
const LOCATION_ENV_VAR: &str = "COSMOS_LOCATION";
const RESOURCE_MANAGER_URL_ENV_VAR: &str = "COSMOS_RESOURCE_MANAGER_URL";

#[derive(Clone)]
pub struct CosmosArmClient {
    endpoint: Url,
    subscription_id: String,
    resource_group: String,
    account_name: String,
    location: Option<String>,
    pipeline: Pipeline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmThroughput {
    Manual(usize),
    Autoscale {
        maximum: usize,
        current: Option<usize>,
        increment_percent: Option<usize>,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SqlResourceCreateUpdateParameters<T> {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    properties: SqlResourceProperties<T>,
}

#[derive(Serialize)]
struct SqlResourceProperties<T> {
    resource: T,
    options: CreateUpdateOptions,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateUpdateOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscale_settings: Option<CreateAutoscaleSettings>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateAutoscaleSettings {
    max_throughput: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleAutoUpgradePolicy {
    throughput_policy: AutoscaleThroughputPolicy,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleThroughputPolicy {
    increment_percent: usize,
}

#[derive(Serialize)]
struct SqlDatabaseResource<'a> {
    id: &'a str,
}

#[derive(Serialize)]
struct ThroughputSettingsUpdateParameters {
    properties: ThroughputSettingsProperties,
}

#[derive(Serialize)]
struct ThroughputSettingsProperties {
    resource: ThroughputSettingsResource,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThroughputSettingsResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    throughput: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscale_settings: Option<ThroughputAutoscaleSettings>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThroughputAutoscaleSettings {
    max_throughput: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    auto_upgrade_policy: Option<AutoscaleAutoUpgradePolicy>,
}

#[derive(Deserialize)]
struct ThroughputSettingsGetResults {
    properties: ThroughputSettingsGetProperties,
}

#[derive(Deserialize)]
struct ThroughputSettingsGetProperties {
    resource: ThroughputSettingsGetResource,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ThroughputSettingsGetResource {
    throughput: Option<usize>,
    autoscale_settings: Option<AutoscaleSettingsResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleSettingsResult {
    max_throughput: usize,
    auto_upgrade_policy: Option<AutoscaleAutoUpgradePolicyResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleAutoUpgradePolicyResult {
    throughput_policy: Option<AutoscaleThroughputPolicyResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleThroughputPolicyResult {
    increment_percent: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PartitionMergeParameters {
    is_dry_run: bool,
}

impl CosmosArmClient {
    pub fn from_env(credential: Arc<dyn TokenCredential>) -> Result<Self> {
        let endpoint = std::env::var(RESOURCE_MANAGER_URL_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_RESOURCE_MANAGER_ENDPOINT.to_string());
        let endpoint = normalize_endpoint(&endpoint)?;
        let scope = format!("{}.default", endpoint.as_str());
        let auth_policy: Arc<dyn Policy> =
            Arc::new(BearerTokenAuthorizationPolicy::new(credential, [scope]));

        Ok(Self {
            endpoint,
            subscription_id: required_env(SUBSCRIPTION_ID_ENV_VAR)?,
            resource_group: required_env(RESOURCE_GROUP_ENV_VAR)?,
            account_name: required_env(ACCOUNT_NAME_ENV_VAR)?,
            location: std::env::var(LOCATION_ENV_VAR)
                .ok()
                .filter(|value| !value.trim().is_empty()),
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                ClientOptions::default(),
                Vec::new(),
                vec![auth_policy],
                None,
            ),
        })
    }

    pub async fn create_database(&self, database_name: &str) -> Result<()> {
        let body = SqlResourceCreateUpdateParameters {
            location: self.location.clone(),
            properties: SqlResourceProperties {
                resource: SqlDatabaseResource { id: database_name },
                options: CreateUpdateOptions::default(),
            },
        };
        let path = format!("{}/sqlDatabases/{database_name}", self.account_path());
        self.send_resource_operation(Method::Put, &path, Some(&body))
            .await
    }

    pub async fn delete_database(&self, database_name: &str) -> Result<()> {
        let path = format!("{}/sqlDatabases/{database_name}", self.account_path());
        self.send_resource_operation::<()>(Method::Delete, &path, None)
            .await
    }

    pub async fn create_or_update_container<T>(
        &self,
        database_name: &str,
        container_name: &str,
        resource: &T,
        throughput: Option<ArmThroughput>,
    ) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let body = SqlResourceCreateUpdateParameters {
            location: self.location.clone(),
            properties: SqlResourceProperties {
                resource,
                options: throughput.map(create_update_options).unwrap_or_default(),
            },
        };
        let path = self.container_path(database_name, container_name);
        self.send_resource_operation(Method::Put, &path, Some(&body))
            .await?;
        if let Some(
            throughput @ ArmThroughput::Autoscale {
                increment_percent: Some(_),
                ..
            },
        ) = throughput
        {
            self.replace_container_throughput(database_name, container_name, throughput)
                .await?;
        }
        Ok(())
    }

    pub async fn delete_container(&self, database_name: &str, container_name: &str) -> Result<()> {
        let path = self.container_path(database_name, container_name);
        self.send_resource_operation::<()>(Method::Delete, &path, None)
            .await
    }

    pub async fn read_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
    ) -> Result<ArmThroughput> {
        let path = format!(
            "{}/throughputSettings/default",
            self.container_path(database_name, container_name)
        );
        self.read_throughput(&path).await?.ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                "Cosmos container does not have dedicated throughput",
            )
        })
    }

    pub async fn read_database_throughput(
        &self,
        database_name: &str,
    ) -> Result<Option<ArmThroughput>> {
        let path = format!(
            "{}/sqlDatabases/{database_name}/throughputSettings/default",
            self.account_path()
        );
        self.read_throughput(&path).await
    }

    async fn read_throughput(&self, path: &str) -> Result<Option<ArmThroughput>> {
        let response = self
            .send(Method::Get, path, RESOURCE_API_VERSION, None::<&()>)
            .await?;
        if response.status() == StatusCode::NotFound {
            return Ok(None);
        }
        ensure_success(&response, "read Cosmos container throughput")?;
        let result: ThroughputSettingsGetResults = response.body().json()?;
        let resource = result.properties.resource;
        Ok(Some(
            match (resource.throughput, resource.autoscale_settings) {
                (current, Some(settings)) => ArmThroughput::Autoscale {
                    maximum: settings.max_throughput,
                    current,
                    increment_percent: settings
                        .auto_upgrade_policy
                        .and_then(|policy| policy.throughput_policy)
                        .map(|policy| policy.increment_percent),
                },
                (Some(throughput), None) => ArmThroughput::Manual(throughput),
                (None, None) => {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::DataConversion,
                        "ARM throughput response contained neither manual nor autoscale settings",
                    ));
                }
            },
        ))
    }

    pub async fn replace_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
        throughput: ArmThroughput,
    ) -> Result<ArmThroughput> {
        let body = ThroughputSettingsUpdateParameters {
            properties: ThroughputSettingsProperties {
                resource: throughput_settings_resource(throughput),
            },
        };
        let path = format!(
            "{}/throughputSettings/default",
            self.container_path(database_name, container_name)
        );
        self.send_resource_operation(Method::Put, &path, Some(&body))
            .await?;
        self.read_container_throughput(database_name, container_name)
            .await
    }

    pub async fn merge_partitions(&self, database_name: &str, container_name: &str) -> Result<()> {
        let path = format!(
            "{}/partitionMerge",
            self.container_path(database_name, container_name)
        );
        self.send_operation(
            Method::Post,
            &path,
            PARTITION_MERGE_API_VERSION,
            Some(&PartitionMergeParameters { is_dry_run: false }),
        )
        .await
    }

    fn account_path(&self) -> String {
        format!(
            "subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}",
            self.subscription_id, self.resource_group, self.account_name
        )
    }

    fn container_path(&self, database_name: &str, container_name: &str) -> String {
        format!(
            "{}/sqlDatabases/{database_name}/containers/{container_name}",
            self.account_path()
        )
    }

    async fn send_resource_operation<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        self.send_operation(method, path, RESOURCE_API_VERSION, body)
            .await
    }

    async fn send_operation<T>(
        &self,
        method: Method,
        path: &str,
        api_version: &str,
        body: Option<&T>,
    ) -> Result<()>
    where
        T: Serialize + ?Sized,
    {
        let response = self.send(method, path, api_version, body).await?;
        if method == Method::Delete && response.status() == StatusCode::NotFound {
            return Ok(());
        }
        ensure_success(&response, "manage Cosmos ARM resource")?;
        if response.status() == StatusCode::Accepted {
            let operation_url = operation_url(&response)?;
            self.wait_for_operation(operation_url).await?;
        }
        Ok(())
    }

    async fn send<T>(
        &self,
        method: Method,
        path: &str,
        api_version: &str,
        body: Option<&T>,
    ) -> Result<RawResponse>
    where
        T: Serialize + ?Sized,
    {
        let mut url = self.endpoint.join(path)?;
        url.query_pairs_mut()
            .append_pair("api-version", api_version);
        let mut request = Request::new(url, method);
        request.insert_header("accept", "application/json");
        if let Some(body) = body {
            request.insert_header("content-type", "application/json");
            request.set_json(body)?;
        }
        self.pipeline
            .send(
                &azure_core::http::Context::new(),
                &mut request,
                Some(PipelineSendOptions {
                    skip_checks: true,
                    ..Default::default()
                }),
            )
            .await
    }

    async fn wait_for_operation(&self, operation_url: Url) -> Result<()> {
        let deadline = tokio::time::Instant::now() + LRO_TIMEOUT;
        loop {
            if tokio::time::Instant::now() >= deadline {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("ARM operation did not complete within {LRO_TIMEOUT:?}"),
                ));
            }

            let mut request = Request::new(operation_url.clone(), Method::Get);
            request.insert_header("accept", "application/json");
            let response = self
                .pipeline
                .send(
                    &azure_core::http::Context::new(),
                    &mut request,
                    Some(PipelineSendOptions {
                        skip_checks: true,
                        ..Default::default()
                    }),
                )
                .await?;
            ensure_success(&response, "poll Cosmos ARM operation")?;
            if response.status() == StatusCode::Accepted {
                tokio::time::sleep(retry_after(&response).unwrap_or(LRO_POLL_INTERVAL)).await;
                continue;
            }
            if response.body().is_empty() {
                return Ok(());
            }

            let body: serde_json::Value = response.body().json()?;
            let status = operation_status(&body);
            match status.as_deref() {
                Some("succeeded" | "completed") => return Ok(()),
                Some("failed" | "canceled" | "cancelled") => {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!("ARM operation failed: {body}"),
                    ));
                }
                Some(_) => {
                    tokio::time::sleep(retry_after(&response).unwrap_or(LRO_POLL_INTERVAL)).await;
                }
                None => return Ok(()),
            }
        }
    }
}

fn required_env(name: &str) -> Result<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{name} must be set for ARM-backed Cosmos tests"),
            )
        })
}

fn normalize_endpoint(endpoint: &str) -> Result<Url> {
    let mut endpoint = Url::parse(endpoint)?;
    endpoint.set_query(None);
    endpoint.set_fragment(None);
    if !endpoint.path().ends_with('/') {
        endpoint.set_path(&format!("{}/", endpoint.path()));
    }
    Ok(endpoint)
}

fn create_update_options(throughput: ArmThroughput) -> CreateUpdateOptions {
    match throughput {
        ArmThroughput::Autoscale { maximum, .. } => CreateUpdateOptions {
            throughput: None,
            autoscale_settings: Some(CreateAutoscaleSettings {
                max_throughput: maximum,
            }),
        },
        ArmThroughput::Manual(throughput) => CreateUpdateOptions {
            throughput: Some(throughput),
            autoscale_settings: None,
        },
    }
}

fn throughput_settings_resource(throughput: ArmThroughput) -> ThroughputSettingsResource {
    match throughput {
        ArmThroughput::Autoscale {
            maximum,
            increment_percent,
            ..
        } => ThroughputSettingsResource {
            throughput: None,
            autoscale_settings: Some(ThroughputAutoscaleSettings {
                max_throughput: maximum,
                auto_upgrade_policy: increment_percent.map(|increment_percent| {
                    AutoscaleAutoUpgradePolicy {
                        throughput_policy: AutoscaleThroughputPolicy { increment_percent },
                    }
                }),
            }),
        },
        ArmThroughput::Manual(throughput) => ThroughputSettingsResource {
            throughput: Some(throughput),
            autoscale_settings: None,
        },
    }
}

fn retry_after(response: &RawResponse) -> Option<Duration> {
    response
        .headers()
        .get_optional_str(&HeaderName::from_static("retry-after"))
        .and_then(|value| value.parse::<u64>().ok())
        .map(Duration::from_secs)
}

fn operation_url(response: &RawResponse) -> Result<Url> {
    for name in ["azure-asyncoperation", "location"] {
        if let Ok(value) = response.headers().get_str(&HeaderName::from_static(name)) {
            return Ok(Url::parse(value)?);
        }
    }
    Err(azure_core::Error::with_message(
        azure_core::error::ErrorKind::DataConversion,
        "ARM 202 response did not include azure-asyncoperation or location",
    ))
}

fn ensure_success(response: &RawResponse, operation: &str) -> Result<()> {
    if response.status().is_success() {
        return Ok(());
    }
    Err(azure_core::Error::with_message(
        azure_core::error::ErrorKind::HttpResponse {
            status: response.status(),
            error_code: None,
            raw_response: None,
        },
        format!(
            "{operation} failed with HTTP {}: {}",
            response.status(),
            String::from_utf8_lossy(response.body().as_ref())
        ),
    ))
}

fn operation_status(body: &serde_json::Value) -> Option<String> {
    body.get("status")
        .and_then(serde_json::Value::as_str)
        .or_else(|| {
            body.pointer("/status/code")
                .and_then(serde_json::Value::as_str)
        })
        .or_else(|| {
            body.pointer("/properties/status")
                .and_then(serde_json::Value::as_str)
        })
        .or_else(|| {
            body.pointer("/properties/provisioningState")
                .and_then(serde_json::Value::as_str)
        })
        .map(str::to_ascii_lowercase)
}

#[cfg(test)]
mod tests {
    use super::{
        create_update_options, operation_status, throughput_settings_resource, ArmThroughput,
    };

    #[test]
    fn maps_manual_throughput() {
        let options = create_update_options(ArmThroughput::Manual(400));
        assert_eq!(options.throughput, Some(400));
        assert!(options.autoscale_settings.is_none());
    }

    #[test]
    fn maps_autoscale_throughput() {
        let options = create_update_options(ArmThroughput::Autoscale {
            maximum: 4000,
            current: Some(400),
            increment_percent: Some(25),
        });
        assert!(options.throughput.is_none());
        assert_eq!(
            options
                .autoscale_settings
                .as_ref()
                .map(|settings| settings.max_throughput),
            Some(4000)
        );
        let create_options = serde_json::to_value(&options).unwrap();
        assert!(create_options
            .pointer("/autoscaleSettings/autoUpgradePolicy")
            .is_none());
        let resource = throughput_settings_resource(ArmThroughput::Autoscale {
            maximum: 4000,
            current: Some(400),
            increment_percent: Some(25),
        });
        assert_eq!(
            resource
                .autoscale_settings
                .and_then(|settings| settings.auto_upgrade_policy)
                .map(|policy| policy.throughput_policy.increment_percent),
            Some(25)
        );
    }

    #[test]
    fn reads_nested_operation_status() {
        let body = serde_json::json!({"properties": {"provisioningState": "Succeeded"}});
        assert_eq!(operation_status(&body).as_deref(), Some("succeeded"));
    }
}
