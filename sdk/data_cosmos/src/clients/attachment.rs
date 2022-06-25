use crate::clients::*;
use crate::operations::*;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};
use bytes::Bytes;

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

    /// Get the attachment.
    pub fn get(&self) -> GetAttachmentBuilder {
        GetAttachmentBuilder::new(self.clone())
    }

    /// Delete the attachment.
    pub fn delete(&self) -> DeleteAttachmentBuilder {
        DeleteAttachmentBuilder::new(self.clone())
    }

    /// Create an attachment with a slug.
    pub fn create_slug(&self, body: Bytes) -> CreateOrReplaceSlugAttachmentBuilder {
        CreateOrReplaceSlugAttachmentBuilder::new(self.clone(), true, body)
    }

    /// Replace an attachment with a slug.
    pub fn replace_slug(&self, body: Bytes) -> CreateOrReplaceSlugAttachmentBuilder {
        CreateOrReplaceSlugAttachmentBuilder::new(self.clone(), false, body)
    }

    /// Create a reference attachment.
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

    /// Replace an attachment.
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
        &self.document
    }

    /// Get the attachment name.
    pub fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    pub(crate) fn attachments_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
            ),
            method,
        )
    }

    pub(crate) fn attachment_request(&self, method: azure_core::Method) -> Request {
        self.cosmos_client().request(
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
