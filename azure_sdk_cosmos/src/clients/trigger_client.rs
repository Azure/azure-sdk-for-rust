use crate::clients::{Client, CollectionClient, CosmosUriBuilder, ResourceType};
use crate::collection::CollectionName;
use crate::database::DatabaseName;
use crate::requests;
use crate::trigger::TriggerName;
use crate::{CollectionTrait, TriggerBuilderTrait, TriggerTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct TriggerClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    trigger_name: &'a dyn TriggerName,
}

impl<'a, CUB> TriggerClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
        trigger_name: &'a dyn TriggerName,
    ) -> Self {
        TriggerClient {
            collection_client,
            trigger_name,
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

impl<'a, CUB> TriggerTrait<'a, CUB> for TriggerClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.collection_client.database_name()
    }

    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_client.collection_name()
    }

    fn trigger_name(&self) -> &'a dyn TriggerName {
        self.trigger_name
    }

    fn create_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, CUB, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, true)
    }

    fn replace_trigger(&self) -> requests::CreateOrReplaceTriggerBuilder<'_, CUB, No, No, No> {
        requests::CreateOrReplaceTriggerBuilder::new(self, false)
    }

    fn delete_trigger(&self) -> requests::DeleteTriggerBuilder<'_, CUB> {
        requests::DeleteTriggerBuilder::new(self)
    }
}

impl<'a, CUB> TriggerBuilderTrait<'a, CUB> for TriggerClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn prepare_request(
        &self,
        method: hyper::Method,
        specify_trigger_name: bool,
    ) -> http::request::Builder {
        if specify_trigger_name {
            self.main_client().prepare_request(
                &format!(
                    "dbs/{}/colls/{}/triggers/{}",
                    self.database_name().name(),
                    self.collection_name().name(),
                    self.trigger_name().name()
                ),
                method,
                ResourceType::Triggers,
            )
        } else {
            self.main_client().prepare_request(
                &format!(
                    "dbs/{}/colls/{}/triggers",
                    self.database_name().name(),
                    self.collection_name().name(),
                ),
                method,
                ResourceType::Triggers,
            )
        }
    }
}
