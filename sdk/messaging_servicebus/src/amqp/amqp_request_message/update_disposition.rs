use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Array, OrderedMap, Uuid};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::update_disposition::UpdateDispositionResponse,
    management_constants::{
        operations::UPDATE_DISPOSITION_OPERATION,
        properties::{
            DEAD_LETTER_DESCRIPTION, DEAD_LETTER_REASON, DISPOSITION_STATUS, LOCK_TOKENS,
            PROPERTIES_TO_MODIFY,
        },
    },
};

type UpdateDispositionRequestBody = OrderedMap<String, Value>;

pub enum DispositionStatus {
    Completed,
    Abandoned,
    Suspended,
}

impl From<DispositionStatus> for String {
    fn from(status: DispositionStatus) -> Self {
        match status {
            DispositionStatus::Completed => "completed".to_string(),
            DispositionStatus::Abandoned => "abandoned".to_string(),
            DispositionStatus::Suspended => "suspended".to_string(),
        }
    }
}

impl From<DispositionStatus> for Value {
    fn from(status: DispositionStatus) -> Self {
        match status {
            DispositionStatus::Completed => Value::String("completed".into()),
            DispositionStatus::Abandoned => Value::String("abandoned".into()),
            DispositionStatus::Suspended => Value::String("suspended".into()),
        }
    }
}

pub(crate) struct UpdateDispositionRequest {
    server_timeout: Option<u32>,
    body: UpdateDispositionRequestBody,
}

impl UpdateDispositionRequest {
    pub fn new(
        disposition_status: DispositionStatus,
        lock_tokens: Array<Uuid>,
        dead_letter_reason: Option<String>,
        dead_letter_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
    ) -> Self {
        let mut body = UpdateDispositionRequestBody::new();
        body.insert(DISPOSITION_STATUS.into(), disposition_status.into());
        body.insert(LOCK_TOKENS.into(), lock_tokens.into());
        if let Some(reason) = dead_letter_reason {
            body.insert(DEAD_LETTER_REASON.into(), reason.into());
        }
        if let Some(description) = dead_letter_description {
            body.insert(DEAD_LETTER_DESCRIPTION.into(), description.into());
        }
        if let Some(properties) = properties_to_modify {
            body.insert(PROPERTIES_TO_MODIFY.into(), properties.into());
        }

        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for UpdateDispositionRequest {
    const OPERATION: &'static str = UPDATE_DISPOSITION_OPERATION;

    type Response = UpdateDispositionResponse;

    type Body = UpdateDispositionRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut UpdateDispositionRequest {
    const OPERATION: &'static str = UPDATE_DISPOSITION_OPERATION;

    type Response = UpdateDispositionResponse;

    type Body = &'a UpdateDispositionRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a UpdateDispositionRequest {
    const OPERATION: &'static str = UPDATE_DISPOSITION_OPERATION;

    type Response = UpdateDispositionResponse;

    type Body = &'a UpdateDispositionRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
