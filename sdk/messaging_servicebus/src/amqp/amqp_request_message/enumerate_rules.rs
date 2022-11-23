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

pub(crate) struct EnumerateRulesRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: EnumerateRulesRequestBody,
}

impl<'a> EnumerateRulesRequest<'a> {
    pub fn new(top: i32, skip: i32, associated_link_name: Option<&'a str>) -> Self {
        let mut body = OrderedMap::new();
        body.insert(TOP.to_string(), top);
        body.insert(SKIP.to_string(), skip);

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

impl<'a> Request for EnumerateRulesRequest<'a> {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = EnumerateRulesRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> Request for &'a EnumerateRulesRequest<'b> {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = &'a EnumerateRulesRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a mut EnumerateRulesRequest<'b> {
    const OPERATION: &'static str = ENUMERATE_RULES_OPERATION;

    type Response = EnumerateRulesResponse;

    type Body = &'a EnumerateRulesRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
