use fe2o3_amqp_management::{error::InvalidType, response::Response};
use fe2o3_amqp_types::primitives::{OrderedMap, Uuid};
use serde_amqp::Value;

use crate::amqp::management_constants::properties::{LOCK_TOKEN, MESSAGE, MESSAGES};

type DeferredMessage = OrderedMap<String, Value>;
type ListOfDeferredMessages = Vec<DeferredMessage>;
type ReceiveBySequenceNumberResponseBody = OrderedMap<String, ListOfDeferredMessages>;

pub struct ReceiveBySequenceNumberResponse {
    pub deferred_messages: Vec<(Uuid, Vec<u8>)>,
}

impl Response for ReceiveBySequenceNumberResponse {
    const STATUS_CODE: u16 = 200;

    type Body = ReceiveBySequenceNumberResponseBody;

    type Error = super::ManagementError;

    fn decode_message(
        mut message: fe2o3_amqp_types::messaging::Message<Self::Body>,
    ) -> Result<Self, Self::Error> {
        let messages = message.body.remove(MESSAGES).ok_or_else(|| InvalidType {
            expected: MESSAGES.into(),
            actual: "None".into(),
        })?;

        let deferred_messages = messages
            .into_iter()
            .map(|mut map| {
                let uuid = map.remove(LOCK_TOKEN).map(Uuid::try_from);
                let message = map.remove(MESSAGE).map(Vec::<u8>::try_from);
                match (uuid, message) {
                    (Some(Ok(uuid)), Some(Ok(message))) => Some((uuid, message)),
                    _ => None,
                }
            })
            .collect::<Option<Vec<_>>>()
            .ok_or(super::ManagementError::DecodeError(None))?;

        Ok(Self { deferred_messages })
    }
}
