// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//! Defines an AMQP `SimpleValue` type, which is a simple value type in AMQP 1.0.
//! Simple types are defined in the [AMQP specification](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-types-v1.0-os.html#section-types)
//! and are used to represent simple values such as integers, strings, and booleans.
//!

use crate::value::{AmqpDescribed, AmqpSymbol, AmqpTimestamp};
use azure_core::Uuid;

/// A simple value type in AMQP 1.0.
///
/// Simple types are the AMQP primitive types (basically the same as [`crate::value::AmqpValue`] without
/// the `Map`, `List`, and `Array` types).
///
/// They are used to represent the `Value` of an Amqp Message's `ApplicationProperties` field.
///
/// See the [AMQP specification](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-messaging-v1.0-os.html#type-application-properties) for
/// more information.
///
#[derive(Debug, Clone, PartialEq, Default)]
pub enum AmqpSimpleValue {
    /// A null value.
    #[default]
    Null,
    /// A boolean value.
    Boolean(bool),
    /// An unsigned 8-bit integer.
    UByte(u8),
    /// A signed 8-bit integer.
    Byte(i8),

    /// A UTF-32BE encoded Unicode character.
    Char(char),

    /// An unsigned 16-bit integer.
    UShort(u16),
    /// A signed 16-bit integer.
    Short(i16),
    /// An unsigned 32-bit integer.
    UInt(u32),
    /// A signed 32-bit integer.
    Int(i32),
    /// An unsigned 64-bit integer.
    ULong(u64),
    /// A signed 64-bit integer.
    Long(i64),
    /// A 32-bit floating point number.
    Float(f32),
    /// A 64-bit floating point number.
    Double(f64),
    /// An IEEE 754-2008 Decimal128 value.
    Decimal128([u8; 16]),
    /// An IEEE 754-2008 Decimal64 value.
    Decimal64([u8; 8]),
    /// An IEEE 754-2008 Decimal32 value.
    Decimal32([u8; 4]),

    /// A timestamp value.
    TimeStamp(AmqpTimestamp),
    /// A UUID value.
    Uuid(Uuid),
    /// A string value.
    String(String),
    /// A symbol value.
    Symbol(AmqpSymbol),
    /// A binary value.
    Binary(Vec<u8>),

    /// A described value.
    Described(Box<AmqpDescribed>),
}

// Note: There is intentionally no conversion from AmqpValue to AmqpSimpleValue. This is because we want a compile time error if you attempt to pass an AmqpValue into something expecting an AmqpSimpleValue.
// This is to prevent accidental misuse of the API. If you need to convert an AmqpValue to an AmqpSimpleValue, you should do so explicitly.

macro_rules! conversions_for_amqp_simple_types {
    ($(($t:ty, $field:ident)),*) => {
        $(
            impl From<$t> for AmqpSimpleValue {
                fn from(v: $t) -> Self {
                    AmqpSimpleValue::$field(v)
                }
            }

            impl From<AmqpSimpleValue> for $t {
                fn from(v: AmqpSimpleValue) -> Self {
                    match v {
                        AmqpSimpleValue::$field(v) => v,
                        _ => panic!("Expected a {}", stringify!($t)),
                    }
                }
            }
            impl From<&AmqpSimpleValue> for $t {
                fn from(v: &AmqpSimpleValue) -> Self {
                    match v {
                        AmqpSimpleValue::$field(v) => v.clone(),
                        _ => panic!("Expected a {}", stringify!($t)),
                    }
                }
            }

            impl PartialEq<$t> for AmqpSimpleValue {
                fn eq(&self, other: &$t) -> bool {
                    match self {
                        AmqpSimpleValue::$field(v) => v == other,
                        _ => false,
                    }
                }
            }
            impl PartialEq<AmqpSimpleValue> for $t {
                fn eq(&self, other: &AmqpSimpleValue) -> bool {
                    match other {
                        AmqpSimpleValue::$field(v) => self == v,
                        _ => false,
                    }
                }
            }
        )*
    }
}

conversions_for_amqp_simple_types!(
    (bool, Boolean),
    (u8, UByte),
    (u16, UShort),
    (u32, UInt),
    (u64, ULong),
    (i8, Byte),
    (i16, Short),
    (i32, Int),
    (i64, Long),
    (f32, Float),
    (f64, Double),
    (char, Char),
    (Uuid, Uuid),
    (Vec<u8>, Binary),
    (std::string::String, String),
    (AmqpSymbol, Symbol),
    (AmqpTimestamp, TimeStamp),
    ([u8; 4], Decimal32),
    ([u8; 8], Decimal64),
    ([u8; 16], Decimal128)
);

impl From<&str> for AmqpSimpleValue {
    fn from(value: &str) -> Self {
        AmqpSimpleValue::String(value.to_string())
    }
}
