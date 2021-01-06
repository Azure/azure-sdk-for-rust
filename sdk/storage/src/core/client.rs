use crate::core::bearer_token_client::BearerTokenClient;
use crate::core::key_client::get_sas_token_parms;
use crate::core::rest_client::ServiceType;
use crate::core::{ConnectionString, KeyClient};
use crate::PerformRequestResponse;
use azure_core::errors::AzureError;
use http::request::Builder;
use hyper::{self, Method};
use hyper_rustls::HttpsConnector;
use std::borrow::Cow;
use url::Url;

pub trait HttpHeaderAdder {
    fn add_optional_headers(&self, builder: ::http::request::Builder) -> ::http::request::Builder;
}

pub trait Client: std::fmt::Debug + Send + Sync {
    fn blob_uri(&self) -> &str;
    fn table_uri(&self) -> &str;
    fn queue_uri(&self) -> &str;
    fn filesystem_uri(&self) -> &str;

    /// Uri scheme + authority e.g. http://myaccount.table.core.windows.net/
    #[inline]
    fn get_uri_prefix(&self, service_type: ServiceType) -> String {
        match service_type {
            ServiceType::Blob => format!("{}/", self.blob_uri()),
            ServiceType::Table => format!("{}/", self.table_uri()),
        }
    }

    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError>;

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError>;
}

impl<C> Client for Box<C>
where
    C: Client + ?Sized,
{
    fn blob_uri(&self) -> &str {
        self.as_ref().blob_uri()
    }
    fn table_uri(&self) -> &str {
        self.as_ref().table_uri()
    }
    fn queue_uri(&self) -> &str {
        self.as_ref().queue_uri()
    }
    fn filesystem_uri(&self) -> &str {
        self.as_ref().filesystem_uri()
    }

    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        self.as_ref()
            .perform_request(uri, method, http_header_adder, request_body)
    }

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        self.as_ref()
            .perform_table_request(segment, method, http_header_adder, request_str)
    }
}

impl<C> Client for std::sync::Arc<C>
where
    C: Client + ?Sized,
{
    fn blob_uri(&self) -> &str {
        self.as_ref().blob_uri()
    }
    fn table_uri(&self) -> &str {
        self.as_ref().table_uri()
    }
    fn queue_uri(&self) -> &str {
        self.as_ref().queue_uri()
    }
    fn filesystem_uri(&self) -> &str {
        self.as_ref().filesystem_uri()
    }

    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        self.as_ref()
            .perform_request(uri, method, http_header_adder, request_body)
    }

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<PerformRequestResponse, AzureError> {
        self.as_ref()
            .perform_table_request(segment, method, http_header_adder, request_str)
    }
}

//
// def impl
//
#[deprecated(
    since = "0.44.0",
    note = "Please use the with_access_key function instead"
)]
pub fn new(account: &str, key: &str) -> KeyClient {
    with_access_key(account, key)
}

pub fn with_azure_sas(account: &str, sas_token: &str) -> KeyClient {
    let client = hyper::Client::builder().build(HttpsConnector::with_native_roots());
    let params = get_sas_token_parms(sas_token);

    KeyClient::new(
        account.to_owned(),
        String::new(),
        Some(params),
        client,
        format!("https://{}.blob.core.windows.net", account),
        format!("https://{}.table.core.windows.net", account),
        format!("https://{}.queue.core.windows.net", account),
        format!("https://{}.dfs.core.windows.net", account),
    )
}

pub fn with_access_key(account: &str, key: &str) -> KeyClient {
    let client = hyper::Client::builder().build(HttpsConnector::with_native_roots());

    KeyClient::new(
        account.to_owned(),
        key.to_owned(),
        None,
        client,
        format!("https://{}.blob.core.windows.net", account),
        format!("https://{}.table.core.windows.net", account),
        format!("https://{}.queue.core.windows.net", account),
        format!("https://{}.dfs.core.windows.net", account),
    )
}

pub fn from_connection_string(connection_string: &str) -> Result<KeyClient, AzureError> {
    let client = hyper::Client::builder().build(HttpsConnector::with_native_roots());

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
                Ok(KeyClient::new(
                    account.to_owned(),
                    String::new(),
                    Some(get_sas_token_parms(sas_token)),
                    client,
                    get_endpoint_uri(blob_endpoint, account, "blob"),
                    get_endpoint_uri(table_endpoint, account, "table"),
                    get_endpoint_uri(queue_endpoint, account,  "queue"),
                    get_endpoint_uri(file_endpoint, account,  "dfs"),
                ))
            }
            ConnectionString {
                account_name: Some(account),
                sas: Some(sas_token),
                blob_endpoint,
                table_endpoint,
                queue_endpoint,
                file_endpoint,
                ..
            } => Ok(KeyClient ::new(
                account.to_owned(),
                String::new(),
                Some(get_sas_token_parms(sas_token)),
                client,
                get_endpoint_uri(blob_endpoint, account, "blob"),
                get_endpoint_uri(table_endpoint, account, "table"),
                get_endpoint_uri(queue_endpoint, account,  "queue"),
                get_endpoint_uri(file_endpoint, account,  "dfs"),
            )),
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                blob_endpoint,
                table_endpoint,
                queue_endpoint,
                file_endpoint,
                ..
            } => Ok(KeyClient::new(
                account.to_owned(),
                key.to_owned(),
                None,
                client,
                get_endpoint_uri(blob_endpoint, account, "blob"),
                get_endpoint_uri(table_endpoint, account, "table"),
                get_endpoint_uri(queue_endpoint, account,  "queue"),
                get_endpoint_uri(file_endpoint, account,  "dfs"),
            )),
            _ => {
                Err(AzureError::GenericErrorWithText(
                    "Could not create a storage client from the provided connection string. Please validate that you have specified the account name and means of authentication (key, SAS, etc.)."
                        .to_owned(),
                ))
            }
        }
}

pub fn with_bearer_token<'a, A, BT>(account: A, bearer_token: BT) -> BearerTokenClient<'a>
where
    A: Into<Cow<'a, str>>,
    BT: Into<Cow<'a, str>>,
{
    let client = hyper::Client::builder().build(HttpsConnector::with_native_roots());

    BearerTokenClient::new(account.into(), bearer_token.into(), client)
}

pub fn with_emulator(blob_storage_url: &Url, table_storage_url: &Url) -> KeyClient {
    let client = hyper::Client::builder().build(HttpsConnector::with_native_roots());

    let blob_uri = format!("{}devstoreaccount1", blob_storage_url.as_str());
    debug!("blob_uri == {}", blob_uri);
    let table_uri = format!("{}devstoreaccount1", table_storage_url.as_str());
    debug!("table_uri == {}", table_uri);
    let queue_uri = format!("{}devstoreaccount1", table_storage_url.as_str());
    debug!("queue_uri == {}", queue_uri);
    let filesystem_uri = format!("{}devstoreaccount1", blob_storage_url.as_str());
    debug!("filesystem_uri = {}", filesystem_uri);

    KeyClient::new(
        "devstoreaccount1".to_owned(),
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw=="
            .to_owned(),
        None,
        client,
        blob_uri,
        table_uri,
        queue_uri,
        filesystem_uri,
    )
}

pub(crate) fn get_endpoint_uri(url: Option<&str>, account: &str, endpoint_type: &str) -> String {
    match url {
        Some(value) => value.to_string(),
        None => format!("https://{}.{}.core.windows.net", account, endpoint_type),
    }
}
