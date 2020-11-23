use crate::collection::CollectionName;
use crate::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::prelude::*;
use crate::responses::CreateCollectionResponse;
use crate::{Offer, ResourceType};

use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<'a> {
    database_client: &'a DatabaseClient,
    offer: Offer,
    collection_name: &'a dyn CollectionName,
    indexing_policy: &'a IndexingPolicy,
    partition_key: &'a PartitionKey,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateCollectionBuilder<'a> {
    pub(crate) fn new(
        database_client: &'a DatabaseClient,
        offer: Offer,
        collection_name: &'a dyn CollectionName,
        indexing_policy: &'a IndexingPolicy,
        partition_key: &'a PartitionKey,
    ) -> Self {
        Self {
            database_client,
            offer,
            collection_name,
            indexing_policy,
            partition_key,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }

    pub fn with_offer(self, offer: Offer) -> Self {
        Self { offer, ..self }
    }

    pub fn with_collection_name(self, collection_name: &'a dyn CollectionName) -> Self {
        Self {
            collection_name,
            ..self
        }
    }

    pub fn with_indexing_policy(self, indexing_policy: &'a IndexingPolicy) -> Self {
        Self {
            indexing_policy,
            ..self
        }
    }

    pub fn with_partition_key(self, partition_key: &'a PartitionKey) -> Self {
        Self {
            partition_key,
            ..self
        }
    }

    pub fn with_user_agent(self, user_agent: &'a str) -> Self {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }

    pub fn with_activity_id(self, activity_id: &'a str) -> Self {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }

    pub fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }

    pub async fn execute(&self) -> Result<CreateCollectionResponse, CosmosError> {
        trace!("CreateCollectionBuilder::execute called");

        let mut req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        // add trait headers
        let req = OfferRequired::add_header(self, req);
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let mut collection = Collection::new(
            self.collection_name().name(),
            self.indexing_policy().to_owned(),
        );
        collection.parition_key = self.partition_key().to_owned();

        let body = serde_json::to_string(&collection)?;
        debug!("body == {}", body);

        let req = req.body(body.as_bytes())?;
        debug!("\nreq == {:?}", req);

        Ok(self
            .database_client
            .http_client()
            .execute_request_check_status(req, StatusCode::CREATED)
            .await?
            .try_into()?)
    }
}

impl<'a> DatabaseClientRequired<'a> for CreateCollectionBuilder<'a> {
    fn database_client(&self) -> &'a DatabaseClient {
        self.database_client
    }
}

impl<'a> OfferRequired for CreateCollectionBuilder<'a> {
    fn offer(&self) -> Offer {
        self.offer
    }
}

impl<'a> CollectionNameRequired<'a> for CreateCollectionBuilder<'a> {
    fn collection_name(&self) -> &'a dyn CollectionName {
        self.collection_name
    }
}

impl<'a> IndexingPolicyRequired<'a> for CreateCollectionBuilder<'a> {
    fn indexing_policy(&self) -> &'a IndexingPolicy {
        self.indexing_policy
    }
}

impl<'a> PartitionKeyRequired<'a> for CreateCollectionBuilder<'a> {
    fn partition_key(&self) -> &'a PartitionKey {
        self.partition_key
    }
}

impl<'a> UserAgentOption<'a> for CreateCollectionBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for CreateCollectionBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for CreateCollectionBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}
