use chrono::{DateTime, Utc};
use ring::hmac;
use std::fmt;
use url::form_urlencoded;

pub mod account_sas;
pub mod service_sas;

pub trait SasToken {
    fn token(&self) -> String;
}

pub(crate) fn sign(key: &str, data: &str) -> String {
    let key = hmac::Key::new(ring::hmac::HMAC_SHA256, &base64::decode(key).unwrap());
    let sig_bytes = hmac::sign(&key, data.as_bytes());
    base64::encode(&sig_bytes)
}

pub(crate) fn format_date(d: DateTime<Utc>) -> String {
    d.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub(crate) fn format_form(d: String) -> String {
    form_urlencoded::byte_serialize(d.as_bytes()).collect::<String>()
}

/// Specifies the protocol permitted for a request made with the SAS ([Azure documentation](https://docs.microsoft.com/rest/api/storageservices/create-service-sas#specifying-the-http-protocol)).
#[derive(Copy, Clone)]
pub enum SasProtocol {
    Https,
    HttpHttps,
}

impl fmt::Display for SasProtocol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SasProtocol::Https => write!(f, "https"),
            SasProtocol::HttpHttps => write!(f, "http,https"),
        }
    }
}
