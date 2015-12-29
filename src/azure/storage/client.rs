use hyper::header::Headers;
use hyper::client::response::Response;
use hyper::error::Error;

use azure::storage::container;
use azure::storage::container::Container;


use azure::core::errors;
use azure::core::{HTTPMethod, perform_request};

#[derive(Debug)]
pub struct Client {
    account: String,
    key: String,
    use_https: bool,
}


impl Client {
    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn use_https(&self) -> bool {
        self.use_https
    }

    pub fn auth_scheme(&self) -> &'static str {
        if self.use_https {
            "https"
        } else {
            "http"
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    #[inline(always)]
    pub fn list_containers(&self) -> Result<Vec<Container>, errors::AzureError> {
        container::list(self)
    }

    #[inline(always)]
    pub fn create_container(&self,
                            container_name: &str,
                            pa: container::PublicAccess)
                            -> Result<(), errors::AzureError> {
        container::create(self, container_name, pa)
    }

    pub fn perform_request(&self,
                           uri: &str,
                           method: HTTPMethod,
                           headers: &Headers)
                           -> Result<Response, Error> {
        perform_request(uri, method, &self.key, headers)
    }
}

pub fn new(account: &str, key: &str, use_https: bool) -> Client {
    Client {
        account: account.to_owned(),
        key: key.to_owned(),
        use_https: use_https,
    }
}
