#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub use azure_storage::{Error, Result};

mod bearer_token_authorization_policy;
pub mod clients;
mod file_system;
pub mod operations;
pub mod requests;
pub mod responses;
mod shared_key_authorization_policy;
pub use file_system::FileSystem;
mod properties;
mod util;
pub use properties::Properties;
pub mod prelude;
