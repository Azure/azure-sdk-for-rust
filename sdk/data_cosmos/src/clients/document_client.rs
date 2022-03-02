use super::{AttachmentClient, CollectionClient, CosmosClient, DatabaseClient};
use crate::operations::*;
use crate::ReadonlyString;
use azure_core::Request;
use serde::Serialize;

/// A client for Cosmos document resources.
#[derive(Debug, Clone)]
pub struct DocumentClient {
    collection_client: CollectionClient,
    document_name: String,
    partition_key_serialized: String,
}

impl DocumentClient {
    /// This function creates a new instance of a DocumentClient. A document is identified by its
    /// primary key and its partition key.
    ///
    /// Partition key is eagerly evaluated: the json representation is generated as soon as you
    /// call the `new` function. This avoids doing the serialization over and over, saving time.
    /// It also releases the borrow since the serialized string is owned by the `DocumentClient`.
    pub(crate) fn new<S: Into<String>, PK: Serialize>(
        collection_client: CollectionClient,
        document_name: S,
        partition_key: &PK,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            collection_client,
            document_name: document_name.into(),
            partition_key_serialized: crate::cosmos_entity::serialize_partition_key(partition_key)?,
        })
    }

    /// Get a [`CosmosClient`]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client().database_client()
    }

    /// Get a [`CollectionClient`]
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    /// Get the document's name
    pub fn document_name(&self) -> &str {
        &self.document_name
    }

    /// Get the partition key
    pub fn partition_key_serialized(&self) -> &str {
        &self.partition_key_serialized
    }

    /// Get a document
    pub fn get_document(&self) -> GetDocumentBuilder {
        GetDocumentBuilder::new(self.clone())
    }

    /// Replace a document in a collection
    pub fn replace_document<D: Serialize + Send + 'static>(
        &self,
        document: D,
    ) -> ReplaceDocumentBuilder<D> {
        ReplaceDocumentBuilder::new(self.clone(), document)
    }

    /// Delete a document
    pub fn delete_document(&self) -> DeleteDocumentBuilder {
        DeleteDocumentBuilder::new(self.clone())
    }

    /// List all attachments for a document
    pub fn list_attachments(&self) -> ListAttachmentsBuilder {
        ListAttachmentsBuilder::new(self.clone())
    }

    /// Convert into an [`AttachmentClient`]
    pub fn into_attachment_client<S: Into<ReadonlyString>>(
        self,
        attachment_name: S,
    ) -> AttachmentClient {
        AttachmentClient::new(self, attachment_name)
    }

    pub(crate) fn prepare_request_pipeline_with_document_name(
        &self,
        method: http::Method,
    ) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_name()
            ),
            method,
        )
    }
}
