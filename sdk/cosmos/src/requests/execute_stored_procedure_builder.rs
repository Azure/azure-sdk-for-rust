use crate::prelude::*;
use crate::responses::ExecuteStoredProcedureResponse;
use crate::stored_procedure::Parameters;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::prelude::*;
use hyper::StatusCode;
use serde::de::DeserializeOwned;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ExecuteStoredProcedureBuilder<'a, 'b> {
    stored_procedure_client: &'a StoredProcedureClient,
    parameters: Option<&'b Parameters>,
    user_agent: Option<&'b str>,
    activity_id: Option<&'b str>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: bool,
    partition_keys: Option<&'b PartitionKeys>,
}

impl<'a, 'b> ExecuteStoredProcedureBuilder<'a, 'b> {
    pub(crate) fn new(stored_procedure_client: &'a StoredProcedureClient) -> Self {
        Self {
            stored_procedure_client,
            parameters: None,
            user_agent: None,
            activity_id: None,
            consistency_level: None,
            allow_tentative_writes: false,
            partition_keys: None,
        }
    }
}

impl<'a, 'b> StoredProcedureClientRequired<'a> for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn stored_procedure_client(&self) -> &'a StoredProcedureClient {
        self.stored_procedure_client
    }
}

impl<'a, 'b> ParametersOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn parameters(&self) -> Option<&'b Parameters> {
        self.parameters
    }
}

impl<'a, 'b> UserAgentOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn user_agent(&self) -> Option<&'b str> {
        self.user_agent
    }
}

impl<'a, 'b> ActivityIdOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn activity_id(&self) -> Option<&'b str> {
        self.activity_id
    }
}

impl<'a, 'b> ConsistencyLevelOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn consistency_level(&self) -> Option<ConsistencyLevel> {
        self.consistency_level.clone()
    }
}

impl<'a, 'b> AllowTentativeWritesOption for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn allow_tentative_writes(&self) -> bool {
        self.allow_tentative_writes
    }
}

impl<'a, 'b> PartitionKeysOption<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    fn partition_keys(&self) -> Option<&'b PartitionKeys> {
        self.partition_keys
    }
}

impl<'a, 'b> ParametersSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    type O = Self;

    fn with_parameters(self, parameters: &'b Parameters) -> Self::O {
        Self {
            parameters: Some(parameters),
            ..self
        }
    }
}

impl<'a, 'b> UserAgentSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    type O = Self;

    fn with_user_agent(self, user_agent: &'b str) -> Self::O {
        Self {
            user_agent: Some(user_agent),
            ..self
        }
    }
}

impl<'a, 'b> ActivityIdSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    type O = Self;

    fn with_activity_id(self, activity_id: &'b str) -> Self::O {
        ExecuteStoredProcedureBuilder {
            activity_id: Some(activity_id),
            ..self
        }
    }
}

impl<'a, 'b> ConsistencyLevelSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    type O = Self;

    fn with_consistency_level(self, consistency_level: ConsistencyLevel) -> Self::O {
        Self {
            consistency_level: Some(consistency_level),
            ..self
        }
    }
}

impl<'a, 'b> AllowTentativeWritesSupport for ExecuteStoredProcedureBuilder<'a, 'b> {
    type O = Self;

    fn with_allow_tentative_writes(self, allow_tentative_writes: bool) -> Self::O {
        Self {
            allow_tentative_writes,
            ..self
        }
    }
}

impl<'a, 'b> PartitionKeysSupport<'b> for ExecuteStoredProcedureBuilder<'a, 'b> {
    type O = Self;

    fn with_partition_keys(self, partition_keys: &'b PartitionKeys) -> Self::O {
        Self {
            partition_keys: Some(partition_keys),
            ..self
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b> ExecuteStoredProcedureBuilder<'a, 'b> {
    pub async fn execute<T>(&self) -> Result<ExecuteStoredProcedureResponse<T>, AzureError>
    where
        T: DeserializeOwned,
    {
        trace!("ExecuteStoredProcedureBuilder::execute called");

        let request = self
            .stored_procedure_client()
            .prepare_request_with_stored_procedure_name(hyper::Method::POST);

        // add trait headers
        let request = UserAgentOption::add_header(self, request);
        let request = ActivityIdOption::add_header(self, request);
        let request = ConsistencyLevelOption::add_header(self, request);
        let request = AllowTentativeWritesOption::add_header(self, request);
        let request = PartitionKeysOption::add_header(self, request);

        let request = request.header(http::header::CONTENT_TYPE, "application/json");

        let body = ParametersOption::generate_body(self);

        let request = request.body(hyper::Body::from(body))?;

        let (headers, body) = check_status_extract_headers_and_body(
            self.stored_procedure_client()
                .hyper_client()
                .request(request),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
