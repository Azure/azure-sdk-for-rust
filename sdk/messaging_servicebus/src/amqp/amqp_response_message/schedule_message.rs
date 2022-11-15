use fe2o3_amqp_management::error::Error as MgmtError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::OrderedMap;

use crate::amqp::management_constants::properties::SEQUENCE_NUMBERS;

pub(crate) struct ScheduleMessageResponse {
    pub(crate) map: OrderedMap<String, Vec<i64>>,
}

impl ScheduleMessageResponse {
    pub fn sequence_numbers(&self) -> Option<&[i64]> {
        self.map.get(SEQUENCE_NUMBERS).map(|v| v.as_slice())
    }
}

impl Response for ScheduleMessageResponse {
    const STATUS_CODE: u16 = 200;

    type Body = OrderedMap<String, Vec<i64>>;

    type Error = MgmtError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        Ok(Self { map: message.body })
    }
}
