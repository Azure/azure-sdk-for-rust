use serde_amqp::{SerializeComposite, DeserializeComposite};

use crate::consumer::{EventPosition, error::OffsetIsEmpty};

/// <summary>Indicates filtering based on the sequence number of a message.</summary>
const SEQUENCE_NUMBER_NAME: &str = "amqp.annotation.x-opt-sequence-number";

/// <summary>Indicates filtering based on the offset of a message.</summary>
const OFFSET_NAME: &str = "amqp.annotation.x-opt-offset";

/// <summary>Indicates filtering based on time that a message was enqueued.</summary>
const ENQUEUED_TIME_NAME: &str = "amqp.annotation.x-opt-enqueued-time";

#[derive(Debug, PartialEq, Eq, Hash, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "apache.org:selector-filter:string",
    code = "0x0000_0013_7000_000A",
    encoding = "basic",
)]
pub struct ConsumerFilter(pub String);

pub fn build_filter_expression(event_position: EventPosition) -> Result<String, OffsetIsEmpty> {
    match event_position {
        EventPosition::Offset { offset, is_inclusive } => {
            if offset.is_empty() {
                return Err(OffsetIsEmpty);
            }
            Ok(format!("{} {} {}", OFFSET_NAME, if is_inclusive { ">=" } else { ">" }, offset))
        },
        EventPosition::SequenceNumber { sequence_number, is_inclusive } => {
            Ok(format!("{} {} {}", SEQUENCE_NUMBER_NAME, if is_inclusive { ">=" } else { ">" }, sequence_number))
        },
        EventPosition::EnqueuedTime(enqueued_time) => {
            let millis = enqueued_time.unix_timestamp_nanos() / 1_000_000;
            Ok(format!("{} > {}", ENQUEUED_TIME_NAME, millis))
        },
    }
}
