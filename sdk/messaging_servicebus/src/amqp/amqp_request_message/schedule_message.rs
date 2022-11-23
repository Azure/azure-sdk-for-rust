use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::schedule_message::ScheduleMessageResponse,
    management_constants::{operations::SCHEDULE_MESSAGE_OPERATION, properties::MESSAGES},
};

/// Type alias for scheduled messages that are encoded as maps
/// List of maps
type EncodedMessages = Vec<OrderedMap<String, Value>>;

pub(crate) struct ScheduleMessageRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: OrderedMap<String, EncodedMessages>,
}

impl<'a> ScheduleMessageRequest<'a> {
    pub fn new(messages: EncodedMessages, associated_link_name: Option<&'a str>) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(MESSAGES.into(), messages);
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

impl<'a> Request for ScheduleMessageRequest<'a> {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = OrderedMap<String, EncodedMessages>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

/// This is to avoid repeated serialization of the same messages
impl<'a, 'b> Request for &'a mut ScheduleMessageRequest<'b> {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = &'a OrderedMap<String, EncodedMessages>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> Request for &'a ScheduleMessageRequest<'b> {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = &'a OrderedMap<String, EncodedMessages>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
