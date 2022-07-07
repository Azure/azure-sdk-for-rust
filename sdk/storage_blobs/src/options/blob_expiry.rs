use azure_core::headers::Headers;
use chrono::{DateTime, Utc};

const EXPIRY_TIME: &str = "x-ms-expiry-time";
const EXPIRY_OPTION: &str = "x-ms-expiry-option";

#[derive(Debug, Clone)]
pub enum BlobExpiry {
    RelativeToCreation(u64),
    RelativeToNow(u64),
    Absolute(DateTime<Utc>),
    NeverExpire,
}

impl BlobExpiry {
    pub fn to_headers(&self) -> Headers {
        let mut headers = Headers::new();
        match self {
            BlobExpiry::RelativeToCreation(duration) => {
                headers.insert(EXPIRY_OPTION, "RelativeToCreation");
                headers.insert(EXPIRY_TIME, duration.to_string());
            }
            BlobExpiry::RelativeToNow(duration) => {
                headers.insert(EXPIRY_OPTION, "RelativeToNow");
                headers.insert(EXPIRY_TIME, duration.to_string());
            }
            BlobExpiry::Absolute(date) => {
                headers.insert(EXPIRY_OPTION, "Abosolute");
                headers.insert(EXPIRY_TIME, date.to_rfc2822());
            }
            BlobExpiry::NeverExpire => {
                headers.insert(EXPIRY_OPTION, "NeverExpire");
            }
        }
        headers
    }
}
