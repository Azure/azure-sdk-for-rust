use crate::clients::{ServiceType, StorageAccountClient};
use crate::table::requests::*;
use azure_core::errors::AzureError;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsTableServiceClient {
    fn as_table_service_client(&self) -> Arc<Box<TableServiceClient>>;
}

impl AsTableServiceClient for Arc<Box<StorageAccountClient>> {
    fn as_table_service_client(&self) -> Arc<Box<TableServiceClient>> {
        TableServiceClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct TableServiceClient {
    storage_account_client: Arc<Box<StorageAccountClient>>,
}

impl TableServiceClient {
    pub(crate) fn new(storage_account_client: Arc<Box<StorageAccountClient>>) -> Arc<Box<Self>> {
        Arc::new(Box::new(Self {
            storage_account_client,
        }))
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_account_client.as_ref().as_ref()
    }

    pub fn query_tables(&self) -> QueryTablesBuilder {
        QueryTablesBuilder::new(self)
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&'a [u8]>,
    ) -> Result<(Request<&'a [u8]>, url::Url), AzureError> {
        self.storage_account_client.prepare_request(
            url,
            method,
            http_header_adder,
            ServiceType::Table,
            request_body,
        )
    }
}
