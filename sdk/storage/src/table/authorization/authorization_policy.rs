use std::sync::Arc;

use azure_core::{PipelineContext, PipelineError, Policy, PolicyResult, Request, Response};
use http::{HeaderMap, HeaderValue, Method, Uri};
use ring::hmac;

use crate::{authorization::AuthorizationToken, table_context::TableContext};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPolicy {
    authorization_token: AuthorizationToken,
}

impl AuthorizationPolicy {
    pub(crate) fn new(authorization_token: AuthorizationToken) -> Self {
        Self {
            authorization_token,
        }
    }
}

#[async_trait::async_trait]
impl Policy<TableContext> for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &mut PipelineContext<TableContext>,
        request: &mut Request,
        next: &[Arc<dyn Policy<TableContext>>],
    ) -> PolicyResult<Response> {
        trace!("called AuthorizationPolicy::send. self == {:#?}", self);

        if next.is_empty() {
            return Err(Box::new(PipelineError::InvalidTailPolicy(
                "Authorization policies cannot be the last policy of a pipeline".to_owned(),
            )));
        }

        match &self.authorization_token {
            AuthorizationToken::SASToken {} => todo!(),
            AuthorizationToken::BearerToken {} => todo!(),
            AuthorizationToken::SharedKeyToken(credential) => {
                let signature = credential.sign(format!(
                    "{}\n{}\n{}\n{}\n/{}{}",
                    request.method().as_str(),
                    request
                        .headers()
                        .get("Content-MD5")
                        .map_or("", |v| v.to_str().unwrap()),
                    request
                        .headers()
                        .get("Content-Type")
                        .map_or("", |v| v.to_str().unwrap()),
                    request
                        .headers()
                        .get("x-ms-date")
                        .map_or("", |v| v.to_str().unwrap()),
                    credential.account(),
                    request.uri().path()
                ));
                let token = format!("SharedKey {}:{}", credential.account(), signature);
                request
                    .headers_mut()
                    .append("authorization", HeaderValue::from_str(&token).unwrap());
            }
        }
        next[0].send(ctx, request, &next[1..]).await
    }
}

/// An authorization shared key token.
/// to create the token first create string to sign, the string should contain the following:
/// * http method
/// * md5 content (if exists else empty string and new a line)
/// * content type (if exists else empty string and new a line)
/// * x-ms-date (utc new formatted as 'Wed, 18 Aug 2021 14:52:59 GMT')
/// * canonicalized resource (for example, /devstoreaccount1/devstoreaccount1/Tables)
fn shared_key_token(
    headers: &HeaderMap,
    uri: &Uri,
    method: &Method,
    account: &str,
    key: &[u8],
) -> String {
    let to_sign = format!(
        "{}\n{}\n{}\n{}\n/{}{}",
        method.as_str(),
        headers
            .get("Content-MD5")
            .map_or("", |v| v.to_str().unwrap()),
        headers
            .get("Content-Type")
            .map_or("", |v| v.to_str().unwrap()),
        headers.get("x-ms-date").map_or("", |v| v.to_str().unwrap()),
        account,
        uri.path()
    );
    let signature = base64::encode(hmac::sign(
        &hmac::Key::new(hmac::HMAC_SHA256, key),
        to_sign.as_bytes(),
    ));
    format!("SharedKey {}:{}", account, signature)
}
