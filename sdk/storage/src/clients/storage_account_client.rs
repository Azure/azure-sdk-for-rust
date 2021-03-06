use crate::core::{ConnectionString, No};
use crate::shared_access_signature::SharedAccessSignatureBuilder;
use azure_core::errors::AzureError;
use azure_core::headers::*;
use azure_core::prelude::*;
use bytes::Bytes;
use http::header::*;
use http::method::Method;
use http::request::{Builder, Request};
use ring::hmac;
use std::sync::Arc;
use url::Url;

pub(crate) const HEADER_VERSION: &str = "x-ms-version";

pub(crate) const AZURE_VERSION: &str = "2019-12-12";
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
    blob_storage_url: Url,
    table_storage_url: Url,
    queue_storage_url: Url,
    queue_storage_secondary_url: Url,
    filesystem_url: Url,
}

fn get_sas_token_parms(sas_token: &str) -> Result<Vec<(String, String)>, url::ParseError> {
    // Any base url will do: we just need to parse the SAS token
    // to get its query pairs.
    let base_url = Url::parse("https://blob.core.windows.net")?;

    let url = Url::options().base_url(Some(&base_url));

    // this code handles the leading ?
    // we support both with or without
    let url = if sas_token.starts_with('?') {
        url.parse(sas_token)
    } else {
        url.parse(&format!("?{}", sas_token))
    }?;

    Ok(url
        .query_pairs()
        .map(|p| (String::from(p.0), String::from(p.1)))
        .collect())
}

impl StorageAccountClient {
    pub fn new_access_key<A, K>(
        http_client: Arc<Box<dyn HttpClient>>,
        account: A,
        key: K,
    ) -> Arc<Self>
    where
        A: Into<String>,
        K: Into<String>,
    {
        let account = account.into();

        Arc::new(Self {
            blob_storage_url: Url::parse(&format!("https://{}.blob.core.windows.net", &account))
                .unwrap(),
            table_storage_url: Url::parse(&format!("https://{}.table.core.windows.net", &account))
                .unwrap(),
            queue_storage_url: Url::parse(&format!("https://{}.queue.core.windows.net", &account))
                .unwrap(),
            queue_storage_secondary_url: Url::parse(&format!(
                "https://{}-secondary.queue.core.windows.net",
                &account
            ))
            .unwrap(),
            filesystem_url: Url::parse(&format!("https://{}.dfs.core.windows.net", &account))
                .unwrap(),
            storage_credentials: StorageCredentials::Key(account, key.into()),
            http_client,
        })
    }

    pub fn new_emulator(
        http_client: Arc<Box<dyn HttpClient>>,
        blob_storage_url: &Url,
        table_storage_url: &Url,
    ) -> Arc<Self> {
        let blob_storage_url =
            Url::parse(&format!("{}devstoreaccount1", blob_storage_url.as_str())).unwrap();
        let table_storage_url =
            Url::parse(&format!("{}devstoreaccount1", table_storage_url.as_str())).unwrap();
        let queue_storage_url =
            Url::parse(&format!("{}devstoreaccount1", table_storage_url.as_str())).unwrap();
        let filesystem_url =
            Url::parse(&format!("{}devstoreaccount1", blob_storage_url.as_str())).unwrap();

        Arc::new(Self {
            blob_storage_url,
            table_storage_url,
            queue_storage_url: queue_storage_url.clone(),
            queue_storage_secondary_url: queue_storage_url,
            filesystem_url,
            storage_credentials: StorageCredentials::Key(
        "devstoreaccount1".to_owned(),
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw=="
            .to_owned()),
            http_client,
        })
    }

    pub fn new_sas_token<A, S>(
        http_client: Arc<Box<dyn HttpClient>>,
        account: A,
        sas_token: S,
    ) -> Result<Arc<Self>, url::ParseError>
    where
        A: Into<String>,
        S: AsRef<str>,
    {
        let account = account.into();

        Ok(Arc::new(Self {
            blob_storage_url: Url::parse(&format!("https://{}.blob.core.windows.net", &account))?,
            table_storage_url: Url::parse(&format!("https://{}.table.core.windows.net", &account))?,
            queue_storage_url: Url::parse(&format!("https://{}.queue.core.windows.net", &account))?,
            queue_storage_secondary_url: Url::parse(&format!(
                "https://{}-secondary.queue.core.windows.net",
                &account
            ))?,
            filesystem_url: Url::parse(&format!("https://{}.dfs.core.windows.net", &account))?,
            storage_credentials: StorageCredentials::SASToken(get_sas_token_parms(
                sas_token.as_ref(),
            )?),
            http_client,
        }))
    }

    pub fn new_bearer_token<A, BT>(
        http_client: Arc<Box<dyn HttpClient>>,
        account: A,
        bearer_token: BT,
    ) -> Arc<Self>
    where
        A: Into<String>,
        BT: Into<String>,
    {
        let account = account.into();
        let bearer_token = bearer_token.into();

        Arc::new(Self {
            blob_storage_url: Url::parse(&format!("https://{}.blob.core.windows.net", &account))
                .unwrap(),
            table_storage_url: Url::parse(&format!("https://{}.table.core.windows.net", &account))
                .unwrap(),
            queue_storage_url: Url::parse(&format!("https://{}.queue.core.windows.net", &account))
                .unwrap(),
            queue_storage_secondary_url: Url::parse(&format!(
                "https://{}-secondary.queue.core.windows.net",
                &account
            ))
            .unwrap(),
            filesystem_url: Url::parse(&format!("https://{}.dfs.core.windows.net", &account))
                .unwrap(),
            storage_credentials: StorageCredentials::BearerToken(bearer_token),
            http_client,
        })
    }

    pub fn new_connection_string(
        http_client: Arc<Box<dyn HttpClient>>,
        connection_string: &str,
    ) -> Result<Arc<Self>, AzureError> {
        match ConnectionString::new(connection_string)? {
            ConnectionString {
                account_name: Some(account),
                account_key: Some(_),
                sas: Some(sas_token),
                blob_endpoint,
                table_endpoint,
                queue_endpoint,
                file_endpoint,
                ..
            } => {
                log::warn!("Both account key and SAS defined in connection string. Using only the provided SAS.");

                Ok(Arc::new(Self {
                    storage_credentials: StorageCredentials::SASToken(get_sas_token_parms(
                        sas_token,
                    )?),
                    blob_storage_url: get_endpoint_uri(blob_endpoint, account, "blob")?,
                    table_storage_url: get_endpoint_uri(table_endpoint, account, "table")?,
                    queue_storage_url: get_endpoint_uri(queue_endpoint, account, "queue")?,
                    queue_storage_secondary_url: get_endpoint_uri(queue_endpoint, &format!("{}-secondary", account), "queue")?,
                    filesystem_url: get_endpoint_uri(file_endpoint, account, "dfs")?,
                    http_client,
                }))
            }
            ConnectionString {
                account_name: Some(account),
                sas: Some(sas_token),
                blob_endpoint,
                table_endpoint,
                queue_endpoint,
                file_endpoint,
                ..
            } => Ok(Arc::new(Self {
                storage_credentials: StorageCredentials::SASToken(get_sas_token_parms(sas_token)?),
                blob_storage_url: get_endpoint_uri(blob_endpoint, account, "blob")?,
                table_storage_url: get_endpoint_uri(table_endpoint, account, "table")?,
                queue_storage_url: get_endpoint_uri(queue_endpoint, account, "queue")?,
                queue_storage_secondary_url: get_endpoint_uri(queue_endpoint, &format!("{}-secondary", account), "queue")?,
                filesystem_url: get_endpoint_uri(file_endpoint, account, "dfs")?,
                http_client,
            })),
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                blob_endpoint,
                table_endpoint,
                queue_endpoint,
                file_endpoint,
                ..
            } => Ok(Arc::new(Self {
                storage_credentials: StorageCredentials::Key(account.to_owned(), key.to_owned()),
                blob_storage_url: get_endpoint_uri(blob_endpoint, account, "blob")?,
                table_storage_url: get_endpoint_uri(table_endpoint, account, "table")?,
                queue_storage_url: get_endpoint_uri(queue_endpoint, account, "queue")?,
                queue_storage_secondary_url: get_endpoint_uri(queue_endpoint, &format!("{}-secondary", account), "queue")?,
                filesystem_url: get_endpoint_uri(file_endpoint, account, "dfs")?,
                http_client,
            })),
           _ => {
                Err(AzureError::GenericErrorWithText(
                    "Could not create a storage client from the provided connection string. Please validate that you have specified the account name and means of authentication (key, SAS, etc.)."
                        .to_owned(),
                ))
            }
        }
    }

    pub fn http_client(&self) -> &dyn HttpClient {
        self.http_client.as_ref().as_ref()
    }

    pub fn blob_storage_url(&self) -> &Url {
        &self.blob_storage_url
    }

    pub fn table_storage_url(&self) -> &Url {
        &self.table_storage_url
    }

    pub fn queue_storage_url(&self) -> &Url {
        &self.queue_storage_url
    }

    pub fn queue_storage_secondary_url(&self) -> &Url {
        &self.queue_storage_secondary_url
    }

    pub fn filesystem_url(&self) -> &Url {
        &self.filesystem_url
    }

    pub fn shared_access_signature(
        &self,
    ) -> Result<SharedAccessSignatureBuilder<No, No, No, No>, AzureError> {
        match self.storage_credentials {
            StorageCredentials::Key(ref account, ref key) => {
                Ok(SharedAccessSignatureBuilder::new(account, key))
            }
            _ => Err(AzureError::OperationNotSupported(
                "Shared access signature generation".to_owned(),
                "SAS can be generated only from key and account clients".to_owned(),
            )),
        }
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        service_type: ServiceType,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
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
        request = match request_body {
            Some(ref b) => request.header(CONTENT_LENGTH, &b.len().to_string() as &str),
            None => request.header(CONTENT_LENGTH, "0"),
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
                if url.query_pairs().find(|(k, _)| k == "sig").is_none() {
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
            request.body(Bytes::from_static(EMPTY_BODY))
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

fn get_endpoint_uri<URL>(
    url: Option<URL>,
    account: &str,
    endpoint_type: &str,
) -> Result<url::Url, url::ParseError>
where
    URL: AsRef<str>,
{
    Ok(match url {
        Some(value) => url::Url::parse(value.as_ref())?,
        None => url::Url::parse(&format!(
            "https://{}.{}.core.windows.net",
            account, endpoint_type
        ))?,
    })
}
