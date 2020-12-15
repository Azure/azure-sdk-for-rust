use crate::clients::TableServiceClient;
use crate::table::requests::*;
use azure_core::errors::AzureError;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsTableClient<TN: Into<String>> {
    fn as_table_client(&self, table_name: TN) -> Arc<TableClient>;
}

impl<TN: Into<String>> AsTableClient<TN> for Arc<TableServiceClient> {
    fn as_table_client(&self, table_name: TN) -> Arc<TableClient> {
        TableClient::new(self.clone(), table_name.into())
    }
}

#[derive(Debug, Clone)]
pub struct TableClient {
    table_service_client: Arc<TableServiceClient>,
    table_name: String,
}

impl TableClient {
    pub(crate) fn new(
        table_service_client: Arc<TableServiceClient>,
        table_name: String,
    ) -> Arc<Self> {
        Arc::new(Self {
            table_service_client,
            table_name,
        })
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }

    pub(crate) fn table_service_client(&self) -> &TableServiceClient {
        self.table_service_client.as_ref()
    }

    pub fn insert_entity<'a, E>(
        &'a self,
        entity: &'a E,
    ) -> InsertEntityBuilder<
        'a,
        E,
        crate::table::requests::RowKeyMissing,
        crate::table::requests::PartitionKeyMissing,
    >
    where
        E: serde::Serialize,
    {
        InsertEntityBuilder::new(self, entity)
    }

    pub(crate) fn prepare_request<'a>(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<&'a [u8]>,
    ) -> Result<(Request<&'a [u8]>, url::Url), AzureError> {
        self.table_service_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
