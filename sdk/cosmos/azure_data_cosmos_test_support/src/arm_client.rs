// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Resource Manager helpers for Cosmos live tests.
//!
//! Resource operations use the published Cosmos management SDK. Its 2024-09
//! container model does not represent newer properties such as
//! `vectorEmbeddingPolicy`, so only those lossless-forwarding cases use raw ARM.

use async_trait::async_trait;
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
use azure_core_0_21 as management_core;
use azure_mgmt_cosmosdb::package_preview_2024_09::{models, Client as ManagementClient};
use serde::Serialize;
use std::{future::Future, sync::Arc, time::Duration};

const DEFAULT_RESOURCE_MANAGER_ENDPOINT: &str = "https://management.azure.com/";
const RESOURCE_API_VERSION: &str = "2026-03-15";
const LRO_POLL_INTERVAL: Duration = Duration::from_secs(2);
const LRO_TIMEOUT: Duration = Duration::from_secs(60 * 60);
const LOCK_RETRY_INTERVAL: Duration = Duration::from_secs(15);
const LOCK_RETRY_TIMEOUT: Duration = Duration::from_secs(15 * 60);

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
    management_client: ManagementClient,
    pipeline: Pipeline,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArmThroughput {
    Manual(u64),
    Autoscale {
        maximum: u64,
        current: Option<u64>,
        increment_percent: Option<u32>,
    },
}

struct OperationResponse {
    status: StatusCode,
    retry_after: Option<Duration>,
    operation_url: Option<Url>,
    body: Vec<u8>,
}

enum OperationCompletion {
    Succeeded,
    Failed(serde_json::Value),
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct RawContainerCreateUpdateParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<String>,
    properties: RawContainerCreateUpdateProperties,
}

#[derive(Serialize)]
struct RawContainerCreateUpdateProperties {
    resource: serde_json::Value,
    options: models::CreateUpdateOptions,
}

#[derive(Debug)]
struct ManagementCredentialAdapter {
    credential: Arc<dyn TokenCredential>,
}

#[async_trait]
impl management_core::auth::TokenCredential for ManagementCredentialAdapter {
    async fn get_token(
        &self,
        scopes: &[&str],
    ) -> management_core::Result<management_core::auth::AccessToken> {
        let token = self
            .credential
            .get_token(scopes, None)
            .await
            .map_err(|error| {
                management_core::Error::message(
                    management_core::error::ErrorKind::Credential,
                    error.to_string(),
                )
            })?;
        Ok(management_core::auth::AccessToken::new(
            token.token.secret().to_owned(),
            token.expires_on,
        ))
    }

    async fn clear_cache(&self) -> management_core::Result<()> {
        Ok(())
    }
}

impl CosmosArmClient {
    pub fn from_env(credential: Arc<dyn TokenCredential>) -> Result<Self> {
        let endpoint = std::env::var(RESOURCE_MANAGER_URL_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_RESOURCE_MANAGER_ENDPOINT.to_string());
        let endpoint = normalize_endpoint(&endpoint)?;
        let scope = format!("{}.default", endpoint.as_str());
        let management_endpoint =
            management_core::Url::parse(endpoint.as_str()).map_err(|error| {
                azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, error)
            })?;
        let management_credential: Arc<dyn management_core::auth::TokenCredential> =
            Arc::new(ManagementCredentialAdapter {
                credential: credential.clone(),
            });
        let management_client = ManagementClient::builder(management_credential)
            .endpoint(management_endpoint)
            .scopes(&[scope.as_str()])
            .build()
            .map_err(management_error)?;

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
            management_client,
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
        let resource = models::SqlDatabaseResource::new(database_name.to_owned());
        let properties = models::SqlDatabaseCreateUpdateProperties::new(resource);
        let mut parameters = models::SqlDatabaseCreateUpdateParameters::new(properties);
        parameters.arm_resource_properties.location = self.location.clone();

        self.send_management_operation(false, || {
            self.management_client
                .sql_resources_client()
                .create_update_sql_database(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                    parameters.clone(),
                )
                .send()
        })
        .await
    }

    pub async fn delete_database(&self, database_name: &str) -> Result<()> {
        self.send_management_operation(true, || {
            self.management_client
                .sql_resources_client()
                .delete_sql_database(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                )
                .send()
        })
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
        let resource_value = serde_json::to_value(resource).map_err(data_conversion_error)?;
        let options = throughput.map(create_update_options).transpose()?;

        if let Some(resource) = management_container_resource(&resource_value)? {
            let mut properties = models::SqlContainerCreateUpdateProperties::new(resource);
            properties.options = options;
            let mut parameters = models::SqlContainerCreateUpdateParameters::new(properties);
            parameters.arm_resource_properties.location = self.location.clone();

            self.send_management_operation(false, || {
                self.management_client
                    .sql_resources_client()
                    .create_update_sql_container(
                        self.subscription_id.clone(),
                        self.resource_group.clone(),
                        self.account_name.clone(),
                        database_name.to_owned(),
                        container_name.to_owned(),
                        parameters.clone(),
                    )
                    .send()
            })
            .await?;
        } else {
            let body = RawContainerCreateUpdateParameters {
                location: self.location.clone(),
                properties: RawContainerCreateUpdateProperties {
                    resource: resource_value,
                    options: options.unwrap_or_default(),
                },
            };
            let path = self.container_path(database_name, container_name);
            self.send_raw_operation(false, || self.send_raw(Method::Put, &path, Some(&body)))
                .await?;
        }

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
        self.send_management_operation(true, || {
            self.management_client
                .sql_resources_client()
                .delete_sql_container(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                    container_name.to_owned(),
                )
                .send()
        })
        .await
    }

    pub async fn read_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
    ) -> Result<ArmThroughput> {
        ensure_legacy_request_logging_disabled()?;
        let response = management_response(
            self.management_client
                .sql_resources_client()
                .get_sql_container_throughput(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                    container_name.to_owned(),
                )
                .send()
                .await,
        )
        .await?;
        self.read_throughput(response).await?.ok_or_else(|| {
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
        ensure_legacy_request_logging_disabled()?;
        let response = management_response(
            self.management_client
                .sql_resources_client()
                .get_sql_database_throughput(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                )
                .send()
                .await,
        )
        .await?;
        self.read_throughput(response).await
    }

    async fn read_throughput(&self, response: OperationResponse) -> Result<Option<ArmThroughput>> {
        if response.status == StatusCode::NotFound {
            return Ok(None);
        }
        ensure_success(&response, "read Cosmos throughput")?;
        let result: models::ThroughputSettingsGetResults =
            serde_json::from_slice(&response.body).map_err(data_conversion_error)?;
        let resource = result
            .properties
            .and_then(|properties| properties.resource)
            .ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    "ARM throughput response did not contain a resource",
                )
            })?;
        let resource: models::ThroughputSettingsResource =
            serde_json::from_value(resource).map_err(data_conversion_error)?;

        Ok(Some(
            match (resource.throughput, resource.autoscale_settings) {
                (current, Some(settings)) => ArmThroughput::Autoscale {
                    maximum: to_u64(settings.max_throughput, "maximum throughput")?,
                    current: current
                        .map(|value| to_u64(value, "current throughput"))
                        .transpose()?,
                    increment_percent: settings
                        .auto_upgrade_policy
                        .and_then(|policy| policy.throughput_policy)
                        .and_then(|policy| policy.increment_percent)
                        .map(|value| to_u32(value, "autoscale increment percentage"))
                        .transpose()?,
                },
                (Some(throughput), None) => {
                    ArmThroughput::Manual(to_u64(throughput, "throughput")?)
                }
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
        let resource = throughput_settings_resource(throughput)?;
        let properties = models::ThroughputSettingsUpdateProperties::new(resource);
        let parameters = models::ThroughputSettingsUpdateParameters::new(properties);

        self.send_management_operation(false, || {
            self.management_client
                .sql_resources_client()
                .update_sql_container_throughput(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                    container_name.to_owned(),
                    parameters.clone(),
                )
                .send()
        })
        .await?;
        self.read_container_throughput(database_name, container_name)
            .await
    }

    pub async fn merge_partitions(&self, database_name: &str, container_name: &str) -> Result<()> {
        let mut parameters = models::MergeParameters::new();
        parameters.is_dry_run = Some(false);

        self.send_management_operation(false, || {
            self.management_client
                .sql_resources_client()
                .list_sql_container_partition_merge(
                    self.subscription_id.clone(),
                    self.resource_group.clone(),
                    self.account_name.clone(),
                    database_name.to_owned(),
                    container_name.to_owned(),
                    parameters.clone(),
                )
                .send()
        })
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

    async fn send_management_operation<F, Fut, R>(&self, delete: bool, send: F) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = management_core::Result<R>>,
        R: Into<management_core::Response>,
    {
        self.run_operation(delete, || async {
            ensure_legacy_request_logging_disabled()?;
            management_response(send().await).await
        })
        .await
    }

    async fn send_raw_operation<F, Fut>(&self, delete: bool, send: F) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<OperationResponse>>,
    {
        self.run_operation(delete, send).await
    }

    async fn run_operation<F, Fut>(&self, delete: bool, send: F) -> Result<()>
    where
        F: Fn() -> Fut,
        Fut: Future<Output = Result<OperationResponse>>,
    {
        let retry_deadline = tokio::time::Instant::now() + LOCK_RETRY_TIMEOUT;
        loop {
            let response = send().await?;
            if delete && response.status == StatusCode::NotFound {
                return Ok(());
            }
            if is_retryable_lock_response(&response) && tokio::time::Instant::now() < retry_deadline
            {
                tokio::time::sleep(response.retry_after.unwrap_or(LOCK_RETRY_INTERVAL)).await;
                continue;
            }
            ensure_success(&response, "manage Cosmos ARM resource")?;
            if response.status != StatusCode::Accepted {
                return Ok(());
            }

            let operation_url = response.operation_url.ok_or_else(|| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    "ARM 202 response did not include azure-asyncoperation or location",
                )
            })?;
            match self.wait_for_operation(operation_url).await? {
                OperationCompletion::Succeeded => return Ok(()),
                OperationCompletion::Failed(body)
                    if is_retryable_operation_failure(&body)
                        && tokio::time::Instant::now() < retry_deadline =>
                {
                    tokio::time::sleep(LOCK_RETRY_INTERVAL).await;
                }
                OperationCompletion::Failed(body) => {
                    return Err(azure_core::Error::with_message(
                        azure_core::error::ErrorKind::Other,
                        format!("ARM operation failed: {body}"),
                    ));
                }
            }
        }
    }

    async fn send_raw<T>(
        &self,
        method: Method,
        path: &str,
        body: Option<&T>,
    ) -> Result<OperationResponse>
    where
        T: Serialize + ?Sized,
    {
        let mut url = self.endpoint.join(path)?;
        url.query_pairs_mut()
            .append_pair("api-version", RESOURCE_API_VERSION);
        let mut request = Request::new(url, method);
        request.insert_header("accept", "application/json");
        if let Some(body) = body {
            request.insert_header("content-type", "application/json");
            request.set_json(body)?;
        }
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
        OperationResponse::from_current(response)
    }

    async fn wait_for_operation(&self, operation_url: Url) -> Result<OperationCompletion> {
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
            let response = OperationResponse::from_current(response)?;
            ensure_success(&response, "poll Cosmos ARM operation")?;
            if response.status == StatusCode::Accepted {
                tokio::time::sleep(response.retry_after.unwrap_or(LRO_POLL_INTERVAL)).await;
                continue;
            }
            if response.body.is_empty() {
                return Ok(OperationCompletion::Succeeded);
            }

            let body: serde_json::Value =
                serde_json::from_slice(&response.body).map_err(data_conversion_error)?;
            let status = operation_status(&body);
            match status.as_deref() {
                Some("succeeded" | "completed") => return Ok(OperationCompletion::Succeeded),
                Some("failed" | "canceled" | "cancelled") => {
                    return Ok(OperationCompletion::Failed(body));
                }
                Some(_) => {
                    tokio::time::sleep(response.retry_after.unwrap_or(LRO_POLL_INTERVAL)).await;
                }
                None => return Ok(OperationCompletion::Succeeded),
            }
        }
    }
}

impl OperationResponse {
    async fn from_management(response: management_core::Response) -> Result<Self> {
        let (status, headers, body) = response.deconstruct();
        let retry_after = headers
            .get_optional_str(&management_core::headers::HeaderName::from_static(
                "retry-after",
            ))
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs);
        let operation_url = operation_url_from_headers(|name| {
            headers
                .get_optional_str(&management_core::headers::HeaderName::from_static(name))
                .map(str::to_owned)
        })?;
        let body = body.collect().await.map_err(management_error)?.to_vec();
        Ok(Self {
            status: StatusCode::from(u16::from(status)),
            retry_after,
            operation_url,
            body,
        })
    }

    fn from_current(response: RawResponse) -> Result<Self> {
        let retry_after = response
            .headers()
            .get_optional_str(&HeaderName::from_static("retry-after"))
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs);
        let operation_url = operation_url_from_headers(|name| {
            response
                .headers()
                .get_optional_str(&HeaderName::from_static(name))
                .map(str::to_owned)
        })?;
        Ok(Self {
            status: response.status(),
            retry_after,
            operation_url,
            body: response.body().to_vec(),
        })
    }

    fn from_management_error(error: management_core::Error) -> Result<Self> {
        let management_core::error::ErrorKind::HttpResponse { status, error_code } = error.kind()
        else {
            return Err(management_error(error));
        };
        let body = error_code
            .as_deref()
            .map(|code| serde_json::json!({"error": {"code": code}}).to_string())
            .unwrap_or_default()
            .into_bytes();
        Ok(Self {
            status: StatusCode::from(u16::from(*status)),
            retry_after: None,
            operation_url: None,
            body,
        })
    }
}

async fn management_response<R>(response: management_core::Result<R>) -> Result<OperationResponse>
where
    R: Into<management_core::Response>,
{
    match response {
        Ok(response) => OperationResponse::from_management(response.into()).await,
        Err(error) => OperationResponse::from_management_error(error),
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

fn management_container_resource(
    resource: &serde_json::Value,
) -> Result<Option<models::SqlContainerResource>> {
    let Ok(model) = serde_json::from_value::<models::SqlContainerResource>(resource.clone()) else {
        return Ok(None);
    };
    let serialized = serde_json::to_value(&model).map_err(data_conversion_error)?;
    Ok((serialized == *resource).then_some(model))
}

fn is_retryable_lock_response(response: &OperationResponse) -> bool {
    if response.status == StatusCode::Locked {
        return true;
    }
    if response.status != StatusCode::Conflict {
        return false;
    }
    serde_json::from_slice(&response.body).is_ok_and(|body| is_retryable_operation_failure(&body))
}

fn is_retryable_operation_failure(body: &serde_json::Value) -> bool {
    [
        body.pointer("/error/code"),
        body.get("code"),
        body.pointer("/properties/error/code"),
    ]
    .into_iter()
    .flatten()
    .any(|code| {
        code.as_u64() == Some(423)
            || code
                .as_str()
                .is_some_and(|code| code == "423" || code.eq_ignore_ascii_case("locked"))
    })
}

fn create_update_options(throughput: ArmThroughput) -> Result<models::CreateUpdateOptions> {
    let mut options = models::CreateUpdateOptions::new();
    match throughput {
        ArmThroughput::Autoscale { maximum, .. } => {
            let mut settings = models::AutoscaleSettings::new();
            settings.max_throughput = Some(to_i64(maximum, "maximum throughput")?);
            options.autoscale_settings = Some(settings);
        }
        ArmThroughput::Manual(throughput) => {
            options.throughput = Some(to_i64(throughput, "throughput")?);
        }
    }
    Ok(options)
}

fn throughput_settings_resource(
    throughput: ArmThroughput,
) -> Result<models::ThroughputSettingsResource> {
    let mut resource = models::ThroughputSettingsResource::new();
    match throughput {
        ArmThroughput::Autoscale {
            maximum,
            increment_percent,
            ..
        } => {
            let mut settings =
                models::AutoscaleSettingsResource::new(to_i64(maximum, "maximum throughput")?);
            if let Some(increment_percent) = increment_percent {
                let mut throughput_policy = models::ThroughputPolicyResource::new();
                throughput_policy.increment_percent = Some(i64::from(increment_percent));
                let mut policy = models::AutoUpgradePolicyResource::new();
                policy.throughput_policy = Some(throughput_policy);
                settings.auto_upgrade_policy = Some(policy);
            }
            resource.autoscale_settings = Some(settings);
        }
        ArmThroughput::Manual(throughput) => {
            resource.throughput = Some(to_i64(throughput, "throughput")?);
        }
    }
    Ok(resource)
}

fn operation_url_from_headers(
    get_header: impl Fn(&'static str) -> Option<String>,
) -> Result<Option<Url>> {
    for name in ["azure-asyncoperation", "location"] {
        if let Some(value) = get_header(name) {
            return Url::parse(&value).map(Some).map_err(Into::into);
        }
    }
    Ok(None)
}

fn ensure_success(response: &OperationResponse, operation: &str) -> Result<()> {
    if response.status.is_success() {
        return Ok(());
    }
    Err(azure_core::Error::with_message(
        azure_core::error::ErrorKind::HttpResponse {
            status: response.status,
            error_code: None,
            raw_response: None,
        },
        format!(
            "{operation} failed with HTTP {}: {}",
            response.status,
            String::from_utf8_lossy(&response.body)
        ),
    ))
}

fn ensure_legacy_request_logging_disabled() -> Result<()> {
    // RUSTSEC-2026-0275: refuse legacy requests while either full request dump is enabled.
    let transport_debug_enabled = tracing::enabled!(
        target: "azure_core::policies::transport",
        tracing::Level::DEBUG
    );
    let retry_trace_enabled = tracing::enabled!(
        target: "azure_core::policies::retry_policies::retry_policy",
        tracing::Level::TRACE
    );
    if transport_debug_enabled || retry_trace_enabled {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "legacy Azure Core request logging must be disabled before using the Cosmos management client",
        ));
    }
    Ok(())
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

fn to_i64(value: u64, description: &str) -> Result<i64> {
    i64::try_from(value).map_err(|_| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("{description} exceeds the ARM integer range"),
        )
    })
}

fn to_u64(value: i64, description: &str) -> Result<u64> {
    u64::try_from(value).map_err(|_| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("{description} was negative"),
        )
    })
}

fn to_u32(value: i64, description: &str) -> Result<u32> {
    u32::try_from(value).map_err(|_| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("{description} was negative or exceeded the u32 range"),
        )
    })
}

fn management_error(error: management_core::Error) -> azure_core::Error {
    azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        format!("Cosmos management SDK failed: {error}"),
    )
}

fn data_conversion_error(error: serde_json::Error) -> azure_core::Error {
    azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, error)
}

#[cfg(test)]
mod tests {
    use super::{
        create_update_options, is_retryable_operation_failure, management_container_resource,
        management_core, models, operation_status, throughput_settings_resource, ArmThroughput,
        OperationResponse, RawContainerCreateUpdateParameters, RawContainerCreateUpdateProperties,
    };

    #[test]
    fn maps_manual_throughput() {
        let options = create_update_options(ArmThroughput::Manual(400)).unwrap();
        assert_eq!(options.throughput, Some(400));
        assert!(options.autoscale_settings.is_none());
    }

    #[test]
    fn maps_autoscale_throughput() {
        let options = create_update_options(ArmThroughput::Autoscale {
            maximum: 4000,
            current: Some(400),
            increment_percent: Some(25),
        })
        .unwrap();
        assert!(options.throughput.is_none());
        assert_eq!(
            options
                .autoscale_settings
                .as_ref()
                .and_then(|settings| settings.max_throughput),
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
        })
        .unwrap();
        assert_eq!(
            resource
                .autoscale_settings
                .and_then(|settings| settings.auto_upgrade_policy)
                .and_then(|policy| policy.throughput_policy)
                .and_then(|policy| policy.increment_percent),
            Some(25)
        );
    }

    #[test]
    fn uses_generated_container_model_when_lossless() {
        let resource = serde_json::json!({
            "id": "items",
            "partitionKey": {
                "paths": ["/partition_key"],
                "kind": "Hash",
                "version": 2
            }
        });
        assert!(management_container_resource(&resource).unwrap().is_some());
    }

    #[test]
    fn preserves_newer_container_fields_with_fallback() {
        let resource = serde_json::json!({
            "id": "items",
            "partitionKey": {
                "paths": ["/partition_key"],
                "kind": "Hash",
                "version": 2
            },
            "vectorEmbeddingPolicy": {
                "vectorEmbeddings": []
            }
        });
        assert!(management_container_resource(&resource).unwrap().is_none());
    }

    #[test]
    fn raw_container_parameters_omit_missing_location() {
        let body = serde_json::to_value(RawContainerCreateUpdateParameters {
            location: None,
            properties: RawContainerCreateUpdateProperties {
                resource: serde_json::json!({"id": "items"}),
                options: models::CreateUpdateOptions::default(),
            },
        })
        .unwrap();
        assert!(body.get("location").is_none());
        assert_eq!(
            body.pointer("/properties/options"),
            Some(&serde_json::json!({}))
        );
    }

    #[test]
    fn reads_nested_operation_status() {
        let body = serde_json::json!({"properties": {"provisioningState": "Succeeded"}});
        assert_eq!(operation_status(&body).as_deref(), Some("succeeded"));
    }

    #[test]
    fn recognizes_retryable_locked_operation() {
        let body = serde_json::json!({
            "status": "Failed",
            "error": {
                "code": "423",
                "message": "There is another scale-up operation currently in progress."
            }
        });
        assert!(is_retryable_operation_failure(&body));
    }

    #[test]
    fn preserves_management_http_error_status() {
        let error = management_core::error::ErrorKind::http_response(
            management_core::StatusCode::NotFound,
            None,
        )
        .into_error();
        let response = OperationResponse::from_management_error(error).unwrap();
        assert_eq!(response.status, azure_core::http::StatusCode::NotFound);
    }

    #[test]
    fn preserves_management_http_error_code() {
        let error = management_core::error::ErrorKind::http_response(
            management_core::StatusCode::Conflict,
            Some("423".to_owned()),
        )
        .into_error();
        let response = OperationResponse::from_management_error(error).unwrap();
        let body: serde_json::Value = serde_json::from_slice(&response.body).unwrap();
        assert!(is_retryable_operation_failure(&body));
    }
}
