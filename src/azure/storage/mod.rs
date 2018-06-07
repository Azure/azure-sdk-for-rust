pub mod blob;
pub mod client;
pub mod container;
mod rest_client;
pub mod table;

mod into_azure_path;
pub use self::into_azure_path::IntoAzurePath;
