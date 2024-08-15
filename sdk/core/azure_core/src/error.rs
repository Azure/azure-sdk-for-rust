// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure client-specific error functions.

pub use typespec::error::*;
pub use typespec_client_core::error::*;

#[cfg(test)]
mod tests {
    use bytes::Bytes;
    use typespec_client_core::{
        error::HttpError,
        http::{headers::Headers, RawResponse, StatusCode},
        stream::BytesStream,
    };

    #[tokio::test]
    async fn get_error_from_header() {
        let mut headers = Headers::new();
        headers.insert("x-ms-error-code", "UnsupportedKind");
        let buffer = Bytes::from_static(
            br#"{"error":{"code":"IAmATeapot","message":"I can't do that, Dave."}}"#,
        );
        let stream: BytesStream = buffer.into();

        let response = RawResponse::new(StatusCode::BadRequest, headers, Box::pin(stream));
        let err = HttpError::new(response).await;

        assert_eq!("UnsupportedKind", err.error_code().unwrap());
    }
}
