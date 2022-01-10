use crate::clients::{ServiceType, StorageCredentials};
use crate::headers::CONTENT_MD5;
use azure_core::{headers::MS_DATE, Context, Policy, PolicyResult, Request, Response};
use http::header::AUTHORIZATION;
use http::HeaderValue;
use http::{header::*, method::Method};
use ring::hmac;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorizationPolicy {
    storage_credentials: StorageCredentials,
}

impl AuthorizationPolicy {
    pub(crate) fn new(storage_credentials: StorageCredentials) -> Self {
        Self {
            storage_credentials,
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
    ) -> PolicyResult<Response> {
        if next.is_empty() {
            return Err(Box::new(azure_core::PipelineError::InvalidTailPolicy(
                "Authorization policies cannot be the last policy of a pipeline".to_owned(),
            )));
        }

        match &self.storage_credentials {
            StorageCredentials::Key(account, key) => {
                let url = url::Url::parse(&request.uri().to_string()).unwrap();
                let auth = generate_authorization(
                    request.headers(),
                    &url,
                    &request.method(),
                    account,
                    key,
                    ctx.get::<ServiceType>()
                        .expect("ServiceType must be in the Context at this point"),
                );
                request
                    .headers_mut()
                    .append(AUTHORIZATION, HeaderValue::from_str(&auth)?);
            }
            StorageCredentials::BearerToken(token) => {
                request
                    .headers_mut()
                    .append(AUTHORIZATION, HeaderValue::from_str(token)?);
            }
            // no headers to add here, the authentication is in the URL
            StorageCredentials::SASToken(_query_pairs) => (),
        };

        // Transport policy must always be last in the pipeline
        next[0].send(ctx, request, &next[1..]).await
    }
}

fn generate_authorization(
    h: &HeaderMap,
    u: &url::Url,
    method: &Method,
    account: &str,
    key: &str,
    service_type: &ServiceType,
) -> String {
    let str_to_sign = string_to_sign(h, u, method, account, service_type);

    // debug!("\nstr_to_sign == {:?}\n", str_to_sign);
    // debug!("str_to_sign == {}", str_to_sign);

    let auth = encode_str_to_sign(&str_to_sign, key);
    // debug!("auth == {:?}", auth);

    format!("SharedKey {}:{}", account, auth)
}

fn encode_str_to_sign(str_to_sign: &str, hmac_key: &str) -> String {
    let key = hmac::Key::new(ring::hmac::HMAC_SHA256, &base64::decode(hmac_key).unwrap());
    let sig = hmac::sign(&key, str_to_sign.as_bytes());

    // let res = hmac.result();
    // debug!("{:?}", res.code());

    base64::encode(sig.as_ref())
}

fn add_if_exists<K: AsHeaderName>(h: &HeaderMap, key: K) -> &str {
    match h.get(key) {
        Some(ce) => ce.to_str().unwrap(),
        None => "",
    }
}

#[allow(unknown_lints)]
fn string_to_sign(
    h: &HeaderMap,
    u: &url::Url,
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
            let cl = h
                .get(CONTENT_LENGTH)
                .map(|s| if s == "0" { "" } else { s.to_str().unwrap() })
                .unwrap_or("");
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
                method.as_str(),
                add_if_exists(h, CONTENT_ENCODING),
                add_if_exists(h, CONTENT_LANGUAGE),
                cl,
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

fn canonicalize_header(h: &HeaderMap) -> String {
    let mut v_headers = h
        .iter()
        .filter(|(k, _v)| k.as_str().starts_with("x-ms"))
        .map(|(k, _)| k.as_str())
        .collect::<Vec<_>>();
    v_headers.sort_unstable();

    let mut can = String::new();

    for header_name in v_headers {
        let s = h.get(header_name).unwrap().to_str().unwrap();
        can = can + header_name + ":" + s + "\n";
    }
    can
}

// For table
fn canonicalized_resource_table(account: &str, u: &url::Url) -> String {
    format!("/{}{}", account, u.path())
}

fn canonicalized_resource(account: &str, u: &url::Url) -> String {
    let mut can_res: String = String::new();
    can_res += "/";
    can_res += account;

    let paths = u.path_segments().unwrap();

    for p in paths {
        can_res.push('/');
        can_res.push_str(&*p);
    }
    can_res += "\n";

    // query parameters
    let query_pairs = u.query_pairs(); //.into_owned();
    {
        let mut qps = Vec::new();
        {
            for (q, _p) in query_pairs {
                trace!("adding to qps {:?}", q);

                // add only once
                if !(qps.iter().any(|x: &String| x == q.as_ref())) {
                    qps.push(q.into_owned());
                }
            }
        }

        qps.sort();

        for qparam in qps {
            // find correct parameter
            let ret = lexy_sort(&query_pairs, &qparam);

            // debug!("adding to can_res {:?}", ret);

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
    vec: &'a url::form_urlencoded::Parse,
    query_param: &str,
) -> Vec<std::borrow::Cow<'a, str>> {
    let mut v_values = Vec::new();

    for item in vec.filter(|x| x.0 == *query_param) {
        v_values.push(item.1)
    }
    v_values.sort();

    v_values
}
