#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub mod clients;
mod continuation_next_partition_and_row_key;
mod continuation_next_table_name;
mod entity_metadata;
mod entity_with_metadata;
mod filter;
mod if_match_condition;
mod model;
pub mod prelude;
pub mod requests;
pub mod responses;
mod return_entity;
mod select;
mod top;
mod transaction;
mod transaction_operation;
use azure_core::Method;
pub use continuation_next_partition_and_row_key::ContinuationNextPartitionAndRowKey;
pub use continuation_next_table_name::ContinuationNextTableName;
pub use entity_metadata::EntityMetadata;
pub use entity_with_metadata::EntityWithMetadata;
pub use filter::Filter;
pub use if_match_condition::IfMatchCondition;
pub use model::Table;
use once_cell::sync::Lazy;
pub use return_entity::ReturnEntity;
pub use select::Select;
pub use top::Top;
pub use transaction::Transaction;
pub use transaction_operation::TransactionOperation;

// we need this since the http::Method does not have the MERGE verb. The unwrap is safe here.
static MERGE: Lazy<Method> = Lazy::new(|| Method::from_bytes(b"MERGE").unwrap());
