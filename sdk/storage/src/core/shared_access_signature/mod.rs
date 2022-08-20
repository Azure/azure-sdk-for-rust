use std::fmt;
use time::OffsetDateTime;
use url::form_urlencoded;

pub mod account_sas;
pub mod service_sas;

pub trait SasToken {
    fn token(&self) -> String;
}

pub(crate) fn format_date(d: OffsetDateTime) -> String {
    // replacing nanosecond with 0 is known to not panic
    azure_core::date::to_rfc3339(&d.replace_nanosecond(0).unwrap())
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
