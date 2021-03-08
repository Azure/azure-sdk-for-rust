use crate::cosmos_entity::{
    add_as_partition_key_header, add_as_partition_key_header_serialized,
    serialize_partition_key_to_string,
};
use crate::prelude::*;
use crate::resources::ResourceType;
use crate::responses::CreateDocumentResponse;
use azure_core::errors::UnexpectedHTTPResult;
use azure_core::prelude::*;
use chrono::{DateTime, Utc};
use http::StatusCode;
use serde::Serialize;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct CreateDocumentBuilder<'a, 'b> {
    collection_client: &'a CollectionClient,
    is_upsert: IsUpsert,
    indexing_directive: IndexingDirective,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<IfModifiedSince<'b>>,
    user_agent: Option<UserAgent<'b>>,
    activity_id: Option<ActivityId<'b>>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TenativeWritesAllowance,
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b> {
    pub(crate) fn new(collection_client: &'a CollectionClient) -> Self {
        Self {
            collection_client,
            is_upsert: IsUpsert::No,
            indexing_directive: IndexingDirective::Default,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: TenativeWritesAllowance::Deny,
        }
    }
}

impl<'a, 'b> CreateDocumentBuilder<'a, 'b> {
    setters! {
        user_agent: &'b str => Some(UserAgent::new(user_agent)),
        activity_id: &'b str => Some(ActivityId::new(activity_id)),
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition<'b> => Some(if_match_condition),
        if_modified_since: &'b DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        allow_tentative_writes: TenativeWritesAllowance,
        is_upsert: bool => if is_upsert { IsUpsert::Yes } else { IsUpsert::No },
        indexing_directive: IndexingDirective,
    }
}

impl<'a, 'b, 'c> CreateDocumentBuilder<'a, 'b> {
    async fn execute_internal<DOC, FNPK>(
        &self,
        document: &'c DOC,
        fn_add_primary_key: FNPK,
    ) -> Result<CreateDocumentResponse, CosmosError>
    where
        DOC: Serialize,
        FNPK: FnOnce(http::request::Builder) -> Result<http::request::Builder, serde_json::Error>,
    {
        let mut req = self.collection_client.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs",
                self.collection_client.database_client().database_name(),
                self.collection_client.collection_name()
            ),
            http::Method::POST,
            ResourceType::Documents,
        );

        req = fn_add_primary_key(req)?;

        req = azure_core::headers::add_optional_header(&self.if_match_condition, req);
        req = azure_core::headers::add_optional_header(&self.if_modified_since, req);
        req = azure_core::headers::add_optional_header(&self.user_agent, req);
        req = azure_core::headers::add_optional_header(&self.activity_id, req);
        req = azure_core::headers::add_optional_header(&self.consistency_level, req);
        req = azure_core::headers::add_mandatory_header(&self.is_upsert, req);
        req = azure_core::headers::add_mandatory_header(&self.indexing_directive, req);
        req = azure_core::headers::add_mandatory_header(&self.allow_tentative_writes, req);

        let serialized = azure_core::to_json(document)?;
        let req = req.body(serialized)?;

        let response = self
            .collection_client
            .http_client()
            .execute_request(req)
            .await?;

        debug!("status_core == {:?}", response.status());
        debug!("headers == {:?}", response.headers());
        debug!("whole body == {:#?}", response.body());

        if self.is_upsert == IsUpsert::No && response.status() != StatusCode::CREATED {
            return Err(UnexpectedHTTPResult::new(
                StatusCode::CREATED,
                response.status(),
                std::str::from_utf8(response.body())?,
            )
            .into());
        } else if response.status() != StatusCode::CREATED && response.status() != StatusCode::OK {
            return Err(UnexpectedHTTPResult::new_multiple(
                vec![StatusCode::CREATED, StatusCode::OK],
                response.status(),
                std::str::from_utf8(response.body())?,
            )
            .into());
        }

        CreateDocumentResponse::try_from(response)
    }

    pub async fn execute_with_partition_key<DOC: Serialize, PK: Serialize>(
        &self,
        document: &'c DOC,
        partition_key: &PK,
    ) -> Result<CreateDocumentResponse, CosmosError> {
        self.execute_internal(document, |req| {
            Ok(add_as_partition_key_header_serialized(
                &serialize_partition_key_to_string(partition_key)?,
                req,
            ))
        })
        .await
    }

    pub async fn execute<PK: Serialize + 'c, T: Serialize + CosmosEntity<'c, PK>>(
        &self,
        document: &'c T,
    ) -> Result<CreateDocumentResponse, CosmosError> {
        self.execute_internal(document, |req| {
            Ok(add_as_partition_key_header(document, req)?)
        })
        .await
    }
}
