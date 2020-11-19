use super::{AttachmentClient, CollectionClient, CosmosClient, DatabaseClient};
use crate::requests;
use crate::{PartitionKeys, ReadonlyString, ResourceType};

#[derive(Debug, Clone)]
pub struct DocumentClient {
    collection_client: CollectionClient,
    document_name: ReadonlyString,
    partition_keys: PartitionKeys,
}

impl DocumentClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection_client: CollectionClient,
        document_name: S,
        partition_keys: PartitionKeys,
    ) -> Self {
        Self {
            collection_client,
            document_name: document_name.into(),
            partition_keys,
        }
    }

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client().hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client().cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client().database_client()
    }

    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    pub fn document_name(&self) -> &str {
        &self.document_name
    }

    pub fn partition_keys(&self) -> &PartitionKeys {
        &self.partition_keys
    }

    pub fn get_document(&self) -> requests::GetDocumentBuilder<'_, '_> {
        requests::GetDocumentBuilder::new(self)
    }

    pub fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_> {
        requests::DeleteDocumentBuilder::new(self)
    }

    pub fn list_attachments(&self) -> requests::ListAttachmentsBuilder<'_, '_> {
        requests::ListAttachmentsBuilder::new(self)
    }

    pub fn into_attachment_client<S: Into<ReadonlyString>>(
        self,
        attachment_name: S,
    ) -> AttachmentClient {
        AttachmentClient::new(self, attachment_name)
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.database_client().database_name(),
                self.collection_client().collection_name()
            ),
            method,
            ResourceType::Documents,
        )
    }

    pub fn prepare_request_with_document_name(
        &self,
        method: hyper::Method,
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
}
