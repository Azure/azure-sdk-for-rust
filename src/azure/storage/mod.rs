pub mod blob;
pub mod client;
pub mod container;
mod rest_client;
pub mod table;

pub trait IntoAzurePath {
    fn container_name(&self) -> &str;
    fn blob_name(&self) -> &str;
}

impl<'a> IntoAzurePath for (&'a str, &'a str) {
    fn container_name(&self) -> &str {
        self.0
    }
    fn blob_name(&self) -> &str {
        self.1
    }
}
