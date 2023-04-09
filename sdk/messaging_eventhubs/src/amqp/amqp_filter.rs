use const_format::concatcp;
use serde_amqp::{
    described::Described, descriptor::Descriptor, DeserializeComposite, SerializeComposite, Value,
};

use crate::{
    amqp::amqp_constants::APACHE,
    consumer::{error::OffsetIsEmpty, EventPosition},
};

/// <summary>Indicates filtering based on the sequence number of a message.</summary>
const SEQUENCE_NUMBER_NAME: &str = "amqp.annotation.x-opt-sequence-number";

/// <summary>Indicates filtering based on the offset of a message.</summary>
const OFFSET_NAME: &str = "amqp.annotation.x-opt-offset";

/// <summary>Indicates filtering based on time that a message was enqueued.</summary>
const ENQUEUED_TIME_NAME: &str = "amqp.annotation.x-opt-enqueued-time";

pub(crate) const CONSUMER_FILTER_NAME: &str = concatcp!(APACHE, ":selector-filter:string");

#[derive(Debug, PartialEq, Eq, Hash, Clone, SerializeComposite, DeserializeComposite)]
#[amqp_contract(
    name = "apache.org:selector-filter:string",
    code = "0x0000_0013_7000_000A",
    encoding = "basic"
)]
pub struct ConsumerFilter(pub String);

impl From<ConsumerFilter> for Described<String> {
    fn from(value: ConsumerFilter) -> Self {
        Self {
            descriptor: Descriptor::Code(0x0000_0013_7000_000A),
            value: value.0,
        }
    }
}

impl From<ConsumerFilter> for Described<Value> {
    fn from(value: ConsumerFilter) -> Self {
        Self {
            descriptor: Descriptor::Code(0x0000_0013_7000_000A),
            value: Value::String(value.0),
        }
    }
}

impl From<ConsumerFilter> for Option<Described<Value>> {
    fn from(value: ConsumerFilter) -> Self {
        Some(value.into())
    }
}

pub fn build_filter_expression(event_position: EventPosition) -> Result<String, OffsetIsEmpty> {
    match event_position {
        EventPosition::Offset {
            offset,
            is_inclusive,
        } => {
            if offset.is_empty() {
                return Err(OffsetIsEmpty);
            }
            Ok(format!(
                "{} {} {}",
                OFFSET_NAME,
                if is_inclusive { ">=" } else { ">" },
                offset
            ))
        }
        EventPosition::SequenceNumber {
            sequence_number,
            is_inclusive,
        } => Ok(format!(
            "{} {} {}",
            SEQUENCE_NUMBER_NAME,
            if is_inclusive { ">=" } else { ">" },
            sequence_number
        )),
        EventPosition::EnqueuedTime(enqueued_time) => {
            let millis = enqueued_time.unix_timestamp_nanos() / 1_000_000;
            Ok(format!("{} > {}", ENQUEUED_TIME_NAME, millis))
        }
    }
}
