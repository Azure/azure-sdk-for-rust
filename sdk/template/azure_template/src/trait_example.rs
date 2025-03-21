// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of traits in Rust

use std::fmt::{Debug, Display};

/// A basic trait defining operations that can be performed on shapes
pub trait Shape {
    /// Calculate the area of the shape
    fn area(&self) -> f64;

    /// Calculate the perimeter of the shape
    fn perimeter(&self) -> f64;

    /// Default implementation to describe the shape
    fn describe(&self) -> String {
        format!(
            "A shape with area {} and perimeter {}",
            self.area(),
            self.perimeter()
        )
    }
}

/// Rectangle struct that will implement Shape
#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

/// Implementation of the Shape trait for Rectangle
impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

/// A trait demonstrating associated types
pub trait Container {
    /// The type of items this container holds
    type Item<T>
    where
        T: Debug;

    /// Add an item to the container
    fn add<T: Debug + 'static>(&mut self, item: Self::Item<T>);
}

/// Example Box container implementing the Container trait
pub struct BoxContainer {
    items: Vec<Box<dyn std::any::Any>>,
}

/// Implementation of Container for BoxContainer
impl Container for BoxContainer {
    // Using "=" syntax for associated type with generics
    type Item<T: Debug> = Box<T> where T: Debug;

    fn add<T: Debug + 'static>(&mut self, item: Self::Item<T>) {
        self.items.push(item);
    }
}

/// A trait that extends multiple other traits
pub trait SuperTrait: Debug + Display + Clone {
    /// A method specific to SuperTrait
    fn super_method(&self) -> String;
}

/// An unsafe trait example - implementors must uphold safety guarantees
/// that the Rust compiler cannot verify
///
/// # Safety
pub unsafe trait UnsafeAccess {
    /// Get a raw pointer to the internal data
    ///
    /// # Safety
    unsafe fn get_raw_ptr(&self) -> *const u8;

    /// Get a mutable raw pointer to the internal data
    ///
    /// # Safety
    unsafe fn get_raw_mut_ptr(&mut self) -> *mut u8;
}

/// A struct that implements the UnsafeAccess trait
#[derive(Debug)]
pub struct RawBuffer {
    data: Vec<u8>,
}

impl RawBuffer {
    /// Create a new buffer with the given size
    pub fn new(size: usize) -> Self {
        RawBuffer {
            data: vec![0; size],
        }
    }
}

/// Implementation of UnsafeAccess for RawBuffer
///
/// This is unsafe because we're exposing raw pointers that could be misused
unsafe impl UnsafeAccess for RawBuffer {
    unsafe fn get_raw_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    unsafe fn get_raw_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

/// A trait with lifetime parameters
pub trait Parser<'a, T> {
    /// Parse a string into type T
    fn parse(&self, input: &'a str) -> Result<T, &'static str>;
}

/// A struct implementing the Parser trait
pub struct NumberParser;

impl<'a> Parser<'a, i32> for NumberParser
where
    i32: 'a,
{
    fn parse(&self, input: &'a str) -> Result<i32, &'static str> {
        input.parse::<i32>().map_err(|_| "Failed to parse number")
    }
}
