//! The kusto prelude.
//!
//! The prelude re-exports most commonly used items from this crate.
//!
//! # Examples
//!
//! Import the prelude with:
//!
//! ```
//! # #[allow(unused_imports)]
//! use azure_data_kusto::prelude::*;
//! ```

pub use crate::client::{KustoClient, KustoClientOptions};
pub use crate::connection_string::ConnectionStringBuilder;
pub use crate::operations::query::ResultTable;
