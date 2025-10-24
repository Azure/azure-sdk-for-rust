// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    error::AmqpErrorKind,
    simple_value::AmqpSimpleValue,
    value::{
        AmqpDescribed, AmqpDescriptor, AmqpList, AmqpOrderedMap, AmqpSymbol, AmqpTimestamp,
        AmqpValue,
    },
    AmqpError,
};
use azure_core::error::ErrorKind;
use serde_amqp::primitives::Timestamp;
use serde_bytes::ByteBuf;
use std::time::{Duration, UNIX_EPOCH};

use super::error::Fe2o3SerializationError;

impl From<&fe2o3_amqp_types::primitives::Symbol> for AmqpSymbol {
    fn from(s: &fe2o3_amqp_types::primitives::Symbol) -> AmqpSymbol {
        AmqpSymbol(s.to_string())
    }
}
impl From<fe2o3_amqp_types::primitives::Symbol> for AmqpSymbol {
    fn from(s: fe2o3_amqp_types::primitives::Symbol) -> AmqpSymbol {
        AmqpSymbol(s.0)
    }
}

impl From<AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn from(s: AmqpSymbol) -> fe2o3_amqp_types::primitives::Symbol {
        fe2o3_amqp_types::primitives::Symbol(s.0)
    }
}

impl From<&AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn from(s: &AmqpSymbol) -> fe2o3_amqp_types::primitives::Symbol {
        fe2o3_amqp_types::primitives::Symbol(s.0.clone())
    }
}

impl PartialEq<AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn eq(&self, other: &AmqpSymbol) -> bool {
        self.0 == other.0
    }
}

// Number of milliseconds between the Unix epoch (1/1/1970) and year 1 CE.
// This is the lowest value that can be represented by an AMQP timestamp.
const CE_ZERO_MILLISECONDS: i64 = -62_135_596_800_000;

impl From<&fe2o3_amqp_types::primitives::Timestamp> for AmqpTimestamp {
    fn from(timestamp: &fe2o3_amqp_types::primitives::Timestamp) -> Self {
        // The AMQP timestamp is the number of milliseconds since the Unix epoch.
        // AMQP brokers represent the lowest value as -62_135_596_800_000 (the
        // number of milliseconds between the Unix epoch (1/1/1970) and year 1 CE) as
        // a sentinel for a time which is not set.
        if (timestamp.milliseconds() as u64) == CE_ZERO_MILLISECONDS as u64 {
            return AmqpTimestamp(None);
        }
        AmqpTimestamp(
            UNIX_EPOCH.checked_add(Duration::from_millis(timestamp.milliseconds() as u64)),
        )
    }
}

impl From<fe2o3_amqp_types::primitives::Timestamp> for AmqpTimestamp {
    fn from(timestamp: fe2o3_amqp_types::primitives::Timestamp) -> Self {
        Self::from(&timestamp)
    }
}

impl From<AmqpTimestamp> for fe2o3_amqp_types::primitives::Timestamp {
    fn from(timestamp: AmqpTimestamp) -> Self {
        Self::from(&timestamp)
    }
}

impl From<&AmqpTimestamp> for fe2o3_amqp_types::primitives::Timestamp {
    fn from(timestamp: &AmqpTimestamp) -> Self {
        if let Some(t) = timestamp.0 {
            let t = t
                .duration_since(UNIX_EPOCH)
                .expect("Could not convert timestamp to time since unix epoch. This likely means that the timestamp is before the Unix epoch.")
                .as_millis();
            fe2o3_amqp_types::primitives::Timestamp::from_milliseconds(t as i64)
        } else {
            fe2o3_amqp_types::primitives::Timestamp::from_milliseconds(CE_ZERO_MILLISECONDS)
        }
    }
}

impl From<&AmqpSimpleValue> for fe2o3_amqp_types::primitives::SimpleValue {
    fn from(v: &AmqpSimpleValue) -> Self {
        match v {
            AmqpSimpleValue::Boolean(b) => fe2o3_amqp_types::primitives::SimpleValue::Bool(*b),
            AmqpSimpleValue::UByte(b) => fe2o3_amqp_types::primitives::SimpleValue::Ubyte(*b),
            AmqpSimpleValue::UShort(s) => fe2o3_amqp_types::primitives::SimpleValue::Ushort(*s),
            AmqpSimpleValue::UInt(i) => fe2o3_amqp_types::primitives::SimpleValue::Uint(*i),
            AmqpSimpleValue::ULong(l) => fe2o3_amqp_types::primitives::SimpleValue::Ulong(*l),
            AmqpSimpleValue::Byte(b) => fe2o3_amqp_types::primitives::SimpleValue::Byte(*b),
            AmqpSimpleValue::Short(s) => fe2o3_amqp_types::primitives::SimpleValue::Short(*s),
            AmqpSimpleValue::Int(i) => fe2o3_amqp_types::primitives::SimpleValue::Int(*i),
            AmqpSimpleValue::Long(l) => fe2o3_amqp_types::primitives::SimpleValue::Long(*l),
            AmqpSimpleValue::Float(f) => {
                fe2o3_amqp_types::primitives::SimpleValue::Float((*f).into())
            }
            AmqpSimpleValue::Double(d) => {
                fe2o3_amqp_types::primitives::SimpleValue::Double((*d).into())
            }
            AmqpSimpleValue::Char(c) => fe2o3_amqp_types::primitives::SimpleValue::Char(*c),
            AmqpSimpleValue::TimeStamp(t) => {
                fe2o3_amqp_types::primitives::SimpleValue::Timestamp(t.into())
            }
            AmqpSimpleValue::Uuid(u) => fe2o3_amqp_types::primitives::SimpleValue::Uuid(
                fe2o3_amqp_types::primitives::Uuid::from(*(*u).as_bytes()),
            ),
            AmqpSimpleValue::Binary(b) => {
                fe2o3_amqp_types::primitives::SimpleValue::Binary(ByteBuf::from(b.clone()))
            }
            AmqpSimpleValue::String(s) => {
                fe2o3_amqp_types::primitives::SimpleValue::String(s.clone())
            }
            AmqpSimpleValue::Symbol(s) => {
                fe2o3_amqp_types::primitives::SimpleValue::Symbol(s.into())
            }
            AmqpSimpleValue::Null => fe2o3_amqp_types::primitives::SimpleValue::Null,
            AmqpSimpleValue::Decimal128(d) => {
                fe2o3_amqp_types::primitives::SimpleValue::Decimal128(
                    serde_amqp::primitives::Dec128::from(*d),
                )
            }
            AmqpSimpleValue::Decimal64(d) => fe2o3_amqp_types::primitives::SimpleValue::Decimal64(
                serde_amqp::primitives::Dec64::from(*d),
            ),
            AmqpSimpleValue::Decimal32(d) => fe2o3_amqp_types::primitives::SimpleValue::Decimal32(
                serde_amqp::primitives::Dec32::from(*d),
            ),
            AmqpSimpleValue::Described(d) => fe2o3_amqp_types::primitives::SimpleValue::Described(
                Box::new(serde_amqp::described::Described {
                    descriptor: (&d.descriptor).into(),
                    value: (&d.value).into(),
                }),
            ),
        }
    }
}
impl From<AmqpSimpleValue> for fe2o3_amqp_types::primitives::SimpleValue {
    fn from(v: AmqpSimpleValue) -> Self {
        match v {
            AmqpSimpleValue::Boolean(b) => fe2o3_amqp_types::primitives::SimpleValue::Bool(b),
            AmqpSimpleValue::UByte(b) => fe2o3_amqp_types::primitives::SimpleValue::Ubyte(b),
            AmqpSimpleValue::UShort(s) => fe2o3_amqp_types::primitives::SimpleValue::Ushort(s),
            AmqpSimpleValue::UInt(i) => fe2o3_amqp_types::primitives::SimpleValue::Uint(i),
            AmqpSimpleValue::ULong(l) => fe2o3_amqp_types::primitives::SimpleValue::Ulong(l),
            AmqpSimpleValue::Byte(b) => fe2o3_amqp_types::primitives::SimpleValue::Byte(b),
            AmqpSimpleValue::Short(s) => fe2o3_amqp_types::primitives::SimpleValue::Short(s),
            AmqpSimpleValue::Int(i) => fe2o3_amqp_types::primitives::SimpleValue::Int(i),
            AmqpSimpleValue::Long(l) => fe2o3_amqp_types::primitives::SimpleValue::Long(l),
            AmqpSimpleValue::Float(f) => fe2o3_amqp_types::primitives::SimpleValue::Float(f.into()),
            AmqpSimpleValue::Double(d) => {
                fe2o3_amqp_types::primitives::SimpleValue::Double(d.into())
            }
            AmqpSimpleValue::Char(c) => fe2o3_amqp_types::primitives::SimpleValue::Char(c),
            AmqpSimpleValue::TimeStamp(t) => {
                fe2o3_amqp_types::primitives::SimpleValue::Timestamp(t.into())
            }
            AmqpSimpleValue::Uuid(u) => fe2o3_amqp_types::primitives::SimpleValue::Uuid(
                fe2o3_amqp_types::primitives::Uuid::from(*u.as_bytes()),
            ),
            AmqpSimpleValue::Binary(b) => {
                fe2o3_amqp_types::primitives::SimpleValue::Binary(ByteBuf::from(b))
            }
            AmqpSimpleValue::String(s) => fe2o3_amqp_types::primitives::SimpleValue::String(s),
            AmqpSimpleValue::Symbol(s) => {
                fe2o3_amqp_types::primitives::SimpleValue::Symbol(s.0.into())
            }
            AmqpSimpleValue::Null => fe2o3_amqp_types::primitives::SimpleValue::Null,
            AmqpSimpleValue::Decimal128(d) => {
                fe2o3_amqp_types::primitives::SimpleValue::Decimal128(
                    serde_amqp::primitives::Dec128::from(d),
                )
            }
            AmqpSimpleValue::Decimal64(d) => fe2o3_amqp_types::primitives::SimpleValue::Decimal64(
                serde_amqp::primitives::Dec64::from(d),
            ),
            AmqpSimpleValue::Decimal32(d) => fe2o3_amqp_types::primitives::SimpleValue::Decimal32(
                serde_amqp::primitives::Dec32::from(d),
            ),
            AmqpSimpleValue::Described(d) => fe2o3_amqp_types::primitives::SimpleValue::Described(
                Box::new(serde_amqp::described::Described {
                    descriptor: (&d.descriptor).into(),
                    value: (&d.value).into(),
                }),
            ),
        }
    }
}

impl From<&fe2o3_amqp_types::primitives::SimpleValue> for AmqpSimpleValue {
    fn from(v: &fe2o3_amqp_types::primitives::SimpleValue) -> Self {
        match v {
            fe2o3_amqp_types::primitives::SimpleValue::Null => AmqpSimpleValue::Null,
            fe2o3_amqp_types::primitives::SimpleValue::Bool(b) => AmqpSimpleValue::Boolean(*b),
            fe2o3_amqp_types::primitives::SimpleValue::Ubyte(b) => AmqpSimpleValue::UByte(*b),
            fe2o3_amqp_types::primitives::SimpleValue::Ushort(s) => AmqpSimpleValue::UShort(*s),
            fe2o3_amqp_types::primitives::SimpleValue::Uint(i) => AmqpSimpleValue::UInt(*i),
            fe2o3_amqp_types::primitives::SimpleValue::Ulong(l) => AmqpSimpleValue::ULong(*l),
            fe2o3_amqp_types::primitives::SimpleValue::Byte(b) => AmqpSimpleValue::Byte(*b),
            fe2o3_amqp_types::primitives::SimpleValue::Short(s) => AmqpSimpleValue::Short(*s),
            fe2o3_amqp_types::primitives::SimpleValue::Int(i) => AmqpSimpleValue::Int(*i),
            fe2o3_amqp_types::primitives::SimpleValue::Long(l) => AmqpSimpleValue::Long(*l),
            fe2o3_amqp_types::primitives::SimpleValue::Float(f) => {
                AmqpSimpleValue::Float((*f).into())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Double(d) => {
                AmqpSimpleValue::Double((*d).into())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Char(c) => AmqpSimpleValue::Char(*c),
            fe2o3_amqp_types::primitives::SimpleValue::Timestamp(t) => {
                AmqpSimpleValue::TimeStamp(AmqpTimestamp::from(t))
            }
            fe2o3_amqp_types::primitives::SimpleValue::Uuid(u) => {
                AmqpSimpleValue::Uuid(azure_core::Uuid::from_bytes(*u.as_inner()))
            }
            fe2o3_amqp_types::primitives::SimpleValue::Binary(b) => {
                AmqpSimpleValue::Binary(b.to_vec())
            }
            fe2o3_amqp_types::primitives::SimpleValue::String(s) => {
                AmqpSimpleValue::String(s.clone())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Symbol(s) => {
                AmqpSimpleValue::Symbol(s.into())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Decimal32(dec32) => {
                AmqpSimpleValue::Decimal32(dec32.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Decimal64(dec64) => {
                AmqpSimpleValue::Decimal64(dec64.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Decimal128(dec128) => {
                AmqpSimpleValue::Decimal128(dec128.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Described(desc) => {
                AmqpSimpleValue::Described(Box::new(AmqpDescribed::new(
                    AmqpDescriptor::from(&desc.descriptor),
                    desc.value.clone(),
                )))
            }
        }
    }
}

impl From<fe2o3_amqp_types::primitives::SimpleValue> for AmqpSimpleValue {
    fn from(v: fe2o3_amqp_types::primitives::SimpleValue) -> Self {
        match v {
            fe2o3_amqp_types::primitives::SimpleValue::Null => AmqpSimpleValue::Null,
            fe2o3_amqp_types::primitives::SimpleValue::Bool(b) => AmqpSimpleValue::Boolean(b),
            fe2o3_amqp_types::primitives::SimpleValue::Ubyte(b) => AmqpSimpleValue::UByte(b),
            fe2o3_amqp_types::primitives::SimpleValue::Ushort(s) => AmqpSimpleValue::UShort(s),
            fe2o3_amqp_types::primitives::SimpleValue::Uint(i) => AmqpSimpleValue::UInt(i),
            fe2o3_amqp_types::primitives::SimpleValue::Ulong(l) => AmqpSimpleValue::ULong(l),
            fe2o3_amqp_types::primitives::SimpleValue::Byte(b) => AmqpSimpleValue::Byte(b),
            fe2o3_amqp_types::primitives::SimpleValue::Short(s) => AmqpSimpleValue::Short(s),
            fe2o3_amqp_types::primitives::SimpleValue::Int(i) => AmqpSimpleValue::Int(i),
            fe2o3_amqp_types::primitives::SimpleValue::Long(l) => AmqpSimpleValue::Long(l),
            fe2o3_amqp_types::primitives::SimpleValue::Float(f) => AmqpSimpleValue::Float(f.into()),
            fe2o3_amqp_types::primitives::SimpleValue::Double(d) => {
                AmqpSimpleValue::Double(d.into())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Char(c) => AmqpSimpleValue::Char(c),
            fe2o3_amqp_types::primitives::SimpleValue::Timestamp(t) => {
                AmqpSimpleValue::TimeStamp(AmqpTimestamp::from(t))
            }
            fe2o3_amqp_types::primitives::SimpleValue::Uuid(u) => {
                AmqpSimpleValue::Uuid(azure_core::Uuid::from_bytes(*u.as_inner()))
            }
            fe2o3_amqp_types::primitives::SimpleValue::Binary(b) => {
                AmqpSimpleValue::Binary(b.to_vec())
            }
            fe2o3_amqp_types::primitives::SimpleValue::String(s) => {
                AmqpSimpleValue::String(s.clone())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Symbol(s) => {
                AmqpSimpleValue::Symbol(s.0.into())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Decimal32(dec32) => {
                AmqpSimpleValue::Decimal32(dec32.into_inner())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Decimal64(dec64) => {
                AmqpSimpleValue::Decimal64(dec64.into_inner())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Decimal128(dec128) => {
                AmqpSimpleValue::Decimal128(dec128.into_inner())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Described(desc) => {
                AmqpSimpleValue::Described(Box::new(AmqpDescribed::new(
                    AmqpDescriptor::from(&desc.descriptor),
                    desc.value,
                )))
            }
        }
    }
}

impl From<&fe2o3_amqp_types::primitives::Value> for AmqpList {
    fn from(value: &fe2o3_amqp_types::primitives::Value) -> Self {
        match value {
            fe2o3_amqp_types::primitives::Value::List(l) => {
                AmqpList(l.iter().map(Into::into).collect::<Vec<AmqpValue>>())
            }
            _ => panic!("Expected a list"),
        }
    }
}

impl From<&AmqpList> for fe2o3_amqp_types::primitives::Value {
    fn from(value: &AmqpList) -> Self {
        fe2o3_amqp_types::primitives::Value::List(
            value
                .0
                .iter()
                .map(Into::into)
                .collect::<Vec<fe2o3_amqp_types::primitives::Value>>(),
        )
    }
}

impl From<&AmqpValue> for fe2o3_amqp_types::primitives::Value {
    fn from(value: &AmqpValue) -> Self {
        match value {
            AmqpValue::Boolean(b) => fe2o3_amqp_types::primitives::Value::Bool(*b),
            AmqpValue::UByte(b) => fe2o3_amqp_types::primitives::Value::Ubyte(*b),
            AmqpValue::UShort(s) => fe2o3_amqp_types::primitives::Value::Ushort(*s),
            AmqpValue::UInt(i) => fe2o3_amqp_types::primitives::Value::Uint(*i),
            AmqpValue::ULong(l) => fe2o3_amqp_types::primitives::Value::Ulong(*l),
            AmqpValue::Byte(b) => fe2o3_amqp_types::primitives::Value::Byte(*b),
            AmqpValue::Short(s) => fe2o3_amqp_types::primitives::Value::Short(*s),
            AmqpValue::Int(i) => fe2o3_amqp_types::primitives::Value::Int(*i),
            AmqpValue::Long(l) => fe2o3_amqp_types::primitives::Value::Long(*l),
            AmqpValue::Float(f) => fe2o3_amqp_types::primitives::Value::Float((*f).into()),
            AmqpValue::Double(d) => fe2o3_amqp_types::primitives::Value::Double((*d).into()),
            AmqpValue::Char(c) => fe2o3_amqp_types::primitives::Value::Char(*c),
            AmqpValue::TimeStamp(t) => fe2o3_amqp_types::primitives::Value::Timestamp(t.into()),
            AmqpValue::Uuid(u) => fe2o3_amqp_types::primitives::Value::Uuid((*u).into()),
            AmqpValue::Binary(b) => {
                fe2o3_amqp_types::primitives::Value::Binary(ByteBuf::from(b.clone()))
            }
            AmqpValue::String(s) => fe2o3_amqp_types::primitives::Value::String(s.into()),
            AmqpValue::Symbol(s) => fe2o3_amqp_types::primitives::Value::Symbol(s.into()),
            AmqpValue::Null => fe2o3_amqp_types::primitives::Value::Null,
            AmqpValue::Decimal32(d) => fe2o3_amqp_types::primitives::Value::Decimal32((*d).into()),
            AmqpValue::Decimal64(d) => fe2o3_amqp_types::primitives::Value::Decimal64((*d).into()),
            AmqpValue::Decimal128(d) => {
                fe2o3_amqp_types::primitives::Value::Decimal128((*d).into())
            }
            AmqpValue::List(amqp_list) => fe2o3_amqp_types::primitives::Value::List(
                amqp_list.0.iter().map(Into::into).collect(),
            ),
            AmqpValue::Map(amqp_ordered_map) => fe2o3_amqp_types::primitives::Value::Map(
                amqp_ordered_map
                    .iter()
                    .map(|(k, v)| (k.into(), v.into()))
                    .collect(),
            ),
            AmqpValue::Described(amqp_described) => fe2o3_amqp_types::primitives::Value::Described(
                Box::new(serde_amqp::described::Described {
                    descriptor: (&amqp_described.descriptor).into(),
                    value: (&amqp_described.value).into(),
                }),
            ),
            #[cfg(feature = "ffi")]
            AmqpValue::Composite(amqp_composite) => fe2o3_amqp_types::primitives::Value::Described(
                Box::new(serde_amqp::described::Described {
                    descriptor: amqp_composite.descriptor().into(),
                    value: amqp_composite.value().into(),
                }),
            ),
            AmqpValue::Array(amqp_values) => fe2o3_amqp_types::primitives::Value::Array(
                amqp_values.iter().map(Into::into).collect(),
            ),
        }
    }
}

impl From<AmqpValue> for fe2o3_amqp_types::primitives::Value {
    fn from(value: AmqpValue) -> Self {
        match value {
            AmqpValue::Null => fe2o3_amqp_types::primitives::Value::Null,
            AmqpValue::Boolean(b) => fe2o3_amqp_types::primitives::Value::Bool(b),
            AmqpValue::UByte(b) => fe2o3_amqp_types::primitives::Value::Ubyte(b),
            AmqpValue::UShort(s) => fe2o3_amqp_types::primitives::Value::Ushort(s),
            AmqpValue::UInt(i) => fe2o3_amqp_types::primitives::Value::Uint(i),
            AmqpValue::ULong(l) => fe2o3_amqp_types::primitives::Value::Ulong(l),
            AmqpValue::Byte(b) => fe2o3_amqp_types::primitives::Value::Byte(b),
            AmqpValue::Short(s) => fe2o3_amqp_types::primitives::Value::Short(s),
            AmqpValue::Int(i) => fe2o3_amqp_types::primitives::Value::Int(i),
            AmqpValue::Long(l) => fe2o3_amqp_types::primitives::Value::Long(l),
            AmqpValue::Float(f) => fe2o3_amqp_types::primitives::Value::Float(f.into()),
            AmqpValue::Double(d) => fe2o3_amqp_types::primitives::Value::Double(d.into()),
            AmqpValue::Char(c) => fe2o3_amqp_types::primitives::Value::Char(c),
            AmqpValue::TimeStamp(t) => fe2o3_amqp_types::primitives::Value::Timestamp(t.into()),
            AmqpValue::Uuid(u) => fe2o3_amqp_types::primitives::Value::Uuid(u.into()),
            AmqpValue::Binary(b) => fe2o3_amqp_types::primitives::Value::Binary(ByteBuf::from(b)),
            AmqpValue::String(s) => fe2o3_amqp_types::primitives::Value::String(s),
            AmqpValue::Symbol(s) => fe2o3_amqp_types::primitives::Value::Symbol(s.0.into()),
            AmqpValue::Decimal32(d) => fe2o3_amqp_types::primitives::Value::Decimal32(d.into()),
            AmqpValue::Decimal64(d) => fe2o3_amqp_types::primitives::Value::Decimal64(d.into()),
            AmqpValue::Decimal128(d) => fe2o3_amqp_types::primitives::Value::Decimal128(d.into()),
            AmqpValue::List(l) => {
                fe2o3_amqp_types::primitives::Value::List(l.0.into_iter().map(Into::into).collect())
            }
            AmqpValue::Map(m) => fe2o3_amqp_types::primitives::Value::Map(
                m.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
            ),
            AmqpValue::Array(a) => {
                fe2o3_amqp_types::primitives::Value::Array(a.into_iter().map(Into::into).collect())
            }

            // An AMQP Composite type is essentially a Described type with a specific descriptor which
            // indicates which AMQP performative it is.
            //
            // Iron Oxide does not directly support Composite types (they're handled via macros), so when a C++
            // component attempts to convert an AMQP Composite type to Iron Oxide, we convert it to a Described type
            #[cfg(feature = "ffi")]
            AmqpValue::Composite(d) => fe2o3_amqp_types::primitives::Value::Described(Box::new(
                serde_amqp::described::Described {
                    descriptor: d.descriptor().into(),
                    value: d.value().into(),
                },
            )),
            AmqpValue::Described(d) => fe2o3_amqp_types::primitives::Value::Described(Box::new(
                serde_amqp::described::Described {
                    descriptor: (&d.descriptor).into(),
                    value: (&d.value).into(),
                },
            )),
        }
    }
}

impl From<&fe2o3_amqp_types::primitives::Value> for AmqpValue {
    fn from(value: &fe2o3_amqp_types::primitives::Value) -> Self {
        match value {
            fe2o3_amqp_types::primitives::Value::Null => AmqpValue::Null,
            fe2o3_amqp_types::primitives::Value::Bool(b) => AmqpValue::Boolean(*b),
            fe2o3_amqp_types::primitives::Value::Ubyte(b) => AmqpValue::UByte(*b),
            fe2o3_amqp_types::primitives::Value::Ushort(s) => AmqpValue::UShort(*s),
            fe2o3_amqp_types::primitives::Value::Uint(i) => AmqpValue::UInt(*i),
            fe2o3_amqp_types::primitives::Value::Ulong(l) => AmqpValue::ULong(*l),
            fe2o3_amqp_types::primitives::Value::Byte(b) => AmqpValue::Byte(*b),
            fe2o3_amqp_types::primitives::Value::Short(s) => AmqpValue::Short(*s),
            fe2o3_amqp_types::primitives::Value::Int(i) => AmqpValue::Int(*i),
            fe2o3_amqp_types::primitives::Value::Long(l) => AmqpValue::Long(*l),
            fe2o3_amqp_types::primitives::Value::Float(f) => AmqpValue::Float((*f).into()),
            fe2o3_amqp_types::primitives::Value::Double(d) => AmqpValue::Double((*d).into()),
            fe2o3_amqp_types::primitives::Value::Char(c) => AmqpValue::Char(*c),
            fe2o3_amqp_types::primitives::Value::Timestamp(t) => AmqpValue::TimeStamp(t.into()),
            fe2o3_amqp_types::primitives::Value::Uuid(u) => {
                AmqpValue::Uuid(azure_core::Uuid::from_bytes(*u.as_inner()))
            }
            fe2o3_amqp_types::primitives::Value::Binary(b) => AmqpValue::Binary(b.to_vec()),
            fe2o3_amqp_types::primitives::Value::String(s) => AmqpValue::String(s.clone()),
            fe2o3_amqp_types::primitives::Value::Symbol(s) => AmqpValue::Symbol(s.into()),
            fe2o3_amqp_types::primitives::Value::Decimal128(d) => {
                AmqpValue::Decimal128(d.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::Value::Decimal32(d) => {
                AmqpValue::Decimal32(d.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::Value::Decimal64(d) => {
                AmqpValue::Decimal64(d.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::Value::List(l) => {
                let l = l.iter().map(Into::into).collect::<Vec<AmqpValue>>();
                AmqpValue::List(AmqpList(l))
            }
            fe2o3_amqp_types::primitives::Value::Map(m) => {
                AmqpValue::Map(m.iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
            fe2o3_amqp_types::primitives::Value::Array(a) => {
                AmqpValue::Array(a.iter().map(Into::into).collect())
            }
            fe2o3_amqp_types::primitives::Value::Described(d) => {
                let descriptor = match &d.descriptor {
                    serde_amqp::descriptor::Descriptor::Code(code) => AmqpDescriptor::Code(*code),
                    serde_amqp::descriptor::Descriptor::Name(symbol) => {
                        AmqpDescriptor::Name(symbol.into())
                    }
                };
                AmqpValue::Described(Box::new(AmqpDescribed::new(descriptor, &d.value)))
            }
        }
    }
}

impl From<fe2o3_amqp_types::primitives::Value> for AmqpValue {
    fn from(value: fe2o3_amqp_types::primitives::Value) -> Self {
        match value {
            fe2o3_amqp_types::primitives::Value::Null => AmqpValue::Null,
            fe2o3_amqp_types::primitives::Value::Bool(b) => AmqpValue::Boolean(b),
            fe2o3_amqp_types::primitives::Value::Ubyte(b) => AmqpValue::UByte(b),
            fe2o3_amqp_types::primitives::Value::Ushort(s) => AmqpValue::UShort(s),
            fe2o3_amqp_types::primitives::Value::Uint(i) => AmqpValue::UInt(i),
            fe2o3_amqp_types::primitives::Value::Ulong(l) => AmqpValue::ULong(l),
            fe2o3_amqp_types::primitives::Value::Byte(b) => AmqpValue::Byte(b),
            fe2o3_amqp_types::primitives::Value::Short(s) => AmqpValue::Short(s),
            fe2o3_amqp_types::primitives::Value::Int(i) => AmqpValue::Int(i),
            fe2o3_amqp_types::primitives::Value::Long(l) => AmqpValue::Long(l),
            fe2o3_amqp_types::primitives::Value::Float(f) => AmqpValue::Float(f.into()),
            fe2o3_amqp_types::primitives::Value::Double(d) => AmqpValue::Double(d.into()),
            fe2o3_amqp_types::primitives::Value::Char(c) => AmqpValue::Char(c),
            fe2o3_amqp_types::primitives::Value::Timestamp(t) => AmqpValue::TimeStamp(t.into()),
            fe2o3_amqp_types::primitives::Value::Uuid(u) => {
                AmqpValue::Uuid(azure_core::Uuid::from_bytes(*u.as_inner()))
            }
            fe2o3_amqp_types::primitives::Value::Binary(b) => AmqpValue::Binary(b.to_vec()),
            fe2o3_amqp_types::primitives::Value::String(s) => AmqpValue::String(s.clone()),
            fe2o3_amqp_types::primitives::Value::Symbol(s) => AmqpValue::Symbol(s.0.into()),
            fe2o3_amqp_types::primitives::Value::Decimal128(d) => {
                AmqpValue::Decimal128(d.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::Value::Decimal32(d) => {
                AmqpValue::Decimal32(d.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::Value::Decimal64(d) => {
                AmqpValue::Decimal64(d.clone().into_inner())
            }
            fe2o3_amqp_types::primitives::Value::List(l) => {
                let l = l.iter().map(Into::into).collect::<Vec<AmqpValue>>();
                AmqpValue::List(AmqpList(l))
            }
            fe2o3_amqp_types::primitives::Value::Map(m) => {
                AmqpValue::Map(m.iter().map(|(k, v)| (k.into(), v.into())).collect())
            }
            fe2o3_amqp_types::primitives::Value::Array(a) => {
                AmqpValue::Array(a.iter().map(Into::into).collect())
            }
            fe2o3_amqp_types::primitives::Value::Described(d) => {
                let descriptor = match &d.descriptor {
                    serde_amqp::descriptor::Descriptor::Code(code) => AmqpDescriptor::Code(*code),
                    serde_amqp::descriptor::Descriptor::Name(symbol) => {
                        AmqpDescriptor::Name(symbol.into())
                    }
                };
                AmqpValue::Described(Box::new(AmqpDescribed::new(descriptor, &d.value)))
            }
        }
    }
}

impl From<&AmqpDescriptor> for serde_amqp::descriptor::Descriptor {
    fn from(descriptor: &AmqpDescriptor) -> Self {
        match descriptor {
            AmqpDescriptor::Code(code) => serde_amqp::descriptor::Descriptor::Code(*code),
            AmqpDescriptor::Name(symbol) => serde_amqp::descriptor::Descriptor::Name(symbol.into()),
        }
    }
}

impl From<&serde_amqp::descriptor::Descriptor> for AmqpDescriptor {
    fn from(descriptor: &serde_amqp::descriptor::Descriptor) -> Self {
        match descriptor {
            serde_amqp::descriptor::Descriptor::Code(code) => AmqpDescriptor::Code(*code),
            serde_amqp::descriptor::Descriptor::Name(symbol) => AmqpDescriptor::Name(symbol.into()),
        }
    }
}

impl PartialEq<AmqpDescribed>
    for serde_amqp::described::Described<fe2o3_amqp_types::primitives::Value>
{
    fn eq(&self, other: &AmqpDescribed) -> bool {
        self.descriptor == other.descriptor && self.value == other.value
    }
}

impl PartialEq<serde_amqp::described::Described<fe2o3_amqp_types::primitives::Value>>
    for AmqpDescribed
{
    fn eq(
        &self,
        other: &serde_amqp::described::Described<fe2o3_amqp_types::primitives::Value>,
    ) -> bool {
        other == self
    }
}

impl PartialEq<AmqpDescriptor> for serde_amqp::descriptor::Descriptor {
    fn eq(&self, other: &AmqpDescriptor) -> bool {
        match self {
            serde_amqp::descriptor::Descriptor::Code(code) => match other {
                AmqpDescriptor::Code(c) => code == c,
                _ => false,
            },
            serde_amqp::descriptor::Descriptor::Name(symbol) => match other {
                AmqpDescriptor::Name(s) => symbol == s,
                _ => false,
            },
        }
    }
}

impl PartialEq<serde_amqp::descriptor::Descriptor> for AmqpDescriptor {
    fn eq(&self, other: &serde_amqp::descriptor::Descriptor) -> bool {
        other == self
    }
}

impl PartialEq<AmqpValue> for fe2o3_amqp_types::primitives::Value {
    fn eq(&self, other: &AmqpValue) -> bool {
        match other {
            AmqpValue::Null => self == &fe2o3_amqp_types::primitives::Value::Null,
            AmqpValue::Boolean(b) => self == &fe2o3_amqp_types::primitives::Value::Bool(*b),
            AmqpValue::UByte(b) => self == &fe2o3_amqp_types::primitives::Value::Ubyte(*b),
            AmqpValue::UShort(s) => self == &fe2o3_amqp_types::primitives::Value::Ushort(*s),
            AmqpValue::UInt(i) => self == &fe2o3_amqp_types::primitives::Value::Uint(*i),
            AmqpValue::ULong(l) => self == &fe2o3_amqp_types::primitives::Value::Ulong(*l),
            AmqpValue::Byte(b) => self == &fe2o3_amqp_types::primitives::Value::Byte(*b),
            AmqpValue::Short(s) => self == &fe2o3_amqp_types::primitives::Value::Short(*s),
            AmqpValue::Int(i) => self == &fe2o3_amqp_types::primitives::Value::Int(*i),
            AmqpValue::Long(l) => self == &fe2o3_amqp_types::primitives::Value::Long(*l),
            AmqpValue::Float(f) => self == &fe2o3_amqp_types::primitives::Value::Float((*f).into()),
            AmqpValue::Double(d) => {
                self == &fe2o3_amqp_types::primitives::Value::Double((*d).into())
            }
            AmqpValue::Char(c) => self == &fe2o3_amqp_types::primitives::Value::Char(*c),
            AmqpValue::TimeStamp(t) => {
                if let Some(t) = t.0 {
                    let t: u64 = t
                        .duration_since(UNIX_EPOCH)
                        .expect("Could not convert timestamp into unix epoch")
                        .as_millis() as u64;
                    self == &fe2o3_amqp_types::primitives::Value::Timestamp(
                        Timestamp::from_milliseconds(t as i64),
                    )
                } else {
                    self == &fe2o3_amqp_types::primitives::Value::Timestamp(
                        Timestamp::from_milliseconds(CE_ZERO_MILLISECONDS),
                    )
                }
            }
            AmqpValue::Uuid(u) => self == &fe2o3_amqp_types::primitives::Value::Uuid((*u).into()),
            AmqpValue::Binary(b) => {
                self == &fe2o3_amqp_types::primitives::Value::Binary(ByteBuf::from(b.as_slice()))
            }
            AmqpValue::String(s) => self == &fe2o3_amqp_types::primitives::Value::String(s.clone()),
            AmqpValue::Symbol(s) => self == &fe2o3_amqp_types::primitives::Value::Symbol(s.into()),
            AmqpValue::List(l) => {
                self == &fe2o3_amqp_types::primitives::Value::List(
                    l.0.iter().map(Into::into).collect(),
                )
            }
            AmqpValue::Map(m) => {
                self == &fe2o3_amqp_types::primitives::Value::Map(
                    m.iter().map(|(k, v)| (k.into(), v.into())).collect(),
                )
            }
            AmqpValue::Array(a) => match self {
                fe2o3_amqp_types::primitives::Value::Array(b) => {
                    a.iter().zip(b.iter()).all(|(a, b)| a == b)
                }
                _ => false,
            },
            AmqpValue::Described(d) => match self {
                fe2o3_amqp_types::primitives::Value::Described(a) => **d == **a,
                _ => false,
            },
            #[cfg(feature = "ffi")]
            AmqpValue::Composite(_) => false,

            AmqpValue::Decimal128(d) => match self {
                fe2o3_amqp_types::primitives::Value::Decimal128(d2) => *d2 == (*d).into(),
                _ => false,
            },
            AmqpValue::Decimal64(d) => match self {
                fe2o3_amqp_types::primitives::Value::Decimal64(d2) => *d2 == (*d).into(),
                _ => false,
            },
            AmqpValue::Decimal32(d) => match self {
                fe2o3_amqp_types::primitives::Value::Decimal32(d2) => *d2 == (*d).into(),
                _ => false,
            },
        }
    }
}

impl PartialEq<fe2o3_amqp_types::primitives::Value> for AmqpValue {
    fn eq(&self, other: &fe2o3_amqp_types::primitives::Value) -> bool {
        other == self
    }
}

impl From<&fe2o3_amqp_types::definitions::Fields> for AmqpOrderedMap<AmqpSymbol, AmqpValue> {
    fn from(fields: &fe2o3_amqp_types::definitions::Fields) -> Self {
        fields.iter().map(|(k, v)| (k.into(), v.into())).collect()
    }
}

impl
    From<
        &fe2o3_amqp_types::primitives::OrderedMap<
            std::string::String,
            fe2o3_amqp_types::primitives::Value,
        >,
    > for AmqpOrderedMap<std::string::String, AmqpValue>
{
    fn from(
        value: &fe2o3_amqp_types::primitives::OrderedMap<
            std::string::String,
            fe2o3_amqp_types::primitives::Value,
        >,
    ) -> Self {
        // Convert the OrderedMap to AmqpOrderedMap
        value
            .iter()
            .map(|(key, value)| (key.clone(), value.into()))
            .collect()
    }
}

impl From<AmqpOrderedMap<AmqpValue, AmqpValue>>
    for fe2o3_amqp_types::primitives::OrderedMap<
        fe2o3_amqp_types::primitives::Value,
        fe2o3_amqp_types::primitives::Value,
    >
{
    fn from(value: AmqpOrderedMap<AmqpValue, AmqpValue>) -> Self {
        // Convert the AmqpOrderedMap to OrderedMap
        value
            .into_iter()
            .map(|(key, value)| (key.into(), value.into()))
            .collect()
    }
}

impl From<AmqpOrderedMap<AmqpSymbol, AmqpValue>>
    for fe2o3_amqp_types::primitives::OrderedMap<
        fe2o3_amqp_types::primitives::Symbol,
        fe2o3_amqp_types::primitives::Value,
    >
{
    fn from(value: AmqpOrderedMap<AmqpSymbol, AmqpValue>) -> Self {
        // Convert the AmqpOrderedMap to OrderedMap

        value
            .into_iter()
            .map(|(key, value)| {
                (
                    fe2o3_amqp_types::primitives::Symbol(key.into()),
                    value.into(),
                )
            })
            .collect()
    }
}

impl From<crate::SenderSettleMode> for fe2o3_amqp_types::definitions::SenderSettleMode {
    fn from(mode: crate::SenderSettleMode) -> fe2o3_amqp_types::definitions::SenderSettleMode {
        match mode {
            crate::SenderSettleMode::Mixed => {
                fe2o3_amqp_types::definitions::SenderSettleMode::Mixed
            }
            crate::SenderSettleMode::Settled => {
                fe2o3_amqp_types::definitions::SenderSettleMode::Settled
            }
            crate::SenderSettleMode::Unsettled => {
                fe2o3_amqp_types::definitions::SenderSettleMode::Unsettled
            }
        }
    }
}

impl From<&fe2o3_amqp_types::definitions::SenderSettleMode> for crate::SenderSettleMode {
    fn from(mode: &fe2o3_amqp_types::definitions::SenderSettleMode) -> crate::SenderSettleMode {
        match mode {
            fe2o3_amqp_types::definitions::SenderSettleMode::Mixed => {
                crate::SenderSettleMode::Mixed
            }
            fe2o3_amqp_types::definitions::SenderSettleMode::Settled => {
                crate::SenderSettleMode::Settled
            }
            fe2o3_amqp_types::definitions::SenderSettleMode::Unsettled => {
                crate::SenderSettleMode::Unsettled
            }
        }
    }
}

impl From<crate::ReceiverSettleMode> for fe2o3_amqp_types::definitions::ReceiverSettleMode {
    fn from(mode: crate::ReceiverSettleMode) -> fe2o3_amqp_types::definitions::ReceiverSettleMode {
        match mode {
            crate::ReceiverSettleMode::First => {
                fe2o3_amqp_types::definitions::ReceiverSettleMode::First
            }
            crate::ReceiverSettleMode::Second => {
                fe2o3_amqp_types::definitions::ReceiverSettleMode::Second
            }
        }
    }
}

impl From<&fe2o3_amqp_types::definitions::ReceiverSettleMode> for crate::ReceiverSettleMode {
    fn from(mode: &fe2o3_amqp_types::definitions::ReceiverSettleMode) -> crate::ReceiverSettleMode {
        match mode {
            fe2o3_amqp_types::definitions::ReceiverSettleMode::First => {
                crate::ReceiverSettleMode::First
            }
            fe2o3_amqp_types::definitions::ReceiverSettleMode::Second => {
                crate::ReceiverSettleMode::Second
            }
        }
    }
}

impl From<Fe2o3SerializationError> for AmqpError {
    fn from(err: Fe2o3SerializationError) -> Self {
        match err.0 {
            serde_amqp::Error::Message(m) => {
                azure_core::Error::with_message(ErrorKind::DataConversion, m).into()
            }
            serde_amqp::Error::Io(error) => azure_core::Error::new(ErrorKind::Io, error).into(),
            serde_amqp::Error::InvalidFormatCode
            | serde_amqp::Error::InvalidUtf8Encoding
            | serde_amqp::Error::SequenceLengthMismatch
            | serde_amqp::Error::InvalidLength
            | serde_amqp::Error::InvalidValue => {
                AmqpError::from(AmqpErrorKind::TransportImplementationError(Box::new(err.0)))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::Uuid;

    #[test]
    fn test_from_fe2o3_amqp_types_primitives_symbol() {
        let symbol = fe2o3_amqp_types::primitives::Symbol::from("test");
        let amqp_symbol = AmqpSymbol::from(symbol.0.clone());
        let symbol2: fe2o3_amqp_types::primitives::Symbol = amqp_symbol.into();
        assert_eq!(symbol, symbol2);
    }

    #[test]
    fn test_from_amqp_value_to_fe2o3_amqp_types_primitives_value() {
        let value = AmqpValue::String("test".to_string());
        let value2: fe2o3_amqp_types::primitives::Value = value.into();
        assert_eq!(
            value2,
            fe2o3_amqp_types::primitives::Value::String("test".to_string())
        );
    }

    #[test]
    fn test_from_fe2o3_amqp_types_primitives_value_to_amqp_value() {
        let value = fe2o3_amqp_types::primitives::Value::String("test".to_string());
        let value2: AmqpValue = (&value).into();
        assert_eq!(value2, AmqpValue::String("test".to_string()));
    }

    #[test]
    fn test_from_fe2o3_amqp_types_to_amqp_value() {
        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Null;
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Bool(true);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Ubyte(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Ushort(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Uint(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Ulong(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Byte(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Short(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Int(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Long(1);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Float(1.0f32.into());
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Double(10.0f64.into());
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Char('a');
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 =
                fe2o3_amqp_types::primitives::Value::Timestamp(Timestamp::from_milliseconds(1));
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let uuid = Uuid::new_v4();
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Uuid(uuid.into());
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Binary(ByteBuf::from(vec![1]));
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::String("test".to_string());
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Symbol(
                fe2o3_amqp_types::primitives::Symbol("test".to_string()),
            );
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::List(vec![
                fe2o3_amqp_types::primitives::Value::String("test".to_string()),
            ]);
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Map(
                fe2o3_amqp_types::primitives::OrderedMap::new(),
            );
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Array(
                vec![fe2o3_amqp_types::primitives::Value::String(
                    "test".to_string(),
                )]
                .into(),
            );
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Described(Box::new(
                serde_amqp::described::Described {
                    descriptor: serde_amqp::descriptor::Descriptor::Code(37),
                    value: fe2o3_amqp_types::primitives::Value::String("test".to_string()),
                },
            ));
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Decimal128(
                fe2o3_amqp_types::primitives::Dec128::from([
                    1u8, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 13, 14, 15, 16, 17,
                ]),
            );
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.into();

            assert_eq!(fe2o3, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Decimal32(
                fe2o3_amqp_types::primitives::Dec32::from([1u8, 2, 3, 4]),
            );
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.into();

            assert_eq!(fe2o3, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Decimal64(
                fe2o3_amqp_types::primitives::Dec64::from([1u8, 2, 3, 4, 5, 6, 7, 8]),
            );
            let amqp: AmqpValue = (&fe2o3).into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.into();
            assert_eq!(fe2o3, fe2o3_2);
        }
    }
}
