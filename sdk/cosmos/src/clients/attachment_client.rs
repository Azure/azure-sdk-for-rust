use crate::requests;
use crate::{ReadonlyString, ResourceType};
use azure_core::No;

use super::*;

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

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.document_client().hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.document_client().cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        self.document_client().database_client()
    }

    pub fn collection_client(&self) -> &CollectionClient {
        self.document_client().collection_client()
    }

    pub fn document_client(&self) -> &DocumentClient {
        &self.document_client
    }

    pub fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    pub fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, No, No> {
        requests::CreateSlugAttachmentBuilder::new(self)
    }

    pub fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, No, No> {
        requests::ReplaceSlugAttachmentBuilder::new(self)
    }

    pub fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, No, No> {
        requests::CreateReferenceAttachmentBuilder::new(self)
    }

    pub fn replace_reference(&self) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, No, No> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    pub fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_> {
        requests::DeleteAttachmentBuilder::new(self)
    }

    pub fn get(&self) -> requests::GetAttachmentBuilder<'_, '_> {
        requests::GetAttachmentBuilder::new(self)
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
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

    pub fn prepare_request_with_attachment_name(
        &self,
        method: hyper::Method,
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
