use crate::clients::*;
use crate::operations::*;
use crate::resources::trigger::{TriggerOperation, TriggerType};
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};

/// A client for Cosmos trigger resources.
#[derive(Debug, Clone)]
pub struct TriggerClient {
    collection: CollectionClient,
    trigger_name: ReadonlyString,
}

impl TriggerClient {
    /// Create a new trigger client
    pub(crate) fn new<S: Into<ReadonlyString>>(
        collection: CollectionClient,
        trigger_name: S,
    ) -> Self {
        Self {
            collection,
            trigger_name: trigger_name.into(),
        }
    }

    /// Create a trigger.
    pub fn create_trigger<B, T, O>(
        &self,
        body: B,
        trigger_type: T,
        trigger_operation: O,
    ) -> CreateOrReplaceTriggerBuilder
    where
        B: Into<String>,
        T: Into<TriggerType>,
        O: Into<TriggerOperation>,
    {
        CreateOrReplaceTriggerBuilder::new(
            self.clone(),
            true,
            body.into(),
            trigger_type.into(),
            trigger_operation.into(),
        )
    }

    /// Replace a trigger.
    pub fn replace_trigger<B, T, O>(
        &self,
        body: B,
        trigger_type: T,
        trigger_operation: O,
    ) -> CreateOrReplaceTriggerBuilder
    where
        B: Into<String>,
        T: Into<TriggerType>,
        O: Into<TriggerOperation>,
    {
        CreateOrReplaceTriggerBuilder::new(
            self.clone(),
            false,
            body.into(),
            trigger_type.into(),
            trigger_operation.into(),
        )
    }

    /// Delete a trigger.
    pub fn delete_trigger(&self) -> DeleteTriggerBuilder {
        DeleteTriggerBuilder::new(self.clone())
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection.cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection.database_client()
    }

    /// Get a [`CollectionClient`].
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection
    }

    /// Get the trigger name.
    pub fn trigger_name(&self) -> &str {
        &self.trigger_name
    }

    /// Create a request for a specific collection trigger
    pub(crate) fn trigger_request(&self, method: http::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/colls/{}/triggers/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.trigger_name()
            ),
            method,
        )
    }

    /// Create a request for collection triggers
    pub(crate) fn triggers_request(&self, method: http::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/colls/{}/triggers",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
