use crate::bearer_token_client::BearerTokenClient;
use crate::key_client::get_sas_token_parms;
use crate::rest_client::ServiceType;
use crate::{ConnectionString, KeyClient};
use azure_sdk_core::errors::AzureError;
use http::request::Builder;
use hyper::{self, Method};
use hyper_rustls::HttpsConnector;
use std::borrow::Cow;
use url::Url;

pub trait HttpHeaderAdder {
    fn add_headers(&self, builder: ::http::request::Builder) -> ::http::request::Builder;
}

pub trait Client: Send + Sync {
    fn blob_uri(&self) -> &str;
    fn table_uri(&self) -> &str;

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
    ) -> Result<hyper::client::ResponseFuture, AzureError>;

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError>;
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

    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        self.as_ref()
            .perform_request(uri, method, http_header_adder, request_body)
    }

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
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

    fn perform_request(
        &self,
        uri: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
        self.as_ref()
            .perform_request(uri, method, http_header_adder, request_body)
    }

    fn perform_table_request(
        &self,
        segment: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_str: Option<&[u8]>,
    ) -> Result<hyper::client::ResponseFuture, AzureError> {
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
    let client = hyper::Client::builder().build(HttpsConnector::new());
    let params = get_sas_token_parms(sas_token);

    KeyClient::new(
        account.to_owned(),
        String::new(),
        Some(params),
        client,
        format!("https://{}.blob.core.windows.net", account),
        format!("https://{}.table.core.windows.net", account),
    )
}

pub fn with_access_key(account: &str, key: &str) -> KeyClient {
    let client = hyper::Client::builder().build(HttpsConnector::new());

    KeyClient::new(
        account.to_owned(),
        key.to_owned(),
        None,
        client,
        format!("https://{}.blob.core.windows.net", account),
        format!("https://{}.table.core.windows.net", account),
    )
}

pub fn from_connection_string(connection_string: &str) -> Result<KeyClient, AzureError> {
    let client = hyper::Client::builder().build(HttpsConnector::new());

    match ConnectionString::new(connection_string)? {
            ConnectionString {
                account_name: Some(account),
                account_key: Some(_),
                sas: Some(sas_token),
                ..
            } => {
                log::warn!("Both account key and SAS defined in connection string. Using only the provided SAS.");
                Ok(KeyClient::new(
                    account.to_owned(),
                    String::new(),
                    Some(get_sas_token_parms(sas_token)),
                    client,
                    format!("https://{}.blob.core.windows.net", account),
                    format!("https://{}.table.core.windows.net", account), 
                ))
            }
            ConnectionString {
                account_name: Some(account),
                sas: Some(sas_token),
                ..
            } => Ok(KeyClient ::new(
                account.to_owned(),
                String::new(),
                Some(get_sas_token_parms(sas_token)),
                client,
                format!("https://{}.blob.core.windows.net", account),
                format!("https://{}.table.core.windows.net", account), 
            )),
            ConnectionString {
                account_name: Some(account),
                account_key: Some(key),
                ..
            } => Ok(KeyClient::new(
                account.to_owned(),
                key.to_owned(),
                None,
                client,
                format!("https://{}.blob.core.windows.net", account),
                format!("https://{}.table.core.windows.net", account), 
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
    let client = hyper::Client::builder().build(HttpsConnector::new());

    BearerTokenClient::new(account.into(), bearer_token.into(), client)
}

pub fn with_emulator(blob_storage_url: &Url, table_storage_url: &Url) -> KeyClient {
    let client = hyper::Client::builder().build(HttpsConnector::new());

    let blob_uri = format!("{}devstoreaccount1", blob_storage_url.as_str());
    debug!("blob_uri == {}", blob_uri);
    let table_uri = format!("{}devstoreaccount1", table_storage_url.as_str());
    debug!("table_uri == {}", table_uri);

    KeyClient::new(
        "devstoreaccount1".to_owned(),
        "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw=="
            .to_owned(),
        None,
        client,
        blob_uri,
        table_uri,
    )
}
