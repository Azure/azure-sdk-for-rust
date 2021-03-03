use crate::clients::StorageClient;
use crate::data_lake::requests::*;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;
use url::Url;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

pub trait AsDataLakeClient<A: Into<String>> {
    fn as_data_lake_client(&self, account: A) -> Result<Arc<DataLakeClient>, url::ParseError>;
}

pub trait AsCustomDataLakeClient<DS: Into<String>, A: Into<String>> {
    fn as_data_lake_client_with_custom_dns_suffix(
        &self,
        account: A,
        dns_suffix: DS,
    ) -> Result<Arc<DataLakeClient>, url::ParseError>;
}

impl<A: Into<String>> AsDataLakeClient<A> for Arc<StorageClient> {
    fn as_data_lake_client(&self, account: A) -> Result<Arc<DataLakeClient>, url::ParseError> {
        DataLakeClient::new(self.clone(), account.into(), None)
    }
}

impl<DS: Into<String>, A: Into<String>> AsCustomDataLakeClient<DS, A> for Arc<StorageClient> {
    fn as_data_lake_client_with_custom_dns_suffix(
        &self,
        account: A,
        dns_suffix: DS,
    ) -> Result<Arc<DataLakeClient>, url::ParseError> {
        DataLakeClient::new(self.clone(), account.into(), Some(dns_suffix.into()))
    }
}

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    storage_client: Arc<StorageClient>,
    account: String,
    custom_dns_suffix: Option<String>,
    url: Url,
}

impl DataLakeClient {
    pub(crate) fn new(
        storage_client: Arc<StorageClient>,
        account: String,
        custom_dns_suffix: Option<String>,
    ) -> Result<Arc<Self>, url::ParseError> {
        // we precalculate the url once in the constructor
        // so we do not have to do it at every request.
        // This means we have to account for possible
        // malfolmed urls in the constructor, hence
        // the Result<_, url::ParseError>.
        let url = url::Url::parse(&format!(
            "https://{}.{}",
            account,
            match custom_dns_suffix.as_ref() {
                Some(custom_dns_suffix) => custom_dns_suffix,
                None => DEFAULT_DNS_SUFFIX,
            }
        ))?;

        Ok(Arc::new(Self {
            storage_client,
            account,
            custom_dns_suffix,
            url,
        }))
    }

    pub fn custom_dns_suffix(&self) -> Option<&str> {
        self.custom_dns_suffix.as_deref()
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.storage_client.storage_account_client().http_client()
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub fn list(&self) -> ListFileSystemsBuilder {
        ListFileSystemsBuilder::new(self)
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.storage_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
