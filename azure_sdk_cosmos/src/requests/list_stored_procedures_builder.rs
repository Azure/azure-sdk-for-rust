use crate::clients::{CosmosUriBuilder, ResourceType};
use crate::prelude::*;
use crate::responses::ListStoredProceduresResponse;
use crate::CollectionClient;
use crate::CollectionClientRequired;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    collection_client: &'a CollectionClient<'a, CUB>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel<'a>>,
}

impl<'a, CUB> ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        collection_client: &'a CollectionClient<'a, CUB>,
    ) -> ListStoredProceduresBuilder<'a, CUB> {
        ListStoredProceduresBuilder {
            collection_client,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, CUB> CollectionClientRequired<'a, CUB> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn collection_client(&self) -> &'a CollectionClient<'a, CUB> {
        self.collection_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB> UserAgentOption<'a> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a, CUB> ActivityIdOption<'a> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a, CUB> ConsistencyLevelOption<'a> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'a>> {
        self.consistency_level.clone()
    }
}

impl<'a, CUB> UserAgentSupport<'a> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListStoredProceduresBuilder<'a, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        ListStoredProceduresBuilder {
            collection_client: self.collection_client,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ActivityIdSupport<'a> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListStoredProceduresBuilder<'a, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        ListStoredProceduresBuilder {
            collection_client: self.collection_client,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, CUB> ConsistencyLevelSupport<'a> for ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = ListStoredProceduresBuilder<'a, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'a>) -> Self::O {
        ListStoredProceduresBuilder {
            collection_client: self.collection_client,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB> ListStoredProceduresBuilder<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute(&self) -> Result<ListStoredProceduresResponse, AzureError> {
        trace!("ListStoredProceduresBuilder::execute called");

        let req = self.collection_client.main_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/sprocs",
                self.collection_client.database_name().name(),
                self.collection_client.collection_name().name(),
            ),
            hyper::Method::GET,
            ResourceType::StoredProcedures,
        );

        // add trait headers
        let req = UserAgentOption::add_header(self, req);
        let req = ActivityIdOption::add_header(self, req);
        let req = ConsistencyLevelOption::add_header(self, req);

        let request = req.body(hyper::Body::empty())?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.collection_client().hyper_client().request(request),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
