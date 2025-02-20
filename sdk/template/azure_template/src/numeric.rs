// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Numeric operations and types module
//!
//! This module provides numeric type definitions and traits for mathematical operations.
//! It demonstrates FFI-compatible types and generic numeric traits.

use azure_template_core::NumericCore;
use azure_template_macros::numeric_operation;
use std::ops::{Add, Sub};

/// Trait demonstrating associated constants
pub trait NumericConstants {
    const MAX_VALUE: f64;
    const MIN_VALUE: f64;
    const PRECISION: u8;
}

/// Trait demonstrating associated types
pub trait NumericContainer {
    type Item;
    type Container;
    fn new() -> Self;
    fn add(&mut self, item: Self::Item);
}

// Example implementation for Vector3
impl<T: NumericConstants> NumericConstants for Vector3<T> {
    const MAX_VALUE: f64 = T::MAX_VALUE;
    const MIN_VALUE: f64 = T::MIN_VALUE;
    const PRECISION: u8 = T::PRECISION;
}

/// Macro for implementing numeric operations
#[macro_export]
macro_rules! implement_numeric {
    ($t:ty) => {
        impl NumericOps for $t {}
    };
}

/// Struct using proc macro attribute
#[derive(Debug, Clone)]
#[numeric_operation]
pub struct NumericWrapper<T> {
    value: T,
    precision: u8,
}

impl<T: NumericCore> NumericWrapper<T> {
    pub fn value(&self) -> &T {
        &self.value
    }
    pub fn precision(&self) -> u8 {
        self.precision
    }
    pub fn new(value: T) -> Self {
        Self {
            value,
            precision: 2,
        }
    }
}

// Example usage of the declarative macro
implement_numeric!(f32);
implement_numeric!(f64);

/// Generic vector type
#[derive(Debug, Clone)]
pub struct Vector3<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Vector3<T> {
    /// Returns the x component
    pub fn x(&self) -> &T {
        &self.x
    }

    /// Returns the y component
    pub fn y(&self) -> &T {
        &self.y
    }

    /// Returns the z component
    pub fn z(&self) -> &T {
        &self.z
    }
}

impl<T> Matrix3<T> {
    /// Returns a reference to the data of the matrix
    pub fn data(&self) -> &[[T; 3]; 3] {
        &self.data
    }
}

/// Matrix type for transformations
#[derive(Debug, Clone)]
pub struct Matrix3<T> {
    data: [[T; 3]; 3],
}

// Type aliases
pub type Vec3f = Vector3<f32>;
pub type Mat3f = Matrix3<f32>;
pub type Transform3D<T> = (Vector3<T>, Matrix3<T>);

/// Core traits for numeric types
pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait Validate {
    fn is_valid(&self) -> bool;
}

pub trait HexFormat {
    fn to_hex(&self) -> String;
}

/// Trait alias example
pub trait NumericFormat: ToBytes + Validate + HexFormat {}

/// Type alias for sizes
pub type Size = u64;

/// Trait for arithmetic operations
pub trait NumericOps: Add<Output = Self> + Sub<Output = Self> + Sized {}

/// FFI-compatible union type
#[repr(C)]
#[derive(Copy, Clone)]
pub union IntOrFloat {
    pub int: i32,
    pub float: f32,
}

impl Add for IntOrFloat {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        unsafe {
            if self.is_int() && other.is_int() {
                IntOrFloat {
                    int: self.int + other.int,
                }
            } else {
                IntOrFloat {
                    float: self.get_as_float() + other.get_as_float(),
                }
            }
        }
    }
}

impl Sub for IntOrFloat {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        unsafe {
            if self.is_int() && other.is_int() {
                IntOrFloat {
                    int: self.int - other.int,
                }
            } else {
                IntOrFloat {
                    float: self.get_as_float() - other.get_as_float(),
                }
            }
        }
    }
}

impl NumericOps for IntOrFloat {}

impl IntOrFloat {
    /// Creates a new `IntOrFloat` from an integer value
    #[inline]
    pub fn from_int(value: i32) -> Self {
        Self { int: value }
    }

    /// Creates a new `IntOrFloat` from a float value
    #[inline]
    pub fn from_float(value: f32) -> Self {
        Self { float: value }
    }

    /// Returns true if the value was stored as an integer
    pub fn is_int(&self) -> bool {
        // Implementation detail: We use a simple heuristic by checking
        // if the float representation would be a whole number
        unsafe {
            let as_float = self.float;
            as_float.fract() == 0.0 && as_float >= i32::MIN as f32 && as_float <= i32::MAX as f32
        }
    }

    /// Safely gets the value as a float
    pub fn get_as_float(&self) -> f32 {
        unsafe {
            if self.is_int() {
                self.int as f32
            } else {
                self.float
            }
        }
    }
}

impl std::fmt::Debug for IntOrFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if self.is_int() {
                write!(f, "IntOrFloat(int: {})", self.int)
            } else {
                write!(f, "IntOrFloat(float: {})", self.float)
            }
        }
    }
}

impl std::fmt::Display for IntOrFloat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unsafe {
            if self.is_int() {
                write!(f, "{}", self.int)
            } else {
                write!(f, "{}", self.float)
            }
        }
    }
}
