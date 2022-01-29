use super::{AttachmentClient, CollectionClient, CosmosClient, DatabaseClient};
use crate::operations::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::{Context, HttpClient, Request};
use serde::de::DeserializeOwned;
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
    pub async fn get_document<T>(
        &self,
        ctx: Context,
        options: GetDocumentOptions,
    ) -> crate::Result<GetDocumentResponse<T>>
    where
        T: DeserializeOwned,
    {
        let mut request = self.prepare_request_pipeline_with_document_name(http::Method::GET);

        options.decorate_request(&mut request, self.partition_key_serialized())?;   

        let response = self
            .cosmos_client()
            .pipeline()
            .send(ctx.clone().insert(ResourceType::Documents), &mut request)
            .await?;

        GetDocumentResponse::try_from(response).await
    }

    /// replace a document in a collection
    pub async fn replace_document<T: Serialize>(
        &self,
        ctx: Context,
        document: &T,
        options: ReplaceDocumentOptions,
    ) -> crate::Result<ReplaceDocumentResponse> {
        let mut request = self.prepare_request_pipeline_with_document_name(http::Method::PUT);

        options.decorate_request(&mut request, document, self.partition_key_serialized())?;

        let response = self
            .cosmos_client()
            .pipeline()
            .send(ctx.clone().insert(ResourceType::Documents), &mut request)
            .await?;

        ReplaceDocumentResponse::try_from(response).await
    }

    /// Delete a document
    pub async fn delete_document(
        &self,
        ctx: Context,
        options: DeleteDocumentOptions,
    ) -> crate::Result<DeleteDocumentResponse> {
        let mut request = self.prepare_request_pipeline_with_document_name(http::Method::DELETE);

        options.decorate_request(&mut request, self.partition_key_serialized())?;

        let response = self
            .cosmos_client()
            .pipeline()
            .send(ctx.clone().insert(ResourceType::Documents), &mut request)
            .await?;

        DeleteDocumentResponse::try_from(response).await
    }

    /// List all attachments for a document
    pub fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_> {
        requests::ListAttachmentsBuilder::new(self)
    }

    /// Convert into an [`AttachmentClient`]
    pub fn into_attachment_client<S: Into<ReadonlyString>>(
        self,
        attachment_name: S,
    ) -> AttachmentClient {
        AttachmentClient::new(self, attachment_name)
    }

    fn prepare_request_pipeline_with_document_name(&self, method: http::Method) -> Request {
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

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }
}
