use crate::attachment::AttachmentName;
use crate::clients::{AttachmentClient, Client, CollectionClient, CosmosUriBuilder, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::requests;
use crate::CollectionTrait;
use crate::{DocumentBuilderTrait, DocumentTrait, PartitionKeys};

#[derive(Debug, Clone)]
pub struct DocumentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    document_name: &'a dyn DocumentName,
    partition_keys: &'a PartitionKeys,
}

impl<'a, CUB> DocumentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
        document_name: &'a dyn DocumentName,
        partition_keys: &'a PartitionKeys,
    ) -> Self {
        Self {
            collection_client,
            document_name,
            partition_keys,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.collection_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> DocumentTrait<'a, CUB> for DocumentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.collection_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_client.collection_name()
    }

    fn document_name(&self) -> &'a dyn DocumentName {
        self.document_name
    }

    fn partition_keys(&self) -> &'a PartitionKeys {
        self.partition_keys
    }

    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, CUB> {
        requests::GetDocumentBuilder::new(self)
    }

    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, CUB> {
        requests::DeleteDocumentBuilder::new(self)
    }

    fn with_attachment(
        &'a self,
        attachment_name: &'a dyn AttachmentName,
    ) -> AttachmentClient<'_, CUB> {
        AttachmentClient::new(&self, attachment_name)
    }

    fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_, CUB> {
        requests::ListAttachmentsBuilder::new(self)
    }
}

impl<'a, CUB> DocumentBuilderTrait<'a, CUB> for DocumentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_name().name(),
                self.collection_name().name(),
                self.document_name().name()
            ),
            method,
            ResourceType::Documents,
        )
    }
}
