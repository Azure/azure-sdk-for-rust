use hyper::{self, Method};
use hyper_tls;
use super::rest_client::{perform_request, ServiceType};
use azure::core::errors::AzureError;

// Can be variant for different cloud environment
const SERVICE_SUFFIX_BLOB: &str = ".blob.core.windows.net";
const SERVICE_SUFFIX_TABLE: &str = ".table.core.windows.net";

pub struct Client {
    account: String,
    key: String,
    hc: hyper::Client<hyper_tls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl Client {
    pub fn new(account: &str, key: &str) -> Result<Client, AzureError> {
        use hyper;

        let client = hyper::Client::builder()
            .build(hyper_tls::HttpsConnector::new(4)?);

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
        perform_request(
            &self.hc,
            uri,
            method,
            &self.key,
            headers_func,
            request_body,
            ServiceType::Blob,
        )
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
