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
//! use async_std::prelude::*;
//! ```

pub use crate::{ConsistencyLevel, CosmosError, IndexingDirective, Offer, PartitionKeys, Query};

pub use crate::clients::*;
pub use crate::resources::*;

// Traits
pub use crate::traits::*;
pub use database::DatabaseName;
pub use user::UserName;

pub use permission::AuthorizationToken;
