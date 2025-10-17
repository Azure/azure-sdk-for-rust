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

/// Type alias matching C# `ReadOnlyCollection<Uri>` semantics in a lightweight form.
/// We use `Arc<[Url]>` for cheap cloning and slice immutability.
pub type ReadOnlyUrlCollection = Arc<[Url]>;

/// Type alias representing `ReadOnlyDictionary<string, Uri>`.
pub type ReadOnlyLocationMap = Arc<HashMap<String, Url>>;

mod location_cache;