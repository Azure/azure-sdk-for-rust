use fe2o3_amqp_types::{
    messaging::ApplicationProperties,
    primitives::{Array, OrderedMap},
};

use crate::amqp::{
    amqp_response_message::cancel_scheduled_message::CancelScheduledMessageResponse,
    management_constants::{
        operations::CANCEL_SCHEDULED_MESSAGE_OPERATION, properties::SEQUENCE_NUMBERS,
    },
};

type SequenceNumbers = Array<i64>;

pub(crate) struct CancelScheduledMessageRequest {
    server_timeout: Option<u32>,
    body: OrderedMap<String, Array<i64>>,
}

impl CancelScheduledMessageRequest {
    pub fn new(sequence_numbers: SequenceNumbers) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(SEQUENCE_NUMBERS.into(), sequence_numbers);
        Self {
            server_timeout: None,
            body,
        }
    }

    pub fn set_server_timeout(&mut self, server_timeout: Option<u32>) {
        self.server_timeout = server_timeout;
    }
}

impl fe2o3_amqp_management::request::Request for CancelScheduledMessageRequest {
    const OPERATION: &'static str = CANCEL_SCHEDULED_MESSAGE_OPERATION;

    type Response = CancelScheduledMessageResponse;

    type Body = OrderedMap<String, Array<i64>>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a> fe2o3_amqp_management::request::Request for &'a mut CancelScheduledMessageRequest {
    const OPERATION: &'static str = CANCEL_SCHEDULED_MESSAGE_OPERATION;

    type Response = CancelScheduledMessageResponse;

    type Body = &'a OrderedMap<String, Array<i64>>;

    // TODO: override the blanket impl of `into_message()` to avoid repeated allocation of
    // `ApplicationProperties`
    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a> fe2o3_amqp_management::request::Request for &'a CancelScheduledMessageRequest {
    const OPERATION: &'static str = CANCEL_SCHEDULED_MESSAGE_OPERATION;

    type Response = CancelScheduledMessageResponse;

    type Body = &'a OrderedMap<String, Array<i64>>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_server_timeout_as_application_properties(self.server_timeout)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
