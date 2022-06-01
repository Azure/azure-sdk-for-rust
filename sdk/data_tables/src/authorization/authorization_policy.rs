use super::AuthorizationToken;
use azure_core::{headers::Headers, Context, Policy, PolicyResult, Request};
use http::{HeaderValue, Method, Uri};
use log::trace;
use ring::hmac;
use std::sync::Arc;

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
impl Policy for AuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        trace!("called AuthorizationPolicy::send. self == {:#?}", self);

        assert!(
            !next.is_empty(),
            "Authorization policies cannot be the last policy of a pipeline"
        );

        match &self.authorization_token {
            AuthorizationToken::SASToken {} => todo!(),
            AuthorizationToken::BearerToken {} => todo!(),
            AuthorizationToken::SharedKeyToken { account, key } => {
                let token = shared_key_token(
                    request.headers(),
                    request.uri(),
                    &request.method(),
                    account,
                    key,
                );
                request
                    .headers_mut()
                    .insert("authorization", HeaderValue::from_str(&token)?);
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
/// log example:
fn shared_key_token(
    headers: &Headers,
    uri: &Uri,
    method: &Method,
    account: &str,
    key: &str,
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
    let signature = hmac::sign(
        &hmac::Key::new(hmac::HMAC_SHA256, &base64::decode(key).unwrap()),
        to_sign.as_bytes(),
    );
    format!(
        "SharedKey {}:{}",
        account,
        base64::encode(signature.as_ref())
    )
}
