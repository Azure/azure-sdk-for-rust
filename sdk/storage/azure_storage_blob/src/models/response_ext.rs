// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{
        headers::{HeaderName, Headers},
        response::AsyncResponseBody,
        Response, XmlFormat,
    },
    Error,
};
use azure_storage_common::models::UserDelegationKey;

const REQUEST_PRIORITY: HeaderName = HeaderName::from_static("x-ms-request-priority");

/// Provides typed response headers for
/// [`BlobServiceClient::get_user_delegation_key()`](crate::clients::BlobServiceClient::get_user_delegation_key).
///
/// # Examples
///
/// ```
/// use azure_core::http::{Response, XmlFormat};
/// use azure_storage_blob::models::UserDelegationKeyHeaders;
/// use azure_storage_common::models::UserDelegationKey;
///
/// fn priority(response: &Response<UserDelegationKey, XmlFormat>) -> azure_core::Result<Option<i32>> {
///     response.effective_request_priority()
/// }
/// ```
pub trait UserDelegationKeyHeaders: private::Sealed {
    /// Returns the effective request priority reported by the service.
    ///
    /// Returns `None` if the header is absent, or an error if its value cannot be parsed as an `i32`.
    fn effective_request_priority(&self) -> azure_core::Result<Option<i32>>;
}

impl UserDelegationKeyHeaders for Response<UserDelegationKey, XmlFormat> {
    fn effective_request_priority(&self) -> azure_core::Result<Option<i32>> {
        Headers::get_optional_as(self.headers(), &REQUEST_PRIORITY)
    }
}

mod private {
    use super::{Response, UserDelegationKey, XmlFormat};

    pub trait Sealed {}

    impl Sealed for Response<UserDelegationKey, XmlFormat> {}
}

#[async_trait]
pub(crate) trait AsyncResponseBodyExt {
    /// Collects the response into the given slice.
    ///
    /// # Returns
    /// Ok if the response body was the exact length of the slice and the contents were
    /// successfully copied into it. Otherwise, returns Err.
    async fn collect_into_exact(mut self, buffer: &mut [u8]) -> Result<(), Error>;
}

#[async_trait]
impl AsyncResponseBodyExt for AsyncResponseBody {
    async fn collect_into_exact(mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let count = self.collect_into(buffer).await?;
        if count == buffer.len() {
            Ok(())
        } else {
            Err(Error::with_message(
                ErrorKind::Other,
                "Received fewer than expected bytes.",
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Headers, Response, UserDelegationKey, UserDelegationKeyHeaders, XmlFormat, REQUEST_PRIORITY,
    };
    use azure_core::http::{RawResponse, StatusCode};

    fn response(priority: Option<&str>) -> Response<UserDelegationKey, XmlFormat> {
        let mut headers = Headers::new();
        if let Some(priority) = priority {
            headers.insert(REQUEST_PRIORITY, priority.to_owned());
        }
        RawResponse::from_bytes(StatusCode::Ok, headers, Vec::<u8>::new()).into()
    }

    #[test]
    fn effective_request_priority_reads_supported_values() -> azure_core::Result<()> {
        for priority in 0..=7 {
            let response = response(Some(&priority.to_string()));
            assert_eq!(response.effective_request_priority()?, Some(priority));
        }
        Ok(())
    }

    #[test]
    fn effective_request_priority_missing() -> azure_core::Result<()> {
        assert_eq!(response(None).effective_request_priority()?, None);
        Ok(())
    }

    #[test]
    fn effective_request_priority_rejects_malformed_values() {
        for value in ["not-an-integer", "1.5", "2147483648"] {
            assert!(
                response(Some(value)).effective_request_priority().is_err(),
                "expected a parsing error for {value}"
            );
        }
    }
}
