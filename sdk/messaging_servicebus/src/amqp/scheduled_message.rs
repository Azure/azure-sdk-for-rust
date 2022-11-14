use fe2o3_amqp_types::primitives::{Binary, OrderedMap};
use serde_amqp::Value;

use super::management_constants::properties::{
    MESSAGE, MESSAGE_ID, PARTITION_KEY, SESSION_ID, VIA_PARTITION_KEY,
};

pub(crate) struct ScheduledMessage {
    pub message_id: String,
    pub session_id: Option<String>,
    pub partition_key: Option<String>,
    pub via_partition_key: Option<String>,
    pub message: Vec<u8>,
}

impl ScheduledMessage {
    pub(crate) fn into_ordered_map(self) -> OrderedMap<String, Value> {
        let mut map = OrderedMap::with_capacity(5);
        map.insert(MESSAGE_ID.into(), self.message_id.into());
        map.insert(
            SESSION_ID.into(),
            self.session_id.map(Value::String).unwrap_or(Value::Null),
        );
        map.insert(
            PARTITION_KEY.into(),
            self.partition_key.map(Value::String).unwrap_or(Value::Null),
        );
        map.insert(
            VIA_PARTITION_KEY.into(),
            self.via_partition_key
                .map(Value::String)
                .unwrap_or(Value::Null),
        );
        map.insert(MESSAGE.into(), Value::Binary(Binary::from(self.message)));
        map
    }
}
