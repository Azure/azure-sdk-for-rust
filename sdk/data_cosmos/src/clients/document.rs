use crate::clients::*;
use crate::operations::*;
use crate::ReadonlyString;
use azure_core::Request;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A client for Cosmos document resources.
#[derive(Debug, Clone)]
pub struct DocumentClient {
    collection: CollectionClient,
    document_name: String,
    partition_key_serialized: String,
}

impl DocumentClient {
    /// Create a new instance of a `DocumentClient`.
    ///
    /// A document is identified by its primary key and its partition key.
    pub(crate) fn new<S: Into<String>, PK: Serialize>(
        collection: CollectionClient,
        document_name: S,
        partition_key: &PK,
    ) -> azure_core::Result<Self> {
        Ok(Self {
            collection,
            document_name: document_name.into(),
            partition_key_serialized: crate::cosmos_entity::serialize_partition_key(partition_key)?,
        })
    }

    /// Get the document.
    pub fn get_document<T: DeserializeOwned + Send>(&self) -> GetDocumentBuilder<T> {
        GetDocumentBuilder::new(self.clone())
    }

    /// Replace the document.
    pub fn replace_document<D: Serialize + Send + 'static>(
        &self,
        document: D,
    ) -> ReplaceDocumentBuilder<D> {
        ReplaceDocumentBuilder::new(self.clone(), document)
    }

    /// Delete the document.
    pub fn delete_document(&self) -> DeleteDocumentBuilder {
        DeleteDocumentBuilder::new(self.clone())
    }

    /// List all attachments for the document.
    pub fn list_attachments(&self) -> ListAttachmentsBuilder {
        ListAttachmentsBuilder::new(self.clone())
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client().database_client()
    }

    /// Get a [`CollectionClient`].
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection
    }

    /// Get an [`AttachmentClient`].
    pub fn attachment_client<S: Into<ReadonlyString>>(
        &self,
        attachment_name: S,
    ) -> AttachmentClient {
        AttachmentClient::new(self.clone(), attachment_name)
    }

    /// Get the document's name
    pub fn document_name(&self) -> &str {
        &self.document_name
    }

    /// Get the partition key
    pub fn partition_key_serialized(&self) -> &str {
        &self.partition_key_serialized
    }

    pub(crate) fn document_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
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
