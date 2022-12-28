use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::{
    amqp_response_message::renew_session_lock::RenewSessionLockResponse,
    management_constants::{operations::RENEW_SESSION_LOCK_OPERATION, properties::SESSION_ID},
};

type RenewSessionLockRequestBody = OrderedMap<String, String>;

pub(crate) struct RenewSessionLockRequest {
    server_timeout: Option<u32>,
    associated_link_name: Option<String>,
    body: OrderedMap<String, String>,
}

impl RenewSessionLockRequest {
    pub fn new(session_id: &str, associated_link_name: Option<String>) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(SESSION_ID.into(), session_id.into());
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

impl Request for RenewSessionLockRequest {
    const OPERATION: &'static str = RENEW_SESSION_LOCK_OPERATION;

    type Response = RenewSessionLockResponse;

    type Body = RenewSessionLockRequestBody;

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

impl<'a> Request for &'a mut RenewSessionLockRequest {
    const OPERATION: &'static str = RENEW_SESSION_LOCK_OPERATION;

    type Response = RenewSessionLockResponse;

    type Body = &'a RenewSessionLockRequestBody;

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

impl<'a> Request for &'a RenewSessionLockRequest {
    const OPERATION: &'static str = RENEW_SESSION_LOCK_OPERATION;

    type Response = RenewSessionLockResponse;

    type Body = &'a RenewSessionLockRequestBody;

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
