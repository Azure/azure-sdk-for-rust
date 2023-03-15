use fe2o3_amqp_types::messaging::{Message, Data};

use crate::Event;

use super::amqp_property;

pub(crate) fn build_amqp_message_from_event(event: Event, partition_key: String) -> Message<Data> {
    let mut message = event.amqp_message;

    // add partition key to message annotation
    message.message_annotations
        .get_or_insert(Default::default())
        .insert(amqp_property::PARTITION_KEY.into(), partition_key.into());

    message
}
