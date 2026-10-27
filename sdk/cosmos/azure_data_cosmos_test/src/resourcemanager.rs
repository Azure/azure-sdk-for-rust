// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure Resource Manager lifecycle support for Cosmos live tests.
//!
//! The test framework depends on [`CosmosResourceManager`] rather than a
//! specific control-plane implementation. The current implementation invokes
//! Azure CLI commands directly, which keeps Resource Manager transport and
//! authentication details out of the Rust test crates.

// cspell:ignore statuscode

use async_trait::async_trait;
use azure_core::credentials::TokenCredential;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::{Display, Formatter},
    fs::OpenOptions,
    io::Write,
    path::PathBuf,
    process::ExitStatus,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::process::Command;

const DEFAULT_RESOURCE_MANAGER_ENDPOINT: &str = "https://management.azure.com/";
const RESOURCE_API_VERSION: &str = "2026-03-15";
const PARTITION_MERGE_API_VERSION: &str = "2026-04-01-preview";
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const OPERATION_TIMEOUT: Duration = Duration::from_secs(60 * 60);
const LOCK_RETRY_INTERVAL: Duration = Duration::from_secs(15);
const LOCK_RETRY_TIMEOUT: Duration = Duration::from_secs(15 * 60);

const SUBSCRIPTION_ID_ENV_VAR: &str = "COSMOS_SUBSCRIPTION_ID";
const RESOURCE_GROUP_ENV_VAR: &str = "COSMOS_RESOURCE_GROUP";
const ACCOUNT_NAME_ENV_VAR: &str = "COSMOS_ACCOUNT_NAME";
const LOCATION_ENV_VAR: &str = "COSMOS_LOCATION";
const RESOURCE_MANAGER_URL_ENV_VAR: &str = "COSMOS_RESOURCE_MANAGER_URL";
static HEADER_FILE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

/// Shared resource manager used by Cosmos live-test frameworks.
pub type ResourceManager = Arc<dyn CosmosResourceManager>;

/// Throughput configuration used by the control-plane abstraction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceThroughput {
    Manual(u64),
    Autoscale {
        maximum: u64,
        current: Option<u64>,
        increment_percent: Option<u32>,
    },
}

/// Test-only abstraction for Cosmos resource lifecycle operations.
#[async_trait]
pub trait CosmosResourceManager: Send + Sync {
    async fn create_database(&self, database_name: &str) -> Result<()>;
    async fn delete_database(&self, database_name: &str) -> Result<()>;
    async fn create_or_update_container(
        &self,
        database_name: &str,
        container_name: &str,
        resource: &serde_json::Value,
        throughput: Option<ResourceThroughput>,
    ) -> Result<()>;
    async fn delete_container(&self, database_name: &str, container_name: &str) -> Result<()>;
    async fn read_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
    ) -> Result<ResourceThroughput>;
    async fn read_database_throughput(
        &self,
        database_name: &str,
    ) -> Result<Option<ResourceThroughput>>;
    async fn replace_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
        throughput: ResourceThroughput,
    ) -> Result<ResourceThroughput>;
    async fn merge_partitions(&self, database_name: &str, container_name: &str) -> Result<()>;
}

/// Creates the configured Azure CLI resource manager.
pub fn resource_manager_from_env(credential: Arc<dyn TokenCredential>) -> Result<ResourceManager> {
    Ok(Arc::new(AzCliResourceManager::from_env(credential)?))
}

/// Error returned by Cosmos test resource management.
#[derive(Debug)]
pub struct ResourceManagerError {
    message: String,
    command_output: Option<String>,
}

impl ResourceManagerError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            command_output: None,
        }
    }

    fn command(message: impl Into<String>, output: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            command_output: Some(output.into()),
        }
    }

    fn output(&self) -> &str {
        self.command_output.as_deref().unwrap_or_default()
    }

    fn is_not_found(&self) -> bool {
        let output = self.output().to_ascii_lowercase();
        output.contains("(notfound)")
            || output.contains("\"code\":\"notfound\"")
            || output.contains("\"code\": \"notfound\"")
            || output.contains("\"status\":404")
            || output.contains("\"status\": 404")
            || output.contains("\"statuscode\":404")
            || output.contains("\"statuscode\": 404")
            || output.contains("statuscode=404")
            || output.contains("status code: 404")
            || output.contains("resource not found")
    }

    fn is_locked(&self) -> bool {
        let output = self.output().to_ascii_lowercase();
        output.contains("(locked)")
            || output.contains("\"code\":\"locked\"")
            || output.contains("\"code\": \"locked\"")
            || output.contains("\"status\":423")
            || output.contains("\"status\": 423")
            || output.contains("\"statuscode\":423")
            || output.contains("\"statuscode\": 423")
            || output.contains("statuscode=423")
            || output.contains("status code: 423")
            || output.contains("operation is in progress")
            || output.contains("another operation")
    }
}

impl Display for ResourceManagerError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.message)?;
        if let Some(output) = &self.command_output {
            write!(formatter, ": {output}")?;
        }
        Ok(())
    }
}

impl Error for ResourceManagerError {}

impl From<serde_json::Error> for ResourceManagerError {
    fn from(error: serde_json::Error) -> Self {
        Self::new(format!("failed to process Azure CLI JSON: {error}"))
    }
}

impl From<azure_core::Error> for ResourceManagerError {
    fn from(error: azure_core::Error) -> Self {
        Self::new(format!(
            "failed to acquire an Azure Resource Manager token: {error}"
        ))
    }
}

/// Result type for Cosmos test resource management.
pub type Result<T> = std::result::Result<T, ResourceManagerError>;

#[derive(Clone)]
struct ResourceManagerConfig {
    endpoint: String,
    subscription_id: String,
    resource_group: String,
    account_name: String,
    location: Option<String>,
}

/// Azure CLI implementation of [`CosmosResourceManager`].
pub struct AzCliResourceManager {
    config: ResourceManagerConfig,
    credential: Arc<dyn TokenCredential>,
    runner: Arc<dyn AzCommandRunner>,
}

impl AzCliResourceManager {
    fn from_env(credential: Arc<dyn TokenCredential>) -> Result<Self> {
        let endpoint = std::env::var(RESOURCE_MANAGER_URL_ENV_VAR)
            .unwrap_or_else(|_| DEFAULT_RESOURCE_MANAGER_ENDPOINT.to_string());
        Ok(Self::new(
            ResourceManagerConfig {
                endpoint: normalize_endpoint(&endpoint)?,
                subscription_id: required_env(SUBSCRIPTION_ID_ENV_VAR)?,
                resource_group: required_env(RESOURCE_GROUP_ENV_VAR)?,
                account_name: required_env(ACCOUNT_NAME_ENV_VAR)?,
                location: std::env::var(LOCATION_ENV_VAR)
                    .ok()
                    .filter(|value| !value.trim().is_empty()),
            },
            credential,
            Arc::new(ProcessAzCommandRunner),
        ))
    }

    fn new(
        config: ResourceManagerConfig,
        credential: Arc<dyn TokenCredential>,
        runner: Arc<dyn AzCommandRunner>,
    ) -> Self {
        Self {
            config,
            credential,
            runner,
        }
    }

    async fn rest(
        &self,
        method: &str,
        path: &str,
        api_version: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<AzRestResponse> {
        let url = format!(
            "{}{}?api-version={api_version}",
            self.config.endpoint,
            path.trim_start_matches('/')
        );
        self.rest_url(method, &url, body).await
    }

    async fn rest_url(
        &self,
        method: &str,
        url: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<AzRestResponse> {
        let scope = format!("{}.default", self.config.endpoint);
        let token = self.credential.get_token(&[&scope], None).await?;
        let headers = AuthorizationHeaderFile::new(token.token.secret())?;
        let mut args = vec![
            "rest".to_string(),
            "--method".to_string(),
            method.to_string(),
            "--url".to_string(),
            url.to_string(),
            "--headers".to_string(),
            format!("@{}", headers.path().to_string_lossy()),
            "--skip-authorization-header".to_string(),
            "--debug".to_string(),
            "--output".to_string(),
            "json".to_string(),
        ];
        if let Some(body) = body {
            args.push("--body".to_string());
            args.push(serde_json::to_string(body)?);
        }
        let output = self
            .runner
            .run("invoke Azure Resource Manager", &args)
            .await?;
        let status = debug_response_status(&output.stderr).ok_or_else(|| {
            ResourceManagerError::new(
                "Azure CLI debug output did not include the HTTP response status",
            )
        })?;
        let stdout = output.stdout.trim();
        let body = if stdout.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_str(stdout)?
        };
        Ok(AzRestResponse {
            status,
            body,
            debug_output: output.stderr,
        })
    }

    async fn resource_operation(
        &self,
        method: &str,
        path: &str,
        api_version: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<()> {
        let retry_deadline = tokio::time::Instant::now() + LOCK_RETRY_TIMEOUT;
        let operation_deadline = tokio::time::Instant::now() + OPERATION_TIMEOUT;
        loop {
            match self.rest(method, path, api_version, body).await {
                Ok(response) if response.status != 202 => return Ok(()),
                Ok(response) => {
                    let operation_url = response.operation_url()?;
                    match self
                        .wait_for_operation(&operation_url, operation_deadline)
                        .await?
                    {
                        OperationCompletion::Succeeded => return Ok(()),
                        OperationCompletion::Failed(body)
                            if is_retryable_operation_failure(&body)
                                && tokio::time::Instant::now() < retry_deadline =>
                        {
                            tokio::time::sleep(LOCK_RETRY_INTERVAL).await;
                        }
                        OperationCompletion::Failed(body) => {
                            return Err(ResourceManagerError::new(format!(
                                "Azure Resource Manager operation failed: {body}"
                            )));
                        }
                    }
                }
                Err(error) if method == "delete" && error.is_not_found() => return Ok(()),
                Err(error) if error.is_locked() && tokio::time::Instant::now() < retry_deadline => {
                    tokio::time::sleep(LOCK_RETRY_INTERVAL).await;
                }
                Err(error) => return Err(error),
            }
        }
    }

    async fn wait_for_operation(
        &self,
        operation_url: &str,
        deadline: tokio::time::Instant,
    ) -> Result<OperationCompletion> {
        loop {
            if tokio::time::Instant::now() >= deadline {
                return Err(ResourceManagerError::new(format!(
                    "Azure Resource Manager operation did not complete within {OPERATION_TIMEOUT:?}"
                )));
            }

            let response = self.rest_url("get", operation_url, None).await?;
            if response.status == 202 {
                sleep_before_deadline(response.retry_after(), deadline).await?;
                continue;
            }
            if response.body.is_null() {
                return Ok(OperationCompletion::Succeeded);
            }

            match operation_status(&response.body).as_deref() {
                Some("succeeded" | "completed") => return Ok(OperationCompletion::Succeeded),
                Some("failed" | "canceled" | "cancelled") => {
                    return Ok(OperationCompletion::Failed(response.body));
                }
                Some(_) => sleep_before_deadline(response.retry_after(), deadline).await?,
                None => return Ok(OperationCompletion::Succeeded),
            }
        }
    }

    async fn wait_for_resource(&self, path: &str, present: bool) -> Result<()> {
        let deadline = tokio::time::Instant::now() + OPERATION_TIMEOUT;
        loop {
            match self.rest("get", path, RESOURCE_API_VERSION, None).await {
                Ok(_) if !present => {}
                Ok(response) if resource_failed(&response.body) => {
                    return Err(ResourceManagerError::new(format!(
                        "Azure resource provisioning failed: {}",
                        response.body
                    )));
                }
                Ok(response) if resource_ready(&response.body) => return Ok(()),
                Ok(_) => {}
                Err(error) if error.is_not_found() && !present => return Ok(()),
                Err(error) if error.is_not_found() => {}
                Err(error) => return Err(error),
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(ResourceManagerError::new(format!(
                    "Azure resource did not become {} within {OPERATION_TIMEOUT:?}",
                    if present { "ready" } else { "deleted" }
                )));
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    fn account_path(&self) -> String {
        format!(
            "subscriptions/{}/resourceGroups/{}/providers/Microsoft.DocumentDB/databaseAccounts/{}",
            self.config.subscription_id, self.config.resource_group, self.config.account_name
        )
    }

    fn database_path(&self, database_name: &str) -> String {
        format!("{}/sqlDatabases/{database_name}", self.account_path())
    }

    fn container_path(&self, database_name: &str, container_name: &str) -> String {
        format!(
            "{}/containers/{container_name}",
            self.database_path(database_name)
        )
    }

    async fn read_throughput(&self, path: &str) -> Result<Option<ResourceThroughput>> {
        let body = match self.rest("get", path, RESOURCE_API_VERSION, None).await {
            Ok(response) => response.body,
            Err(error) if error.is_not_found() => return Ok(None),
            Err(error) => return Err(error),
        };
        let result: ThroughputSettingsGetResults = serde_json::from_value(body)?;
        let resource = result.properties.resource;
        Ok(Some(
            match (resource.throughput, resource.autoscale_settings) {
                (current, Some(settings)) => ResourceThroughput::Autoscale {
                    maximum: settings.max_throughput,
                    current,
                    increment_percent: settings
                        .auto_upgrade_policy
                        .and_then(|policy| policy.throughput_policy)
                        .and_then(|policy| policy.increment_percent),
                },
                (Some(throughput), None) => ResourceThroughput::Manual(throughput),
                (None, None) => {
                    return Err(ResourceManagerError::new(
                        "throughput response contained neither manual nor autoscale settings",
                    ));
                }
            },
        ))
    }
}

#[async_trait]
impl CosmosResourceManager for AzCliResourceManager {
    async fn create_database(&self, database_name: &str) -> Result<()> {
        let path = self.database_path(database_name);
        let body = serde_json::to_value(SqlResourceCreateUpdateParameters {
            location: self.config.location.clone(),
            properties: SqlResourceProperties {
                resource: SqlDatabaseResource { id: database_name },
                options: CreateUpdateOptions::default(),
            },
        })?;
        self.resource_operation("put", &path, RESOURCE_API_VERSION, Some(&body))
            .await?;
        self.wait_for_resource(&path, true).await
    }

    async fn delete_database(&self, database_name: &str) -> Result<()> {
        let path = self.database_path(database_name);
        self.resource_operation("delete", &path, RESOURCE_API_VERSION, None)
            .await?;
        self.wait_for_resource(&path, false).await
    }

    async fn create_or_update_container(
        &self,
        database_name: &str,
        container_name: &str,
        resource: &serde_json::Value,
        throughput: Option<ResourceThroughput>,
    ) -> Result<()> {
        let path = self.container_path(database_name, container_name);
        let body = serde_json::to_value(SqlResourceCreateUpdateParameters {
            location: self.config.location.clone(),
            properties: SqlResourceProperties {
                resource,
                options: throughput.map(create_update_options).unwrap_or_default(),
            },
        })?;
        self.resource_operation("put", &path, RESOURCE_API_VERSION, Some(&body))
            .await?;
        self.wait_for_resource(&path, true).await?;
        if let Some(
            throughput @ ResourceThroughput::Autoscale {
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

    async fn delete_container(&self, database_name: &str, container_name: &str) -> Result<()> {
        let path = self.container_path(database_name, container_name);
        self.resource_operation("delete", &path, RESOURCE_API_VERSION, None)
            .await?;
        self.wait_for_resource(&path, false).await
    }

    async fn read_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
    ) -> Result<ResourceThroughput> {
        let path = format!(
            "{}/throughputSettings/default",
            self.container_path(database_name, container_name)
        );
        self.read_throughput(&path).await?.ok_or_else(|| {
            ResourceManagerError::new("Cosmos container does not have dedicated throughput")
        })
    }

    async fn read_database_throughput(
        &self,
        database_name: &str,
    ) -> Result<Option<ResourceThroughput>> {
        let path = format!(
            "{}/throughputSettings/default",
            self.database_path(database_name)
        );
        self.read_throughput(&path).await
    }

    async fn replace_container_throughput(
        &self,
        database_name: &str,
        container_name: &str,
        throughput: ResourceThroughput,
    ) -> Result<ResourceThroughput> {
        let path = format!(
            "{}/throughputSettings/default",
            self.container_path(database_name, container_name)
        );
        let body = serde_json::to_value(ThroughputSettingsUpdateParameters {
            properties: ThroughputSettingsProperties {
                resource: throughput_settings_resource(throughput),
            },
        })?;
        self.resource_operation("put", &path, RESOURCE_API_VERSION, Some(&body))
            .await?;

        let deadline = tokio::time::Instant::now() + OPERATION_TIMEOUT;
        loop {
            let actual = self
                .read_container_throughput(database_name, container_name)
                .await?;
            if throughput_matches(throughput, actual) {
                return Ok(actual);
            }
            if tokio::time::Instant::now() >= deadline {
                return Err(ResourceManagerError::new(format!(
                    "throughput did not reach {throughput:?} within {OPERATION_TIMEOUT:?}; last value was {actual:?}"
                )));
            }
            tokio::time::sleep(POLL_INTERVAL).await;
        }
    }

    async fn merge_partitions(&self, database_name: &str, container_name: &str) -> Result<()> {
        let path = format!(
            "{}/partitionMerge",
            self.container_path(database_name, container_name)
        );
        let body = serde_json::to_value(PartitionMergeParameters { is_dry_run: false })?;
        self.resource_operation("post", &path, PARTITION_MERGE_API_VERSION, Some(&body))
            .await
    }
}

struct AzRestResponse {
    status: u16,
    body: serde_json::Value,
    debug_output: String,
}

impl AzRestResponse {
    fn operation_url(&self) -> Result<String> {
        ["azure-asyncoperation", "location"]
            .into_iter()
            .find_map(|name| debug_response_header(&self.debug_output, name))
            .ok_or_else(|| {
                ResourceManagerError::new(
                    "Azure Resource Manager 202 response did not include azure-asyncoperation or location",
                )
            })
    }

    fn retry_after(&self) -> Duration {
        debug_response_header(&self.debug_output, "retry-after")
            .and_then(|value| value.parse::<u64>().ok())
            .map(Duration::from_secs)
            .unwrap_or(POLL_INTERVAL)
    }
}

enum OperationCompletion {
    Succeeded,
    Failed(serde_json::Value),
}

struct CommandOutput {
    stdout: String,
    stderr: String,
}

struct AuthorizationHeaderFile {
    path: PathBuf,
}

impl AuthorizationHeaderFile {
    fn new(token: &str) -> Result<Self> {
        let sequence = HEADER_FILE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "azure-cosmos-test-headers-{}-{sequence}.json",
            std::process::id()
        ));
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(&path).map_err(|error| {
            ResourceManagerError::new(format!(
                "failed to create temporary Azure CLI header file: {error}"
            ))
        })?;
        let header_file = Self { path };
        serde_json::to_writer(
            &mut file,
            &serde_json::json!({"Authorization": format!("Bearer {token}")}),
        )?;
        file.flush().map_err(|error| {
            ResourceManagerError::new(format!(
                "failed to flush temporary Azure CLI header file: {error}"
            ))
        })?;
        Ok(header_file)
    }

    fn path(&self) -> &PathBuf {
        &self.path
    }
}

impl Drop for AuthorizationHeaderFile {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.path);
    }
}

#[async_trait]
trait AzCommandRunner: Send + Sync {
    async fn run(&self, operation: &str, args: &[String]) -> Result<CommandOutput>;
}

struct ProcessAzCommandRunner;

#[async_trait]
impl AzCommandRunner for ProcessAzCommandRunner {
    async fn run(&self, operation: &str, args: &[String]) -> Result<CommandOutput> {
        let output = Command::new("az")
            .args(args)
            .output()
            .await
            .map_err(|error| {
                ResourceManagerError::new(format!(
                    "failed to start Azure CLI while attempting to {operation}: {error}"
                ))
            })?;
        command_output(operation, output.status, &output.stdout, &output.stderr)
    }
}

fn command_output(
    operation: &str,
    status: ExitStatus,
    stdout: &[u8],
    stderr: &[u8],
) -> Result<CommandOutput> {
    if status.success() {
        return Ok(CommandOutput {
            stdout: String::from_utf8_lossy(stdout).into_owned(),
            stderr: String::from_utf8_lossy(stderr).into_owned(),
        });
    }
    let stdout = String::from_utf8_lossy(stdout);
    let stderr = String::from_utf8_lossy(stderr);
    let output = match (stdout.trim(), stderr.trim()) {
        ("", stderr) => stderr.to_owned(),
        (stdout, "") => stdout.to_owned(),
        (stdout, stderr) => format!("{stderr}\n{stdout}"),
    };
    Err(ResourceManagerError::command(
        format!("Azure CLI failed to {operation} with {status}"),
        output,
    ))
}

fn debug_response_status(debug_output: &str) -> Option<u16> {
    debug_output.lines().find_map(|line| {
        line.split_once("Response status: ")
            .and_then(|(_, status)| status.trim().parse().ok())
    })
}

fn debug_response_header(debug_output: &str, name: &str) -> Option<String> {
    let mut in_response_headers = false;
    for line in debug_output.lines() {
        if line.contains("Response headers:") {
            in_response_headers = true;
            continue;
        }
        if in_response_headers && line.contains("Response content:") {
            return None;
        }
        if !in_response_headers {
            continue;
        }

        let Some((_, header)) = line.split_once("    '") else {
            continue;
        };
        let Some((header_name, value)) = header.split_once("': '") else {
            continue;
        };
        if header_name.eq_ignore_ascii_case(name) {
            return value.strip_suffix('\'').map(str::to_owned);
        }
    }
    None
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

async fn sleep_before_deadline(delay: Duration, deadline: tokio::time::Instant) -> Result<()> {
    let Some(remaining) = deadline.checked_duration_since(tokio::time::Instant::now()) else {
        return Err(ResourceManagerError::new(format!(
            "Azure Resource Manager operation did not complete within {OPERATION_TIMEOUT:?}"
        )));
    };
    tokio::time::sleep(delay.min(remaining)).await;
    if delay >= remaining {
        return Err(ResourceManagerError::new(format!(
            "Azure Resource Manager operation did not complete within {OPERATION_TIMEOUT:?}"
        )));
    }
    Ok(())
}

fn required_env(name: &str) -> Result<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| ResourceManagerError::new(format!("{name} must be set")))
}

fn normalize_endpoint(endpoint: &str) -> Result<String> {
    let endpoint = endpoint.trim();
    if !(endpoint.starts_with("https://") || endpoint.starts_with("http://")) {
        return Err(ResourceManagerError::new(format!(
            "{RESOURCE_MANAGER_URL_ENV_VAR} must be an absolute HTTP(S) URL"
        )));
    }
    Ok(format!("{}/", endpoint.trim_end_matches('/')))
}

fn resource_state(body: &serde_json::Value) -> Option<&str> {
    body.pointer("/properties/provisioningState")
        .and_then(serde_json::Value::as_str)
        .or_else(|| body.get("status").and_then(serde_json::Value::as_str))
}

fn resource_ready(body: &serde_json::Value) -> bool {
    resource_state(body).is_none_or(|state| {
        state.eq_ignore_ascii_case("succeeded") || state.eq_ignore_ascii_case("completed")
    })
}

fn resource_failed(body: &serde_json::Value) -> bool {
    resource_state(body).is_some_and(|state| {
        state.eq_ignore_ascii_case("failed")
            || state.eq_ignore_ascii_case("canceled")
            || state.eq_ignore_ascii_case("cancelled")
    })
}

fn throughput_matches(expected: ResourceThroughput, actual: ResourceThroughput) -> bool {
    match (expected, actual) {
        (ResourceThroughput::Manual(expected), ResourceThroughput::Manual(actual)) => {
            expected == actual
        }
        (
            ResourceThroughput::Autoscale {
                maximum: expected,
                increment_percent: expected_increment,
                ..
            },
            ResourceThroughput::Autoscale {
                maximum: actual,
                increment_percent: actual_increment,
                ..
            },
        ) => expected == actual && expected_increment == actual_increment,
        _ => false,
    }
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

fn create_update_options(throughput: ResourceThroughput) -> CreateUpdateOptions {
    match throughput {
        ResourceThroughput::Autoscale { maximum, .. } => CreateUpdateOptions {
            throughput: None,
            autoscale_settings: Some(CreateAutoscaleSettings {
                max_throughput: maximum,
            }),
        },
        ResourceThroughput::Manual(throughput) => CreateUpdateOptions {
            throughput: Some(throughput),
            autoscale_settings: None,
        },
    }
}

fn throughput_settings_resource(throughput: ResourceThroughput) -> ThroughputSettingsResource {
    match throughput {
        ResourceThroughput::Autoscale {
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
        ResourceThroughput::Manual(throughput) => ThroughputSettingsResource {
            throughput: Some(throughput),
            autoscale_settings: None,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        create_update_options, debug_response_header, debug_response_status, normalize_endpoint,
        resource_failed, resource_ready, throughput_matches, throughput_settings_resource,
        AzCliResourceManager, AzCommandRunner, CommandOutput, CosmosResourceManager,
        ResourceManagerConfig, ResourceThroughput, Result,
    };
    use async_trait::async_trait;
    use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
    use std::{
        collections::VecDeque,
        sync::{Arc, Mutex},
    };

    struct MockRunner {
        outputs: Mutex<VecDeque<Result<CommandOutput>>>,
        calls: Mutex<Vec<Vec<String>>>,
    }

    #[derive(Debug)]
    struct MockCredential;

    #[async_trait]
    impl TokenCredential for MockCredential {
        async fn get_token(
            &self,
            _scopes: &[&str],
            _options: Option<TokenRequestOptions<'_>>,
        ) -> azure_core::Result<AccessToken> {
            Ok(AccessToken::new(
                "token",
                time::OffsetDateTime::now_utc() + time::Duration::hours(1),
            ))
        }
    }

    impl MockRunner {
        fn new(outputs: impl IntoIterator<Item = Result<CommandOutput>>) -> Self {
            Self {
                outputs: Mutex::new(outputs.into_iter().collect()),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<Vec<String>> {
            self.calls.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl AzCommandRunner for MockRunner {
        async fn run(&self, _operation: &str, args: &[String]) -> Result<CommandOutput> {
            if let Some(header_index) = args.iter().position(|argument| argument == "--headers") {
                let path = args[header_index + 1]
                    .strip_prefix('@')
                    .expect("header file argument");
                let headers: serde_json::Value =
                    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
                assert_eq!(headers["Authorization"], "Bearer token");
            }
            self.calls.lock().unwrap().push(args.to_vec());
            self.outputs
                .lock()
                .unwrap()
                .pop_front()
                .expect("mock command output")
        }
    }

    fn output(status: u16, headers: &[(&str, &str)], stdout: &str) -> Result<CommandOutput> {
        let mut stderr = format!(
            "INFO: cli.azure.cli.core.util: Response status: {status}\n\
             INFO: cli.azure.cli.core.util: Response headers:\n"
        );
        for (name, value) in headers {
            stderr.push_str(&format!(
                "INFO: cli.azure.cli.core.util:     '{name}': '{value}'\n"
            ));
        }
        stderr.push_str("INFO: cli.azure.cli.core.util: Response content:\n");
        Ok(CommandOutput {
            stdout: stdout.to_string(),
            stderr,
        })
    }

    fn config() -> ResourceManagerConfig {
        ResourceManagerConfig {
            endpoint: "https://management.azure.com/".to_string(),
            subscription_id: "subscription".to_string(),
            resource_group: "resource-group".to_string(),
            account_name: "account".to_string(),
            location: Some("eastus".to_string()),
        }
    }

    #[test]
    fn endpoint_is_normalized() {
        assert_eq!(
            normalize_endpoint("https://management.azure.com").unwrap(),
            "https://management.azure.com/"
        );
    }

    #[test]
    fn provisioning_state_is_interpreted() {
        assert!(resource_ready(&serde_json::json!({
            "properties": {"provisioningState": "Succeeded"}
        })));
        assert!(resource_failed(&serde_json::json!({
            "properties": {"provisioningState": "Failed"}
        })));
    }

    #[test]
    fn azure_cli_debug_response_is_parsed() {
        let output = output(
            202,
            &[
                (
                    "Azure-AsyncOperation",
                    "https://management.azure.com/operations/1",
                ),
                ("Retry-After", "7"),
            ],
            "{}",
        )
        .unwrap();
        assert_eq!(debug_response_status(&output.stderr), Some(202));
        assert_eq!(
            debug_response_header(&output.stderr, "azure-asyncoperation").as_deref(),
            Some("https://management.azure.com/operations/1")
        );
        assert_eq!(
            debug_response_header(&output.stderr, "retry-after").as_deref(),
            Some("7")
        );
    }

    #[test]
    fn throughput_payloads_preserve_modes() {
        let manual = create_update_options(ResourceThroughput::Manual(400));
        assert_eq!(manual.throughput, Some(400));
        assert!(manual.autoscale_settings.is_none());

        let autoscale = throughput_settings_resource(ResourceThroughput::Autoscale {
            maximum: 5000,
            current: Some(500),
            increment_percent: Some(25),
        });
        assert!(autoscale.throughput.is_none());
        let settings = autoscale.autoscale_settings.unwrap();
        assert_eq!(settings.max_throughput, 5000);
        assert_eq!(
            settings
                .auto_upgrade_policy
                .unwrap()
                .throughput_policy
                .increment_percent,
            25
        );
    }

    #[test]
    fn throughput_comparison_ignores_current_autoscale_value() {
        assert!(throughput_matches(
            ResourceThroughput::Autoscale {
                maximum: 5000,
                current: None,
                increment_percent: Some(25),
            },
            ResourceThroughput::Autoscale {
                maximum: 5000,
                current: Some(500),
                increment_percent: Some(25),
            }
        ));
    }

    #[tokio::test]
    async fn database_creation_uses_az_rest() {
        let runner = Arc::new(MockRunner::new([
            output(200, &[], "{}"),
            output(
                200,
                &[],
                r#"{"properties":{"provisioningState":"Succeeded"}}"#,
            ),
        ]));
        let manager = AzCliResourceManager::new(config(), Arc::new(MockCredential), runner.clone());

        manager.create_database("database").await.unwrap();

        let calls = runner.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(&calls[0][..3], ["rest", "--method", "put"]);
        assert!(calls[0]
            .iter()
            .any(|argument| argument.contains("/sqlDatabases/database?api-version=2026-03-15")));
        let header_index = calls[0]
            .iter()
            .position(|argument| argument == "--headers")
            .unwrap();
        assert!(calls[0][header_index + 1].starts_with('@'));
        assert!(!calls[0][header_index + 1].contains("token"));
        assert!(!std::path::Path::new(calls[0][header_index + 1].trim_start_matches('@')).exists());
        assert!(calls[0]
            .iter()
            .any(|argument| argument == "--skip-authorization-header"));
        assert!(calls[0].iter().any(|argument| argument == "--debug"));
        let body_index = calls[0]
            .iter()
            .position(|argument| argument == "--body")
            .unwrap();
        let body: serde_json::Value = serde_json::from_str(&calls[0][body_index + 1]).unwrap();
        assert_eq!(body["location"], "eastus");
        assert_eq!(body["properties"]["resource"]["id"], "database");
        assert_eq!(&calls[1][..3], ["rest", "--method", "get"]);
    }

    #[tokio::test]
    async fn database_creation_polls_async_operation() {
        let operation_url = "https://management.azure.com/operations/1?api-version=2026-03-15";
        let runner = Arc::new(MockRunner::new([
            output(202, &[("Azure-AsyncOperation", operation_url)], "{}"),
            output(200, &[], r#"{"status":"Succeeded"}"#),
            output(
                200,
                &[],
                r#"{"properties":{"provisioningState":"Succeeded"}}"#,
            ),
        ]));
        let manager = AzCliResourceManager::new(config(), Arc::new(MockCredential), runner.clone());

        manager.create_database("database").await.unwrap();

        let calls = runner.calls();
        assert_eq!(calls.len(), 3);
        let operation_url_index = calls[1]
            .iter()
            .position(|argument| argument == "--url")
            .unwrap();
        assert_eq!(calls[1][operation_url_index + 1], operation_url);
    }

    #[tokio::test]
    async fn database_creation_surfaces_async_failure() {
        let operation_url = "https://management.azure.com/operations/1?api-version=2026-03-15";
        let runner = Arc::new(MockRunner::new([
            output(202, &[("Azure-AsyncOperation", operation_url)], "{}"),
            output(
                200,
                &[],
                r#"{"status":"Failed","error":{"code":"BadRequest"}}"#,
            ),
        ]));
        let manager = AzCliResourceManager::new(config(), Arc::new(MockCredential), runner);

        let error = manager.create_database("database").await.unwrap_err();

        assert!(error
            .to_string()
            .contains("Azure Resource Manager operation failed"));
        assert!(error.to_string().contains("BadRequest"));
    }
}
