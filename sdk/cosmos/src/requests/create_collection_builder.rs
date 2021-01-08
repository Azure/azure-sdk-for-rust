use crate::prelude::*;
use crate::resources::collection::{Collection, IndexingPolicy, PartitionKey};
use crate::resources::ResourceType;
use crate::responses::CreateCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<'a> {
    collection_name: &'a str,
    database_client: &'a DatabaseClient,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a> CreateCollectionBuilder<'a> {
    pub(crate) fn new(database_client: &'a DatabaseClient, collection_name: &'a str) -> Self {
        Self {
            collection_name,
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a> CreateCollectionBuilder<'a> {
    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
    }
}

impl<'a> CreateCollectionBuilder<'a> {
    // call this function to complete the builder
    pub async fn execute<O, I, P>(
        &self,
        partition_key: P,
        offer: O,
        indexing_policy: I,
    ) -> Result<CreateCollectionResponse, CosmosError>
    where
        P: Into<PartitionKey>,
        O: Into<Offer>,
        I: Into<IndexingPolicy>,
    {
        trace!("CreateCollectionBuilder::execute called");

        let mut req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        // add trait headers
        let req = azure_core::headers::add_mandatory_header(&offer.into(), req);
        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let mut collection = Collection::new(self.collection_name, indexing_policy.into());
        collection.parition_key = partition_key.into();

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
