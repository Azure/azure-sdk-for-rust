// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration management module
//!
//! This module provides types and traits for managing configuration settings
//! in the Azure Template SDK. It demonstrates proper configuration management
//! patterns and timeout handling.

use std::time::Duration;

/// A trait for types that can be configured at runtime
///
/// Implement this trait for types that need to be configured after initialization
/// but before use.
pub trait Configurable {
    /// Configure the implementing type with runtime settings
    fn configure(&mut self);
}

/// Configuration settings container for the template SDK
///
/// Holds various configuration options that control the behavior
/// of template operations.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Timeout duration in seconds for operations
    pub timeout: u64,
}

impl Configuration {
    /// Creates a new Configuration with the specified timeout
    ///
    /// # Arguments
    /// * `timeout` - The timeout value in seconds
    pub fn new(timeout: u64) -> Self {
        Self { timeout }
    }

    /// Convert the timeout to a std::time::Duration
    ///
    /// Useful when integrating with async operations or standard library timing functions
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }
}

/// Default timeout value in seconds for operations
///
/// This value is used when no explicit timeout is specified
pub static DEFAULT_TIMEOUT: u64 = 30;

/// Maximum number of retry attempts for operations
///
/// Used to limit the number of retries for fallible operations
pub const MAX_RETRIES: u32 = 3;

/// Macro for creating a new Configuration instance
#[macro_export]
macro_rules! create_configuration {
    ($timeout:expr) => {
        Configuration::new($timeout)
    };
}
