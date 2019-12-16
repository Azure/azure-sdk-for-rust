#![recursion_limit = "128"]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
pub mod table;
mod table_entry;
pub use table_entry::TableEntry;
