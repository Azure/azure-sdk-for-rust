// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// An array of homogeneous attribute values.
#[derive(Debug, PartialEq, Clone)]
pub enum AttributeArray {
    /// An array of boolean values.
    Bool(Vec<bool>),
    /// An array of 64-bit signed integers.
    I64(Vec<i64>),
    /// An array of 64bit floating point values.
    F64(Vec<f64>),
    /// An array of strings.
    String(Vec<String>),
}

/// Represents a single attribute value, which can be of various types
#[derive(Debug, PartialEq, Clone)]
pub enum AttributeValue {
    /// A boolean attribute value.
    Bool(bool),
    /// A signed 64-bit integer attribute value.
    I64(i64),
    /// A 64-bit floating point attribute value
    F64(f64),
    /// A string attribute value.
    String(String),
    /// An array of attribute values.
    Array(AttributeArray),
}

/// Represents a key-value pair attribute, which is used for tracing and telemetry.
///
/// Attributes are used to provide additional context and metadata about a span or event.
/// They can be of various types, including strings, integers, booleans, and arrays.
///
/// Attributes are typically used to enrich telemetry data with additional information
/// that can be useful for debugging, monitoring, and analysis.
#[derive(Debug, PartialEq, Clone)]
pub struct Attribute {
    /// A key-value pair attribute.
    pub key: &'static str,
    pub value: AttributeValue,
}

impl PartialEq<&str> for AttributeValue {
    fn eq(&self, other: &&str) -> bool {
        match self {
            AttributeValue::String(s) => s == *other,
            _ => false,
        }
    }
}

impl PartialEq<i64> for AttributeValue {
    fn eq(&self, other: &i64) -> bool {
        match self {
            AttributeValue::I64(i) => i == other,
            _ => false,
        }
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        AttributeValue::String(value)
    }
}

impl From<&str> for AttributeValue {
    fn from(value: &str) -> Self {
        AttributeValue::String(value.to_string())
    }
}

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        AttributeValue::Bool(value)
    }
}

impl From<i32> for AttributeValue {
    fn from(value: i32) -> Self {
        AttributeValue::I64(value as i64)
    }
}

impl From<u16> for AttributeValue {
    fn from(value: u16) -> Self {
        AttributeValue::I64(value as i64)
    }
}

impl From<u32> for AttributeValue {
    fn from(value: u32) -> Self {
        AttributeValue::I64(value as i64)
    }
}

impl From<i64> for AttributeValue {
    fn from(value: i64) -> Self {
        AttributeValue::I64(value)
    }
}

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        AttributeValue::F64(value)
    }
}

impl From<Vec<bool>> for AttributeValue {
    fn from(value: Vec<bool>) -> Self {
        AttributeValue::Array(AttributeArray::Bool(value))
    }
}

impl From<Vec<i64>> for AttributeValue {
    fn from(value: Vec<i64>) -> Self {
        AttributeValue::Array(AttributeArray::I64(value))
    }
}

impl From<Vec<f64>> for AttributeValue {
    fn from(value: Vec<f64>) -> Self {
        AttributeValue::Array(AttributeArray::F64(value))
    }
}

impl From<Vec<String>> for AttributeValue {
    fn from(value: Vec<String>) -> Self {
        AttributeValue::Array(AttributeArray::String(value))
    }
}
