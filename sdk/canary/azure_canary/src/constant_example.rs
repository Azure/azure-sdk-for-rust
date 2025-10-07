// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of constants in Rust

/// A basic constant declaration with a type annotation
pub const MAX_CONNECTIONS: usize = 100;

/// A string constant
pub const API_VERSION: &str = "v1.0.0";

/// Constants can be expressions too
pub const MILLISECONDS_PER_DAY: u64 = 24 * 60 * 60 * 1000;

/// Constants in a specific type
pub const DEFAULT_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);

/// Constants can be defined in modules to limit scope
pub mod config {
    /// Database connection timeout
    pub const DB_TIMEOUT: u64 = 10;
}

/// Constants can be defined in traits as associated constants
pub trait Limits {
    /// The maximum allowed value
    const MAX: Self;
    /// The minimum allowed value
    const MIN: Self;
}

impl Limits for i32 {
    const MAX: i32 = i32::MAX;
    const MIN: i32 = i32::MIN;
}

/// Constants inside impl blocks (associated constants)
pub struct Temperature {
    pub value: f64,
    pub unit: TemperatureUnit,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
    Kelvin,
    Rankine,    // Testing APIView
}

impl Temperature {
    /// Absolute zero in Celsius
    pub const ABSOLUTE_ZERO_C: f64 = -273.15;

    pub fn new(value: f64, unit: TemperatureUnit) -> Self {
        Self { value, unit }
    }

    pub fn is_below_freezing(&self) -> bool {
        match self.unit {
            TemperatureUnit::Celsius => self.value < 0.0,
            TemperatureUnit::Fahrenheit => self.value < 32.0,
            TemperatureUnit::Kelvin => self.value < 273.15,
        }
    }
}
