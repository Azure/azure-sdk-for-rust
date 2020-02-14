#![recursion_limit = "128"]

extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quick_error;

pub mod table_client;
pub use table_client::*;
mod cloud_table;
pub use cloud_table::*;
mod table_entity;
pub use table_entity::*;
mod batch;
pub use batch::*;
pub mod de;
