use crate::{
    clients::{StorageAccountClient, StorageClient},
    table::requests::ListTablesBuilder,
};
use azure_core::errors::AzureError;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;
use url::Url;

pub trait AsTableServiceClient {
    fn as_table_service_client(&self) -> Result<Arc<TableServiceClient>, url::ParseError>;
}

impl AsTableServiceClient for Arc<StorageClient> {
    fn as_table_service_client(&self) -> Result<Arc<TableServiceClient>, url::ParseError> {
        TableServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TableServiceClient {
    storage_client: Arc<StorageClient>,
    url: Url,
}

impl TableServiceClient {
    pub(crate) fn new(storage_client: Arc<StorageClient>) -> Result<Arc<Self>, url::ParseError> {
        let url = storage_client
            .storage_account_client()
            .table_storage_url()
            .join("/Tables")?;

        Ok(Arc::new(Self {
            storage_client,
            url,
        }))
    }

    pub fn list(&self) -> ListTablesBuilder {
        ListTablesBuilder::new(self)
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_client.storage_account_client()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.storage_client.http_client()
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.storage_client
            .storage_account_client()
            .prepare_request(
                url,
                method,
                http_header_adder,
                crate::clients::ServiceType::Table,
                request_body,
            )
    }
}
