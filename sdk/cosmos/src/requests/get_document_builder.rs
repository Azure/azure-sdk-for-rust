use crate::prelude::*;
use crate::responses::GetDocumentResponse;
use crate::DocumentClientRequired;
use azure_core::errors::{extract_status_headers_and_body, AzureError, UnexpectedHTTPResult};
use azure_core::modify_conditions::IfMatchCondition;
use azure_core::prelude::*;
use azure_core::{IfMatchConditionOption, IfMatchConditionSupport};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryFrom;

#[derive(Debug, Clone)]
pub struct GetDocumentBuilder<'a, 'b> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'b>>,
    if_modified_since: Option<&'b DateTime<Utc>>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
}

impl<'a, 'b> GetDocumentBuilder<'a, 'b> {
    pub(crate) fn new(document_client: &'a DocumentClient) -> Self {
        Self {
            document_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
        }
    }
}

impl<'a, 'b> DocumentClientRequired<'a> for GetDocumentBuilder<'a, 'b> {
    fn document_client(&self) -> &'a DocumentClient {
        self.document_client
    }
}

impl<'a, 'b> IfMatchConditionOption<'b> for GetDocumentBuilder<'a, 'b> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'b>> {
        self.if_match_condition
    }
}

impl<'a, 'b> IfModifiedSinceOption<'b> for GetDocumentBuilder<'a, 'b> {
    fn if_modified_since(&self) -> Option<&'b DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a, 'b> UserAgentOption<'b> for GetDocumentBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for GetDocumentBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for GetDocumentBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> IfMatchConditionSupport<'b> for GetDocumentBuilder<'a, 'b> {
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'b>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a, 'b> IfModifiedSinceSupport<'b> for GetDocumentBuilder<'a, 'b> {
    type O = Self;

    fn with_if_modified_since(self, if_modified_since: &'b DateTime<Utc>) -> Self::O {
        Self {
            if_modified_since: Some(if_modified_since),
            ..self
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for GetDocumentBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for GetDocumentBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for GetDocumentBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> GetDocumentBuilder<'a, 'b> {
    pub async fn execute<T>(&self) -> Result<GetDocumentResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        let mut req = self
            .document_client
            .prepare_request_with_document_name(hyper::Method::GET);

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
