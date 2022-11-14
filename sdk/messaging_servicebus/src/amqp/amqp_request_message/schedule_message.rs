use fe2o3_amqp_management::request::Request;
use fe2o3_amqp_types::{messaging::ApplicationProperties, primitives::OrderedMap};
use serde_amqp::Value;

use crate::amqp::{
    amqp_response_message::schedule_message::ScheduleMessageResponse,
    management_constants::{
        operations::SCHEDULE_MESSAGE_OPERATION,
        properties::{MESSAGES, SERVER_TIMEOUT},
    },
    scheduled_message::ScheduledMessage,
};

type Messages = Vec<OrderedMap<String, Value>>;

pub(crate) struct ScheduleMessageRequest {
    pub server_timeout: u32,
    pub messages: Vec<ScheduledMessage>,
}

impl Request for ScheduleMessageRequest {
    const OPERATION: &'static str = SCHEDULE_MESSAGE_OPERATION;

    type Response = ScheduleMessageResponse;

    type Body = OrderedMap<String, Messages>;

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
        let messages: Messages = self
            .messages
            .into_iter()
            .map(|message| message.into_ordered_map())
            .collect();
        let mut map = OrderedMap::with_capacity(1);
        map.insert(MESSAGES.into(), messages);
        map
    }
}
