use crate::core::clients::StorageAccountClient;
use crate::table::prelude::*;
use crate::table::requests::*;
use azure_core::errors::AzureError;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsPartitionKeyClient<PK: Into<String>> {
    fn as_partition_key_client(&self, partition_key: PK) -> Arc<PartitionKeyClient>;
}

impl<PK: Into<String>> AsPartitionKeyClient<PK> for Arc<TableClient> {
    fn as_partition_key_client(&self, partition_key: PK) -> Arc<PartitionKeyClient> {
        PartitionKeyClient::new(self.clone(), partition_key)
    }
}

#[derive(Debug, Clone)]
pub struct PartitionKeyClient {
    table_client: Arc<TableClient>,
    partition_key: String,
}

impl PartitionKeyClient {
    pub(crate) fn new<PK: Into<String>>(
        table_client: Arc<TableClient>,
        partition_key: PK,
    ) -> Arc<Self> {
        Arc::new(Self {
            table_client,
            partition_key: partition_key.into(),
        })
    }

    pub fn submit_transaction(&self) -> SubmitTransactionBuilder {
        SubmitTransactionBuilder::new(self)
    }

    pub fn partition_key(&self) -> &str {
        &self.partition_key
    }

    pub(crate) fn table_client(&self) -> &TableClient {
        &self.table_client
    }

    pub(crate) fn storage_account_client(&self) -> &StorageAccountClient {
        self.table_client.storage_account_client()
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.table_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.table_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
