// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of struct fields in Rust

pub use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;

/// A struct with fields of different types and visibility
#[derive(Debug)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub age: Option<u8>,
    pub is_active: bool,
}

/// A struct with documented fields
#[derive(Debug)]
pub struct Configuration {
    /// The hostname to connect to
    pub host: String,
    /// The port number
    pub port: u16,
    /// Whether to use TLS/HTTPS
    pub secure: bool,
}

/// A struct with fields that implement specific traits
#[derive(Debug)]
pub struct TraitBounded<T, U>
where
    T: Debug + Clone,
    U: Debug + Default,
{
    /// Field with T type that implements Debug and Clone
    pub first: T,
    /// Field with U type that implements Debug and Default
    pub second: U,
}

/// A struct with tuple fields
#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64, pub f64);

/// A struct with a PhantomData field
#[derive(Debug)]
pub struct TypedId<T> {
    pub id: u64,
    pub _type: PhantomData<T>,
}

/// A struct with lifetime parameters in fields
#[derive(Debug)]
pub struct Document<'a> {
    pub title: &'a str,
    pub content: &'a str,
    pub author: &'a str,
}

/// A struct using Box for heap allocation
#[derive(Debug)]
pub struct LargeData {
    pub data: Box<[u8]>,
    pub size: usize,
}

/// A struct with renamed fields for serialization
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "success")]
    pub is_success: bool,

    #[serde(rename = "data")]
    pub response_data: String,

    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
}
