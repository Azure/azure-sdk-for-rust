// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! HTTP headers.

// Re-export headers to flatten out this module with typespec_client_core.
pub use typespec_client_core::http::headers::*;

// HTTP headers are case-insensitive.
// We use lowercase below for simple comparisons downstream.

/// x-ms-client-request-id header. See <https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#distributed-tracing--telemetry> for more details.
pub const CLIENT_REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-client-request-id");
/// x-ms-error-code header. See <https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#handling-errors> for more details.
pub const ERROR_CODE: HeaderName = HeaderName::from_static("x-ms-error-code");
/// x-ms-date header.
pub const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");
/// x-ms-request-id header. See <https://github.com/microsoft/api-guidelines/blob/vNext/azure/Guidelines.md#distributed-tracing--telemetry> for more details.
pub(crate) const REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-request-id");
/// retry-after-ms header.
pub const RETRY_AFTER_MS: HeaderName = HeaderName::from_static("retry-after-ms");
/// x-ms-version header.
pub const VERSION: HeaderName = HeaderName::from_static("x-ms-version");
/// x-ms-retry-after-ms header.
pub const X_MS_RETRY_AFTER_MS: HeaderName = HeaderName::from_static("x-ms-retry-after-ms");

/// Constants related to the Content-Type header
///
/// <https://developer.mozilla.org/docs/Web/HTTP/Headers/Content-Type>
pub mod content_type {
    use crate::http::headers::HeaderValue;

    // Form content types
    // https://www.w3.org/TR/html401/interact/forms.html#h-17.13.4

    /// The content type for form payloads. See <https://www.w3.org/TR/html401/interact/forms.html#h-17.13.4> for more details.
    pub const APPLICATION_X_WWW_FORM_URLENCODED: HeaderValue =
        HeaderValue::from_static("application/x-www-form-urlencoded");
}
