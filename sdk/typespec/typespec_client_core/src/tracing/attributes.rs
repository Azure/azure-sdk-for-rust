// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// An array of homogeneous attribute values.
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

/// Represents a single attribute value, which can be of various types.
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
