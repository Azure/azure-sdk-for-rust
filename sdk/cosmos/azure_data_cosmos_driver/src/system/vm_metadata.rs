// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure VM metadata from the Instance Metadata Service (IMDS).

use async_lock::Mutex;
use serde::Deserialize;
use std::sync::{Arc, OnceLock};
use std::time::Duration;
use uuid::Uuid;

/// Azure Instance Metadata Service endpoint.
const IMDS_ENDPOINT: &str = "http://169.254.169.254/metadata/instance?api-version=2020-06-01";

/// Timeout for connecting to the IMDS endpoint.
///
/// IMDS is a link-local address that responds in sub-millisecond time on Azure
/// VMs. A 2-second connect timeout is generous enough for slow hosts while still
/// keeping non-Azure environments from blocking callers.
const IMDS_CONNECT_TIMEOUT: Duration = Duration::from_secs(2);

/// Overall request timeout for the IMDS fetch (connect + response).
const IMDS_REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Prefix for VM ID in machine identifiers.
pub(crate) const VM_ID_PREFIX: &str = "vmId_";

/// Prefix for generated UUID in machine identifiers (when not on Azure VM).
pub(crate) const UUID_PREFIX: &str = "uuid_";

/// Global singleton for Azure VM metadata.
static VM_METADATA: OnceLock<Arc<VmMetadataServiceInner>> = OnceLock::new();

/// Azure VM metadata retrieved from IMDS.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub(crate) struct AzureVmMetadata {
    /// Compute metadata.
    compute: ComputeMetadata,
}

impl AzureVmMetadata {
    /// Returns the Azure region/location.
    pub(crate) fn location(&self) -> &str {
        &self.compute.location
    }

    /// Returns the VM SKU.
    pub(crate) fn sku(&self) -> &str {
        &self.compute.sku
    }

    /// Returns the Azure environment (e.g., "AzurePublicCloud").
    pub(crate) fn az_environment(&self) -> &str {
        &self.compute.az_environment
    }

    /// Returns the OS type (e.g., "Linux", "Windows").
    pub(crate) fn os_type(&self) -> &str {
        &self.compute.os_type
    }

    /// Returns the VM size (e.g., "Standard_D2s_v3").
    pub(crate) fn vm_size(&self) -> &str {
        &self.compute.vm_size
    }

    /// Returns the VM ID.
    pub(crate) fn vm_id(&self) -> &str {
        &self.compute.vm_id
    }

    /// Returns the machine ID with the VM ID prefix.
    pub(crate) fn machine_id(&self) -> String {
        if self.compute.vm_id.is_empty() {
            String::new()
        } else {
            format!("{}{}", VM_ID_PREFIX, self.compute.vm_id)
        }
    }

    /// Returns the host environment info string.
    pub(crate) fn host_env_info(&self) -> String {
        format!(
            "{}|{}|{}|{}",
            self.os_type(),
            self.sku(),
            self.vm_size(),
            self.az_environment()
        )
    }
}

/// Compute-specific metadata from IMDS.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct ComputeMetadata {
    location: String,
    sku: String,
    #[serde(rename = "azEnvironment")]
    az_environment: String,
    os_type: String,
    vm_size: String,
    vm_id: String,
}

/// Handle to the VM metadata service singleton.
///
/// Provides access to cached Azure VM metadata fetched from IMDS.
/// The metadata is fetched once on first initialization and cached.
///
/// # Machine ID
///
/// The `machine_id` is always available and uniquely identifies the machine:
/// - On Azure VMs: Uses the VM ID from IMDS (prefixed with "vmId_")
/// - Off Azure: Uses a process-wide generated UUID (prefixed with "uuid_")
///
/// This ensures that client telemetry always has a stable machine identifier.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) struct VmMetadataService {
    /// Cached metadata (None if fetch failed or not on Azure).
    metadata: Option<Arc<AzureVmMetadata>>,
    /// Machine ID - always available (VM ID or generated UUID).
    machine_id: Arc<String>,
}

impl VmMetadataService {
    /// Gets or creates the VM metadata service singleton.
    ///
    /// On first call, this will attempt to fetch metadata from IMDS.
    /// This is an async operation since it uses azure_core's HTTP client.
    /// The fetch is protected by a single-flight mutex so that only one
    /// network call is ever made, even under concurrent access.
    ///
    /// This method never fails - if IMDS is unreachable, the service will
    /// still be available with a generated UUID as the machine ID.
    pub(crate) async fn get_or_init() -> Self {
        let inner = VM_METADATA.get_or_init(|| Arc::new(VmMetadataServiceInner::new()));

        // Single-flight: the mutex ensures only one task runs the fetch.
        // Other callers block on the lock and find the completed state.
        let state = inner.ensure_initialized().await;

        Self {
            metadata: state.metadata.clone(),
            machine_id: state
                .machine_id
                .clone()
                .expect("machine_id is always set after initialization"),
        }
    }

    /// Returns the cached VM metadata, if available.
    ///
    /// Returns `None` if:
    /// - The fetch failed (not running on Azure)
    /// - IMDS access is disabled
    pub(crate) fn metadata(&self) -> Option<&AzureVmMetadata> {
        self.metadata.as_deref()
    }

    /// Returns the machine ID.
    ///
    /// This is always available:
    /// - On Azure VMs: "vmId_{vm-id}" from IMDS
    /// - Off Azure: "uuid_{generated-uuid}" (stable for process lifetime)
    pub(crate) fn machine_id(&self) -> &str {
        &self.machine_id
    }

    /// Returns `true` if Azure VM metadata has been fetched successfully.
    ///
    /// Note: Even if this returns `false`, `machine_id()` is still available.
    pub(crate) fn is_on_azure(&self) -> bool {
        self.metadata.is_some()
    }
}

/// Internal state for the VM metadata service (used for async initialization).
#[derive(Debug)]
struct VmMetadataServiceInner {
    /// Mutex-protected state. The mutex provides single-flight semantics:
    /// the first task to acquire it runs the IMDS fetch, and all other tasks
    /// wait on the lock and observe the completed result.
    state: Mutex<VmMetadataState>,
}

/// Mutable state behind the single-flight mutex.
#[derive(Debug, Clone)]
struct VmMetadataState {
    /// Cached metadata (None if fetch failed or not on Azure).
    metadata: Option<Arc<AzureVmMetadata>>,
    /// Machine ID (VM ID or generated UUID). None only before first fetch.
    machine_id: Option<Arc<String>>,
    /// Whether fetch has completed (success or failure).
    fetch_complete: bool,
}

impl VmMetadataServiceInner {
    fn new() -> Self {
        Self {
            state: Mutex::new(VmMetadataState {
                metadata: None,
                machine_id: None,
                fetch_complete: false,
            }),
        }
    }

    /// Ensures the IMDS fetch has completed exactly once, then returns a
    /// snapshot of the state. Concurrent callers wait on the mutex.
    async fn ensure_initialized(&self) -> VmMetadataState {
        let mut state = self.state.lock().await;

        if state.fetch_complete {
            return state.clone();
        }

        // We hold the lock — we are the single task that runs the fetch.
        Self::do_init(&mut state).await;
        state.clone()
    }

    /// Runs the actual metadata fetch or sets a fallback. Called exactly once
    /// under the mutex.
    async fn do_init(state: &mut VmMetadataState) {
        if std::env::var("COSMOS_DISABLE_IMDS").is_ok() {
            tracing::info!("IMDS access disabled via COSMOS_DISABLE_IMDS");
            state.machine_id = Some(Arc::new(Self::generate_fallback_machine_id()));
            state.fetch_complete = true;
            return;
        }

        match Self::do_fetch().await {
            Ok(metadata) => {
                tracing::debug!("Fetched Azure VM metadata: {:?}", metadata);
                let vm_id = metadata.vm_id();
                let machine_id = if vm_id.is_empty() {
                    Self::generate_fallback_machine_id()
                } else {
                    format!("{}{}", VM_ID_PREFIX, vm_id)
                };
                state.machine_id = Some(Arc::new(machine_id));
                state.metadata = Some(Arc::new(metadata));
            }
            Err(e) => {
                tracing::debug!("Failed to fetch Azure VM metadata (not on Azure?): {}", e);
                state.machine_id = Some(Arc::new(Self::generate_fallback_machine_id()));
            }
        }

        state.fetch_complete = true;
    }

    fn generate_fallback_machine_id() -> String {
        format!("{}{}", UUID_PREFIX, Uuid::new_v4())
    }

    async fn do_fetch() -> azure_core::Result<AzureVmMetadata> {
        // Build a dedicated client with short timeouts so non-Azure hosts
        // fail fast instead of blocking callers for a full TCP timeout.
        let http_client = reqwest::Client::builder()
            .connect_timeout(IMDS_CONNECT_TIMEOUT)
            .timeout(IMDS_REQUEST_TIMEOUT)
            .build()
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Other, e))?;

        let response = http_client
            .get(IMDS_ENDPOINT)
            .header("metadata", "true")
            .send()
            .await
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Io, e))?;

        let body = response
            .text()
            .await
            .map_err(|e| azure_core::Error::new(azure_core::error::ErrorKind::Io, e))?;

        let metadata: AzureVmMetadata = serde_json::from_str(&body)?;
        Ok(metadata)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn azure_vm_metadata_deserialize() {
        let json = r#"{
            "compute": {
                "location": "eastus",
                "sku": "Standard",
                "azEnvironment": "AzurePublicCloud",
                "osType": "Linux",
                "vmSize": "Standard_D2s_v3",
                "vmId": "12345678-1234-1234-1234-123456789012"
            }
        }"#;

        let metadata: AzureVmMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(metadata.location(), "eastus");
        assert_eq!(metadata.sku(), "Standard");
        assert_eq!(metadata.az_environment(), "AzurePublicCloud");
        assert_eq!(metadata.os_type(), "Linux");
        assert_eq!(metadata.vm_size(), "Standard_D2s_v3");
        assert_eq!(metadata.vm_id(), "12345678-1234-1234-1234-123456789012");
        assert_eq!(
            metadata.machine_id(),
            "vmId_12345678-1234-1234-1234-123456789012"
        );
    }

    #[test]
    fn azure_vm_metadata_empty() {
        let metadata = AzureVmMetadata::default();
        assert_eq!(metadata.location(), "");
        assert_eq!(metadata.machine_id(), "");
    }

    #[test]
    fn azure_vm_metadata_host_env_info() {
        let json = r#"{
            "compute": {
                "osType": "Linux",
                "sku": "18.04-LTS",
                "vmSize": "Standard_D2s_v3",
                "azEnvironment": "AzurePublicCloud"
            }
        }"#;

        let metadata: AzureVmMetadata = serde_json::from_str(json).unwrap();
        assert_eq!(
            metadata.host_env_info(),
            "Linux|18.04-LTS|Standard_D2s_v3|AzurePublicCloud"
        );
    }
}
