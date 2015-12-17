use azure::storage::container;
use azure::storage::container::{Container, PublicAccess};

use azure::core::errors;

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
}

pub fn new(account: String, key: String, use_https: bool) -> Client {
    Client {
        account: account,
        key: key,
        use_https: use_https,
    }
}
