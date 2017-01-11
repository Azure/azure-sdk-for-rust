use hyper::client::response::Response;
use hyper::error::Error;
use hyper::header::{ Accept, ContentType, Headers, qitem };
use hyper::mime::{ Attr, Mime, SubLevel, TopLevel, Value };
use std::io::Read;
use azure::core::HTTPMethod;
use super::rest_client::{perform_request, ServiceType};

#[derive(Debug)]
pub struct Client {
    account: String,
    key: String,
    use_https: bool,
}

#[inline]
fn get_default_json_mime() -> Mime {
    return Mime(TopLevel::Application, SubLevel::Json, vec![(Attr::Charset, Value::Utf8)]);
}

impl Client {
    pub fn new(account: &str, key: &str, use_https: bool) -> Client {
        Client {
            account: account.to_owned(),
            key: key.to_owned(),
            use_https: use_https,
        }
    }

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

    pub fn perform_request(&self,
                           uri: &str,
                           method: HTTPMethod,
                           headers: &Headers,
                           request_body: Option<(&mut Read, u64)>)
                           -> Result<Response, Error> {
        perform_request(uri, method, &self.key, headers, request_body, ServiceType::Blob)
    }

    pub fn perform_table_request(&self,
                           uri: &str,
                           method: HTTPMethod,
                           request_body: Option<(&mut Read, u64)>)
                           -> Result<Response, Error> {

        let mut headers = Headers::new();
        headers.set(Accept(vec![qitem(get_default_json_mime())]));
        if request_body.is_some() {
            headers.set(ContentType(get_default_json_mime()));
        }

        perform_request(uri, method, &self.key, &headers, request_body, ServiceType::Table)
    }
}
