use crate::core::clients::StorageAccountClient;
use crate::table::clients::TableServiceClient;
use crate::table::requests::*;
use azure_core::errors::AzureError;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsTableClient<S: Into<String>> {
    fn as_table_client(&self, s: S) -> Arc<TableClient>;
}

impl<S: Into<String>> AsTableClient<S> for Arc<TableServiceClient> {
    fn as_table_client(&self, s: S) -> Arc<TableClient> {
        TableClient::new(self.clone(), s)
    }
}

#[derive(Debug, Clone)]
pub struct TableClient {
    table_service_client: Arc<TableServiceClient>,
    table_name: String,
}

impl TableClient {
    pub(crate) fn new<S: Into<String>>(
        table_service_client: Arc<TableServiceClient>,
        s: S,
    ) -> Arc<Self> {
        Arc::new(Self {
            table_service_client,
            table_name: s.into(),
        })
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub fn create(&self) -> CreateTableBuilder {
        CreateTableBuilder::new(self)
    }

    pub fn query(&self) -> QueryEntityBuilder {
        QueryEntityBuilder::new(self)
    }

    pub fn delete(&self) -> DeleteTableBuilder {
        DeleteTableBuilder::new(self)
    }

    pub fn insert(&self) -> InsertEntityBuilder {
        InsertEntityBuilder::new(self)
    }

    pub(crate) fn url(&self) -> &url::Url {
        self.table_service_client.url()
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.table_service_client.storage_account_client()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.table_service_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.table_service_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
