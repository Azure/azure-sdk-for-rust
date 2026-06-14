// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Bare function: generate and attach the Authorization header.
//!
//! This replaces `AuthorizationPolicy` from the old policy-chain pipeline.

use azure_core::http::headers::{HeaderName, HeaderValue, AUTHORIZATION};
use azure_core::time::{self, OffsetDateTime};

use crate::models::Credential;

use super::{cosmos_transport_client::HttpRequest, generate_authorization, AuthorizationContext};

const MS_DATE: HeaderName = HeaderName::from_static("x-ms-date");

/// Generates and attaches the Authorization header to an HTTP request.
///
/// Computes the HMAC-SHA256 signature (master key) or obtains an AAD token,
/// then sets both `x-ms-date` and `Authorization` headers.
///
/// Returns a Cosmos-typed [`crate::error::CosmosError`]. Foreign errors from the
/// credential provider and the HMAC routine are classified into typed
/// Cosmos errors at the boundary by [`generate_authorization`].
pub(crate) async fn sign_request(
    request: &mut HttpRequest,
    credential: &Credential,
    auth_context: &AuthorizationContext,
) -> crate::error::Result<()> {
    let date_string = time::to_rfc7231(&OffsetDateTime::now_utc()).to_string();

    let auth = generate_authorization(credential, auth_context, &date_string).await?;

    request
        .headers
        .insert(MS_DATE, HeaderValue::from(date_string));
    request
        .headers
        .insert(AUTHORIZATION, HeaderValue::from(auth));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::ResourceType;
    use azure_core::credentials::Secret;
    use azure_core::http::Method;

    fn empty_request() -> HttpRequest {
        HttpRequest {
            url: "https://account.documents.azure.com/dbs/db1/colls/coll1"
                .parse()
                .unwrap(),
            method: Method::Get,
            headers: azure_core::http::headers::Headers::new(),
            body: None,
            timeout: None,
            #[cfg(feature = "fault_injection")]
            evaluation_collector: None,
        }
    }

    /// Regression: `sign_request` must write `x-ms-date` in proper RFC 7231
    /// case (e.g. `"Wed, 21 Oct 2015 07:28:00 GMT"`). The Gateway 2.0 RNTBD
    /// `Date` token (0x0003) reads the same value, and the proxy rejects
    /// fully-lowercased dates. The internal HMAC lowercasing lives in
    /// `build_string_to_sign`, not here.
    #[tokio::test]
    async fn sign_request_writes_x_ms_date_in_proper_rfc_7231_case() {
        // 32-byte dummy key, base64-encoded — `hmac_sha256` only requires that
        // the input decodes to bytes; the value is not validated against any
        // real account.
        let key = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let credential = Credential::MasterKey(Secret::new(key.to_owned()));
        let auth_ctx = AuthorizationContext::new(
            Method::Get,
            ResourceType::DocumentCollection,
            "dbs/db1/colls/coll1",
        );

        let mut request = empty_request();
        sign_request(&mut request, &credential, &auth_ctx)
            .await
            .unwrap();

        let date_header = request.headers.get_optional_str(&MS_DATE).unwrap();
        // RFC 7231 IMF-fixdate has capital letters in the day-of-week
        // abbreviation, month abbreviation, and `GMT` suffix. A fully
        // lowercased date would fail this check.
        assert!(
            date_header.contains(" GMT"),
            "x-ms-date must end with uppercase `GMT`; got {date_header:?}"
        );
        let weekday: String = date_header.chars().take(3).collect();
        assert!(
            weekday
                .chars()
                .next()
                .map(|c| c.is_ascii_uppercase())
                .unwrap_or(false)
                && weekday.chars().skip(1).all(|c| c.is_ascii_lowercase()),
            "x-ms-date weekday must be Title-Case (e.g. `Wed`); got {weekday:?}"
        );
        // Defensive: x-ms-date must not be the lowercased form.
        assert_ne!(date_header, date_header.to_ascii_lowercase());
    }
}
