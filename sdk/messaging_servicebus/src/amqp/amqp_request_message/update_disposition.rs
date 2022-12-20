use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Array, OrderedMap, Uuid};
use serde_amqp::Value;

use crate::{
    amqp::{
        amqp_response_message::update_disposition::UpdateDispositionResponse,
        management_constants::{
            operations::UPDATE_DISPOSITION_OPERATION,
            properties::{
                DEAD_LETTER_DESCRIPTION, DEAD_LETTER_REASON, DISPOSITION_STATUS, LOCK_TOKENS,
                PROPERTIES_TO_MODIFY, SESSION_ID,
            },
        },
    },
    primitives::disposition_status::DispositionStatus,
};

type UpdateDispositionRequestBody = OrderedMap<String, Value>;

pub(crate) struct UpdateDispositionRequest {
    server_timeout: Option<u32>,
    associated_link_name: Option<String>,
    body: UpdateDispositionRequestBody,
}

impl UpdateDispositionRequest {
    pub fn new(
        disposition_status: DispositionStatus,
        lock_tokens: Array<Uuid>,
        dead_letter_reason: Option<String>,
        dead_letter_description: Option<String>,
        properties_to_modify: Option<OrderedMap<String, Value>>,
        session_id: Option<&str>,
        associated_link_name: Option<String>,
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
        if let Some(session_id) = session_id {
            body.insert(SESSION_ID.into(), session_id.into());
        }

        Self {
            server_timeout: None,
            associated_link_name,
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
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
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
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
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
        // TODO: reduce clones?
        super::encode_application_properties(self.server_timeout, self.associated_link_name.clone())
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
