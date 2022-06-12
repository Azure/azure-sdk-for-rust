use super::AuthorizationToken;
use azure_core::{
    headers::{HeaderName, Headers},
    Context, Policy, PolicyResult, Request,
};
use hmac::{Hmac, Mac};
use http::Method;
use log::trace;
use sha2::Sha256;
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

        let value = match &self.authorization_token {
            AuthorizationToken::SASToken {} => todo!(),
            AuthorizationToken::BearerToken {} => todo!(),
            AuthorizationToken::SharedKeyToken { account, key } => {
                let data = string_to_sign(
                    &request.method(),
                    request.headers(),
                    account,
                    request.uri().path(),
                );
                let signature = sign_and_encode(&data, key.as_bytes());
                format!("SharedKey {}:{}", account, signature)
            }
        };
        request.headers_mut().insert("authorization", value);
        next[0].send(ctx, request, &next[1..]).await
    }
}

fn sign_and_encode(data: &str, key: &[u8]) -> String {
    let key = base64::decode(key).unwrap();
    let mut hmac = Hmac::<Sha256>::new_from_slice(&key).unwrap();
    hmac.update(data.as_bytes());
    let signature = hmac.finalize().into_bytes();
    base64::encode(&signature)
}

/// from the docs, to create the token first create string to sign, the string should contain the following:
/// * http method
/// * md5 content (if exists else empty string and new a line)
/// * content type (if exists else empty string and new a line)
/// * x-ms-date (utc new formatted as 'Wed, 18 Aug 2021 14:52:59 GMT')
/// * canonicalized resource (for example, /devstoreaccount1/devstoreaccount1/Tables)
fn string_to_sign(method: &Method, headers: &Headers, account: &str, path: &str) -> String {
    format!(
        "{}\n{}\n{}\n{}\n/{}{}",
        method.as_str(),
        headers
            .get(&HeaderName::from("content-md5"))
            .map_or("", |v| v.as_str()),
        headers
            .get(&HeaderName::from("content-type"))
            .map_or("", |v| v.as_str()),
        headers
            .get(&HeaderName::from("x-ms-date"))
            .map_or("", |v| v.as_str()),
        account,
        path
    )
}
