//! Global endpoint routing abstractions.
//!
//! This module mirrors (in shape only) the C# `IGlobalEndpointManager` interface used
//! in the Azure Cosmos DB .NET SDK. Here we introduce a Rust trait `IGlobalEndpointManager`
//! with the same method names and high-level semantics. The goal is to enable incremental
//! porting while keeping naming recognizable. Once fully ported, the API can be made
//! idiomatic (snake_case) and integrated more tightly with the rest of the crate.
//!
//! NOTE: All method names intentionally retain their original PascalCase per user request.
//! They are non-idiomatic Rust; `#[allow(non_camel_case_types)]` / `#[allow(non_snake_case)]`
//! are applied to suppress lint warnings.

use std::{collections::HashMap, sync::Arc};
use url::Url;
use azure_core::Result as AzureResult;

/// Placeholder for a low-level service request (mirrors `DocumentServiceRequest`).
/// In a future implementation this will likely wrap HTTP method, resource type,
/// operation type, headers, payload, and context.
#[derive(Debug)]
pub struct DocumentServiceRequest {
    pub resource_type: ResourceType,
    pub operation_type: OperationType,
    pub path: String,
}

/// Placeholder account properties (mirrors `AccountProperties`).
#[derive(Debug, Clone)]
pub struct AccountProperties {
    /// Whether multiple write locations are enabled.
    pub enable_multiple_write_locations: bool,
    /// Ordered list of readable region endpoints.
    pub readable_locations: Vec<String>,
    /// Ordered list of writable region endpoints.
    pub writable_locations: Vec<String>,
}

/// Resource type enumeration (subset / placeholder).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ResourceType {
    Databases,
    Containers,
    Items,
    Other,
}

/// Operation type enumeration (subset / placeholder).
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperationType {
    Create,
    Read,
    Replace,
    Delete,
    Query,
    Upsert,
    Patch,
    Other,
}

/// Type alias matching C# `ReadOnlyCollection<Uri>` semantics in a lightweight form.
/// We use `Arc<[Url]>` for cheap cloning and slice immutability.
pub type ReadOnlyUrlCollection = Arc<[Url]>;

/// Type alias representing `ReadOnlyDictionary<string, Uri>`.
pub type ReadOnlyLocationMap = Arc<HashMap<String, Url>>;

/// Represents a global endpoint manager abstraction.
///
/// Implementations are expected to be thread-safe (`Send + Sync`) so they can be shared
/// across client clones. Drop acts as the equivalent of `IDisposable.Dispose`.
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub trait IGlobalEndpointManager: Send + Sync {
    // Properties (exposed as methods returning read-only views)
    fn ReadEndpoints(&self) -> Vec<String>;
    fn AccountReadEndpoints(&self) -> Vec<String>;
    fn WriteEndpoints(&self) -> Vec<String>;
    fn ThinClientReadEndpoints(&self) -> Vec<String>;
    fn ThinClientWriteEndpoints(&self) -> Vec<String>;
    fn PreferredLocationCount(&self) -> i32;

    // Core behaviors
    fn ResolveServiceEndpoint(&self, request: &DocumentServiceRequest) -> String;
    fn GetLocation(&self, endpoint: &Url) -> Option<String>;
    fn MarkEndpointUnavailableForRead(&self, endpoint: &Url);
    fn MarkEndpointUnavailableForWrite(&self, endpoint: &Url);
    fn CanUseMultipleWriteLocations(&self, request: &DocumentServiceRequest) -> bool;
    fn InitializeAccountPropertiesAndStartBackgroundRefresh(&self, account: AccountProperties);
    fn RefreshLocationAsync<'a>(
        &'a self,
        forceRefresh: bool,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AzureResult<()>> + Send + 'a>>;
    fn GetAvailableWriteEndpointsByLocation(&self) -> ReadOnlyLocationMap;
    fn GetAvailableReadEndpointsByLocation(&self) -> ReadOnlyLocationMap;
    fn CanSupportMultipleWriteLocations(
        &self,
        resource_type: ResourceType,
        operation_type: OperationType,
    ) -> bool;
}

/// A no-op implementation useful for tests / scaffolding.
#[derive(Debug, Default)]
pub struct NoopGlobalEndpointManager;

#[allow(non_snake_case)]
impl IGlobalEndpointManager for NoopGlobalEndpointManager {
    fn ReadEndpoints(&self) -> Vec<String> { Vec::new() }
    fn AccountReadEndpoints(&self) -> Vec<String> { self.ReadEndpoints() }
    fn WriteEndpoints(&self) -> Vec<String> { self.ReadEndpoints() }
    fn ThinClientReadEndpoints(&self) -> Vec<String> { self.ReadEndpoints() }
    fn ThinClientWriteEndpoints(&self) -> Vec<String> { self.ReadEndpoints() }
    fn PreferredLocationCount(&self) -> i32 { 0 }

    fn ResolveServiceEndpoint(&self, _request: &DocumentServiceRequest) -> String {
        // Return a dummy URL; real implementation would select based on read/write preferences.
        // Ok(Url::parse("https://localhost")?)
        String::from("https://localhost")
    }
    fn GetLocation(&self, _endpoint: &Url) -> Option<String> { None }
    fn MarkEndpointUnavailableForRead(&self, _endpoint: &Url) {}
    fn MarkEndpointUnavailableForWrite(&self, _endpoint: &Url) {}
    fn CanUseMultipleWriteLocations(&self, _request: &DocumentServiceRequest) -> bool { false }
    fn InitializeAccountPropertiesAndStartBackgroundRefresh(&self, _account: AccountProperties) {}
    fn RefreshLocationAsync<'a>(
        &'a self,
        _forceRefresh: bool,
    ) -> core::pin::Pin<Box<dyn core::future::Future<Output = AzureResult<()>> + Send + 'a>> {
        Box::pin(async { Ok(()) })
    }
    fn GetAvailableWriteEndpointsByLocation(&self) -> ReadOnlyLocationMap { Arc::new(HashMap::new()) }
    fn GetAvailableReadEndpointsByLocation(&self) -> ReadOnlyLocationMap { Arc::new(HashMap::new()) }
    fn CanSupportMultipleWriteLocations(&self, _resource_type: ResourceType, _operation_type: OperationType) -> bool { false }
}

mod global_endpoint_manager;
mod location_cache;

pub use global_endpoint_manager::GlobalEndpointManager;
