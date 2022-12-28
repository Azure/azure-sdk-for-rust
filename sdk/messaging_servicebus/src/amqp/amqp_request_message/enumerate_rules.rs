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
    associated_link_name: Option<String>,
    body: EnumerateRulesRequestBody,
}

impl EnumerateRulesRequest {
    pub fn new(skip: i32, top: i32, associated_link_name: Option<String>) -> Self {
        let mut body = OrderedMap::new();
        body.insert(SKIP.to_string(), skip);
        body.insert(TOP.to_string(), top);

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

impl Request for EnumerateRulesRequest {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = EnumerateRulesRequestBody;

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

impl<'a> Request for &'a EnumerateRulesRequest {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = &'a EnumerateRulesRequestBody;

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

impl<'a> Request for &'a mut EnumerateRulesRequest {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = &'a EnumerateRulesRequestBody;

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
