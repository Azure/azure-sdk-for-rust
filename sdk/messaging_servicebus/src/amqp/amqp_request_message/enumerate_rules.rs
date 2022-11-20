use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::{
    amqp_response_message::enumerate_rules::EnumerateRulesResponse,
    management_constants::{
        operations::ENUMERATE_RULES_OPERATION,
        properties::{SKIP, TOP},
    },
};

type EnumerateRulesRequestBody = OrderedMap<String, i32>;

pub(crate) struct EnumerateRulesRequest {
    server_timeout: Option<u32>,
    body: EnumerateRulesRequestBody,
}

impl EnumerateRulesRequest {
    pub fn new(top: i32, skip: i32) -> Self {
        let mut body = OrderedMap::new();
        body.insert(TOP.to_string(), top);
        body.insert(SKIP.to_string(), skip);

        Self {
            server_timeout: None,
            body,
        }
    }
}

impl Request for EnumerateRulesRequest {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = EnumerateRulesRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a EnumerateRulesRequest {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = &'a EnumerateRulesRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a mut EnumerateRulesRequest {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = &'a EnumerateRulesRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
