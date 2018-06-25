use super::rest_client::{perform_request, ServiceType};
use azure::core::errors::AzureError;
use azure::core::No;
use azure::storage::container;
use hyper::{self, Method};
use hyper_tls;

// Can be variant for different cloud environment
const SERVICE_SUFFIX_BLOB: &str = ".blob.core.windows.net";
const SERVICE_SUFFIX_TABLE: &str = ".table.core.windows.net";

pub trait Container {
    fn create<'a>(&'a self) -> container::requests::CreateBuilder<'a, No, No>;
    fn delete<'a>(&'a self) -> container::requests::DeleteBuilder<'a, No>;
    fn list<'a>(&'a self) -> container::requests::ListBuilder<'a>;
    fn get_acl<'a>(&'a self) -> container::requests::GetACLBuilder<'a, No>;
    fn set_acl<'a>(&'a self) -> container::requests::SetACLBuilder<'a, No, No>;
    fn get_properties<'a>(&'a self) -> container::requests::GetPropertiesBuilder<'a, No>;
    fn acquire_lease<'a>(&'a self) -> container::requests::AcquireLeaseBuilder<'a, No, No>;
    fn renew_lease<'a>(&'a self) -> container::requests::RenewLeaseBuilder<'a, No, No>;
    fn release_lease<'a>(&'a self) -> container::requests::ReleaseLeaseBuilder<'a, No, No>;
    fn break_lease<'a>(&'a self) -> container::requests::BreakLeaseBuilder<'a, No>;
}

#[derive(Debug)]
pub struct Client {
    account: String,
    key: String,
    hc: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl Container for Client {
    fn create<'a>(&'a self) -> container::requests::CreateBuilder<'a, No, No> {
        container::requests::CreateBuilder::new(self)
    }

    fn delete<'a>(&'a self) -> container::requests::DeleteBuilder<'a, No> {
        container::requests::DeleteBuilder::new(self)
    }

    fn list<'a>(&'a self) -> container::requests::ListBuilder<'a> {
        container::requests::ListBuilder::new(self)
    }

    fn get_acl<'a>(&'a self) -> container::requests::GetACLBuilder<'a, No> {
        container::requests::GetACLBuilder::new(self)
    }

    fn set_acl<'a>(&'a self) -> container::requests::SetACLBuilder<'a, No, No> {
        container::requests::SetACLBuilder::new(self)
    }

    fn get_properties<'a>(&'a self) -> container::requests::GetPropertiesBuilder<'a, No> {
        container::requests::GetPropertiesBuilder::new(self)
    }

    fn acquire_lease<'a>(&'a self) -> container::requests::AcquireLeaseBuilder<'a, No, No> {
        container::requests::AcquireLeaseBuilder::new(self)
    }

    fn renew_lease<'a>(&'a self) -> container::requests::RenewLeaseBuilder<'a, No, No> {
        container::requests::RenewLeaseBuilder::new(self)
    }

    fn release_lease<'a>(&'a self) -> container::requests::ReleaseLeaseBuilder<'a, No, No> {
        container::requests::ReleaseLeaseBuilder::new(self)
    }

    fn break_lease<'a>(&'a self) -> container::requests::BreakLeaseBuilder<'a, No> {
        container::requests::BreakLeaseBuilder::new(self)
    }
}

impl Client {
    pub fn new(account: &str, key: &str) -> Result<Client, AzureError> {
        use hyper;

        let client = hyper::Client::builder().build(hyper_tls::HttpsConnector::new(4)?);

        Ok(Client {
            account: account.to_owned(),
            key: key.to_owned(),
            hc: client,
        })
    }

    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn perform_request<F>(
        &self,
        uri: &str,
        method: Method,
        headers_func: F,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(&mut ::http::request::Builder),
    {
        perform_request(&self.hc, uri, method, &self.key, headers_func, request_body, ServiceType::Blob)
    }

    pub fn perform_table_request<F>(
        &self,
        segment: &str,
        method: Method,
        headers_func: F,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>
    where
        F: FnOnce(&mut ::http::request::Builder),
    {
        debug!("segment: {}, method: {:?}", segment, method,);
        perform_request(
            &self.hc,
            (self.get_uri_prefix(&ServiceType::Table) + segment).as_str(),
            method,
            &self.key,
            headers_func,
            request_str,
            ServiceType::Table,
        )
    }

    /// Uri scheme + authority e.g. http://myaccount.table.core.windows.net/
    pub fn get_uri_prefix(&self, service_type: &ServiceType) -> String {
        "https://".to_owned() + self.account() + match *service_type {
            ServiceType::Blob => SERVICE_SUFFIX_BLOB,
            ServiceType::Table => SERVICE_SUFFIX_TABLE,
        } + "/"
    }
}
