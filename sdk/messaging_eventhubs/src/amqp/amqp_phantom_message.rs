use std::marker::PhantomData;

use fe2o3_amqp_types::messaging::{
    message::__private::Serializable, ApplicationProperties, Batch, Data, DeliveryAnnotations,
    Footer, Header, Message, MessageAnnotations, Properties,
};
use serde_amqp::serialized_size;

pub(crate) const U8_MAX: usize = u8::MAX as usize;

// Variable type will spend a byte on size
pub(crate) const U8_MAX_MINUS_1: usize = u8::MAX as usize - 1;

// Variable type will spend 4 bytes on size
pub(crate) const U32_MAX_MINUS_4: usize = u32::MAX as usize - 4;

/// A phantom that is used for calculating the serialized size of a type.
///
/// # Phantom<Data>
///
/// This can be constructed from two types.
///
/// 1. `&Data`
///
/// This is a simple case where the serialized size is copied over.
///
/// 2. `&Message<Data>`
///
/// This is a slightly more complex case because the message is serialized
/// entirely into a buffer and then the buffer is wrapped in a `Data` type.
/// So the total size will be the size of the buffer plus the size of the
/// overhead of the `Data` type.
#[derive(Debug)]
pub(crate) struct Phantom<T> {
    serialized_len: usize,
    marker: PhantomData<T>,
}

impl<T> Clone for Phantom<T> {
    fn clone(&self) -> Self {
        Self {
            serialized_len: self.serialized_len,
            marker: PhantomData,
        }
    }
}

impl<T> Copy for Phantom<T> {}

impl<T> Phantom<T> {
    pub(crate) fn new(serialized_len: usize) -> Self {
        Self {
            serialized_len,
            marker: PhantomData,
        }
    }
}

macro_rules! impl_try_from_type_for_phantom {
    ($ty:ty) => {
        impl<'a> TryFrom<&'a $ty> for Phantom<$ty> {
            type Error = serde_amqp::Error;

            fn try_from(value: &'a $ty) -> Result<Self, Self::Error> {
                let serialized_len = serialized_size(value)?;

                Ok(Self::new(serialized_len))
            }
        }
    };
    ($($ty:ty),+) => {
        $(impl_try_from_type_for_phantom!($ty);)+
    };
}

impl_try_from_type_for_phantom! {
    Header,
    DeliveryAnnotations,
    MessageAnnotations,
    Properties,
    ApplicationProperties,
    Data,
    Batch<Data>,
    Footer
}

macro_rules! impl_try_from_optional_type_for_phantom {
    (Option<$ty:ty>) => {
        impl<'a> TryFrom<&'a Option<$ty>> for Phantom<Option<$ty>> {
            type Error = serde_amqp::Error;

            fn try_from(value: &'a Option<$ty>) -> Result<Self, Self::Error> {
                match value {
                    Some(value) => {
                        Ok(Self::new(Phantom::try_from(value)?.serialized_len))
                    },
                    None => Ok(Phantom::new(0)),
                }
            }
        }
    };
    ($(Option<$ty:ty>),+) => {
        $(impl_try_from_optional_type_for_phantom!{Option<$ty>})+
    };
}

impl_try_from_optional_type_for_phantom! {
    Option<Header>,
    Option<DeliveryAnnotations>,
    Option<MessageAnnotations>,
    Option<Properties>,
    Option<ApplicationProperties>,
    Option<Footer>
}

impl Phantom<Data> {
    /// Create a new `Phantom<Data>` with the given message size.
    ///
    /// This is used when the message is serialized into a buffer and then wrapped in a `Data` type.
    pub(crate) fn with_message_size(message_size: usize) -> Result<Self, serde_amqp::Error> {
        // Number of bytes reserved for the length of the message
        let length_size = match message_size {
            0..=U8_MAX_MINUS_1 => 1usize,
            U8_MAX..=U32_MAX_MINUS_4 => 4usize,
            _ => return Err(serde_amqp::Error::InvalidLength),
        };

        let overhead = 1 // 0x00
            + 1 // 0x53, format code for `Descriptor::Code(SmallUlong)`
            + 1 // "0x0000_0000:0x0000_0075", value of descriptor for `Data`
            + 1 // format code for vbin8 or vbin32
            + length_size;

        Ok(Self::new(message_size + overhead))
    }
}

impl<'a> TryFrom<&'a Message<Data>> for Phantom<Data> {
    type Error = serde_amqp::Error;

    fn try_from(value: &'a Message<Data>) -> Result<Self, Self::Error> {
        // In the batched format, each message is serialized to bytes and wrapped in a Data section.
        // So there will be repeated descriptors, format code, and length byte(s) for the Data
        // section. The length bytes might be different.
        let message_size = serialized_size(&Serializable(value))?;

        Self::with_message_size(message_size)
    }
}

impl Phantom<Batch<Data>> {
    pub(crate) fn push(&mut self, data: Phantom<Data>) {
        self.serialized_len += data.serialized_len;
    }

    pub(crate) fn pop(&mut self, data: Phantom<Data>) {
        self.serialized_len -= data.serialized_len;
    }

    pub(crate) fn clear(&mut self) {
        self.serialized_len = 0;
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct PhantomMessage<T> {
    /// Transport headers for a message.
    pub(crate) header: Phantom<Option<Header>>,

    /// The delivery-annotations section is used for delivery-specific non-standard properties at the head of the message.
    pub(crate) delivery_annotations: Phantom<Option<DeliveryAnnotations>>,

    /// The message-annotations section is used for properties of the message which are aimed at the infrastructure
    /// and SHOULD be propagated across every delivery step
    pub(crate) message_annotations: Phantom<Option<MessageAnnotations>>,

    /// Immutable properties of the message.
    pub(crate) properties: Phantom<Option<Properties>>,

    /// The application-properties section is a part of the bare message used for structured application data.
    /// Intermediaries can use the data within this structure for the purposes of filtering or routin
    pub(crate) application_properties: Phantom<Option<ApplicationProperties>>,

    /// The body consists of one of the following three choices: one or more data sections, one or more amqp-sequence
    /// sections, or a single amqp-value section.
    pub(crate) body: Phantom<T>,

    /// Transport footers for a message.
    pub(crate) footer: Phantom<Option<Footer>>,
}

impl<T> PhantomMessage<T> {
    pub(crate) fn serialized_size(&self) -> usize {
        self.header.serialized_len
            + self.delivery_annotations.serialized_len
            + self.message_annotations.serialized_len
            + self.properties.serialized_len
            + self.application_properties.serialized_len
            + self.body.serialized_len
            + self.footer.serialized_len
    }
}

impl<'a> TryFrom<&'a Message<Data>> for PhantomMessage<Data> {
    type Error = serde_amqp::Error;

    fn try_from(value: &'a Message<Data>) -> Result<Self, Self::Error> {
        let header = Phantom::try_from(&value.header)?;
        let delivery_annotations = Phantom::try_from(&value.delivery_annotations)?;
        let message_annotations = Phantom::try_from(&value.message_annotations)?;
        let properties = Phantom::try_from(&value.properties)?;
        let application_properties = Phantom::try_from(&value.application_properties)?;
        let body = Phantom::try_from(&value.body)?;
        let footer = Phantom::try_from(&value.footer)?;

        Ok(Self {
            header,
            delivery_annotations,
            message_annotations,
            properties,
            application_properties,
            body,
            footer,
        })
    }
}

impl<'a> TryFrom<&'a Message<Batch<Data>>> for PhantomMessage<Batch<Data>> {
    type Error = serde_amqp::Error;

    fn try_from(value: &'a Message<Batch<Data>>) -> Result<Self, Self::Error> {
        let header = Phantom::try_from(&value.header)?;
        let delivery_annotations = Phantom::try_from(&value.delivery_annotations)?;
        let message_annotations = Phantom::try_from(&value.message_annotations)?;
        let properties = Phantom::try_from(&value.properties)?;
        let application_properties = Phantom::try_from(&value.application_properties)?;
        let body = Phantom::try_from(&value.body)?;
        let footer = Phantom::try_from(&value.footer)?;

        Ok(Self {
            header,
            delivery_annotations,
            message_annotations,
            properties,
            application_properties,
            body,
            footer,
        })
    }
}

#[cfg(test)]
mod tests {
    use fe2o3_amqp_types::messaging::message::__private::Serializable;
    use serde_amqp::{serialized_size, to_value, to_vec, Value};

    use crate::{
        amqp::amqp_message_converter::{build_amqp_batch_from_messages, SendableEnvelope},
        EventData,
    };

    use super::{Phantom, PhantomMessage};

    fn phantom_size_and_serialized_size_of_sendable_envelope(
        sendable: SendableEnvelope,
    ) -> (usize, usize) {
        match sendable {
            SendableEnvelope::Single(sendable) => {
                let phantom_message = PhantomMessage::try_from(&sendable.message).unwrap();
                let phantom_size = phantom_message.serialized_size();
                let serializable = Serializable(sendable.message);
                let ssize = serialized_size(&serializable).unwrap();

                (phantom_size, ssize)
            }
            SendableEnvelope::Batch(sendable) => {
                let phantom_message = PhantomMessage::try_from(&sendable.message).unwrap();
                let phantom_size = phantom_message.serialized_size();
                let serializable = Serializable(sendable.message);
                let ssize = serialized_size(&serializable).unwrap();

                (phantom_size, ssize)
            }
        }
    }

    fn serialized_value_of_sendable(sendable: SendableEnvelope) -> Value {
        match sendable {
            SendableEnvelope::Single(sendable) => {
                let serializable = Serializable(sendable.message);
                
                to_value(&serializable).unwrap()
            }
            SendableEnvelope::Batch(sendable) => {
                let serializable = Serializable(sendable.message);
                
                to_value(&serializable).unwrap()
            }
        }
    }

    fn serialized_bytes_of_sendable(sendable: SendableEnvelope) -> Vec<u8> {
        match sendable {
            SendableEnvelope::Single(sendable) => {
                let serializable = Serializable(sendable.message);
                
                to_vec(&serializable).unwrap()
            }
            SendableEnvelope::Batch(sendable) => {
                let serializable = Serializable(sendable.message);
                
                to_vec(&serializable).unwrap()
            }
        }
    }

    #[test]
    fn phantom_message_size_with_one_event() {
        let data = "abcdefghij";
        let event = EventData::from(data);
        let message_iter = std::iter::once(event.amqp_message);

        let batch = build_amqp_batch_from_messages(message_iter.clone(), None).unwrap();
        let serialized_value = serialized_value_of_sendable(batch.sendable);
        println!("serialized_value: {:?}", serialized_value);

        let batch = build_amqp_batch_from_messages(message_iter.clone(), None).unwrap();
        let serialized_bytes = serialized_bytes_of_sendable(batch.sendable);
        println!("serialized_bytes: {:?}", serialized_bytes);

        let batch = build_amqp_batch_from_messages(message_iter, None).unwrap();
        let (phantom_size, ssize) =
            phantom_size_and_serialized_size_of_sendable_envelope(batch.sendable);
        println!("serialized_size: {}", ssize);
        assert_eq!(phantom_size, ssize)
    }

    #[test]
    fn phantom_message_size_with_multiple_events() {
        let data = "abcdefghij";
        let event = EventData::from(data);
        let messages = vec![event.amqp_message.clone(); 2];

        let batch = build_amqp_batch_from_messages(messages.clone().into_iter(), None).unwrap();
        let serialized_bytes = serialized_bytes_of_sendable(batch.sendable);

        let batch = build_amqp_batch_from_messages(messages.into_iter(), None).unwrap();
        let (_, serialized_size) =
            phantom_size_and_serialized_size_of_sendable_envelope(batch.sendable);

        assert_eq!(serialized_size, serialized_bytes.len());

        let mut phantom_envelope = PhantomMessage {
            header: Phantom::try_from(&None).unwrap(),
            delivery_annotations: Phantom::try_from(&None).unwrap(),
            message_annotations: Phantom::try_from(&None).unwrap(),
            properties: Phantom::try_from(&None).unwrap(),
            application_properties: Phantom::try_from(&None).unwrap(),
            body: Phantom::new(0),
            footer: Phantom::try_from(&None).unwrap(),
        };
        let phantom_event_body = Phantom::try_from(&event.amqp_message).unwrap();
        phantom_envelope.body.push(phantom_event_body);
        let phantom_event_body = Phantom::try_from(&event.amqp_message).unwrap();
        phantom_envelope.body.push(phantom_event_body);
        let phantom_size = phantom_envelope.serialized_size();

        assert_eq!(phantom_size, serialized_size)
    }
}
