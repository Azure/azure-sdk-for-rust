use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::schedule_message::ScheduleMessageResponse,
    management_constants::{
        operations::SCHEDULE_MESSAGE_OPERATION,
        properties::{MESSAGES},
    },
};

/// Type alias for scheduled messages that are encoded as maps
/// List of maps
type EncodedMessages = Vec<OrderedMap<String, Value>>;

pub(crate) struct ScheduleMessageRequest {
    server_timeout: Option<u32>,
    body: OrderedMap<String, EncodedMessages>,
}

impl ScheduleMessageRequest {
    pub fn new(messages: EncodedMessages) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(MESSAGES.into(), messages);
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl Request for ScheduleMessageRequest {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = OrderedMap<String, EncodedMessages>;

    fn encode_application_properties(
        &mut self,
    ) -> Option<fe2o3_amqp_types::messaging::ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

/// This is to avoid repeated serialization of the same messages
impl<'a> Request for &'a mut ScheduleMessageRequest {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = &'a OrderedMap<String, EncodedMessages>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
