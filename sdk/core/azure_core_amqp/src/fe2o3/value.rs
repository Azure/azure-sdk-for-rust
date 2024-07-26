// Copyright (c) Microsoft Corp. All Rights Reserved.
// cspell: words amqp

use serde_bytes::ByteBuf;
use std::time::UNIX_EPOCH;

use serde_amqp::primitives::Timestamp;

use crate::value::{
    AmqpDescribed, AmqpDescriptor, AmqpList, AmqpOrderedMap, AmqpSymbol, AmqpTimestamp, AmqpValue,
};

impl From<fe2o3_amqp_types::primitives::Symbol> for AmqpSymbol {
    fn from(s: fe2o3_amqp_types::primitives::Symbol) -> AmqpSymbol {
        AmqpSymbol(s.to_string())
    }
}

impl From<AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn from(s: AmqpSymbol) -> fe2o3_amqp_types::primitives::Symbol {
        fe2o3_amqp_types::primitives::Symbol(s.0)
    }
}

impl PartialEq<AmqpSymbol> for fe2o3_amqp_types::primitives::Symbol {
    fn eq(&self, other: &AmqpSymbol) -> bool {
        self.0 == other.0
    }
}

impl From<AmqpValue> for fe2o3_amqp_types::primitives::Symbol {
    fn from(v: AmqpValue) -> Self {
        match v {
            AmqpValue::Symbol(s) => s.into(),
            _ => panic!("Expected a symbol"),
        }
    }
}

impl From<fe2o3_amqp_types::primitives::Timestamp> for AmqpTimestamp {
    fn from(timestamp: fe2o3_amqp_types::primitives::Timestamp) -> Self {
        AmqpTimestamp(
            std::time::UNIX_EPOCH
                + std::time::Duration::from_millis(timestamp.milliseconds() as u64),
        )
    }
}

impl From<AmqpTimestamp> for fe2o3_amqp_types::primitives::Timestamp {
    fn from(timestamp: AmqpTimestamp) -> Self {
        let t = timestamp.0.duration_since(UNIX_EPOCH).unwrap().as_millis();
        fe2o3_amqp_types::primitives::Timestamp::from_milliseconds(t as i64)
    }
}

impl From<AmqpValue> for fe2o3_amqp_types::primitives::SimpleValue {
    fn from(v: AmqpValue) -> Self {
        match v {
            AmqpValue::Boolean(b) => fe2o3_amqp_types::primitives::SimpleValue::Bool(b),
            AmqpValue::UByte(b) => fe2o3_amqp_types::primitives::SimpleValue::Ubyte(b),
            AmqpValue::UShort(s) => fe2o3_amqp_types::primitives::SimpleValue::Ushort(s),
            AmqpValue::UInt(i) => fe2o3_amqp_types::primitives::SimpleValue::Uint(i),
            AmqpValue::ULong(l) => fe2o3_amqp_types::primitives::SimpleValue::Ulong(l),
            AmqpValue::Byte(b) => fe2o3_amqp_types::primitives::SimpleValue::Byte(b),
            AmqpValue::Short(s) => fe2o3_amqp_types::primitives::SimpleValue::Short(s),
            AmqpValue::Int(i) => fe2o3_amqp_types::primitives::SimpleValue::Int(i),
            AmqpValue::Long(l) => fe2o3_amqp_types::primitives::SimpleValue::Long(l),
            AmqpValue::Float(f) => fe2o3_amqp_types::primitives::SimpleValue::Float(f.into()),
            AmqpValue::Double(d) => fe2o3_amqp_types::primitives::SimpleValue::Double(d.into()),
            AmqpValue::Char(c) => fe2o3_amqp_types::primitives::SimpleValue::Char(c),
            AmqpValue::TimeStamp(t) => {
                fe2o3_amqp_types::primitives::SimpleValue::Timestamp(t.into())
            }
            AmqpValue::Uuid(u) => fe2o3_amqp_types::primitives::SimpleValue::Uuid(u.into()),
            AmqpValue::Binary(b) => {
                fe2o3_amqp_types::primitives::SimpleValue::Binary(ByteBuf::from(b))
            }
            AmqpValue::String(s) => fe2o3_amqp_types::primitives::SimpleValue::String(s),
            AmqpValue::Symbol(s) => fe2o3_amqp_types::primitives::SimpleValue::Symbol(s.into()),

            _ => panic!("Expected a simple value."),
        }
    }
}

impl From<fe2o3_amqp_types::primitives::SimpleValue> for AmqpValue {
    fn from(v: fe2o3_amqp_types::primitives::SimpleValue) -> Self {
        match v {
            fe2o3_amqp_types::primitives::SimpleValue::Null => AmqpValue::Null,
            fe2o3_amqp_types::primitives::SimpleValue::Bool(b) => AmqpValue::Boolean(b),
            fe2o3_amqp_types::primitives::SimpleValue::Ubyte(b) => AmqpValue::UByte(b),
            fe2o3_amqp_types::primitives::SimpleValue::Ushort(s) => AmqpValue::UShort(s),
            fe2o3_amqp_types::primitives::SimpleValue::Uint(i) => AmqpValue::UInt(i),
            fe2o3_amqp_types::primitives::SimpleValue::Ulong(l) => AmqpValue::ULong(l),
            fe2o3_amqp_types::primitives::SimpleValue::Byte(b) => AmqpValue::Byte(b),
            fe2o3_amqp_types::primitives::SimpleValue::Short(s) => AmqpValue::Short(s),
            fe2o3_amqp_types::primitives::SimpleValue::Int(i) => AmqpValue::Int(i),
            fe2o3_amqp_types::primitives::SimpleValue::Long(l) => AmqpValue::Long(l),
            fe2o3_amqp_types::primitives::SimpleValue::Float(f) => AmqpValue::Float(f.into()),
            fe2o3_amqp_types::primitives::SimpleValue::Double(d) => AmqpValue::Double(d.into()),
            fe2o3_amqp_types::primitives::SimpleValue::Char(c) => AmqpValue::Char(c),
            fe2o3_amqp_types::primitives::SimpleValue::Timestamp(t) => {
                AmqpValue::TimeStamp(t.into())
            }
            fe2o3_amqp_types::primitives::SimpleValue::Uuid(u) => AmqpValue::Uuid(u.into()),
            fe2o3_amqp_types::primitives::SimpleValue::Binary(b) => {
                AmqpValue::Binary(ByteBuf::into_vec(b))
            }
            fe2o3_amqp_types::primitives::SimpleValue::String(s) => AmqpValue::String(s),
            fe2o3_amqp_types::primitives::SimpleValue::Symbol(s) => AmqpValue::Symbol(s.into()),
            _ => panic!("Expected a simple value."),
        }
    }
}

impl From<AmqpDescriptor> for serde_amqp::descriptor::Descriptor {
    fn from(descriptor: AmqpDescriptor) -> Self {
        match descriptor {
            AmqpDescriptor::Code(code) => serde_amqp::descriptor::Descriptor::Code(code),
            AmqpDescriptor::Name(symbol) => serde_amqp::descriptor::Descriptor::Name(symbol.into()),
        }
    }
}

impl From<serde_amqp::descriptor::Descriptor> for AmqpDescriptor {
    fn from(descriptor: serde_amqp::descriptor::Descriptor) -> Self {
        match descriptor {
            serde_amqp::descriptor::Descriptor::Code(code) => AmqpDescriptor::Code(code),
            serde_amqp::descriptor::Descriptor::Name(symbol) => AmqpDescriptor::Name(symbol.into()),
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
            AmqpValue::List(l) => fe2o3_amqp_types::primitives::Value::List(
                l.0.into_iter().map(|v| v.into()).collect(),
            ),
            AmqpValue::Map(m) => fe2o3_amqp_types::primitives::Value::Map(
                m.into_iter().map(|(k, v)| (k.into(), v.into())).collect(),
            ),
            AmqpValue::Array(a) => fe2o3_amqp_types::primitives::Value::Array(
                a.into_iter().map(|v| v.into()).collect(),
            ),
            AmqpValue::Described(d) => fe2o3_amqp_types::primitives::Value::Described(Box::new(
                serde_amqp::described::Described {
                    descriptor: d.descriptor.clone().into(),
                    value: d.value.clone().into(),
                },
            )),
            AmqpValue::Unknown => todo!(),
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
            fe2o3_amqp_types::primitives::Value::Uuid(u) => AmqpValue::Uuid(u.into()),
            fe2o3_amqp_types::primitives::Value::Binary(b) => {
                AmqpValue::Binary(ByteBuf::into_vec(b))
            }
            fe2o3_amqp_types::primitives::Value::String(s) => AmqpValue::String(s),
            fe2o3_amqp_types::primitives::Value::Symbol(s) => AmqpValue::Symbol(s.into()),
            fe2o3_amqp_types::primitives::Value::List(l) => {
                let l = l.into_iter().map(|v| v.into()).collect::<Vec<AmqpValue>>();
                AmqpValue::List(AmqpList(l))
            }
            fe2o3_amqp_types::primitives::Value::Map(m) => {
                let mut map = AmqpOrderedMap::new();
                for (k, v) in m {
                    map.insert(k, v);
                }
                AmqpValue::Map(map)
            }
            fe2o3_amqp_types::primitives::Value::Array(a) => {
                let mut vec = Vec::new();
                for i in a {
                    vec.push(i.into());
                }
                AmqpValue::Array(vec)
            }
            fe2o3_amqp_types::primitives::Value::Described(d) => {
                let descriptor: serde_amqp::descriptor::Descriptor = d.descriptor;
                let value: AmqpValue = d.value.into();
                let descriptor = match descriptor {
                    serde_amqp::descriptor::Descriptor::Code(code) => AmqpDescriptor::Code(code),
                    serde_amqp::descriptor::Descriptor::Name(symbol) => {
                        AmqpDescriptor::Name(symbol.into())
                    }
                };
                AmqpValue::Described(Box::new(AmqpDescribed { descriptor, value }))
            }
            fe2o3_amqp_types::primitives::Value::Decimal128(_) => todo!(),
            fe2o3_amqp_types::primitives::Value::Decimal32(_) => todo!(),
            fe2o3_amqp_types::primitives::Value::Decimal64(_) => todo!(),
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
                let t: u64 = t.0.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
                self == &fe2o3_amqp_types::primitives::Value::Timestamp(
                    Timestamp::from_milliseconds(t as i64),
                )
            }
            AmqpValue::Uuid(u) => self == &fe2o3_amqp_types::primitives::Value::Uuid((*u).into()),
            AmqpValue::Binary(b) => {
                self == &fe2o3_amqp_types::primitives::Value::Binary(ByteBuf::from(b.clone()))
            }
            AmqpValue::String(s) => self == &fe2o3_amqp_types::primitives::Value::String(s.clone()),
            AmqpValue::Symbol(s) => {
                self == &fe2o3_amqp_types::primitives::Value::Symbol((*s).clone().into())
            }
            AmqpValue::List(l) => {
                let l: Vec<fe2o3_amqp_types::primitives::Value> =
                    l.0.iter().map(|v| v.clone().into()).collect();
                self == &fe2o3_amqp_types::primitives::Value::List(l)
            }
            AmqpValue::Map(m) => {
                let m: fe2o3_amqp_types::primitives::OrderedMap<
                    fe2o3_amqp_types::primitives::Value,
                    fe2o3_amqp_types::primitives::Value,
                > = m.clone().into();
                self == &fe2o3_amqp_types::primitives::Value::Map(m)
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

            AmqpValue::Unknown => todo!(),
        }
    }
}

impl PartialEq<fe2o3_amqp_types::primitives::Value> for AmqpValue {
    fn eq(&self, other: &fe2o3_amqp_types::primitives::Value) -> bool {
        other == self
    }
}

impl<K, V> From<fe2o3_amqp_types::definitions::Fields> for AmqpOrderedMap<K, V>
where
    K: PartialEq + From<fe2o3_amqp_types::primitives::Symbol> + Clone + Default,
    V: From<fe2o3_amqp_types::primitives::Value> + Clone + Default,
{
    fn from(fields: fe2o3_amqp_types::definitions::Fields) -> Self {
        let mut map = AmqpOrderedMap::new();
        for (k, v) in fields {
            map.insert(k, v);
        }
        map
    }
}

impl
    From<
        fe2o3_amqp_types::primitives::OrderedMap<
            std::string::String,
            fe2o3_amqp_types::primitives::Value,
        >,
    > for AmqpOrderedMap<std::string::String, AmqpValue>
{
    fn from(
        value: fe2o3_amqp_types::primitives::OrderedMap<
            std::string::String,
            fe2o3_amqp_types::primitives::Value,
        >,
    ) -> Self {
        // Convert the OrderedMap to AmqpOrderedMap
        let mut amqp_ordered_map = AmqpOrderedMap::new();
        for (key, value) in value.into_iter() {
            amqp_ordered_map.insert(key, value);
        }
        amqp_ordered_map
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
        let mut ordered_map = fe2o3_amqp_types::primitives::OrderedMap::new();
        for (key, value) in value.into_iter() {
            ordered_map.insert(key.into(), value.into());
        }
        ordered_map
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
        let mut ordered_map = fe2o3_amqp_types::primitives::OrderedMap::new();
        for (key, value) in value.into_iter() {
            ordered_map.insert(key.into(), value.into());
        }
        ordered_map
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

impl From<fe2o3_amqp_types::definitions::SenderSettleMode> for crate::SenderSettleMode {
    fn from(mode: fe2o3_amqp_types::definitions::SenderSettleMode) -> crate::SenderSettleMode {
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

impl From<fe2o3_amqp_types::definitions::ReceiverSettleMode> for crate::ReceiverSettleMode {
    fn from(mode: fe2o3_amqp_types::definitions::ReceiverSettleMode) -> crate::ReceiverSettleMode {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_fe2o3_amqp_types_primitives_symbol() {
        let symbol = fe2o3_amqp_types::primitives::Symbol::from("test");
        let amqp_symbol = AmqpSymbol::from(symbol.clone());
        let symbol2: fe2o3_amqp_types::primitives::Symbol = amqp_symbol.into();
        assert_eq!(symbol, symbol2);
    }

    #[test]
    fn test_from_amqp_value_to_fe2o3_amqp_types_primitives_symbol() {
        let symbol = AmqpValue::Symbol(AmqpSymbol("test".to_string()));
        let symbol2: fe2o3_amqp_types::primitives::Symbol = symbol.into();
        assert_eq!(
            symbol2,
            fe2o3_amqp_types::primitives::Symbol("test".to_string())
        );
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
        let value2: AmqpValue = value.into();
        assert_eq!(value2, AmqpValue::String("test".to_string()));
    }

    #[test]
    fn test_from_fe2o3_amqp_types_to_amqp_value() {
        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Null;
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Bool(true);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Ubyte(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Ushort(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Uint(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Ulong(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Byte(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Short(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Int(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Long(1);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Float(1.0f32.into());
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Double(10.0f64.into());
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Char('a');
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 =
                fe2o3_amqp_types::primitives::Value::Timestamp(Timestamp::from_milliseconds(1));
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let uuid = uuid::Uuid::new_v4();
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Uuid(uuid.into());
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Binary(ByteBuf::from(vec![1]));
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::String("test".to_string());
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Symbol(
                fe2o3_amqp_types::primitives::Symbol("test".to_string()),
            );
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::List(vec![
                fe2o3_amqp_types::primitives::Value::String("test".to_string()),
            ]);
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        {
            let fe2o3 = fe2o3_amqp_types::primitives::Value::Map(
                fe2o3_amqp_types::primitives::OrderedMap::new(),
            );
            let amqp: AmqpValue = fe2o3.clone().into();
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
            let amqp: AmqpValue = fe2o3.clone().into();
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
            let amqp: AmqpValue = fe2o3.clone().into();
            let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.clone().into();
            assert_eq!(fe2o3, fe2o3_2);
            assert_eq!(fe2o3_2, amqp);
            assert_eq!(amqp, fe2o3_2);
        }

        // {
        //     let fe2o3 = fe2o3_amqp_types::primitives::Value::Decimal128(Decimal128::from(1));
        //     let amqp: AmqpValue = fe2o3.into();
        //     let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.into();

        //     assert_eq!(fe2o3, fe2o3_2);
        // }

        // {
        //     let fe2o3 = fe2o3_amqp_types::primitives::Value::Decimal32(Decimal32::from(1));
        //     let amqp: AmqpValue = fe2o3.into();
        //     let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.into();

        //     assert_eq!(fe2o3, fe2o3_2);
        // }

        // {
        //     let fe2o3 = fe2o3_amqp_types::primitives::Value::Decimal64(Decimal64::from(1));
        //     let amqp: AmqpValue = fe2o3.into();
        //     let fe2o3_2: fe2o3_amqp_types::primitives::Value = amqp.into();
        //     assert_eq!(fe2o3, fe2o3_2);
        // }
    }
}
