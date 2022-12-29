use fe2o3_amqp_types::{
    messaging::{message::__private::Serializable, Data, Message},
    primitives::{Binary, OrderedMap},
};
use serde_amqp::Value;

use super::{
    amqp_message_converter::{build_amqp_batch_from_messages, SendableEnvelope},
    amqp_message_extensions::AmqpMessageExt,
    management_constants::properties::{
        MESSAGE, MESSAGE_ID, PARTITION_KEY, SESSION_ID, VIA_PARTITION_KEY,
    },
};

pub(crate) struct ScheduledBatchEnvelope {
    pub message_id: Option<String>,
    pub session_id: Option<String>,
    pub partition_key: Option<String>,
    pub via_partition_key: Option<String>,
    pub batch_envelope_bytes: Vec<u8>,
}

impl ScheduledBatchEnvelope {
    pub(crate) fn into_ordered_map(self) -> OrderedMap<String, Value> {
        let mut map = OrderedMap::with_capacity(5);
        map.insert(
            MESSAGE_ID.into(),
            self.message_id.map(Value::String).unwrap_or(Value::Null),
        );
        if let Some(session_id) = self.session_id {
            map.insert(SESSION_ID.into(), session_id.into());
        }
        if let Some(partition_key) = self.partition_key {
            map.insert(PARTITION_KEY.into(), partition_key.into());
        }
        if let Some(via_partition_key) = self.via_partition_key {
            map.insert(VIA_PARTITION_KEY.into(), via_partition_key.into());
        }

        map.insert(
            MESSAGE.into(),
            Value::Binary(Binary::from(self.batch_envelope_bytes)),
        );
        map
    }

    pub(crate) fn try_from_amqp_message(
        message: Message<Data>,
    ) -> Result<Option<Self>, serde_amqp::Error> {
        let message_id = message.message_id().map(|id| id.to_string());
        let session_id = message.session_id().map(|id| id.to_string());
        let partition_key = message.partition_key().map(|key| key.to_string());
        let via_partition_key = message.via_partition_key().map(|key| key.to_string());
        let batch_envelope = match build_amqp_batch_from_messages(std::iter::once(message), false) {
            Some(envelope) => envelope,
            None => return Ok(None),
        };

        let bytes = match batch_envelope.sendable {
            SendableEnvelope::Single(sendable) => {
                serde_amqp::to_vec(&Serializable(sendable.message))?
            }
            SendableEnvelope::Batch(sendable) => {
                serde_amqp::to_vec(&Serializable(sendable.message))?
            }
        };

        Ok(Some(Self {
            message_id,
            session_id,
            partition_key,
            via_partition_key,
            batch_envelope_bytes: bytes,
        }))
    }
}
