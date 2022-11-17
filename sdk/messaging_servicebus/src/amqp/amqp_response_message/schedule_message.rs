use fe2o3_amqp_management::error::Error as MgmtError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::{Array, OrderedMap};

use crate::amqp::management_constants::properties::SEQUENCE_NUMBERS;

pub(crate) struct ScheduleMessageResponse {
    pub(crate) map: OrderedMap<String, Array<i64>>,
}

impl ScheduleMessageResponse {
    pub fn into_sequence_numbers(mut self) -> Option<Array<i64>> {
        self.map.remove(SEQUENCE_NUMBERS)
    }
}

impl Response for ScheduleMessageResponse {
    const STATUS_CODE: u16 = 200;

    type Body = OrderedMap<String, Array<i64>>;

    type Error = MgmtError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self { map: message.body })
    }
}
