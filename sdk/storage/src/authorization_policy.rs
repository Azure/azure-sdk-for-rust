use azure_core::error::{ErrorKind, ResultExt};
use azure_core::{headers::*, Context, Policy, PolicyResult, Request};
use http::header::AUTHORIZATION;
use http::{Method, Uri};
use std::sync::Arc;

use crate::clients::{ServiceType, StorageCredentials};

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
        let request = match &self.credentials {
            StorageCredentials::Key(account, key) => {
                if !request
                    .uri()
                    .query()
                    .unwrap_or_default()
                    .split("&")
                    .any(|pair| matches!(pair.trim().split_once("="), Some(("sig", _))))
                {
                    let auth = generate_authorization(
                        request.headers(),
                        request.uri(),
                        &request.method(),
                        account,
                        key,
                        ctx.get()
                            .expect("ServiceType must be in the Context at this point"),
                    );
                    request.headers_mut().insert(AUTHORIZATION, auth)
                }
                request
            }
            StorageCredentials::SASToken(query_pairs) => {
                // TODO: switch to `url` crate.
                // This is already very complex and we're not even url encoding
                let query = request.uri().query();
                let new = query_pairs
                    .iter()
                    .map(|(k, v)| format!("{k}={v}"))
                    .collect::<Vec<String>>()
                    .join("&");
                let new = match query {
                    Some(existing) => format!("{existing}&{new}"),
                    None => format!("?{new}"),
                };
                let new = format!("{}{}", request.uri().path(), new);
                let mut parts = request.uri().clone().into_parts();
                parts.path_and_query =
                    Some(http::uri::PathAndQuery::from_maybe_shared(new).unwrap());
                *request.uri_mut() = Uri::from_parts(parts).unwrap();
                request
            }
            StorageCredentials::BearerToken(token) => {
                request
                    .headers_mut()
                    .insert(AUTHORIZATION, format!("Bearer {}", token));
                request
            }
            StorageCredentials::TokenCredential(token_credential) => {
                let bearer_token_future = token_credential.get_token(STORAGE_TOKEN_SCOPE);
                let bearer_token = futures::executor::block_on(bearer_token_future)
                    .context(ErrorKind::Credential, "failed to get bearer token")?;

                request.headers_mut().insert(
                    AUTHORIZATION,
                    format!("Bearer {}", bearer_token.token.secret()),
                );
                request
            }
        };

        next[0].send(ctx, request, &next[1..]).await
    }
}

fn generate_authorization(
    h: &Headers,
    u: &Uri,
    method: &Method,
    account: &str,
    key: &str,
    service_type: &ServiceType,
) -> String {
    let str_to_sign = string_to_sign(h, u, method, account, &service_type);
    let auth = crate::hmac::sign(&str_to_sign, key).unwrap();
    format!("SharedKey {}:{}", account, auth)
}

fn add_if_exists<'a>(h: &'a Headers, key: &'static str) -> &'a str {
    h.get(key).map(|ce| ce.as_str()).unwrap_or_default()
}

#[allow(unknown_lints)]
fn string_to_sign(
    h: &Headers,
    u: &Uri,
    method: &Method,
    account: &str,
    service_type: &ServiceType,
) -> String {
    match service_type {
        ServiceType::Table => {
            format!(
                "{}\n{}\n{}\n{}\n{}",
                method.as_str(),
                add_if_exists(h, CONTENT_MD5),
                add_if_exists(h, CONTENT_TYPE),
                add_if_exists(h, MS_DATE),
                canonicalized_resource_table(account, u)
            )
        }
        _ => {
            // content lenght must only be specified if != 0
            // this is valid from 2015-02-21
            let content_length = h
                .get(CONTENT_LENGTH)
                .map(|v| if v.as_str() == "0" { "" } else { v.as_str() })
                .unwrap_or_default();
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
                method.as_str(),
                add_if_exists(h, CONTENT_ENCODING),
                add_if_exists(h, CONTENT_LANGUAGE),
                content_length,
                add_if_exists(h, CONTENT_MD5),
                add_if_exists(h, CONTENT_TYPE),
                add_if_exists(h, DATE),
                add_if_exists(h, IF_MODIFIED_SINCE),
                add_if_exists(h, IF_MATCH),
                add_if_exists(h, IF_NONE_MATCH),
                add_if_exists(h, IF_UNMODIFIED_SINCE),
                add_if_exists(h, RANGE),
                canonicalize_header(h),
                canonicalized_resource(account, u)
            )
        }
    }

    // expected
    // GET\n /*HTTP Verb*/
    // \n    /*Content-Encoding*/
    // \n    /*Content-Language*/
    // \n    /*Content-Length (include value when zero)*/
    // \n    /*Content-MD5*/
    // \n    /*Content-Type*/
    // \n    /*Date*/
    // \n    /*If-Modified-Since */
    // \n    /*If-Match*/
    // \n    /*If-None-Match*/
    // \n    /*If-Unmodified-Since*/
    // \n    /*Range*/
    // x-ms-date:Sun, 11 Oct 2009 21:49:13 GMT\nx-ms-version:2009-09-19\n
    //                                  /*CanonicalizedHeaders*/
    // /myaccount /mycontainer\ncomp:metadata\nrestype:container\ntimeout:20
    //                                  /*CanonicalizedResource*/
    //
    //
}

fn canonicalize_header(h: &Headers) -> String {
    let mut v_headers = h
        .iter()
        .filter(|(k, _)| k.as_str().starts_with("x-ms"))
        .map(|(k, _)| k.as_str().to_owned())
        .collect::<Vec<_>>();
    v_headers.sort_unstable();

    let mut can = String::new();

    for header_name in v_headers {
        let s = h.get(header_name.clone()).unwrap().as_str();
        can = format!("{can}{header_name}:{s}\n");
    }
    can
}

fn canonicalized_resource_table(account: &str, u: &Uri) -> String {
    format!("/{}{}", account, u.path())
}

fn canonicalized_resource(account: &str, uri: &Uri) -> String {
    let mut can_res: String = String::new();
    can_res += "/";
    can_res += account;

    let path = uri.path();

    for p in path.split("/") {
        can_res.push('/');
        can_res.push_str(&*p);
    }
    can_res += "\n";

    // query parameters
    let query_pairs = uri
        .query()
        .unwrap_or_default()
        .split("&")
        .filter_map(|p| p.split_once("="));
    {
        let mut qps = Vec::new();
        for (q, _p) in query_pairs.clone() {
            if !(qps.iter().any(|x| x == q)) {
                qps.push(q.to_owned());
            }
        }

        qps.sort();

        for qparam in qps {
            // find correct parameter
            let ret = lexy_sort(query_pairs.clone(), &qparam);

            can_res = can_res + &qparam.to_lowercase() + ":";

            for (i, item) in ret.iter().enumerate() {
                if i > 0 {
                    can_res += ","
                }
                can_res += item;
            }

            can_res += "\n";
        }
    };

    can_res[0..can_res.len() - 1].to_owned()
}

fn lexy_sort<'a>(
    vec: impl Iterator<Item = (&'a str, &'a str)> + 'a,
    query_param: &str,
) -> Vec<&'a str> {
    let mut values = vec
        .filter(|(k, _)| *k == query_param)
        .map(|(_, v)| v)
        .collect::<Vec<_>>();
    values.sort();
    values
}
