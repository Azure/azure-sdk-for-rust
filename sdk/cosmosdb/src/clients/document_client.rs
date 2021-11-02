use super::{AttachmentClient, CollectionClient, CosmosClient, DatabaseClient};
use crate::prelude::{GetDocumentOptions, GetDocumentResponse};
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::{Context, HttpClient, PipelineContext, Request};
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
    /// primary key and its partition key. Partition key is eagerly evaluated: the json
    /// representation is generated as soon as you call the `new` function. This avoids doing the
    /// serialization over and over, saving time. It also releases the borrow since the serialized
    /// string is owned by the DocumentClient.
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

    /// replace a document in a collection
    pub fn replace_document<'a>(&'a self) -> requests::ReplaceDocumentBuilder<'a, '_> {
        requests::ReplaceDocumentBuilder::new(self)
    }

    /// Get a document
    pub async fn get_document<T>(
        &self,
        ctx: Context,
        options: GetDocumentOptions<'_>,
    ) -> crate::Result<GetDocumentResponse<T>>
    where
        T: DeserializeOwned,
    {
        let mut request = self.prepare_request_pipeline_with_document_name(http::Method::GET);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request)?;

        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        GetDocumentResponse::try_from(response).await
    }

    /// Delete a document
    pub fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_> {
        requests::DeleteDocumentBuilder::new(self)
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

    pub(crate) fn prepare_request_with_document_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_name()
            ),
            method,
            ResourceType::Documents,
        )
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
