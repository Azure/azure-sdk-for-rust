use azure_core::headers::{self, HeaderName, HeaderValue, Headers};
use azure_core::{date, Method};
use azure_core::{Context, Policy, PolicyResult, Request};
use azure_storage::{core::storage_shared_key_credential::StorageSharedKeyCredential, hmac::sign};
use std::sync::Arc;
use time::OffsetDateTime;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedKeyAuthorizationPolicy {
    credential: StorageSharedKeyCredential,
}

impl SharedKeyAuthorizationPolicy {
    pub(crate) fn new(credential: StorageSharedKeyCredential) -> Self {
        Self { credential }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl Policy for SharedKeyAuthorizationPolicy {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        assert!(
            !next.is_empty(),
            "Authorization policies cannot be the last policy of a pipeline"
        );

        request.insert_header(
            azure_core::headers::MS_DATE,
            HeaderValue::from(date::to_rfc1123(&OffsetDateTime::now_utc())),
        );
        request.insert_header(
            azure_core::headers::VERSION,
            HeaderValue::from_static("2019-12-12"),
        ); // TODO: Remove duplication with storage_client.rs

        let auth = generate_authorization(
            request.headers(),
            request.url(),
            request.method(),
            &self.credential.account_name,
            &self.credential.account_key,
        );

        request.insert_header(headers::AUTHORIZATION, HeaderValue::from(auth));

        // now next[0] is safe (will not panic) because we checked
        // at the beginning of the function.
        next[0].send(ctx, request, &next[1..]).await
    }
}

fn generate_authorization(
    http_headers: &Headers,
    url: &url::Url,
    http_method: &Method,
    storage_account_name: &str,
    shared_key: &str,
) -> String {
    let str_to_sign = string_to_sign(http_headers, url, http_method, storage_account_name);

    // println!("\nstr_to_sign == {:?}\n", str_to_sign);
    // debug!("str_to_sign == {}", str_to_sign);

    let auth = sign(&str_to_sign, shared_key).unwrap();
    // debug!("auth == {:?}", auth);

    format!("SharedKey {}:{}", storage_account_name, auth)
}

#[allow(unknown_lints)]
fn string_to_sign(
    http_headers: &Headers,
    url: &url::Url,
    http_method: &Method,
    storage_account_name: &str,
) -> String {
    // content length must only be specified if != 0
    // this is valid from 2015-02-21
    let cl = http_headers
        .get_optional_str(&headers::CONTENT_LENGTH)
        .filter(|&s| s != "0")
        .unwrap_or_default();
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
        http_method.as_ref(),
        add_if_exists(http_headers, &headers::CONTENT_ENCODING),
        add_if_exists(http_headers, &headers::CONTENT_LANGUAGE),
        cl,
        add_if_exists(http_headers, &headers::CONTENT_MD5),
        add_if_exists(http_headers, &headers::CONTENT_TYPE),
        add_if_exists(http_headers, &headers::DATE),
        add_if_exists(http_headers, &headers::IF_MODIFIED_SINCE),
        add_if_exists(http_headers, &headers::IF_MATCH),
        add_if_exists(http_headers, &headers::IF_NONE_MATCH),
        add_if_exists(http_headers, &headers::IF_UNMODIFIED_SINCE),
        add_if_exists(http_headers, &headers::RANGE),
        canonicalize_header(http_headers),
        canonicalized_resource(storage_account_name, url)
    )

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

fn add_if_exists<'a>(h: &'a Headers, key: &HeaderName) -> &'a str {
    h.get_optional_str(key).unwrap_or_default()
}

fn canonicalize_header(headers: &Headers) -> String {
    let mut names = headers
        .iter()
        .filter_map(|(k, _)| k.as_str().starts_with("x-ms").then(|| k))
        .collect::<Vec<_>>();
    names.sort_unstable();

    let mut result = String::new();

    for name in names {
        let value = headers.get_optional_str(name).unwrap();
        result = result + name.as_str() + ":" + value + "\n";
    }
    result
}

fn canonicalized_resource(account: &str, u: &url::Url) -> String {
    let mut can_res: String = String::new();
    can_res += "/";
    can_res += account;

    let paths = u.path_segments().unwrap();

    for p in paths {
        can_res.push('/');
        can_res.push_str(p);
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
