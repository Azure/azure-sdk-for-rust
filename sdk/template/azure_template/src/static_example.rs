// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of static variables in Rust

pub use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

/// A basic static variable with a primitive type
pub static PROGRAM_NAME: &str = "Azure Template Example";

/// A mutable static that requires unsafe code to modify
pub static mut ERROR_COUNT: u32 = 0;

/// Using atomic types for thread-safe static variables
pub static OPERATION_COUNT: AtomicUsize = AtomicUsize::new(0);

/// Using atomic static safely
pub fn count_operation() -> usize {
    OPERATION_COUNT.fetch_add(1, Ordering::SeqCst)
}

/// A static variable with a custom type
#[derive(Debug, Clone, Copy)]
pub struct AppVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl std::fmt::Display for AppVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

/// A static variable with a custom struct type
pub static APP_VERSION: AppVersion = AppVersion {
    major: 1,
    minor: 0,
    patch: 0,
};

/// A module with its own static variables
pub mod logger {
    use std::sync::atomic::{AtomicBool, Ordering};

    /// Static flag to control logging
    pub static LOGGING_ENABLED: AtomicBool = AtomicBool::new(false);

    /// Log a message if logging is enabled
    pub fn log(message: &str) {
        if LOGGING_ENABLED.load(Ordering::SeqCst) {
            println!("[LOG]: {}", message);
        }
    }
}
