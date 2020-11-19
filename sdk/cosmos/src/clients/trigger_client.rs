use super::*;
use crate::requests;
use crate::{ReadonlyString, ResourceType};
use azure_core::No;

#[derive(Debug, Clone)]
pub struct TriggerClient {
    collection_client: CollectionClient,
    trigger_name: ReadonlyString,
}

impl TriggerClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection_client: CollectionClient,
        trigger_name: S,
    ) -> Self {
        Self {
            collection_client,
            trigger_name: trigger_name.into(),
        }
    }

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.collection_client.hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client.cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client.database_client()
    }

    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/triggers",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
            ),
            method,
            ResourceType::Triggers,
        )
    }

    pub fn prepare_request_with_trigger_name(
        &self,
        method: hyper::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/triggers/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.trigger_name()
            ),
            method,
            ResourceType::Triggers,
        )
    }

    pub fn trigger_name(&self) -> &str {
        &self.trigger_name
    }

    pub fn create_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, true)
    }

    pub fn replace_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, false)
    }

    pub fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, '_> {
        requests::DeleteTriggerBuilder::new(self)
    }
}
