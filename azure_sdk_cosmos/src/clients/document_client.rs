use crate::clients::{Client, CollectionClient, CosmosUriBuilder, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::document::DocumentName;
use crate::requests;
use crate::CollectionTrait;
use crate::{DocumentBuilderTrait, DocumentTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct DocumentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    document_name: &'a dyn DocumentName,
}

impl<'a, CUB> DocumentClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
        document_name: &'a dyn DocumentName,
    ) -> Self {
        Self {
            collection_client,
            document_name,
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

    fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_, CUB, No> {
        requests::GetDocumentBuilder::new(self)
    }

    fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_, CUB, No> {
        requests::DeleteDocumentBuilder::new(self)
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
