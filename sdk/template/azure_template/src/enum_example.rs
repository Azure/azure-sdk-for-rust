// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of enums in Rust

/// A basic enum with simple variants
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
    Canceled,
}

/// An enum with variants containing data
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Text(String),
    Number(i32),
    Empty,
    Complex {
        subject: String,
        content: String,
        urgent: bool,
    },
}

impl Message {
    /// Create a new text message
    pub fn text(content: &str) -> Self {
        Self::Text(content.to_string())
    }

    /// Get the message as a string
    pub fn as_string(&self) -> String {
        match self {
            Self::Text(text) => text.clone(),
            Self::Number(num) => num.to_string(),
            Self::Empty => String::new(),
            Self::Complex {
                subject,
                content,
                urgent,
            } => {
                let priority = if *urgent { "URGENT" } else { "Normal" };
                format!("[{}] {}: {}", priority, subject, content)
            }
        }
    }
}

/// An enum with recursive variants
#[derive(Debug, Clone)]
pub enum JsonValue {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<JsonValue>),
    Object(std::collections::HashMap<String, JsonValue>),
}

/// An enum representing results with detailed error information
#[derive(Debug)]
pub enum ApiResult<T> {
    Success(T),
    NotFound { resource: String },
    Unauthorized { reason: String },
    ServerError { code: i32, message: String },
}
