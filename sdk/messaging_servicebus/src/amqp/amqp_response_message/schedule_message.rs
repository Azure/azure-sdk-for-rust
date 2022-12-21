use fe2o3_amqp_management::error::Error as ManagementError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::management_constants::properties::SEQUENCE_NUMBERS;

type SequenceNumbers = Array<i64>;
type ScheduleMessageResponseBody = OrderedMap<String, SequenceNumbers>;

pub(crate) struct ScheduleMessageResponse {
    pub sequence_numbers: Vec<i64>,
}

impl ScheduleMessageResponse {
    pub fn into_sequence_numbers(self) -> Vec<i64> {
        self.sequence_numbers
    }
}

impl Response for ScheduleMessageResponse {
    const STATUS_CODE: u16 = 200;

    type Body = Option<ScheduleMessageResponseBody>;

    type Error = ManagementError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let sequence_numbers = message
            .body
            .ok_or(ManagementError::DecodeError(None))?
            .remove(SEQUENCE_NUMBERS)
            .ok_or_else(|| fe2o3_amqp_management::error::InvalidType {
                expected: SEQUENCE_NUMBERS.to_string(),
                actual: "None".to_string(),
            })?
            .into_inner();

        Ok(Self { sequence_numbers })
    }
}
