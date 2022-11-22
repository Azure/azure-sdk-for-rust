use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::rules::RuleDescription;

type Rules = Array<OrderedMap<String, RuleDescription>>;
type EnumerateRulesResponseBody = OrderedMap<String, Rules>;

pub(crate) struct EnumerateRulesResponse {
    // TODO: The documentation is confusing. It says "an array of described objects" while
    // the dotnet sdk only decodes one described object. This needs to be investigated.
    pub body: EnumerateRulesResponseBody,
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
