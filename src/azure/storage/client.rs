use hyper::client::response::Response;
use hyper::client::Client as HyperClient;
use hyper::error::Error;
use hyper::header::{Accept, ContentType, Headers, qitem};
use hyper::mime::{Attr, Mime, SubLevel, TopLevel, Value};
use std::io::Read;
use azure::core::HTTPMethod;
use super::rest_client::{perform_request, perform_request_with_client, ServiceType};

pub struct Client {
    account: String,
    key: String,
    use_https: bool,
    hc: HyperClient,
}

#[inline]
fn get_default_json_mime() -> Mime {
    return Mime(TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Charset, Value::Utf8)]);
}

#[inline]
fn get_json_mime_nometadata() -> Mime {
    return Mime(TopLevel::Application,
                SubLevel::Json,
                vec![(Attr::Ext("odata".to_owned()), Value::Ext("nometadata".to_owned()))]);
}

#[inline]
fn get_batch_mime() -> Mime {
    return Mime(TopLevel::Multipart,
                SubLevel::Ext("Mixed".to_owned()),
                vec![(Attr::Ext("boundary".to_owned()), Value::Ext("batch_a1e9d677-b28b-435e-a89e-87e6a768a431".to_owned()))]);
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
        perform_request(uri,
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
                                 request_str: Option<&str>,
                                 is_batch: bool)
                                 -> Result<Response, Error> {

        let mut headers = Headers::new();
        if is_batch {
            headers.set(ContentType(get_batch_mime()));
        } else {
            headers.set(Accept(vec![qitem(get_json_mime_nometadata())]));
            if request_str.is_some() {
                headers.set(ContentType(get_default_json_mime()));
            }
        }

        perform_request_with_client(&self.hc,
                                    uri,
                                    method,
                                    &self.key,
                                    &headers,
                                    None,
                                    request_str,
                                    ServiceType::Table)
    }
}
