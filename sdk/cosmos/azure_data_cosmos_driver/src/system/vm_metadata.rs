// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure VM metadata from the Instance Metadata Service (IMDS).

use azure_core::http::{new_http_client, Method, Request};
use serde::Deserialize;
use std::sync::{Arc, OnceLock, RwLock};
use url::Url;
use uuid::Uuid;

/// Azure Instance Metadata Service endpoint.
const IMDS_ENDPOINT: &str = "http://169.254.169.254/metadata/instance?api-version=2020-06-01";

/// Prefix for VM ID in machine identifiers.
pub const VM_ID_PREFIX: &str = "vmId_";

/// Prefix for generated UUID in machine identifiers (when not on Azure VM).
pub const UUID_PREFIX: &str = "uuid_";

/// Global singleton for Azure VM metadata.
static VM_METADATA: OnceLock<Arc<VmMetadataServiceInner>> = OnceLock::new();

/// Azure VM metadata retrieved from IMDS.
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(default)]
pub struct AzureVmMetadata {
    /// Compute metadata.
    compute: ComputeMetadata,
}

impl AzureVmMetadata {
    /// Returns the Azure region/location.
    pub fn location(&self) -> &str {
        &self.compute.location
    }

    /// Returns the VM SKU.
    pub fn sku(&self) -> &str {
        &self.compute.sku
    }

    /// Returns the Azure environment (e.g., "AzurePublicCloud").
    pub fn az_environment(&self) -> &str {
        &self.compute.az_environment
    }

    /// Returns the OS type (e.g., "Linux", "Windows").
    pub fn os_type(&self) -> &str {
        &self.compute.os_type
    }

    /// Returns the VM size (e.g., "Standard_D2s_v3").
    pub fn vm_size(&self) -> &str {
        &self.compute.vm_size
    }

    /// Returns the VM ID.
    pub fn vm_id(&self) -> &str {
        &self.compute.vm_id
    }

    /// Returns the machine ID with the VM ID prefix.
    pub fn machine_id(&self) -> String {
        if self.compute.vm_id.is_empty() {
            String::new()
        } else {
            format!("{}{}", VM_ID_PREFIX, self.compute.vm_id)
        }
    }

    /// Returns the host environment info string.
    pub fn host_env_info(&self) -> String {
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
pub struct VmMetadataService {
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
    ///
    /// This method never fails - if IMDS is unreachable, the service will
    /// still be available with a generated UUID as the machine ID.
    pub async fn get_or_init() -> Self {
        // Use OnceLock to ensure we only fetch once
        let inner = VM_METADATA.get_or_init(|| Arc::new(VmMetadataServiceInner::new()));

        // Check if we need to fetch metadata
        if !inner.is_fetch_complete() {
            // Fetch metadata (this will be a no-op if already fetched by another task)
            inner.fetch_metadata().await;
        }

        // Extract the cached metadata and machine ID
        let metadata = inner.get_metadata();
        let machine_id = inner.get_machine_id();

        Self {
            metadata,
            machine_id,
        }
    }

    /// Returns the cached VM metadata, if available.
    ///
    /// Returns `None` if:
    /// - The fetch failed (not running on Azure)
    /// - IMDS access is disabled
    pub fn metadata(&self) -> Option<&AzureVmMetadata> {
        self.metadata.as_deref()
    }

    /// Returns the machine ID.
    ///
    /// This is always available:
    /// - On Azure VMs: "vmId_{vm-id}" from IMDS
    /// - Off Azure: "uuid_{generated-uuid}" (stable for process lifetime)
    pub fn machine_id(&self) -> &str {
        &self.machine_id
    }

    /// Returns `true` if Azure VM metadata has been fetched successfully.
    ///
    /// Note: Even if this returns `false`, `machine_id()` is still available.
    pub fn is_on_azure(&self) -> bool {
        self.metadata.is_some()
    }
}

/// Internal state for the VM metadata service (used for async initialization).
#[derive(Debug)]
struct VmMetadataServiceInner {
    /// Cached metadata.
    metadata: RwLock<Option<Arc<AzureVmMetadata>>>,
    /// Machine ID (VM ID or generated UUID).
    machine_id: RwLock<Option<Arc<String>>>,
    /// Whether fetch has completed (success or failure).
    fetch_complete: RwLock<bool>,
}

impl VmMetadataServiceInner {
    fn new() -> Self {
        Self {
            metadata: RwLock::new(None),
            machine_id: RwLock::new(None),
            fetch_complete: RwLock::new(false),
        }
    }

    fn is_fetch_complete(&self) -> bool {
        *self.fetch_complete.read().unwrap()
    }

    fn get_metadata(&self) -> Option<Arc<AzureVmMetadata>> {
        self.metadata.read().unwrap().clone()
    }

    fn get_machine_id(&self) -> Arc<String> {
        self.machine_id
            .read()
            .unwrap()
            .clone()
            .expect("machine_id should be set after fetch completes")
    }

    async fn fetch_metadata(&self) {
        // Check if already fetched (race condition protection)
        {
            let complete = self.fetch_complete.read().unwrap();
            if *complete {
                return;
            }
        }

        // Check if IMDS access is disabled via environment variable
        if std::env::var("COSMOS_DISABLE_IMDS").is_ok() {
            tracing::info!("IMDS access disabled via COSMOS_DISABLE_IMDS");
            self.set_fallback_machine_id();
            *self.fetch_complete.write().unwrap() = true;
            return;
        }

        let result = Self::do_fetch().await;

        match result {
            Ok(metadata) => {
                tracing::debug!("Fetched Azure VM metadata: {:?}", metadata);
                let vm_id = metadata.vm_id();
                let machine_id = if vm_id.is_empty() {
                    // VM ID is empty, use fallback
                    self.generate_fallback_machine_id()
                } else {
                    format!("{}{}", VM_ID_PREFIX, vm_id)
                };
                *self.machine_id.write().unwrap() = Some(Arc::new(machine_id));
                *self.metadata.write().unwrap() = Some(Arc::new(metadata));
            }
            Err(e) => {
                tracing::debug!("Failed to fetch Azure VM metadata (not on Azure?): {}", e);
                self.set_fallback_machine_id();
            }
        }

        *self.fetch_complete.write().unwrap() = true;
    }

    fn set_fallback_machine_id(&self) {
        let machine_id = self.generate_fallback_machine_id();
        *self.machine_id.write().unwrap() = Some(Arc::new(machine_id));
    }

    fn generate_fallback_machine_id(&self) -> String {
        format!("{}{}", UUID_PREFIX, Uuid::new_v4())
    }

    async fn do_fetch() -> azure_core::Result<AzureVmMetadata> {
        let url: Url = IMDS_ENDPOINT.parse().expect("valid IMDS URL");
        let mut request = Request::new(url, Method::Get);
        request.insert_header("metadata", "true");

        let http_client = new_http_client();
        let response = http_client.execute_request(&request).await?;
        let body = response.into_body().collect_string().await?;
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
