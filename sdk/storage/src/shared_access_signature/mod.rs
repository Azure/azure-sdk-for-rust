use std::fmt;
use time::OffsetDateTime;
use url::form_urlencoded;

pub mod account_sas;
pub mod service_sas;

pub trait SasToken {
    fn token(&self) -> String;
}

/// Converts an OffsetDateTime to an RFC3339 formatted string after truncating
/// any partial seconds.
pub(crate) fn format_date(d: OffsetDateTime) -> String {
    // When validating signatures, Azure Storage server creates a canonicalized
    // version of the request, then verifies the signature from the request with
    // the canonicalized version.
    //
    // The canonicalization at the server truncates the timestamps without
    // microseconds or nanoseconds.  As such, this needs to be truncated here
    // too.
    //
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

#[cfg(test)]
mod tests {
    use super::*;
    use time::macros::datetime;

    #[test]
    // verify format_date truncates as expected.
    fn test_format_date_truncation() {
        let date = datetime!(2022-08-22 15:11:43.4185122 +00:00:00);
        assert_eq!(format_date(date), "2022-08-22T15:11:43Z");
    }
}
