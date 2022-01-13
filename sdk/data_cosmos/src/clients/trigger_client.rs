use super::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::HttpClient;

/// A client for Cosmos trigger resources.
#[derive(Debug, Clone)]
pub struct TriggerClient {
    collection_client: CollectionClient,
    trigger_name: ReadonlyString,
}

impl TriggerClient {
    /// Create a new trigger client
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection_client: CollectionClient,
        trigger_name: S,
    ) -> Self {
        Self {
            collection_client,
            trigger_name: trigger_name.into(),
        }
    }

    /// Get a [`CosmosClient`]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client.cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client.database_client()
    }

    /// Get a [`CollectionClient`]
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    /// Get the trigger name
    pub fn trigger_name(&self) -> &str {
        &self.trigger_name
    }

    /// Create a trigger
    pub fn create_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_> {
        requests::CreateOrReplaceTriggerBuilder::new(self, true)
    }

    /// Replace a trigger
    pub fn replace_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_> {
        requests::CreateOrReplaceTriggerBuilder::new(self, false)
    }

    /// Delete a trigger
    pub fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, '_> {
        requests::DeleteTriggerBuilder::new(self)
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    pub(crate) fn prepare_request_with_trigger_name(
        &self,
        method: http::Method,
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

    pub(crate) fn prepare_request(&self, method: http::Method) -> http::request::Builder {
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
}
