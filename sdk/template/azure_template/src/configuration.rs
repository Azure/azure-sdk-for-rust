// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Configuration management module
//!
//! This module provides types and traits for managing configuration settings
//! in the Azure Template SDK. It demonstrates proper configuration management
//! patterns and timeout handling.

use std::fmt::Debug;
use std::time::Duration;

/// A trait for types that can be configured at runtime
///
/// Implement this trait for types that need to be configured after initialization
/// but before use.
pub trait Configurable {
    /// Configure the implementing type with runtime settings
    fn configure(&mut self);
}

impl<S: ConfigurationStorage> StorableConfiguration<S> {
    pub fn get_base(&self) -> &Configuration {
        &self.base
    }

    pub fn get_storage(&self) -> &S {
        &self.storage
    }
}

/// A trait alias combining common configuration capabilities
///
/// This demonstrates how to use trait aliases to create a more concise API
/// by combining multiple traits into a single name.
pub trait ConfigurationCapable: Configurable + Debug + Send + Sync {}

/// Configuration settings container for the template SDK
///
/// Holds various configuration options that control the behavior
/// of template operations.
#[derive(Debug, Clone)]
pub struct Configuration {
    /// Timeout duration in seconds for operations
    pub timeout: u64,
}

/// Trait defining configuration limits through associated constants
pub trait ConfigurationLimits {
    /// Maximum allowed timeout in seconds
    const MAX_TIMEOUT: u64;
    /// Minimum allowed timeout in seconds
    const MIN_TIMEOUT: u64;
}

impl ConfigurationLimits for Configuration {
    const MAX_TIMEOUT: u64 = 3600; // 1 hour
    const MIN_TIMEOUT: u64 = 1; // 1 second
}

impl Configuration {
    /// Creates a new Configuration with the specified timeout
    ///
    /// # Arguments
    /// * `timeout` - The timeout value in seconds
    pub fn new(timeout: u64) -> Self {
        Self { timeout }
    }

    /// Creates a new Configuration with bounds checking
    ///
    /// # Arguments
    /// * `timeout` - The timeout value in seconds, clamped to valid range
    pub fn new_bounded(timeout: u64) -> Self {
        let timeout = timeout.clamp(Self::MIN_TIMEOUT, Self::MAX_TIMEOUT);
        Self { timeout }
    }

    /// Convert the timeout to a std::time::Duration
    ///
    /// Useful when integrating with async operations or standard library timing functions
    pub fn timeout_duration(&self) -> Duration {
        Duration::from_secs(self.timeout)
    }
}

/// Trait for configuration storage types with an associated value type
pub trait ConfigurationStorage {
    /// The type of values stored in this configuration
    type Value;

    /// Store a value in the configuration
    fn store(&mut self, value: Self::Value);

    /// Retrieve the stored value
    fn retrieve(&self) -> Option<&Self::Value>;
}

/// In-memory storage implementation for string values
#[derive(Debug, Default)]
pub struct StringStorage {
    value: Option<String>,
}

impl ConfigurationStorage for StringStorage {
    type Value = String;

    fn store(&mut self, value: Self::Value) {
        self.value = Some(value);
    }

    fn retrieve(&self) -> Option<&Self::Value> {
        self.value.as_ref()
    }
}

/// Extended Configuration with storage capabilities
#[derive(Debug)]
pub struct StorableConfiguration<S: ConfigurationStorage> {
    base: Configuration,
    storage: S,
}

impl<S: ConfigurationStorage> StorableConfiguration<S> {
    pub fn new(timeout: u64, storage: S) -> Self {
        Self {
            base: Configuration::new(timeout),
            storage,
        }
    }
}

/// Example type implementing the ConfigurationCapable trait alias
#[derive(Debug)]
pub struct ConfigurableResource {
    name: String,
    config: Configuration,
}

impl ConfigurableResource {
    pub fn new(name: String, config: Configuration) -> Self {
        Self { name, config }
    }
}

impl Configurable for ConfigurableResource {
    fn configure(&mut self) {
        // Example configuration logic
        println!("Configuring resource: {}", self.name);
        println!("Timeout duration: {:?}", self.config.timeout_duration());
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
