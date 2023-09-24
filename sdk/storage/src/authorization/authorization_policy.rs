use crate::{clients::ServiceType, StorageCredentials, StorageCredentialsInner};
use azure_core::{
    error::{ErrorKind, ResultExt},
    headers::*,
    Context, Method, Policy, PolicyResult, Request,
};
use std::{borrow::Cow, ops::Deref, sync::Arc};
use url::Url;

const STORAGE_TOKEN_SCOPE: &str = "https://storage.azure.com/";

#[derive(Debug, Clone)]
pub struct AuthorizationPolicy {
    credentials: StorageCredentials,
}

impl AuthorizationPolicy {
    pub(crate) fn new(credentials: StorageCredentials) -> Self {
        Self { credentials }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
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

        // lock the credentials within a scope so that it is released as soon as possible
        {
            let creds = self.credentials.0.lock().await;

            match creds.deref() {
                StorageCredentialsInner::Key(account, key) => {
                    if !request.url().query_pairs().any(|(k, _)| &*k == "sig") {
                        let auth = generate_authorization(
                            request.headers(),
                            request.url(),
                            *request.method(),
                            account,
                            key,
                            *ctx.get()
                                .expect("ServiceType must be in the Context at this point"),
                        )?;
                        request.insert_header(AUTHORIZATION, auth);
                    }
                }
                StorageCredentialsInner::SASToken(query_pairs) => {
                    // Ensure the signature param is not already present
                    if !request.url().query_pairs().any(|(k, _)| &*k == "sig") {
                        request
                            .url_mut()
                            .query_pairs_mut()
                            .extend_pairs(query_pairs);
                    }
                }
                StorageCredentialsInner::BearerToken(token) => {
                    request.insert_header(AUTHORIZATION, format!("Bearer {token}"));
                }
                StorageCredentialsInner::TokenCredential(token_credential) => {
                    let bearer_token = token_credential
                        .get_token(STORAGE_TOKEN_SCOPE)
                        .await
                        .context(ErrorKind::Credential, "failed to get bearer token")?;

                    request.insert_header(
                        AUTHORIZATION,
                        format!("Bearer {}", bearer_token.token.secret()),
                    );
                }
                StorageCredentialsInner::Anonymous => {}
            }
        };

        next[0].send(ctx, request, &next[1..]).await
    }
}

fn generate_authorization(
    h: &Headers,
    u: &Url,
    method: Method,
    account: &str,
    key: &str,
    service_type: ServiceType,
) -> azure_core::Result<String> {
    let str_to_sign = string_to_sign(h, u, method, account, service_type);
    let auth = crate::hmac::sign(&str_to_sign, key).context(
        azure_core::error::ErrorKind::Credential,
        "failed to sign the hmac",
    )?;
    Ok(format!("SharedKey {account}:{auth}"))
}

fn add_if_exists<'a>(h: &'a Headers, key: &HeaderName) -> &'a str {
    h.get_optional_str(key).unwrap_or_default()
}

#[allow(unknown_lints)]
fn string_to_sign(
    h: &Headers,
    u: &Url,
    method: Method,
    account: &str,
    service_type: ServiceType,
) -> String {
    if matches!(service_type, ServiceType::Table) {
        format!(
            "{}\n{}\n{}\n{}\n{}",
            method.as_ref(),
            add_if_exists(h, &CONTENT_MD5),
            add_if_exists(h, &CONTENT_TYPE),
            add_if_exists(h, &MS_DATE),
            canonicalized_resource_table(account, u)
        )
    } else {
        // content length must only be specified if != 0
        // this is valid from 2015-02-21
        let content_length = h
            .get_optional_str(&CONTENT_LENGTH)
            .filter(|&v| v != "0")
            .unwrap_or_default();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
            method.as_ref(),
            add_if_exists(h, &CONTENT_ENCODING),
            add_if_exists(h, &CONTENT_LANGUAGE),
            content_length,
            add_if_exists(h, &CONTENT_MD5),
            add_if_exists(h, &CONTENT_TYPE),
            add_if_exists(h, &DATE),
            add_if_exists(h, &IF_MODIFIED_SINCE),
            add_if_exists(h, &IF_MATCH),
            add_if_exists(h, &IF_NONE_MATCH),
            add_if_exists(h, &IF_UNMODIFIED_SINCE),
            add_if_exists(h, &RANGE),
            canonicalize_header(h),
            canonicalized_resource(account, u)
        )
    }
}

fn canonicalize_header(headers: &Headers) -> String {
    let mut names = headers
        .iter()
        .filter_map(|(k, _)| (k.as_str().starts_with("x-ms")).then_some(k))
        .collect::<Vec<_>>();
    names.sort_unstable();

    let mut result = String::new();

    for header_name in names {
        let value = headers.get_optional_str(header_name).unwrap();
        let name = header_name.as_str();
        result = format!("{result}{name}:{value}\n");
    }
    result
}

fn canonicalized_resource_table(account: &str, u: &Url) -> String {
    format!("/{}{}", account, u.path())
}

fn canonicalized_resource(account: &str, uri: &Url) -> String {
    let mut can_res: String = String::new();
    can_res += "/";
    can_res += account;

    for p in uri.path_segments().into_iter().flatten() {
        can_res.push('/');
        can_res.push_str(p);
    }
    can_res += "\n";

    // query parameters
    let query_pairs = uri.query_pairs();
    {
        let mut qps: Vec<String> = Vec::new();
        for (q, _) in query_pairs {
            if !(qps.iter().any(|x| x == &*q)) {
                qps.push(q.into_owned());
            }
        }

        qps.sort();

        for qparam in qps {
            // find correct parameter
            let ret = lexy_sort(query_pairs, &qparam);

            can_res = can_res + &qparam.to_lowercase() + ":";

            for (i, item) in ret.iter().enumerate() {
                if i > 0 {
                    can_res += ",";
                }
                can_res += item;
            }

            can_res += "\n";
        }
    };

    can_res[0..can_res.len() - 1].to_owned()
}

fn lexy_sort<'a>(
    vec: impl Iterator<Item = (Cow<'a, str>, Cow<'a, str>)> + 'a,
    query_param: &str,
) -> Vec<Cow<'a, str>> {
    let mut values = vec
        .filter(|(k, _)| *k == query_param)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    values.sort_unstable();
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{BytesStream, Response};

    #[derive(Debug, Clone)]
    struct AssertSigHeaderUniqueMockPolicy;

    #[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
    #[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
    impl Policy for AssertSigHeaderUniqueMockPolicy {
        async fn send(
            &self,
            _ctx: &Context,
            request: &mut Request,
            _next: &[Arc<dyn Policy>],
        ) -> PolicyResult {
            let sig_header_count = request
                .url()
                .query_pairs()
                .filter(|param| param.0 == "sig")
                .count();
            assert_eq!(sig_header_count, 1);

            Ok(Response::new(
                azure_core::StatusCode::Accepted,
                Headers::new(),
                Box::pin(BytesStream::new(vec![])),
            ))
        }
    }

    const SAMPLE_SAS_TOKEN: &str = "sp=r&st=1970-01-01T00:00:00Z&se=1970-01-01T00:00:00Z&spr=https&sv=1970-01-01&sr=c&sig=AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    #[tokio::test]
    async fn authorization_policy_applies_sas_token() {
        let ctx = Context::default();
        let storage_credentials = StorageCredentials::sas_token(SAMPLE_SAS_TOKEN).unwrap();
        let auth_policy = AuthorizationPolicy::new(storage_credentials);
        let mut request = Request::new(Url::parse("https://example.com").unwrap(), Method::Get);

        let assert_sig_header_unique_mock_policy = Arc::new(AssertSigHeaderUniqueMockPolicy);

        auth_policy
            .send(&ctx, &mut request, &[assert_sig_header_unique_mock_policy])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn authorization_policy_with_sas_token_does_not_apply_twice() {
        let ctx = Context::default();
        let storage_credentials = StorageCredentials::sas_token(SAMPLE_SAS_TOKEN).unwrap();
        let auth_policy = AuthorizationPolicy::new(storage_credentials);
        let mut request = Request::new(Url::parse("https://example.com").unwrap(), Method::Get);

        let assert_sig_header_unique_mock_policy = Arc::new(AssertSigHeaderUniqueMockPolicy);

        // apply policy twice
        auth_policy
            .send(
                &ctx,
                &mut request,
                &[assert_sig_header_unique_mock_policy.clone()],
            )
            .await
            .unwrap();
        auth_policy
            .send(&ctx, &mut request, &[assert_sig_header_unique_mock_policy])
            .await
            .unwrap();
    }
}
