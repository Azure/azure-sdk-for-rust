use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::{
    administration::{RuleDescription, RuleProperties},
    amqp::management_constants,
};

// type Rules = Vec<OrderedMap<String, Described<Value>>>;
type Rules = Vec<OrderedMap<String, RuleDescription>>;
type EnumerateRulesResponseBody = OrderedMap<String, Rules>;

pub(crate) struct EnumerateRulesResponse {
    pub body: EnumerateRulesResponseBody,
}

impl EnumerateRulesResponse {
    pub fn into_get_rules_response(mut self) -> Vec<RuleProperties> {
        let rules = self
            .body
            .remove(management_constants::properties::RULES)
            .unwrap_or_else(|| Vec::with_capacity(0));

        rules
            .into_iter()
            .filter_map(|mut entry| {
                entry.remove(management_constants::properties::RULE_DESCRIPTION)
            })
            .map(RuleProperties::from)
            .collect()
    }
}

impl Response for EnumerateRulesResponse {
    const STATUS_CODE: u16 = 200;

    type Body = Option<EnumerateRulesResponseBody>;

    type Error = super::ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            body: message.body.ok_or(Self::Error::DecodeError(None))?,
        })
    }
}
