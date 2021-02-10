use crate::prelude::*;
use crate::resources::collection::{IndexingPolicy, PartitionKey};
use crate::responses::CreateCollectionResponse;
use azure_core::prelude::*;
use http::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplaceCollectionBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    partition_key: Option<PartitionKey>,
    indexing_policy: Option<&'a IndexingPolicy>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            partition_key: None,
            indexing_policy: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        indexing_policy: &'a IndexingPolicy => Some(indexing_policy),
    }
}

impl<'a, 'b> ReplaceCollectionBuilder<'a, 'b> {
    pub async fn execute<P: Into<PartitionKey>>(
        &self,
        partition_key: P,
    ) -> Result<CreateCollectionResponse, CosmosError> {
        trace!("ReplaceCollectionBuilder::execute called");

        let req = self
            .collection_client
            .prepare_request_with_collection_name(http::Method::PUT);

        let req = azure_core::headers::add_optional_header(&self.user_agent, req);
        let req = azure_core::headers::add_optional_header(&self.activity_id, req);
        let req = azure_core::headers::add_optional_header(&self.consistency_level, req);

        let req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Debug, Clone, Serialize)]
        #[serde(rename_all = "camelCase")]
        struct Request<'k> {
            id: &'k str,
            #[serde(skip_serializing_if = "Option::is_none")]
            indexing_policy: Option<&'k IndexingPolicy>,
            partition_key: PartitionKey,
        }

        let request = Request {
            id: self.collection_client.collection_name(),
            indexing_policy: self.indexing_policy,
            partition_key: partition_key.into(),
        };

        let body = azure_core::to_json(&request)?;
        debug!("body == {:?}", body);

        let req = req.body(body)?;
        debug!("\nreq == {:?}", req);

        // the docs are wrong here
        // [https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection](https://docs.microsoft.com/en-us/rest/api/cosmos-db/replace-a-collection).
        // They say you should receive 201 instead azure returns 200 upon success. I've filed a PR
        // to correct it.
        Ok(self
            .collection_client
            .http_client()
            .execute_request_check_status(req, StatusCode::OK)
            .await?
            .try_into()
            .map_err(HttpRequestError::ResponseDeserializationError)?)
    }
}
