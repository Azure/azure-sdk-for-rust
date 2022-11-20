use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::{
    amqp_response_message::remove_rule::RemoveRuleResponse,
    management_constants::{operations::REMOVE_RULE_OPERATION, properties::RULE_NAME},
};

type RemoveRuleRequestBody = OrderedMap<String, String>;

pub(crate) struct RemoveRuleRequest {
    server_timeout: Option<u32>,
    body: RemoveRuleRequestBody,
}

impl RemoveRuleRequest {
    pub fn new(rule_name: String) -> Self {
        let mut body = OrderedMap::new();
        body.insert(RULE_NAME.to_string(), rule_name);

        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for RemoveRuleRequest {
    const OPERATION: &'static str = REMOVE_RULE_OPERATION;

    type Response = RemoveRuleResponse;

    type Body = RemoveRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> Request for &'a RemoveRuleRequest {
    const OPERATION: &'static str = REMOVE_RULE_OPERATION;

    type Response = RemoveRuleResponse;

    type Body = &'a RemoveRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> Request for &'a mut RemoveRuleRequest {
    const OPERATION: &'static str = REMOVE_RULE_OPERATION;

    type Response = RemoveRuleResponse;

    type Body = &'a RemoveRuleRequestBody;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
