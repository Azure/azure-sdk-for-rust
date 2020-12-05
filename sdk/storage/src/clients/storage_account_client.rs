use azure_core::errors::AzureError;
use azure_core::headers::*;
use azure_core::HttpClient;
use azure_core::EMPTY_BODY;
use http::header::*;
use http::method::Method;
use http::request::{Builder, Request};
use ring::hmac;
use std::sync::Arc;
use url::Url;

pub(crate) const HEADER_VERSION: &str = "x-ms-version"; //=> [String] }

pub(crate) const AZURE_VERSION: &str = "2019-07-07";
//pub(crate) const SAS_VERSION: &str = "2019-02-02";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageCredentials {
    Key(String, String),
    SASToken(Vec<(String, String)>),
    BearerToken(String),
}

#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    Blob,
    // Queue,
    // File,
    Table,
}

#[derive(Debug, Clone)]
pub struct StorageAccountClient {
    storage_credentials: StorageCredentials,
    http_client: Arc<Box<dyn HttpClient>>,
    blob_storage_uri: String,
    table_storage_uri: String,
    queue_storage_uri: String,
    filesystem_uri: String,
}

fn get_sas_token_parms(sas_token: &str) -> Vec<(String, String)> {
    Url::options()
        // Any base url will do: we just need to parse the SAS token
        // to get its query pairs.
        .base_url(Some(&Url::parse("https://blob.core.windows.net").unwrap()))
        .parse(sas_token)
        .unwrap()
        .query_pairs()
        .map(|p| (String::from(p.0), String::from(p.1)))
        .collect()
}

impl StorageAccountClient {
    pub fn new_access_key<A, K>(
        http_client: Arc<Box<dyn HttpClient>>,
        account: A,
        key: K,
    ) -> Arc<Box<Self>>
    where
        A: Into<String>,
        K: Into<String>,
    {
        let account = account.into();

        Arc::new(Box::new(Self {
            blob_storage_uri: format!("https://{}.blob.core.windows.net", &account),
            table_storage_uri: format!("https://{}.table.core.windows.net", &account),
            queue_storage_uri: format!("https://{}.queue.core.windows.net", &account),
            filesystem_uri: format!("https://{}.dfs.core.windows.net", &account),
            storage_credentials: StorageCredentials::Key(account, key.into()),
            http_client,
        }))
    }

    pub fn new_sas_token<A, S>(
        http_client: Arc<Box<dyn HttpClient>>,
        account: A,
        sas_token: S,
    ) -> Arc<Box<Self>>
    where
        A: Into<String>,
        S: AsRef<str>,
    {
        let account = account.into();

        Arc::new(Box::new(Self {
            blob_storage_uri: format!("https://{}.blob.core.windows.net", &account),
            table_storage_uri: format!("https://{}.table.core.windows.net", &account),
            queue_storage_uri: format!("https://{}.queue.core.windows.net", &account),
            filesystem_uri: format!("https://{}.dfs.core.windows.net", &account),
            storage_credentials: StorageCredentials::SASToken(get_sas_token_parms(
                sas_token.as_ref(),
            )),
            http_client,
        }))
    }

    pub fn http_client(&self) -> &dyn HttpClient {
        self.http_client.as_ref().as_ref()
    }

    pub fn blob_storage_uri(&self) -> &str {
        &self.blob_storage_uri
    }

    pub fn table_storage_uri(&self) -> &str {
        &self.table_storage_uri
    }
    pub fn queue_storage_uri(&self) -> &str {
        &self.queue_storage_uri
    }

    pub fn filesystem_uri(&self) -> &str {
        &self.filesystem_uri
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        service_type: ServiceType,
        request_body: Option<&'a [u8]>,
    ) -> Result<(Request<&'a [u8]>, url::Url), AzureError> {
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

        let mut url = url::Url::parse(url)?;

        // if we have a SAS token (in form of query pairs), let's add it to the url here
        if let StorageCredentials::SASToken(query_pairs) = &self.storage_credentials {
            for (k, v) in query_pairs {
                url.query_pairs_mut().append_pair(k, v);
            }
        }

        let mut request = Request::builder();
        request = request.method(method).uri(url.as_str());

        // let's add content length to avoid "chunking" errors.
        match request_body {
            Some(ref b) => request = request.header(CONTENT_LENGTH, &b.len().to_string() as &str),
            None => request = request.header(CONTENT_LENGTH, "0"),
        };

        // This will give the caller the ability to add custom headers.
        // The closure is needed to because request.headers_mut().set_raw(...) requires
        // a Cow with 'static lifetime...
        request = http_header_adder(request);

        request = request
            .header(MS_DATE, time)
            .header(HEADER_VERSION, AZURE_VERSION);

        // We sign the request only if it is not already signed (with the signature of an
        // SAS token for example)
        let request = match &self.storage_credentials {
            StorageCredentials::Key(account, key) => {
                if url.query_pairs().find(|p| p.0 == "sig").is_none() {
                    let auth = generate_authorization(
                        request.headers_ref().unwrap(),
                        &url,
                        method,
                        account,
                        key,
                        service_type,
                    );
                    request.header(AUTHORIZATION, auth)
                } else {
                    request
                }
            }
            StorageCredentials::SASToken(_query_pairs) => {
                // no headers to add here, the authentication
                // is in the URL
                request
            }
            StorageCredentials::BearerToken(token) => {
                request.header(AUTHORIZATION, format!("Bearer {}", token))
            }
        };

        let request = if let Some(request_body) = request_body {
            request.body(request_body)
        } else {
            request.body(&EMPTY_BODY as &[u8])
        }?;

        debug!("using request == {:#?}", request);

        Ok((request, url))
    }

    //fn perform_table_request(
    //    &self,
    //    segment: &str,
    //    method: &Method,
    //    http_header_adder: &dyn Fn(Builder) -> Builder,
    //    request_str: Option<&[u8]>,
    //) -> Result<PerformRequestResponse, AzureError> {
    //    debug!("segment: {}, method: {:?}", segment, method,);

    //    let uri =
    //        self.add_sas_token_to_uri((self.get_uri_prefix(ServiceType::Table) + segment).as_str());

    //    debug!("perform_table_request uri: {}", uri);

    //    perform_request(
    //        self,
    //        &uri,
    //        method,
    //        http_header_adder,
    //        request_str,
    //        ServiceType::Table,
    //    )
    //}
}
//
//impl ClientEndpoint for KeyClient {
//    fn account(&self) -> &str {
//        &self.account
//    }
//
//    fn key(&self) -> &str {
//        &self.key
//    }
//}
//
//impl HyperClientEndpoint for KeyClient {
//    fn hyper_http_client(&self) -> &hyper::Client<HttpsConnector<hyper::http_client::HttpConnector>> {
//        &self.hc
//    }
//}

fn generate_authorization(
    h: &HeaderMap,
    u: &url::Url,
    method: &Method,
    account: &str,
    key: &str,
    service_type: ServiceType,
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
    service_type: ServiceType,
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
    can_res += &account;

    let paths = u.path_segments().unwrap();

    {
        let mut path = String::new();
        for p in paths {
            path.push('/');
            path.push_str(&*p);
        }

        can_res += &path;
    }
    can_res += "\n";

    // query parameters
    let query_pairs = u.query_pairs(); //.into_owned();
    {
        let mut qps = Vec::new();
        {
            for qp in query_pairs {
                trace!("adding to qps {:?}", qp);

                // add only once
                if !(qps.iter().any(|x: &String| x == &qp.0)) {
                    qps.push(qp.0.into_owned());
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

fn lexy_sort(vec: &url::form_urlencoded::Parse, query_param: &str) -> Vec<String> {
    let mut v_values: Vec<String> = Vec::new();

    for item in vec.filter(|x| x.0 == *query_param) {
        v_values.push(item.1.into_owned())
    }
    v_values.sort();

    v_values
}
