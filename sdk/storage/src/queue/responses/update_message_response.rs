use azure_core::headers::{
    rfc2822_from_headers_mandatory, string_from_headers_mandatory, CommonStorageResponseHeaders,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use http::response::Response;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct UpdateMessageResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub time_next_visible: DateTime<Utc>,
    pub pop_receipt: String,
}

impl std::convert::TryFrom<&Response<Bytes>> for UpdateMessageResponse {
    type Error = crate::Error;

    fn try_from(response: &Response<Bytes>) -> Result<Self, Self::Error> {
        debug!("response == {:?}", response);

        Ok(UpdateMessageResponse {
            common_storage_response_headers: response.headers().try_into()?,
            time_next_visible: rfc2822_from_headers_mandatory(
                response.headers(),
                "x-ms-time-next-visible",
            )?,
            pop_receipt: string_from_headers_mandatory(response.headers(), "x-ms-popreceipt")?
                .to_owned(),
        })
    }
}
