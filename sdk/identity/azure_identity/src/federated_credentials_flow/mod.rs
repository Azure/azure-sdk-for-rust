// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod response;

use azure_core::{
    content_type,
    error::{http_response_from_body, ErrorKind, ResultExt},
    headers, HttpClient, Method, Request, Response, Url,
};
use response::LoginResponse;
use std::sync::Arc;
use tracing::{debug, error};
use url::form_urlencoded;

/// Authorize the client using the federated credentials flow.
pub async fn authorize(
    http_client: Arc<dyn HttpClient>,
    client_id: &str,
    client_assertion: &str,
    scopes: &[&str],
    tenant_id: &str,
    host: &Url,
) -> azure_core::Result<LoginResponse> {
    let encoded: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id)
        .append_pair("scope", &scopes.join(" "))
        .append_pair(
            "client_assertion_type",
            "urn:ietf:params:oauth:client-assertion-type:jwt-bearer",
        )
        .append_pair("client_assertion", client_assertion)
        .append_pair("grant_type", "client_credentials")
        .finish();

    let url = host
        .join(&format!("/{tenant_id}/oauth2/v2.0/token"))
        .with_context(ErrorKind::DataConversion, || {
            format!("The supplied tenant id could not be url encoded: {tenant_id}")
        })?;

    let mut req = Request::new(url, Method::Post);
    req.insert_header(
        headers::CONTENT_TYPE,
        content_type::APPLICATION_X_WWW_FORM_URLENCODED,
    );
    req.set_body(encoded);
    let rsp: Response = http_client.execute_request(&req).await?;
    let rsp_status = rsp.status();
    debug!("rsp_status == {:?}", rsp_status);
    if rsp_status.is_success() {
        rsp.into_json_body().await
    } else {
        let rsp_body = rsp.into_raw_body().collect().await?;
        let text = std::str::from_utf8(&rsp_body)?;
        error!("rsp_body == {:?}", text);
        Err(http_response_from_body(rsp_status, &rsp_body).into_error())
    }
}
