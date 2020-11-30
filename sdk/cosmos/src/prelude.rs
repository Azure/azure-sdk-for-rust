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

#[doc(inline)]
pub use crate::{ConsistencyLevel, CosmosError, PartitionKeys};

#[doc(inline)]
pub use crate::clients::*;

// Resources
pub use crate::resources::collection::Offer;
pub use crate::resources::document::{IndexingDirective, Query};
#[doc(inline)]
pub use crate::resources::*;

// Traits
pub use crate::traits::*;

pub use permission::AuthorizationToken;
