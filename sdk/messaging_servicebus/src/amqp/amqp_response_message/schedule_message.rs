use fe2o3_amqp_management::error::Error as MgmtError;
use fe2o3_amqp_management::response::Response;
use fe2o3_amqp_types::primitives::OrderedMap;

pub(crate) struct ScheduleMessageResponse {}

impl Response for ScheduleMessageResponse {
    const STATUS_CODE: u16 = 200;

    type Body = OrderedMap<String, Vec<i64>>;

    type Error = MgmtError;

    fn decode_message(
        message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        todo!()
    }
}
