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
pub use crate::{ConsistencyLevel, CosmosError, MaxItemCount, PartitionKeys};

#[doc(inline)]
pub use crate::clients::*;

// Resources
pub use crate::resources::collection::Offer;
pub use crate::resources::document::*;
#[doc(inline)]
pub use crate::resources::*;

pub use permission::AuthorizationToken;
