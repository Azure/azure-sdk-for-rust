use azure_core::{Context, Policy, PolicyResult, Request, Response};
use azure_storage::core::storage_shared_key_credential::StorageSharedKeyCredential;
use http::{HeaderMap, HeaderValue, Method};
use ring::hmac;
use std::sync::Arc;

const AZURE_VERSION: &str = "2019-12-12";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedKeyAuthorizationPolicy {
    base_url: String,
    credential: StorageSharedKeyCredential,
}

impl SharedKeyAuthorizationPolicy {
    pub(crate) fn new(base_url: String, credential: StorageSharedKeyCredential) -> Self {
        Self {
            base_url,
            credential,
        }
    }
}

#[async_trait::async_trait]
impl Policy for SharedKeyAuthorizationPolicy {
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

        let headers_mut = request.headers_mut();
        headers_mut.append(
            azure_core::headers::MS_DATE,
            HeaderValue::from_str(
                &chrono::Utc::now().format("%a, %d %h %Y %T GMT").to_string(),
            )?,
        );
        headers_mut.append(
            azure_core::headers::VERSION,
            HeaderValue::from_str(AZURE_VERSION)?,
        ); // TODO: Remove duplication with storage_account_client.rs

        let url = url::Url::parse(&request.uri().to_string()).unwrap();
        let auth = generate_authorization(
            request.headers(),
            &url,
            &request.method(),
            &self.credential.account_name,
            &self.credential.account_key,
        );

        request
            .headers_mut()
            .append(http::header::AUTHORIZATION, HeaderValue::from_str(&auth)?);

        // now next[0] is safe (will not panic) because we checked
        // at the beginning of the function.
        next[0].send(ctx, request, &next[1..]).await
    }
}

fn generate_authorization(
    http_headers: &HeaderMap,
    url: &url::Url,
    http_method: &Method,
    storage_account_name: &str,
    shared_key: &str,
) -> String {
    let str_to_sign = string_to_sign(http_headers, url, http_method, storage_account_name);

    // println!("\nstr_to_sign == {:?}\n", str_to_sign);
    // debug!("str_to_sign == {}", str_to_sign);

    let auth = encode_str_to_sign(&str_to_sign, shared_key);
    // debug!("auth == {:?}", auth);

    format!("SharedKey {}:{}", storage_account_name, auth)
}

#[allow(unknown_lints)]
fn string_to_sign(
    http_headers: &HeaderMap,
    url: &url::Url,
    http_method: &Method,
    storage_account_name: &str,
) -> String {
    // content lenght must only be specified if != 0
    // this is valid from 2015-02-21
    let cl = http_headers
        .get(http::header::CONTENT_LENGTH)
        .map(|s| if s == "0" { "" } else { s.to_str().unwrap() })
        .unwrap_or("");
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
        http_method.as_str(),
        add_if_exists(http_headers, http::header::CONTENT_ENCODING),
        add_if_exists(http_headers, http::header::CONTENT_LANGUAGE),
        cl,
        add_if_exists(http_headers, azure_storage::headers::CONTENT_MD5),
        add_if_exists(http_headers, http::header::CONTENT_TYPE),
        add_if_exists(http_headers, http::header::DATE),
        add_if_exists(http_headers, http::header::IF_MODIFIED_SINCE),
        add_if_exists(http_headers, http::header::IF_MATCH),
        add_if_exists(http_headers, http::header::IF_NONE_MATCH),
        add_if_exists(http_headers, http::header::IF_UNMODIFIED_SINCE),
        add_if_exists(http_headers, http::header::RANGE),
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

fn add_if_exists<K: http::header::AsHeaderName>(h: &HeaderMap, key: K) -> &str {
    match h.get(key) {
        Some(ce) => ce.to_str().unwrap(),
        None => "",
    }
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

fn canonicalized_resource(account: &str, u: &url::Url) -> String {
    let mut can_res = String::new();
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

fn encode_str_to_sign(str_to_sign: &str, hmac_key: &str) -> String {
    let key = hmac::Key::new(ring::hmac::HMAC_SHA256, &base64::decode(hmac_key).unwrap());
    let sig = hmac::sign(&key, str_to_sign.as_bytes());

    // let res = hmac.result();
    // debug!("{:?}", res.code());

    base64::encode(sig.as_ref())
}
