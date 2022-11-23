use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::{
    amqp_response_message::renew_session_lock::RenewSessionLockResponse,
    management_constants::{operations::RENEW_SESSION_LOCK_OPERATION, properties::SESSION_ID},
};

pub(crate) struct RenewSessionLockRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: OrderedMap<String, String>,
}

impl<'a> RenewSessionLockRequest<'a> {
    pub fn new(associated_link_name: Option<&'a str>, session_id: &str) -> Self {
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

impl<'a> Request for RenewSessionLockRequest<'a> {
    const OPERATION: &'static str = RENEW_SESSION_LOCK_OPERATION;

    type Response = RenewSessionLockResponse;

    type Body = OrderedMap<String, String>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> Request for &'a mut RenewSessionLockRequest<'b> {
    const OPERATION: &'static str = RENEW_SESSION_LOCK_OPERATION;

    type Response = RenewSessionLockResponse;

    type Body = &'a OrderedMap<String, String>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a RenewSessionLockRequest<'b> {
    const OPERATION: &'static str = RENEW_SESSION_LOCK_OPERATION;

    type Response = RenewSessionLockResponse;

    type Body = &'a OrderedMap<String, String>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
