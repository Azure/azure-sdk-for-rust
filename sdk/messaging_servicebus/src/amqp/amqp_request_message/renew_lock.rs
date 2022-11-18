use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::{management_constants::{properties::LOCK_TOKENS, operations::RENEW_LOCK_OPERATION}, amqp_response_message::renew_lock::RenewLockResponse};

type LockTokens = Array<serde_amqp::primitives::Uuid>;

pub(crate) struct RenewLockRequest {
    server_timeout: Option<u32>,
    body: OrderedMap<String, LockTokens>,
}

impl RenewLockRequest {
    pub fn new(lock_tokens: LockTokens) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(LOCK_TOKENS.into(), lock_tokens);
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for RenewLockRequest {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = OrderedMap<String, LockTokens>;

    fn encode_application_properties(&mut self) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a mut RenewLockRequest {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = &'a OrderedMap<String, LockTokens>;

    fn encode_application_properties(&mut self) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a RenewLockRequest {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = &'a OrderedMap<String, LockTokens>;

    fn encode_application_properties(&mut self) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
