// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Data processing and status tracking module

/// Basic enum for operation status
#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Active,
    Inactive,
    Pending,
}

/// Detailed status with variants demonstrating different enum patterns
#[derive(Debug, Clone, PartialEq)]
pub enum StatusVariant {
    InProgress { percent: u8 },
    Waiting(u32),
    Complete(u64),
    Failed { reason: String },
}

/// Trait for data processing operations with associated type and constant
pub trait DataProcessor {
    const CHUNK_SIZE: usize = 1024;
    type Output;
    fn process(&self) -> Self::Output;
}

impl From<StatusVariant> for Status {
    fn from(variant: StatusVariant) -> Self {
        match variant {
            StatusVariant::InProgress { .. } => Status::Active,
            StatusVariant::Waiting(..) => Status::Pending,
            _ => Status::Inactive,
        }
    }
}

/// Example enum showing different variant types
#[derive(Debug)]
pub enum Message {
    Text(String),
    Number(i32),
    User { name: String, id: u64 },
    Empty,
}
