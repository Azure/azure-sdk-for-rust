use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::{
    amqp_response_message::renew_lock::RenewLockResponse,
    management_constants::{operations::RENEW_LOCK_OPERATION, properties::LOCK_TOKENS},
};

type LockTokens = Array<serde_amqp::primitives::Uuid>;

pub(crate) struct RenewLockRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: OrderedMap<String, LockTokens>,
}

impl<'a> RenewLockRequest<'a> {
    pub fn new(lock_tokens: LockTokens, associated_link_name: Option<&'a str>) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(LOCK_TOKENS.into(), lock_tokens);
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

impl<'a> Request for RenewLockRequest<'a> {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = OrderedMap<String, LockTokens>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> Request for &'a mut RenewLockRequest<'b> {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = &'a OrderedMap<String, LockTokens>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a RenewLockRequest<'b> {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = &'a OrderedMap<String, LockTokens>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
