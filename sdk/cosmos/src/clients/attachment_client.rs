use crate::requests;
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::{HttpClient, No};

use super::*;

/// A client for Cosmos attachment resources.
#[derive(Debug, Clone)]
pub struct AttachmentClient {
    document_client: DocumentClient,
    attachment_name: ReadonlyString,
}

impl AttachmentClient {
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

    /// Get a raw [`HttpClient`].
    pub fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    /// Get the attachment name.
    pub fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    /// Initiate a request to get an attachment.
    pub fn get(&self) -> requests::GetAttachmentBuilder<'_, '_> {
        requests::GetAttachmentBuilder::new(self)
    }

    /// Initiate a request to delete an attachment.
    pub fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_> {
        requests::DeleteAttachmentBuilder::new(self)
    }

    /// Initiate a request to create an attachment with a slug.
    pub fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_> {
        requests::CreateSlugAttachmentBuilder::new(self)
    }

    /// Initiate a request to replace an attachment.
    pub fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, No, No> {
        requests::ReplaceSlugAttachmentBuilder::new(self)
    }

    /// Initiate a request to create an attachment.
    pub fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_> {
        requests::CreateReferenceAttachmentBuilder::new(self)
    }

    /// Initiate a request to replace an attachment.
    pub fn replace_reference(&self) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    pub(crate) fn prepare_request(&self, method: http::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
            ),
            method,
            ResourceType::Attachments,
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
}
