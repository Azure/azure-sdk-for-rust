use crate::operations::*;
use crate::requests;
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::HttpClient;
use azure_core::Pipeline;
use azure_core::Request;
use bytes::Bytes;

use super::*;

/// A client for Cosmos attachment resources.
#[derive(Debug, Clone)]
pub struct AttachmentClient {
    document_client: DocumentClient,
    attachment_name: ReadonlyString,
}

impl AttachmentClient {
    /// Create a new client
    pub(crate) fn new<S: Into<ReadonlyString>>(
        document_client: DocumentClient,
        attachment_name: S,
    ) -> Self {
        Self {
            document_client,
            attachment_name: attachment_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.document_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        self.document_client().database_client()
    }

    /// Get a [`CollectionClient`].
    pub fn collection_client(&self) -> &CollectionClient {
        self.document_client().collection_client()
    }

    /// Get a [`DocumentClient`].
    pub fn document_client(&self) -> &DocumentClient {
        &self.document_client
    }

    /// Get the attachment name.
    pub fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    /// Initiate a request to get an attachment.
    pub fn get(&self) -> GetAttachmentBuilder {
        GetAttachmentBuilder::new(self.clone())
    }

    /// Initiate a request to delete an attachment.
    pub fn delete(&self) -> DeleteAttachmentBuilder {
        DeleteAttachmentBuilder::new(self.clone())
    }

    /// Initiate a request to create an attachment with a slug.
    pub fn create_slug(&self, body: Bytes) -> CreateOrReplaceSlugAttachmentBuilder {
        CreateOrReplaceSlugAttachmentBuilder::new(self.clone(), true, body)
    }

    /// Initiate a request to replace an attachment.
    pub fn replace_slug(&self, body: Bytes) -> CreateOrReplaceSlugAttachmentBuilder {
        CreateOrReplaceSlugAttachmentBuilder::new(self.clone(), false, body)
    }

    /// Initiate a request to create ant.
    pub fn create_reference<M, C>(
        &self,
        media: M,
        content_type: C,
    ) -> CreateReferenceAttachmentBuilder
    where
        M: Into<String>,
        C: Into<String>,
    {
        CreateReferenceAttachmentBuilder::new(self.clone(), media.into(), content_type.into())
    }

    /// Initiate a request to replace an attachment.
    pub fn replace_reference(&self) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    /// Get a raw [`HttpClient`].
    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    pub(crate) fn prepare_pipeline(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
            ),
            method,
        )
    }

    pub(crate) fn prepare_request_with_attachment_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
                self.attachment_name()
            ),
            method,
            ResourceType::Attachments,
        )
    }

    pub(crate) fn prepare_pipeline_with_attachment_name(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
                self.attachment_name()
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
