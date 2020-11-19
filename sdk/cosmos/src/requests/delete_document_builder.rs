use crate::prelude::*;
use crate::responses::DeleteDocumentResponse;
use crate::DocumentClientRequired;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::modify_conditions::IfMatchCondition;
use azure_core::prelude::*;
use azure_core::{IfMatchConditionOption, IfMatchConditionSupport};
use chrono::{DateTime, Utc};
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder<'a> {
    document_client: &'a DocumentClient,
    if_match_condition: Option<IfMatchCondition<'a>>,
    if_modified_since: Option<&'a DateTime<Utc>>,
    user_agent: Option<&'a str>,
    activity_id: Option<&'a str>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: bool,
}

impl<'a> DeleteDocumentBuilder<'a> {
    pub(crate) fn new(document_client: &'a DocumentClient) -> DeleteDocumentBuilder<'a> {
        Self {
            document_client,
            if_match_condition: None,
            if_modified_since: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
        }
    }
}

impl<'a> DocumentClientRequired<'a> for DeleteDocumentBuilder<'a> {
    fn document_client(&self) -> &'a DocumentClient {
        self.document_client
    }
}

impl<'a> IfMatchConditionOption<'a> for DeleteDocumentBuilder<'a> {
    fn if_match_condition(&self) -> Option<IfMatchCondition<'a>> {
        self.if_match_condition
    }
}

impl<'a> IfModifiedSinceOption<'a> for DeleteDocumentBuilder<'a> {
    fn if_modified_since(&self) -> Option<&'a DateTime<Utc>> {
        self.if_modified_since
    }
}

impl<'a> UserAgentOption<'a> for DeleteDocumentBuilder<'a> {
    fn user_agent(&self) -> Option<&'a str> {
        self.user_agent
    }
}

impl<'a> ActivityIdOption<'a> for DeleteDocumentBuilder<'a> {
    fn activity_id(&self) -> Option<&'a str> {
        self.activity_id
    }
}

impl<'a> ConsistencyLevelOption<'a> for DeleteDocumentBuilder<'a> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a> AllowTentativeWritesOption for DeleteDocumentBuilder<'a> {
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a> IfMatchConditionSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_if_match_condition(self, if_match_condition: IfMatchCondition<'a>) -> Self::O {
        Self {
            if_match_condition: Some(if_match_condition),
            ..self
        }
    }
}

impl<'a> IfModifiedSinceSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_if_modified_since(self, if_modified_since: &'a DateTime<Utc>) -> Self::O {
        Self {
            if_modified_since: Some(if_modified_since),
            ..self
        }
    }
}

impl<'a> UserAgentSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'a str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a> ActivityIdSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'a str) -> Self::O {
        Self {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a> ConsistencyLevelSupport<'a> for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a> AllowTentativeWritesSupport for DeleteDocumentBuilder<'a> {
    type O = Self;

    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        Self {
            allow_tentative_writes,
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a> DeleteDocumentBuilder<'a> {
    pub async fn execute(&self) -> Result<DeleteDocumentResponse, AzureError> {
        trace!("DeleteDocumentBuilder::execute called");

        let mut req = self
            .document_client
            .prepare_request_with_document_name(hyper::Method::DELETE);

        // add trait headers
        req = IfMatchConditionOption::add_header(self, req);
        req = IfModifiedSinceOption::add_header(self, req);
        req = UserAgentOption::add_header(self, req);
        req = ActivityIdOption::add_header(self, req);
        req = ConsistencyLevelOption::add_header(self, req);
        req = AllowTentativeWritesOption::add_header(self, req);

        req = crate::add_partition_keys_header(self.document_client.partition_keys(), req);

        let req = req.body(hyper::Body::empty())?;
        debug!("{:?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.document_client.hyper_client().request(req),
            StatusCode::NO_CONTENT,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
