use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use crate::resources::ResourceType;
use crate::responses::CreateCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreateCollectionBuilder<'a> {
    partition_key: PartitionKey,
    database_client: &'a DatabaseClient,
    user_agent: Option<UserAgent<'a>>,
    activity_id: Option<ActivityId<'a>>,
    consistency_level: Option<ConsistencyLevel>,
    indexing_policy: Option<IndexingPolicy>,
    offer: Option<Offer>,
}

impl<'a> CreateCollectionBuilder<'a> {
    pub(crate) fn new(database_client: &'a DatabaseClient, partition_key: PartitionKey) -> Self {
        Self {
            partition_key,
            database_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            indexing_policy: None,
            offer: None,
        }
    }
}

impl<'a> CreateCollectionBuilder<'a> {
    setters! {
        user_agent: &'a str => Some(UserAgent::new(user_agent)),
        activity_id: &'a str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        indexing_policy: IndexingPolicy => Some(indexing_policy),
        offer: Offer => Some(offer),
    }
}

impl<'a> CreateCollectionBuilder<'a> {
    // call this function to complete the builder
    pub async fn execute<C: AsRef<str>>(
        &self,
        collection_name: C,
    ) -> Result<CreateCollectionResponse, HttpRequestError> {
        trace!("CreateCollectionBuilder::execute called");

        let mut req = self.database_client.cosmos_client().prepare_request(
            &format!("dbs/{}/colls", self.database_client.database_name()),
            http::Method::POST,
            ResourceType::Collections,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        // add trait headers
        let req = azure_core::headers::add_optional_header(&self.offer, req);
        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let collection = CreateCollectionBody {
            id: collection_name.as_ref(),
            indexing_policy: &self.indexing_policy,
            partition_key: &self.partition_key,
        };

        let body =
            azure_core::to_json(&collection).map_err(HttpRequestError::BodySerializationError)?;
        debug!("body == {:?}", body);

        let req = req.body(body).map_err(HttpRequestError::InvalidRequest)?;
        debug!("\nreq == {:?}", req);

        let response = self
            .database_client
            .http_client()
            .execute_request(req)
            .await
            .map_err(HttpRequestError::ErrorInTransit)
            .and_then(|r| HttpRequestError::check_response(r, vec![StatusCode::CREATED]))?;

        Ok(response.try_into()?)
    }
}

/// Body for the create collection request
#[derive(Serialize, Debug)]
struct CreateCollectionBody<'a> {
    pub id: &'a str,
    #[serde(rename = "indexingPolicy", skip_serializing_if = "Option::is_none")]
    pub indexing_policy: &'a Option<IndexingPolicy>,
    #[serde(rename = "partitionKey")]
    pub partition_key: &'a PartitionKey,
}
