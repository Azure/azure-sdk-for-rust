use crate::attachment::AttachmentName;
use crate::clients::{Client, CosmosUriBuilder, DocumentClient, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::requests;
use crate::DocumentTrait;
use crate::{AttachmentBuilderTrait, AttachmentTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    document_client: &'a DocumentClient<'a, CUB>,
    attachment_name: &'a dyn AttachmentName,
}

impl<'a, CUB> AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        document_client: &'a DocumentClient<'a, CUB>,
        attachment_name: &'a dyn AttachmentName,
    ) -> Self {
        AttachmentClient {
            document_client,
            attachment_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.document_client.main_client()
    }

    pub(crate) fn document_client(&self) -> &'a DocumentClient<'a, CUB> {
        &self.document_client
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> AttachmentTrait<'a, CUB> for AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.document_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.document_client.collection_name()
    }

    fn document_name(&self) -> &'a dyn DocumentName {
        self.document_client.document_name()
    }

    fn attachment_name(&self) -> &'a dyn AttachmentName {
        self.attachment_name
    }

    fn create_slug(&self) -> requests::CreateSlugAttachmentBuilder<'_, '_, CUB, No, No> {
        requests::CreateSlugAttachmentBuilder::new(self)
    }

    fn replace_slug(&self) -> requests::ReplaceSlugAttachmentBuilder<'_, '_, CUB, No, No> {
        requests::ReplaceSlugAttachmentBuilder::new(self)
    }

    fn create_reference(&self) -> requests::CreateReferenceAttachmentBuilder<'_, '_, CUB, No, No> {
        requests::CreateReferenceAttachmentBuilder::new(self)
    }

    fn replace_reference(
        &self,
    ) -> requests::ReplaceReferenceAttachmentBuilder<'_, '_, CUB, No, No> {
        requests::ReplaceReferenceAttachmentBuilder::new(self)
    }

    fn delete(&self) -> requests::DeleteAttachmentBuilder<'_, '_, CUB> {
        requests::DeleteAttachmentBuilder::new(self)
    }

    fn get(&self) -> requests::GetAttachmentBuilder<'_, '_, CUB> {
        requests::GetAttachmentBuilder::new(self)
    }
}

impl<'a, CUB> AttachmentBuilderTrait<'a, CUB> for AttachmentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database_name().name(),
                self.collection_name().name(),
                self.document_name().name(),
                self.attachment_name().name()
            ),
            method,
            ResourceType::Attachments,
        )
    }
}
