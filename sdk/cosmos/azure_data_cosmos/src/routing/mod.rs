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

pub(crate) mod global_endpoint_manager;
mod location_cache;