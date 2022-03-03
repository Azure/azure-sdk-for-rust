use crate::operations::*;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};
use bytes::Bytes;

use super::*;

/// A client for Cosmos attachment resources.
#[derive(Debug, Clone)]
pub struct AttachmentClient {
    document: DocumentClient,
    attachment_name: ReadonlyString,
}

impl AttachmentClient {
    /// Create a new client
    pub(crate) fn new<S: Into<ReadonlyString>>(
        document: DocumentClient,
        attachment_name: S,
    ) -> Self {
        Self {
            document,
            attachment_name: attachment_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    pub fn client(&self) -> &CosmosClient {
        self.document().client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database(&self) -> &DatabaseClient {
        self.document().database()
    }

    /// Get a [`CollectionClient`].
    pub fn collection(&self) -> &CollectionClient {
        self.document().collection()
    }

    /// Get a [`DocumentClient`].
    pub fn document(&self) -> &DocumentClient {
        &self.document
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

    /// Initiate a request to create a reference attachment.
    pub fn create_attachment<M, C>(
        &self,
        media: M,
        content_type: C,
    ) -> CreateOrReplaceAttachmentBuilder
    where
        M: Into<String>,
        C: Into<String>,
    {
        CreateOrReplaceAttachmentBuilder::new(self.clone(), true, media.into(), content_type.into())
    }

    /// Initiate a request to replace an attachment.
<<<<<<< HEAD:sdk/data_cosmos/src/clients/attachment_client.rs
    pub fn replace_attachment<M, C>(
        &self,
        media: M,
        content_type: C,
    ) -> CreateOrReplaceAttachmentBuilder
    where
        M: Into<String>,
        C: Into<String>,
    {
        CreateOrReplaceAttachmentBuilder::new(
            self.clone(),
            false,
            media.into(),
            content_type.into(),
        )
=======
    pub fn replace_reference(&self) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    /// Get a raw [`HttpClient`].
    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.client().http_client()
>>>>>>> 495b38c8e... Remove `_client` suffix from cosmos methods:sdk/data_cosmos/src/clients/attachment.rs
    }

    pub(crate) fn prepare_pipeline(&self, method: http::Method) -> Request {
        self.client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.database().database_name(),
                self.collection().collection_name(),
                self.document().document_name(),
            ),
            method,
        )
    }

<<<<<<< HEAD:sdk/data_cosmos/src/clients/attachment_client.rs
=======
    pub(crate) fn prepare_request_with_attachment_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database().database_name(),
                self.collection().collection_name(),
                self.document().document_name(),
                self.attachment_name()
            ),
            method,
            ResourceType::Attachments,
        )
    }

>>>>>>> 495b38c8e... Remove `_client` suffix from cosmos methods:sdk/data_cosmos/src/clients/attachment.rs
    pub(crate) fn prepare_pipeline_with_attachment_name(&self, method: http::Method) -> Request {
        self.client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database().database_name(),
                self.collection().collection_name(),
                self.document().document_name(),
                self.attachment_name()
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.client().pipeline()
    }
}
