use crate::clients::{CosmosUriBuilder, DocumentClient};
use crate::prelude::*;
use crate::responses::GetDocumentResponse;
use crate::DocumentBuilderTrait;
use crate::DocumentClientRequired;
use azure_sdk_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use azure_sdk_core::modify_conditions::IfMatchCondition;
use azure_sdk_core::prelude::*;
use azure_sdk_core::{IfMatchConditionOption, IfMatchConditionSupport};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    document_client: &'a DocumentClient<'a, CUB>,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel<'b>>,
}

impl<'a, 'b, CUB> GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    pub(crate) fn new(
        document_client: &'a DocumentClient<'a, CUB>,
    ) -> GetDocumentBuilder<'a, 'b, CUB> {
        GetDocumentBuilder {
            document_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b, CUB> DocumentClientRequired<'a, CUB> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn document_client(&self) -> &'a DocumentClient<'a, CUB> {
        self.document_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, 'b, CUB> IfMatchConditionOption<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b, CUB> IfModifiedSinceOption<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b, CUB> UserAgentOption<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b, CUB> ActivityIdOption<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b, CUB> ConsistencyLevelOption<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    #[inline]
    fn consistency_level(&self) -> Option<ConsistencyLevel<'b>> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b, CUB> IfMatchConditionSupport<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        GetDocumentBuilder {
            document_client: self.document_client,
            if_match_condition: Some(if_match_condition),
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> IfModifiedSinceSupport<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        GetDocumentBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: Some(if_modified_since),
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> UserAgentSupport<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        GetDocumentBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: Some(user_agent),
            activity_id: self.activity_id,
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> ActivityIdSupport<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        GetDocumentBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: Some(activity_id),
            consistency_level: self.consistency_level,
        }
    }
}

impl<'a, 'b, CUB> ConsistencyLevelSupport<'b> for GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    type O = GetDocumentBuilder<'a, 'b, CUB>;

    #[inline]
    fn with_consistency_level(self, consistency_level: ConsistencyLevel<'b>) -> Self::O {
        GetDocumentBuilder {
            document_client: self.document_client,
            if_match_condition: self.if_match_condition,
            if_modified_since: self.if_modified_since,
            user_agent: self.user_agent,
            activity_id: self.activity_id,
            consistency_level: Some(consistency_level),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, CUB> GetDocumentBuilder<'a, 'b, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub async fn execute<T>(&self) -> Result<GetDocumentResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        let mut req = self.document_client.prepare_request(hyper::Method::GET);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);

        req = crate::add_partition_keys_header(self.document_client.partition_keys(), req);

        let req = req.body(hyper::Body::empty())?;

        let (status_code, headers, whole_body) =
            extract_status_headers_and_body(self.document_client.hyper_client().request(req))
                .await?;

        if status_code != StatusCode::OK
            && status_code != StatusCode::NOT_MODIFIED
            && status_code != StatusCode::NOT_FOUND
        {
            return Err(UnexpectedHTTPResult::new_multiple(
                vec![
                    StatusCode::OK,
                    StatusCode::NOT_MODIFIED,
                    StatusCode::NOT_FOUND,
                ],
                status_code,
                std::str::from_utf8(&whole_body)?,
            )
            .into());
        }

        debug!("\nheaders == {:?}", headers);
        debug!("\nwhole body == {:#?}", whole_body);

        let resp = GetDocumentResponse::try_from((status_code, &headers, &whole_body as &[u8]))?;
        Ok(resp)
    }
}
