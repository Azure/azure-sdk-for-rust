// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Attribute conversions between typespec_client_core and OpenTelemetry.

// Re-export typespec_client_core tracing attributes for convenience
use azure_core::tracing::{
    AttributeArray as AzureAttributeArray, AttributeValue as AzureAttributeValue,
};

pub(super) struct AttributeArray(AzureAttributeArray);

pub(super) struct AttributeValue(pub AzureAttributeValue);

impl From<bool> for AttributeValue {
    fn from(value: bool) -> Self {
        AttributeValue(AzureAttributeValue::Bool(value))
    }
}

impl From<i64> for AttributeValue {
    fn from(value: i64) -> Self {
        AttributeValue(AzureAttributeValue::I64(value))
    }
}

impl From<u64> for AttributeValue {
    fn from(value: u64) -> Self {
        AttributeValue(AzureAttributeValue::U64(value))
    }
}

impl From<String> for AttributeValue {
    fn from(value: String) -> Self {
        AttributeValue(AzureAttributeValue::String(value))
    }
}

impl From<Vec<bool>> for AttributeArray {
    fn from(values: Vec<bool>) -> Self {
        AttributeArray(AzureAttributeArray::Bool(values))
    }
}

impl From<Vec<i64>> for AttributeArray {
    fn from(values: Vec<i64>) -> Self {
        AttributeArray(AzureAttributeArray::I64(values))
    }
}

impl From<Vec<u64>> for AttributeArray {
    fn from(values: Vec<u64>) -> Self {
        AttributeArray(AzureAttributeArray::U64(values))
    }
}

impl From<Vec<String>> for AttributeArray {
    fn from(values: Vec<String>) -> Self {
        AttributeArray(AzureAttributeArray::String(values))
    }
}

/// Conversion from typespec_client_core AttributeValue to OpenTelemetry Value
impl From<AttributeValue> for opentelemetry::Value {
    fn from(value: AttributeValue) -> Self {
        match value.0 {
            AzureAttributeValue::Bool(b) => opentelemetry::Value::Bool(b),
            AzureAttributeValue::I64(i) => opentelemetry::Value::I64(i),
            AzureAttributeValue::U64(u) => opentelemetry::Value::I64(u as i64),
            AzureAttributeValue::String(s) => opentelemetry::Value::String(s.into()),
            AzureAttributeValue::Array(arr) => {
                opentelemetry::Value::Array(opentelemetry::Array::from(AttributeArray(arr)))
            }
        }
    }
}

/// Conversion from typespec_client_core AttributeArray to OpenTelemetry Array
impl From<AttributeArray> for opentelemetry::Array {
    fn from(array: AttributeArray) -> Self {
        match array.0 {
            AzureAttributeArray::Bool(values) => values.into(),
            AzureAttributeArray::I64(values) => values.into(),
            AzureAttributeArray::U64(values) => {
                let i64_values: Vec<i64> = values.into_iter().map(|v| v as i64).collect();
                i64_values.into()
            }
            AzureAttributeArray::String(values) => {
                let string_values: Vec<opentelemetry::StringValue> =
                    values.into_iter().map(|s| s.into()).collect();
                string_values.into()
            }
        }
    }
}
