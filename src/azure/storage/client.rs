use std::io::Read;
use hyper::client::response::Response;
use hyper::client::Client as HyperClient;
use hyper::error::Error;
use hyper::header::Headers;
use azure::core::HTTPMethod;
use super::rest_client::{perform_request, ServiceType};

pub struct Client {
    account: String,
    key: String,
    use_https: bool,
    hc: HyperClient,
}

impl Client {
    pub fn new(account: &str, key: &str, use_https: bool) -> Client {
        Client {
            account: account.to_owned(),
            key: key.to_owned(),
            use_https: use_https,
            hc: HyperClient::new(),
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
                                 uri: &str,
                                 method: HTTPMethod,
                                 headers: Headers,
                                 request_str: Option<&str>)
                                 -> Result<Response, Error> {
        perform_request(&self.hc,
                        uri,
                        method,
                        &self.key,
                        &headers,
                        None,
                        request_str,
                        ServiceType::Table)
    }
}
