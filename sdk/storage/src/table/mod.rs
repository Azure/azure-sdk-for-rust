pub mod clients;
mod entity_metadata;
mod entity_with_metadata;
mod filter;
mod if_match_condition;
pub mod prelude;
pub mod requests;
pub mod responses;
mod return_entity;
mod select;
mod table;
mod top;
pub use entity_metadata::EntityMetadata;
pub use entity_with_metadata::EntityWithMetadata;
pub use filter::Filter;
pub use if_match_condition::IfMatchCondition;
pub use return_entity::ReturnEntity;
pub use select::Select;
pub use table::Table;
pub use top::Top;

// we need this since the http::Method does not have the MERGE verb. The unwrap is safe here.
lazy_static! {
    static ref MERGE: http::Method = http::Method::from_bytes(b"MERGE").unwrap();
}
