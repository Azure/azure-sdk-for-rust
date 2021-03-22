use crate::table_storage::prelude::*;
use crate::table_storage::requests::*;
use azure_core::errors::AzureError;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;
use url::Url;

pub trait AsEntityClient<RK: Into<String>> {
    fn as_entity_client(&self, row_key: RK) -> Result<Arc<EntityClient>, url::ParseError>;
}

impl<RK: Into<String>> AsEntityClient<RK> for Arc<PartitionKeyClient> {
    fn as_entity_client(&self, row_key: RK) -> Result<Arc<EntityClient>, url::ParseError> {
        EntityClient::new(self.clone(), row_key)
    }
}

#[derive(Debug, Clone)]
pub struct EntityClient {
    partition_key_client: Arc<PartitionKeyClient>,
    row_key: String,
    url: Url,
}

impl EntityClient {
    pub(crate) fn new<RK: Into<String>>(
        partition_key_client: Arc<PartitionKeyClient>,
        row_key: RK,
    ) -> Result<Arc<Self>, url::ParseError> {
        let row_key = row_key.into();
        let url = partition_key_client
            .storage_account_client()
            .table_storage_url()
            .join(&format!(
                "/{}(PartitionKey='{}',RowKey='{}')",
                partition_key_client.table_client().table_name(),
                partition_key_client.partition_key(),
                &row_key
            ))?;

        Ok(Arc::new(Self {
            partition_key_client,
            row_key,
            url,
        }))
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
        self.partition_key_client.http_client()
    }

    pub(crate) fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> Result<(Request<Bytes>, url::Url), AzureError> {
        self.partition_key_client
            .prepare_request(url, method, http_header_adder, request_body)
    }
}
