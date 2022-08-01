use azure_core::{date, headers::Headers};
use time::OffsetDateTime;

const EXPIRY_TIME: &str = "x-ms-expiry-time";
const EXPIRY_OPTION: &str = "x-ms-expiry-option";

#[derive(Debug, Clone)]
pub enum BlobExpiry {
    RelativeToCreation(u64),
    RelativeToNow(u64),
    Absolute(OffsetDateTime),
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
                headers.insert(EXPIRY_TIME, date::to_rfc1123(date));
            }
            BlobExpiry::NeverExpire => {
                headers.insert(EXPIRY_OPTION, "NeverExpire");
            }
        }
        headers
    }
}
