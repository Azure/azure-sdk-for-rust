use super::*;
use crate::operations::*;
use crate::resources::trigger::{TriggerOperation, TriggerType};
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};

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

    /// Replace a trigger
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

    /// Delete a trigger
    pub fn delete_trigger(&self) -> DeleteTriggerBuilder {
        DeleteTriggerBuilder::new(self.clone())
    }

    pub(crate) fn prepare_pipeline_with_trigger_name(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/triggers/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.trigger_name()
            ),
            method,
        )
    }

    pub(crate) fn prepare_pipeline(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
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
