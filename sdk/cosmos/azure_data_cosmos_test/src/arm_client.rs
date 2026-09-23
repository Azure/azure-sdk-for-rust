// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Typed Azure Resource Manager client helpers for Cosmos live tests.
//!
//! This test-only client uses the current Azure Core stack instead of the legacy
//! generated Cosmos management SDK dependency graph. The private wire types are
//! an intentional test-only exception until Cosmos ARM generation uses current
//! Azure Core; transport-level tests pin the required ARM contract.

use azure_core::{
    credentials::TokenCredential,
    http::{
        headers::{HeaderName, ERROR_CODE},
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
    throughput: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscale_settings: Option<CreateAutoscaleSettings>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateAutoscaleSettings {
    max_throughput: u64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleAutoUpgradePolicy {
    throughput_policy: AutoscaleThroughputPolicy,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleThroughputPolicy {
    increment_percent: u32,
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
    throughput: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    autoscale_settings: Option<ThroughputAutoscaleSettings>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ThroughputAutoscaleSettings {
    max_throughput: u64,
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
    throughput: Option<u64>,
    autoscale_settings: Option<AutoscaleSettingsResult>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct AutoscaleSettingsResult {
    max_throughput: u64,
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
    increment_percent: Option<u32>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct PartitionMergeParameters {
    is_dry_run: bool,
}

enum OperationCompletion {
    Succeeded,
    Failed(serde_json::Value),
}

impl CosmosArmClient {
    pub fn from_env(credential: Arc<dyn TokenCredential>) -> Result<Self> {
        let endpoint = std::env::var(RESOURCE_MANAGER_URL_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_RESOURCE_MANAGER_ENDPOINT.to_string());
        let endpoint = normalize_endpoint(&endpoint)?;
        Self::new(
            endpoint,
            required_env(SUBSCRIPTION_ID_ENV_VAR)?,
            required_env(RESOURCE_GROUP_ENV_VAR)?,
            required_env(ACCOUNT_NAME_ENV_VAR)?,
            std::env::var(LOCATION_ENV_VAR)
                .ok()
                .filter(|value| !value.trim().is_empty()),
            credential,
            ClientOptions::default(),
        )
    }

    fn new(
        endpoint: Url,
        subscription_id: String,
        resource_group: String,
        account_name: String,
        location: Option<String>,
        credential: Arc<dyn TokenCredential>,
        client_options: ClientOptions,
    ) -> Result<Self> {
        let scope = format!("{}.default", endpoint.as_str());
        let auth_policy: Arc<dyn Policy> =
            Arc::new(BearerTokenAuthorizationPolicy::new(credential, [scope]));

        Ok(Self {
            endpoint,
            subscription_id,
            resource_group,
            account_name,
            location,
            pipeline: Pipeline::new(
                option_env!("CARGO_PKG_NAME"),
                option_env!("CARGO_PKG_VERSION"),
                client_options,
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
        ensure_success(&response, "read Cosmos throughput")?;
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
                        .and_then(|policy| policy.increment_percent),
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

    fn operation_failed_error(body: serde_json::Value) -> azure_core::Error {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!("ARM operation failed: {body}"),
        )
    }

    fn operation_timeout_error() -> azure_core::Error {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            format!("ARM operation did not complete within {LRO_TIMEOUT:?}"),
        )
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
        let retry_deadline = tokio::time::Instant::now() + LOCK_RETRY_TIMEOUT;
        let operation_deadline = tokio::time::Instant::now() + LRO_TIMEOUT;
        loop {
            let response = self
                .send_before_deadline(method, path, api_version, body, operation_deadline)
                .await?;
            if method == Method::Delete && response.status() == StatusCode::NotFound {
                return Ok(());
            }
            if is_retryable_lock_response(&response) && tokio::time::Instant::now() < retry_deadline
            {
                let deadline = retry_deadline.min(operation_deadline);
                if sleep_with_deadline(
                    retry_after(&response).unwrap_or(LOCK_RETRY_INTERVAL),
                    deadline,
                )
                .await
                {
                    continue;
                }
            }
            ensure_success(&response, "manage Cosmos ARM resource")?;
            if response.status() != StatusCode::Accepted {
                return Ok(());
            }

            let operation_url = operation_url(&response)?;
            match self
                .wait_for_operation(operation_url, operation_deadline)
                .await?
            {
                OperationCompletion::Succeeded => return Ok(()),
                OperationCompletion::Failed(body)
                    if is_retryable_operation_failure(&body)
                        && tokio::time::Instant::now() < retry_deadline =>
                {
                    if !sleep_with_deadline(
                        LOCK_RETRY_INTERVAL,
                        retry_deadline.min(operation_deadline),
                    )
                    .await
                    {
                        return Err(Self::operation_failed_error(body));
                    }
                }
                OperationCompletion::Failed(body) => {
                    return Err(Self::operation_failed_error(body));
                }
            }
        }
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

    async fn send_before_deadline<T>(
        &self,
        method: Method,
        path: &str,
        api_version: &str,
        body: Option<&T>,
        deadline: tokio::time::Instant,
    ) -> Result<RawResponse>
    where
        T: Serialize + ?Sized,
    {
        tokio::time::timeout_at(deadline, self.send(method, path, api_version, body))
            .await
            .map_err(|_| Self::operation_timeout_error())?
    }

    async fn wait_for_operation(
        &self,
        operation_url: Url,
        deadline: tokio::time::Instant,
    ) -> Result<OperationCompletion> {
        loop {
            if tokio::time::Instant::now() >= deadline {
                return Err(Self::operation_timeout_error());
            }

            let mut request = Request::new(operation_url.clone(), Method::Get);
            request.insert_header("accept", "application/json");
            let response = tokio::time::timeout_at(
                deadline,
                self.pipeline.send(
                    &azure_core::http::Context::new(),
                    &mut request,
                    Some(PipelineSendOptions {
                        skip_checks: true,
                        ..Default::default()
                    }),
                ),
            )
            .await
            .map_err(|_| Self::operation_timeout_error())??;
            ensure_success(&response, "poll Cosmos ARM operation")?;
            if response.status() == StatusCode::Accepted {
                if !sleep_with_deadline(
                    retry_after(&response).unwrap_or(LRO_POLL_INTERVAL),
                    deadline,
                )
                .await
                {
                    return Err(Self::operation_timeout_error());
                }
                continue;
            }
            if response.body().is_empty() {
                return Ok(OperationCompletion::Succeeded);
            }

            let body: serde_json::Value = response.body().json()?;
            let status = operation_status(&body);
            match status.as_deref() {
                Some("succeeded" | "completed") => return Ok(OperationCompletion::Succeeded),
                Some("failed" | "canceled" | "cancelled") => {
                    return Ok(OperationCompletion::Failed(body));
                }
                Some(_) => {
                    if !sleep_with_deadline(
                        retry_after(&response).unwrap_or(LRO_POLL_INTERVAL),
                        deadline,
                    )
                    .await
                    {
                        return Err(Self::operation_timeout_error());
                    }
                }
                None => return Ok(OperationCompletion::Succeeded),
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

fn is_retryable_lock_response(response: &RawResponse) -> bool {
    if response.status() == StatusCode::Locked {
        return true;
    }
    if response.status() != StatusCode::Conflict {
        return false;
    }
    if response
        .headers()
        .get_optional_str(&ERROR_CODE)
        .is_some_and(is_retryable_lock_code)
    {
        return true;
    }
    serde_json::from_slice(response.body().as_ref())
        .is_ok_and(|body| is_retryable_operation_failure(&body))
}

fn is_retryable_lock_code(code: &str) -> bool {
    code == "423" || code.eq_ignore_ascii_case("locked")
}

fn is_retryable_operation_failure(body: &serde_json::Value) -> bool {
    [
        body.pointer("/error/code"),
        body.get("code"),
        body.pointer("/properties/error/code"),
    ]
    .into_iter()
    .flatten()
    .any(|code| code.as_u64() == Some(423) || code.as_str().is_some_and(is_retryable_lock_code))
}

async fn sleep_with_deadline(delay: Duration, deadline: tokio::time::Instant) -> bool {
    let Some(remaining) = deadline.checked_duration_since(tokio::time::Instant::now()) else {
        return false;
    };
    let sleep = delay.min(remaining);
    tokio::time::sleep(sleep).await;
    sleep < remaining
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
            error_code: response
                .headers()
                .get_optional_str(&ERROR_CODE)
                .map(str::to_owned),
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
        create_update_options, is_retryable_operation_failure, normalize_endpoint,
        operation_status, sleep_with_deadline, throughput_settings_resource, ArmThroughput,
        AutoscaleThroughputPolicyResult, CosmosArmClient, CreateUpdateOptions,
        SqlResourceCreateUpdateParameters, SqlResourceProperties,
    };
    use azure_core::{
        http::{
            headers::{HeaderName, Headers, AUTHORIZATION, ERROR_CODE},
            AsyncRawResponse, Body, ClientOptions, Method, StatusCode, Transport,
        },
        Bytes,
    };
    use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
    use futures::FutureExt as _;
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
        time::Duration,
    };

    const ASYNC_OPERATION: HeaderName = HeaderName::from_static("azure-asyncoperation");
    const RETRY_AFTER: HeaderName = HeaderName::from_static("retry-after");

    #[derive(Debug, PartialEq)]
    struct RecordedRequest {
        method: Method,
        path_and_query: String,
        body: Option<serde_json::Value>,
        authorization: Option<String>,
    }

    struct MockResponse {
        status: StatusCode,
        headers: Vec<(HeaderName, &'static str)>,
        body: Bytes,
        delay: Duration,
    }

    fn response(
        status: StatusCode,
        headers: impl IntoIterator<Item = (HeaderName, &'static str)>,
        body: impl Into<Bytes>,
    ) -> MockResponse {
        MockResponse {
            status,
            headers: headers.into_iter().collect(),
            body: body.into(),
            delay: Duration::ZERO,
        }
    }

    fn delayed_response(status: StatusCode, delay: Duration) -> MockResponse {
        MockResponse {
            status,
            headers: Vec::new(),
            body: Bytes::new(),
            delay,
        }
    }

    fn test_client(
        responses: impl IntoIterator<Item = MockResponse>,
    ) -> (CosmosArmClient, Arc<Mutex<Vec<RecordedRequest>>>) {
        let mut responses: VecDeque<_> = responses.into_iter().collect();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let captured_requests = requests.clone();
        let transport = Transport::new(Arc::new(MockHttpClient::new(move |request| {
            let body = match request.body() {
                Body::Bytes(body) if !body.is_empty() => {
                    Some(serde_json::from_slice(body).expect("request body should be JSON"))
                }
                _ => None,
            };
            captured_requests.lock().unwrap().push(RecordedRequest {
                method: request.method(),
                path_and_query: request.path_and_query(),
                body,
                authorization: request
                    .headers()
                    .get_optional_str(&AUTHORIZATION)
                    .map(str::to_owned),
            });
            let response = responses.pop_front().expect("unexpected ARM request");
            let mut headers = Headers::new();
            for (name, value) in response.headers {
                headers.insert(name, value);
            }
            async move {
                tokio::time::sleep(response.delay).await;
                Ok(AsyncRawResponse::from_bytes(
                    response.status,
                    headers,
                    response.body,
                ))
            }
            .boxed()
        })));
        let client = CosmosArmClient::new(
            normalize_endpoint("https://management.example").unwrap(),
            "subscription".into(),
            "resource-group".into(),
            "account".into(),
            None,
            MockCredential::new().unwrap(),
            ClientOptions {
                transport: Some(transport),
                ..Default::default()
            },
        )
        .unwrap();
        (client, requests)
    }

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
    fn reads_missing_or_null_increment_percent() {
        for value in [
            serde_json::json!({}),
            serde_json::json!({"incrementPercent": null}),
        ] {
            let policy: AutoscaleThroughputPolicyResult = serde_json::from_value(value)
                .expect("optional incrementPercent should deserialize");
            assert_eq!(policy.increment_percent, None);
        }
    }

    #[test]
    fn resource_parameters_include_empty_options() {
        let parameters = SqlResourceCreateUpdateParameters {
            location: None,
            properties: SqlResourceProperties {
                resource: serde_json::json!({"id": "container"}),
                options: CreateUpdateOptions::default(),
            },
        };

        let body = serde_json::to_value(parameters).unwrap();
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

    #[tokio::test]
    async fn retries_header_only_conflict_and_preserves_request_shape() {
        let (client, requests) = test_client([
            response(
                StatusCode::Conflict,
                [(ERROR_CODE, "423"), (RETRY_AFTER, "0")],
                Bytes::new(),
            ),
            response(StatusCode::Ok, [], Bytes::new()),
        ]);

        client.create_database("database").await.unwrap();

        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0], requests[1]);
        assert_eq!(
            requests[0],
            RecordedRequest {
                method: Method::Put,
                path_and_query: "/subscriptions/subscription/resourceGroups/resource-group/providers/Microsoft.DocumentDB/databaseAccounts/account/sqlDatabases/database?api-version=2026-03-15".into(),
                body: Some(serde_json::json!({
                    "properties": {
                        "resource": {"id": "database"},
                        "options": {}
                    }
                })),
                authorization: Some(
                    "Bearer TEST TOKEN https://management.example/.default".into()
                ),
            }
        );
    }

    #[tokio::test]
    async fn delete_not_found_is_idempotent() {
        let (client, requests) = test_client([response(StatusCode::NotFound, [], Bytes::new())]);

        client.delete_database("database").await.unwrap();

        let requests = requests.lock().unwrap();
        assert_eq!(
            requests.as_slice(),
            [RecordedRequest {
                method: Method::Delete,
                path_and_query: "/subscriptions/subscription/resourceGroups/resource-group/providers/Microsoft.DocumentDB/databaseAccounts/account/sqlDatabases/database?api-version=2026-03-15".into(),
                body: None,
                authorization: Some(
                    "Bearer TEST TOKEN https://management.example/.default".into()
                ),
            }]
        );
    }

    #[tokio::test]
    async fn polls_async_operation_without_resetting_request_contract() {
        let (client, requests) = test_client([
            response(
                StatusCode::Accepted,
                [(ASYNC_OPERATION, "https://management.example/operations/1")],
                Bytes::new(),
            ),
            response(
                StatusCode::Ok,
                [],
                Bytes::from_static(br#"{"status":"Succeeded"}"#),
            ),
        ]);

        client.create_database("database").await.unwrap();

        let requests = requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].method, Method::Put);
        assert_eq!(
            requests[1],
            RecordedRequest {
                method: Method::Get,
                path_and_query: "/operations/1".into(),
                body: None,
                authorization: Some("Bearer TEST TOKEN https://management.example/.default".into()),
            }
        );
    }

    #[tokio::test]
    async fn partition_merge_uses_preview_contract() {
        let (client, requests) = test_client([response(StatusCode::Ok, [], Bytes::new())]);

        client
            .merge_partitions("database", "container")
            .await
            .unwrap();

        let requests = requests.lock().unwrap();
        assert_eq!(
            requests.as_slice(),
            [RecordedRequest {
                method: Method::Post,
                path_and_query: "/subscriptions/subscription/resourceGroups/resource-group/providers/Microsoft.DocumentDB/databaseAccounts/account/sqlDatabases/database/containers/container/partitionMerge?api-version=2026-04-01-preview".into(),
                body: Some(serde_json::json!({"isDryRun": false})),
                authorization: Some(
                    "Bearer TEST TOKEN https://management.example/.default".into()
                ),
            }]
        );
    }

    #[tokio::test(start_paused = true)]
    async fn clamps_sleep_to_deadline() {
        let start = tokio::time::Instant::now();
        let deadline = start + Duration::from_secs(5);

        assert!(!sleep_with_deadline(Duration::from_secs(60), deadline).await);
        assert_eq!(tokio::time::Instant::now(), deadline);
    }

    #[tokio::test(start_paused = true)]
    async fn bounds_transport_request_by_operation_deadline() {
        let (client, requests) = test_client([delayed_response(
            StatusCode::Ok,
            Duration::from_secs(2 * 60 * 60),
        )]);

        let error = client.create_database("database").await.unwrap_err();

        assert!(error
            .to_string()
            .contains("ARM operation did not complete within 3600s"));
        assert_eq!(requests.lock().unwrap().len(), 1);
    }
}
