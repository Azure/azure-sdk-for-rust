use crate::table::prelude::*;
use crate::table::requests::*;
use azure_core::errors::AzureError;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;
use url::Url;

pub trait AsEntityClient<PK: Into<String>, RK: Into<String>> {
    fn as_entity_client(
        &self,
        partition_key: PK,
        row_key: RK,
    ) -> Result<Arc<EntityClient>, url::ParseError>;
}

impl<PK: Into<String>, RK: Into<String>> AsEntityClient<PK, RK> for Arc<TableClient> {
    fn as_entity_client(
        &self,
        partition_key: PK,
        row_key: RK,
    ) -> Result<Arc<EntityClient>, url::ParseError> {
        EntityClient::new(self.clone(), partition_key, row_key)
    }
}

#[derive(Debug, Clone)]
pub struct EntityClient {
    table_client: Arc<TableClient>,
    partition_key: String,
    row_key: String,
    url: Url,
}

impl EntityClient {
    pub(crate) fn new<PK: Into<String>, RK: Into<String>>(
        table_client: Arc<TableClient>,
        partition_key: PK,
        row_key: RK,
    ) -> Result<Arc<Self>, url::ParseError> {
        let partition_key = partition_key.into();
        let row_key = row_key.into();
        let url = table_client
            .storage_account_client()
            .table_storage_url()
            .join(&format!(
                "/{}(PartitionKey='{}',RowKey='{}')",
                table_client.table_name(),
                &partition_key,
                &row_key
            ))?;

        Ok(Arc::new(Self {
            table_client,
            partition_key,
            row_key,
            url,
        }))
    }

    pub fn partition_key(&self) -> &str {
        &self.partition_key
    }

    pub fn row_key(&self) -> &str {
        &self.row_key
    }

    pub fn update(&self) -> UpdateOrMergeEntityBuilder {
        UpdateOrMergeEntityBuilder::new(self, update_or_merge_entity_builder::Operation::Update)
    }

    pub fn merge(&self) -> UpdateOrMergeEntityBuilder {
        UpdateOrMergeEntityBuilder::new(self, update_or_merge_entity_builder::Operation::Merge)
    }

    pub fn insert_or_replace(&self) -> InsertOrReplaceOrMergeEntityBuilder {
        InsertOrReplaceOrMergeEntityBuilder::new(
            self,
            insert_or_replace_or_merge_entity_builder::Operation::InsertOrReplace,
        )
    }

    pub fn insert_or_merge(&self) -> InsertOrReplaceOrMergeEntityBuilder {
        InsertOrReplaceOrMergeEntityBuilder::new(
            self,
            insert_or_replace_or_merge_entity_builder::Operation::InsertOrMerge,
        )
    }

    pub(crate) fn url(&self) -> &Url {
        &self.url
    }

    pub(crate) fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.table_client.http_client()
    }

    pub(crate) fn prepare_request<'a>(
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
