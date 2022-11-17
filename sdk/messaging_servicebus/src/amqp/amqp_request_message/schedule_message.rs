use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::schedule_message::ScheduleMessageResponse,
    management_constants::{
        operations::SCHEDULE_MESSAGE_OPERATION,
        properties::{MESSAGES, SERVER_TIMEOUT},
    },
};

/// Type alias for scheduled messages that are encoded as maps
type EncodedMessages = Vec<OrderedMap<String, Value>>;

pub struct ScheduleMessageRequestBody(OrderedMap<String, EncodedMessages>);

impl AsRef<OrderedMap<String, EncodedMessages>> for ScheduleMessageRequestBody {
    fn as_ref(&self) -> &OrderedMap<String, EncodedMessages> {
        &self.0
    }
}

impl ScheduleMessageRequestBody {
    pub fn new(messages: EncodedMessages) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(MESSAGES.into(), messages);
        Self(body)
    }

    pub fn into_inner(self) -> OrderedMap<String, EncodedMessages> {
        self.0
    }
}

pub(crate) struct ScheduleMessageRequest {
    server_timeout: u32,
    messages: OrderedMap<String, EncodedMessages>,
}

impl ScheduleMessageRequest {
    pub fn new(server_timeout: u32, body: ScheduleMessageRequestBody) -> Self {
        Self {
            server_timeout,
            messages: body.into_inner(),
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: u32) {
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
        Some(
            ApplicationProperties::builder()
                .insert(SERVER_TIMEOUT, self.server_timeout)
                .build(),
        )
    }

    fn encode_body(self) -> Self::Body {
        self.messages
    }
}

/// This is to avoid repeated serialization of the same messages
impl<'a> Request for &'a mut ScheduleMessageRequest {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = &'a OrderedMap<String, EncodedMessages>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        Some(
            ApplicationProperties::builder()
                .insert(SERVER_TIMEOUT, self.server_timeout)
                .build(),
        )
    }

    fn encode_body(self) -> Self::Body {
        &self.messages
    }
}
