use crate::authorization_policy::AuthorizationPolicy;
use crate::headers::CONTENT_MD5;
use crate::ConnectionString;
use crate::{
    core::No,
    hmac::sign,
    shared_access_signature::account_sas::{
        AccountSharedAccessSignatureBuilder, ClientAccountSharedAccessSignature,
    },
};
use azure_core::auth::TokenCredential;
use azure_core::error::{Error, ErrorKind, ResultExt};
use azure_core::Request;
use azure_core::{headers::*, Pipeline};
use azure_core::{ClientOptions, HttpClient};
use bytes::Bytes;
use http::method::Method;
use std::sync::Arc;
use url::Url;

/// The well-known account used by Azurite and the legacy Azure Storage Emulator.
/// https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key
pub const EMULATOR_ACCOUNT: &str = "devstoreaccount1";

/// The well-known account key used by Azurite and the legacy Azure Storage Emulator.
/// https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key
pub const EMULATOR_ACCOUNT_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";

pub const STORAGE_TOKEN_SCOPE: &str = "https://storage.azure.com/";

const HEADER_VERSION: &str = "x-ms-version";

const AZURE_VERSION: &str = "2019-12-12";

#[derive(Clone)]
pub enum StorageCredentials {
    Key(String, String),
    SASToken(Vec<(String, String)>),
    BearerToken(String),
    TokenCredential(Arc<dyn TokenCredential>),
}

impl std::fmt::Debug for StorageCredentials {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            StorageCredentials::TokenCredential(_) => f
                .debug_struct("StorageCredentials")
                .field("credential", &"TokenCredential")
                .finish(),
            _ => self.fmt(f),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ServiceType {
    Blob,
    // Queue,
    // File,
    Table,
}

#[derive(Debug)]
pub struct StorageAccountClient {
    storage_credentials: StorageCredentials,
    http_client: Arc<dyn HttpClient>,
    blob_storage_url: Url,
    table_storage_url: Url,
    queue_storage_url: Url,
    queue_storage_secondary_url: Url,
    filesystem_url: Url,
    account: String,
    pipeline: Pipeline,
}

fn get_sas_token_parms(sas_token: &str) -> azure_core::Result<Vec<(String, String)>> {
    // Any base url will do: we just need to parse the SAS token
    // to get its query pairs.
    let base_url = Url::parse("https://blob.core.windows.net").unwrap();

    let url = Url::options().base_url(Some(&base_url));

    // this code handles the leading ?
    // we support both with or without
    let url = if sas_token.starts_with('?') {
        url.parse(sas_token)
    } else {
        url.parse(&format!("?{}", sas_token))
    }
    .with_context(ErrorKind::DataConversion, || {
        format!("failed to parse SAS token: {sas_token}")
    })?;

    Ok(url
        .query_pairs()
        .map(|p| (String::from(p.0), String::from(p.1)))
        .collect())
}

impl StorageAccountClient {
    pub fn new_access_key<A, K>(http_client: Arc<dyn HttpClient>, account: A, key: K) -> Arc<Self>
    where
        A: Into<String>,
        K: Into<String>,
    {
        let account = account.into();
        let key = key.into();
        let storage_credentials = StorageCredentials::Key(account.clone(), key);
        let pipeline =
            new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());

        Arc::new(Self {
            blob_storage_url: get_endpoint_uri(None, &account, "blob").unwrap(),
            table_storage_url: get_endpoint_uri(None, &account, "table").unwrap(),
            queue_storage_url: get_endpoint_uri(None, &account, "queue").unwrap(),
            queue_storage_secondary_url: get_endpoint_uri(
                None,
                &format!("{account}-secondary"),
                "queue",
            )
            .unwrap(),
            filesystem_url: get_endpoint_uri(None, &account, "dfs").unwrap(),
            storage_credentials,
            http_client,
            account,
            pipeline,
        })
    }

    /// Create a new client for customized emulator endpoints.
    #[must_use]
    pub fn new_emulator(
        http_client: Arc<dyn HttpClient>,
        blob_storage_url: &Url,
        table_storage_url: &Url,
        queue_storage_url: &Url,
        filesystem_url: &Url,
    ) -> Arc<Self> {
        Self::new_emulator_with_account(
            http_client,
            blob_storage_url,
            table_storage_url,
            queue_storage_url,
            filesystem_url,
            EMULATOR_ACCOUNT,
            EMULATOR_ACCOUNT_KEY,
        )
    }

    /// Create a new client using the default HttpClient and the default emulator endpoints.
    #[must_use]
    pub fn new_emulator_default() -> Arc<Self> {
        let http_client = azure_core::new_http_client();
        let blob_storage_url = Url::parse("http://127.0.0.1:10000").unwrap();
        let queue_storage_url = Url::parse("http://127.0.0.1:10001").unwrap();
        let table_storage_url = Url::parse("http://127.0.0.1:10002").unwrap();
        let filesystem_url = Url::parse("http://127.0.0.1:10004").unwrap();
        Self::new_emulator(
            http_client,
            &blob_storage_url,
            &table_storage_url,
            &queue_storage_url,
            &filesystem_url,
        )
    }

    pub fn new_emulator_with_account<A, K>(
        http_client: Arc<dyn HttpClient>,
        blob_storage_url: &Url,
        table_storage_url: &Url,
        queue_storage_url: &Url,
        filesystem_url: &Url,
        account: A,
        key: K,
    ) -> Arc<Self>
    where
        A: Into<String>,
        K: Into<String>,
    {
        let account = account.into();
        let key = key.into();
        let storage_credentials = StorageCredentials::Key(account.clone(), key.clone());
        let pipeline = new_pipeline_from_options(StorageOptions::new(), storage_credentials);
        let blob_storage_url =
            Url::parse(&format!("{}{}", blob_storage_url.as_str(), account)).unwrap();
        let table_storage_url =
            Url::parse(&format!("{}{}", table_storage_url.as_str(), account)).unwrap();
        let queue_storage_url =
            Url::parse(&format!("{}{}", queue_storage_url.as_str(), account)).unwrap();
        let filesystem_url =
            Url::parse(&format!("{}{}", filesystem_url.as_str(), account)).unwrap();

        Arc::new(Self {
            blob_storage_url,
            table_storage_url,
            queue_storage_url: queue_storage_url.clone(),
            queue_storage_secondary_url: queue_storage_url,
            filesystem_url,
            storage_credentials: StorageCredentials::Key(account.clone(), key),
            http_client,
            account,
            pipeline,
        })
    }

    pub fn new_sas_token<A, S>(
        http_client: Arc<dyn HttpClient>,
        account: A,
        sas_token: S,
    ) -> azure_core::Result<Arc<Self>>
    where
        A: Into<String>,
        S: AsRef<str>,
    {
        let account = account.into();

        let storage_credentials =
            StorageCredentials::SASToken(get_sas_token_parms(sas_token.as_ref())?);
        let pipeline =
            new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());

        Ok(Arc::new(Self {
            blob_storage_url: get_endpoint_uri(None, &account, "blob")?,
            table_storage_url: get_endpoint_uri(None, &account, "table")?,
            queue_storage_url: get_endpoint_uri(None, &account, "queue")?,
            queue_storage_secondary_url: get_endpoint_uri(
                None,
                &format!("{account}-secondary"),
                "queue",
            )?,
            filesystem_url: get_endpoint_uri(None, &account, "dfs")?,
            storage_credentials,
            http_client,
            account,
            pipeline,
        }))
    }

    pub fn new_bearer_token<A, BT>(
        http_client: Arc<dyn HttpClient>,
        account: A,
        bearer_token: BT,
    ) -> Arc<Self>
    where
        A: Into<String>,
        BT: Into<String>,
    {
        let account = account.into();
        let bearer_token = bearer_token.into();
        let storage_credentials = StorageCredentials::BearerToken(bearer_token);
        let pipeline =
            new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());

        Arc::new(Self {
            blob_storage_url: get_endpoint_uri(None, &account, "blob").unwrap(),
            table_storage_url: get_endpoint_uri(None, &account, "table").unwrap(),
            queue_storage_url: get_endpoint_uri(None, &account, "queue").unwrap(),
            queue_storage_secondary_url: get_endpoint_uri(
                None,
                &format!("{}-secondary", account),
                "queue",
            )
            .unwrap(),
            filesystem_url: get_endpoint_uri(None, &account, "dfs").unwrap(),
            storage_credentials,
            http_client,
            account,
            pipeline,
        })
    }

    pub fn new_token_credential<A>(
        http_client: Arc<dyn HttpClient>,
        account: A,
        token_credential: Arc<dyn TokenCredential>,
    ) -> Arc<Self>
    where
        A: Into<String>,
    {
        let account = account.into();
        let storage_credentials = StorageCredentials::TokenCredential(token_credential);
        let pipeline =
            new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());

        Arc::new(Self {
            blob_storage_url: get_endpoint_uri(None, &account, "blob").unwrap(),
            table_storage_url: get_endpoint_uri(None, &account, "table").unwrap(),
            queue_storage_url: get_endpoint_uri(None, &account, "queue").unwrap(),
            queue_storage_secondary_url: get_endpoint_uri(
                None,
                &format!("{}-secondary", account),
                "queue",
            )
            .unwrap(),
            filesystem_url: get_endpoint_uri(None, &account, "dfs").unwrap(),
            storage_credentials,
            http_client,
            account,
            pipeline,
        })
    }

    pub fn new_connection_string(
        http_client: Arc<dyn HttpClient>,
        connection_string: &str,
    ) -> azure_core::Result<Arc<Self>> {
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

                let storage_credentials =  StorageCredentials::SASToken(get_sas_token_parms(
                    sas_token,
                )?);
                let pipeline = new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());

                Ok(Arc::new(Self {
                    storage_credentials,
                    blob_storage_url: get_endpoint_uri(blob_endpoint, account, "blob")?,
                    table_storage_url: get_endpoint_uri(table_endpoint, account, "table")?,
                    queue_storage_url: get_endpoint_uri(queue_endpoint, account, "queue")?,
                    queue_storage_secondary_url: get_endpoint_uri(queue_endpoint, &format!("{}-secondary", account), "queue")?,
                    filesystem_url: get_endpoint_uri(file_endpoint, account, "dfs")?,
                    http_client,
                    account: account.to_string(),
                    pipeline
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
            } => {
                let storage_credentials = StorageCredentials::SASToken(get_sas_token_parms(sas_token)?);
                let pipeline =
                new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());
                Ok(Arc::new(Self {
                    storage_credentials,
                    blob_storage_url: get_endpoint_uri(blob_endpoint, account, "blob")?,
                    table_storage_url: get_endpoint_uri(table_endpoint, account, "table")?,
                    queue_storage_url: get_endpoint_uri(queue_endpoint, account, "queue")?,
                    queue_storage_secondary_url: get_endpoint_uri(queue_endpoint, &format!("{}-secondary", account), "queue")?,
                    filesystem_url: get_endpoint_uri(file_endpoint, account, "dfs")?,
                    http_client,
                    account: account.to_string(),
                    pipeline
            }))},
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                blob_endpoint,
                table_endpoint,
                queue_endpoint,
                file_endpoint,
                ..
            } => {

                let storage_credentials = StorageCredentials::Key(account.to_owned(), key.to_owned());
                let pipeline = new_pipeline_from_options(StorageOptions::new(), storage_credentials.clone());
                Ok(Arc::new(Self {
                storage_credentials,
                blob_storage_url: get_endpoint_uri(blob_endpoint, account, "blob")?,
                table_storage_url: get_endpoint_uri(table_endpoint, account, "table")?,
                queue_storage_url: get_endpoint_uri(queue_endpoint, account, "queue")?,
                queue_storage_secondary_url: get_endpoint_uri(queue_endpoint, &format!("{}-secondary", account), "queue")?,
                filesystem_url: get_endpoint_uri(file_endpoint, account, "dfs")?,
                http_client,
                account: account.to_string(),
                pipeline
            }))
        },
           _ => {
                Err(Error::message(ErrorKind::Other,
                    "Could not create a storage client from the provided connection string. Please validate that you have specified the account name and means of authentication (key, SAS, etc.)."
                ))
            }
        }
    }

    #[must_use]
    pub fn http_client(&self) -> &dyn HttpClient {
        self.http_client.as_ref()
    }

    #[must_use]
    pub fn blob_storage_url(&self) -> &Url {
        &self.blob_storage_url
    }

    #[must_use]
    pub fn table_storage_url(&self) -> &Url {
        &self.table_storage_url
    }

    #[must_use]
    pub fn queue_storage_url(&self) -> &Url {
        &self.queue_storage_url
    }

    #[must_use]
    pub fn queue_storage_secondary_url(&self) -> &Url {
        &self.queue_storage_secondary_url
    }

    #[must_use]
    pub fn filesystem_url(&self) -> &Url {
        &self.filesystem_url
    }

    #[must_use]
    pub fn account(&self) -> &str {
        &self.account
    }

    #[must_use]
    pub fn storage_credentials(&self) -> &StorageCredentials {
        &self.storage_credentials
    }

    pub fn prepare_request(
        &self,
        url: &str,
        method: Method,
        service_type: ServiceType,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        let dt = chrono::Utc::now();
        let time = format!("{}", dt.format("%a, %d %h %Y %T GMT"));

        let mut url = url::Url::parse(url).with_context(ErrorKind::DataConversion, || {
            format!("failed to parse request url: {url}")
        })?;

        // if we have a SAS token (in form of query pairs), let's add it to the url here
        if let StorageCredentials::SASToken(query_pairs) = &self.storage_credentials {
            for (k, v) in query_pairs {
                url.query_pairs_mut().append_pair(k, v);
            }
        }

        let mut request = Request::new(url, method);

        // let's add content length to avoid "chunking" errors.
        match request_body {
            Some(ref b) => request.insert_header(CONTENT_LENGTH, b.len().to_string()),
            None => request.insert_header(CONTENT_LENGTH, "0"),
        };

        request.insert_header(MS_DATE, time);
        request.insert_header(HEADER_VERSION, AZURE_VERSION);

        // We sign the request only if it is not already signed (with the signature of an
        // SAS token for example)
        match &self.storage_credentials {
            StorageCredentials::Key(account, key) => {
                if !request.url().query_pairs().any(|(k, _)| k == "sig") {
                    let auth = generate_authorization(
                        request.headers(),
                        request.url(),
                        request.method(),
                        account,
                        key,
                        service_type,
                    );
                    request.insert_header(AUTHORIZATION, auth);
                }
            }
            StorageCredentials::SASToken(_query_pairs) => {
                // no headers to add here, the authentication is in the URL
            }
            StorageCredentials::BearerToken(token) => {
                request.insert_header(AUTHORIZATION, format!("Bearer {}", token))
            }
            StorageCredentials::TokenCredential(token_credential) => {
                let bearer_token_future = token_credential.get_token(STORAGE_TOKEN_SCOPE);
                let bearer_token = futures::executor::block_on(bearer_token_future)
                    .context(ErrorKind::Credential, "failed to get bearer token")?;

                request.insert_header(
                    AUTHORIZATION,
                    format!("Bearer {}", bearer_token.token.secret()),
                )
            }
        };

        if let Some(request_body) = request_body {
            request.set_body(request_body);
        } else {
            request.set_body(azure_core::EMPTY_BODY);
        };

        Ok(request)
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        &self.pipeline
    }

    /// Prepares' an `azure_core::Request`.
    pub(crate) fn blob_storage_request(&self, http_method: http::Method) -> Request {
        Request::new(self.blob_storage_url().clone(), http_method)
    }
}

impl ClientAccountSharedAccessSignature for StorageAccountClient {
    fn shared_access_signature(
        &self,
    ) -> azure_core::Result<AccountSharedAccessSignatureBuilder<No, No, No, No>> {
        match self.storage_credentials {
            StorageCredentials::Key(ref account, ref key) => {
                Ok(AccountSharedAccessSignatureBuilder::new(account, key))
            }
            _ => Err(Error::message(ErrorKind::Other, "failed shared access signature generation. SAS can be generated only from key and account clients")),
        }
    }
}

fn generate_authorization(
    headers: &Headers,
    url: &url::Url,
    method: &Method,
    account: &str,
    key: &str,
    service_type: ServiceType,
) -> String {
    let str_to_sign = string_to_sign(headers, url, method, account, service_type);
    let auth = sign(&str_to_sign, key).unwrap();
    format!("SharedKey {}:{}", account, auth)
}

fn add_if_exists<K: Into<HeaderName>>(headers: &Headers, key: K) -> &str {
    match headers.get(key.into()) {
        Some(value) => value.as_str(),
        None => "",
    }
}

#[allow(unknown_lints)]
fn string_to_sign(
    headers: &Headers,
    url: &url::Url,
    method: &Method,
    account: &str,
    service_type: ServiceType,
) -> String {
    match service_type {
        ServiceType::Table => {
            format!(
                "{}\n{}\n{}\n{}\n{}",
                method.as_str(),
                add_if_exists(headers, CONTENT_MD5),
                add_if_exists(headers, CONTENT_TYPE),
                add_if_exists(headers, MS_DATE),
                canonicalized_resource_table(account, url)
            )
        }
        _ => {
            // content lenght must only be specified if != 0
            // this is valid from 2015-02-21
            let cl = headers
                .get(CONTENT_LENGTH)
                .map(|s| if s.as_str() == "0" { "" } else { s.as_str() })
                .unwrap_or("");
            format!(
                "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}{}",
                method.as_str(),
                add_if_exists(headers, CONTENT_ENCODING),
                add_if_exists(headers, CONTENT_LANGUAGE),
                cl,
                add_if_exists(headers, CONTENT_MD5),
                add_if_exists(headers, CONTENT_TYPE),
                add_if_exists(headers, DATE),
                add_if_exists(headers, IF_MODIFIED_SINCE),
                add_if_exists(headers, IF_MATCH),
                add_if_exists(headers, IF_NONE_MATCH),
                add_if_exists(headers, IF_UNMODIFIED_SINCE),
                add_if_exists(headers, RANGE),
                canonicalize_header(headers),
                canonicalized_resource(account, url)
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

fn canonicalize_header(headers: &Headers) -> String {
    let mut v_headers = headers
        .iter()
        .filter(|(name, _value)| name.as_str().starts_with("x-ms"))
        .collect::<Vec<_>>();
    v_headers.sort_unstable_by_key(|(name, _value)| name.as_str());

    let mut can = String::new();

    for (name, value) in v_headers {
        can = can + name.as_str() + ":" + value.as_str() + "\n";
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

fn get_endpoint_uri(
    url: Option<&str>,
    account: &str,
    endpoint_type: &str,
) -> azure_core::Result<url::Url> {
    Ok(match url {
        Some(value) => url::Url::parse(value)?,
        None => url::Url::parse(&format!(
            "https://{}.{}.core.windows.net",
            account, endpoint_type
        ))
        .with_context(ErrorKind::DataConversion, || {
            format!("failed to parse url: https://{account}.{endpoint_type}.core.windows.net")
        })?,
    })
}

/// Create a Pipeline from CosmosOptions
fn new_pipeline_from_options(options: StorageOptions, credentials: StorageCredentials) -> Pipeline {
    let auth_policy: Arc<dyn azure_core::Policy> = Arc::new(AuthorizationPolicy::new(credentials));

    // The `AuthorizationPolicy` must be the **last** retry policy.
    // Policies can change the url and/or the headers, and the `AuthorizationPolicy`
    // must be able to inspect them or the resulting token will be invalid.
    let per_retry_policies = vec![auth_policy];

    Pipeline::new(
        option_env!("CARGO_PKG_NAME"),
        option_env!("CARGO_PKG_VERSION"),
        options.options,
        Vec::new(),
        per_retry_policies,
    )
}

#[derive(Debug, Clone, Default)]
pub struct StorageOptions {
    options: ClientOptions,
}
impl StorageOptions {
    fn new() -> StorageOptions {
        Self::default()
    }
}
