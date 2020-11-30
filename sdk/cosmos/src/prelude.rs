//! The cosmos prelude.
//!
//! The prelude re-exports most commonly used items from this crate.
//!
//! # Examples
//!
//! Import the prelude with:
//!
//! ```
//! # #[allow(unused_imports)]
//! use azure_cosmos::prelude::*;
//! ```

pub use crate::{ConsistencyLevel, CosmosError, PartitionKeys, Query};

pub use crate::clients::*;

// Resources
pub use crate::resources::collection::Offer;
pub use crate::resources::document::IndexingDirective;
pub use crate::resources::*;

// Traits
pub use crate::traits::*;
pub use database::DatabaseName;
pub use user::UserName;

pub use permission::AuthorizationToken;
