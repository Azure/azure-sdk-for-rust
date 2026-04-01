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
pub(crate) async fn sign_request(
    request: &mut HttpRequest,
    credential: &Credential,
    auth_context: &AuthorizationContext,
) -> azure_core::Result<()> {
    let date_string = time::to_rfc7231(&OffsetDateTime::now_utc()).to_lowercase();

    let auth = generate_authorization(credential, auth_context, &date_string).await?;

    request
        .headers
        .insert(MS_DATE, HeaderValue::from(date_string));
    request
        .headers
        .insert(AUTHORIZATION, HeaderValue::from(auth));

    Ok(())
}
