use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::{
    amqp_response_message::renew_lock::RenewLockResponse,
    management_constants::{operations::RENEW_LOCK_OPERATION, properties::LOCK_TOKENS},
};

pub(super) type LockTokens = Array<serde_amqp::primitives::Uuid>;
type RenewLockRequestBody = OrderedMap<String, LockTokens>;

pub(crate) struct RenewLockRequest {
    server_timeout: Option<u32>,
    associated_link_name: Option<String>,
    body: OrderedMap<String, LockTokens>,
}

impl RenewLockRequest {
    pub fn new(lock_tokens: LockTokens, associated_link_name: Option<String>) -> Self {
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

impl Request for RenewLockRequest {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = RenewLockRequestBody;

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

impl<'a> Request for &'a mut RenewLockRequest {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = &'a RenewLockRequestBody;

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

impl<'a> Request for &'a RenewLockRequest {
    const OPERATION: &'static str = RENEW_LOCK_OPERATION;

    type Response = RenewLockResponse;

    type Body = &'a RenewLockRequestBody;

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
