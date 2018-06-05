pub mod blob;
pub mod client;
pub mod container;
mod rest_client;
pub mod table;

pub trait IntoAzurePath {
    fn into(&self) -> (&str, &str);
}
