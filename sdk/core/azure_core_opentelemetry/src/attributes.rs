// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Attribute conversions between typespec_client_core and OpenTelemetry.

// Re-export typespec_client_core tracing attributes for convenience
use azure_core::tracing::{
    Attribute as AzureAttribute, AttributeArray as AzureAttributeArray,
    AttributeValue as AzureAttributeValue,
};
use opentelemetry::KeyValue;

pub(super) struct AttributeArray(AzureAttributeArray);

pub(super) struct AttributeValue(pub AzureAttributeValue);

pub(super) struct OpenTelemetryAttribute(pub AzureAttribute);

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

impl From<f64> for AttributeValue {
    fn from(value: f64) -> Self {
        AttributeValue(AzureAttributeValue::I64(value as i64))
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
impl From<Vec<f64>> for AttributeArray {
    fn from(values: Vec<f64>) -> Self {
        AttributeArray(AzureAttributeArray::F64(values))
    }
}

impl From<Vec<String>> for AttributeArray {
    fn from(values: Vec<String>) -> Self {
        AttributeArray(AzureAttributeArray::String(values))
    }
}

impl From<OpenTelemetryAttribute> for KeyValue {
    fn from(attr: OpenTelemetryAttribute) -> Self {
        KeyValue::new(
            opentelemetry::Key::from(attr.0.key.to_string()),
            opentelemetry::Value::from(AttributeValue(attr.0.value)),
        )
    }
}

/// Conversion from typespec_client_core AttributeValue to OpenTelemetry Value
impl From<AttributeValue> for opentelemetry::Value {
    fn from(value: AttributeValue) -> Self {
        match value.0 {
            AzureAttributeValue::Bool(b) => opentelemetry::Value::Bool(b),
            AzureAttributeValue::I64(i) => opentelemetry::Value::I64(i),
            AzureAttributeValue::F64(f) => opentelemetry::Value::F64(f),
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
            AzureAttributeArray::F64(values) => values.into(),
            AzureAttributeArray::String(values) => {
                let string_values: Vec<opentelemetry::StringValue> =
                    values.into_iter().map(|s| s.into()).collect();
                string_values.into()
            }
        }
    }
}
