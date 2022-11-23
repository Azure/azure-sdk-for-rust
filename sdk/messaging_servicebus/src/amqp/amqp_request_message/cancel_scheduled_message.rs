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

pub(crate) struct CancelScheduledMessageRequest<'a> {
    server_timeout: Option<u32>,
    associated_link_name: Option<&'a str>,
    body: OrderedMap<String, Array<i64>>,
}

impl<'a> CancelScheduledMessageRequest<'a> {
    pub fn new(sequence_numbers: SequenceNumbers, associated_link_name: Option<&'a str>) -> Self {
        let mut body = OrderedMap::with_capacity(1);
        body.insert(SEQUENCE_NUMBERS.into(), sequence_numbers);
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

impl<'a> fe2o3_amqp_management::request::Request for CancelScheduledMessageRequest<'a> {
    const OPERATION: &'static str = CANCEL_SCHEDULED_MESSAGE_OPERATION;

    type Response = CancelScheduledMessageResponse;

    type Body = OrderedMap<String, Array<i64>>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        self.body
    }
}

impl<'a, 'b> fe2o3_amqp_management::request::Request for &'a mut CancelScheduledMessageRequest<'b> {
    const OPERATION: &'static str = CANCEL_SCHEDULED_MESSAGE_OPERATION;

    type Response = CancelScheduledMessageResponse;

    type Body = &'a OrderedMap<String, Array<i64>>;

    // TODO: override the blanket impl of `into_message()` to avoid repeated allocation of
    // `ApplicationProperties`
    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}

impl<'a, 'b> fe2o3_amqp_management::request::Request for &'a CancelScheduledMessageRequest<'b> {
    const OPERATION: &'static str = CANCEL_SCHEDULED_MESSAGE_OPERATION;

    type Response = CancelScheduledMessageResponse;

    type Body = &'a OrderedMap<String, Array<i64>>;

    fn encode_application_properties(&mut self) -> Option<ApplicationProperties> {
        super::encode_application_properties(self.server_timeout, self.associated_link_name)
    }

    fn encode_body(self) -> Self::Body {
        &self.body
    }
}
