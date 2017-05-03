use std::io::Read;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::client::response::Response;
use hyper::client::Client as HyperClient;
use hyper::error::Error;
use hyper::header::Headers;
use azure::core::HTTPMethod;
use super::rest_client::{perform_request, ServiceType};

// Can be variant for different cloud environment
const SERVICE_SUFFIX_BLOB: &'static str = ".blob.core.windows.net";
const SERVICE_SUFFIX_TABLE: &'static str = ".table.core.windows.net";

pub struct Client {
    account: String,
    key: String,
    use_https: bool,
    hc: HyperClient,
}

impl Client {
    pub fn new(account: &str, key: &str, use_https: bool) -> Client {
        use hyper;

        let ssl = NativeTlsClient::new().unwrap();
        let connector = HttpsConnector::new(ssl);
        let client = hyper::Client::with_connector(connector);

        Client {
            account: account.to_owned(),
            key: key.to_owned(),
            use_https: use_https,
            hc: client,
        }
    }

    pub fn account(&self) -> &str {
        &self.account
    }

    pub fn use_https(&self) -> bool {
        self.use_https
    }

    pub fn auth_scheme(&self) -> &'static str {
        if self.use_https { "https" } else { "http" }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn perform_request(&self,
                           uri: &str,
                           method: HTTPMethod,
                           headers: &Headers,
                           request_body: Option<(&mut Read, u64)>)
                           -> Result<Response, Error> {
        perform_request(&self.hc,
                        uri,
                        method,
                        &self.key,
                        headers,
                        request_body,
                        None,
                        ServiceType::Blob)
    }

    pub fn perform_table_request(&self,
                                 segment: &str,
                                 method: HTTPMethod,
                                 headers: Headers,
                                 request_str: Option<&str>)
                                 -> Result<Response, Error> {

        debug!("segment: {}, method: {:?}, headers: {:?}",
               segment,
               method,
               headers);
        perform_request(&self.hc,
                        (self.get_uri_prefix(ServiceType::Table) + segment).as_str(),
                        method,
                        &self.key,
                        &headers,
                        None,
                        request_str,
                        ServiceType::Table)
    }

    /// Uri scheme + authority e.g. http://myaccount.table.core.windows.net/
    pub fn get_uri_prefix(&self, service_type: ServiceType) -> String {
        self.auth_scheme().to_owned() + "://" + self.account() +
        match service_type {
            ServiceType::Blob => SERVICE_SUFFIX_BLOB,
            ServiceType::Table => SERVICE_SUFFIX_TABLE,
        } + "/"
    }
}
