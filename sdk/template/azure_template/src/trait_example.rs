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
    type Item;

    /// Add an item to the container
    fn add(&mut self, item: Self::Item);

    /// Get the number of items in the container
    fn len(&self) -> usize;

    /// Check if the container is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// A simple vector-based collection
#[derive(Debug)]
pub struct VecContainer<T> {
    pub items: Vec<T>,
}

/// Implementation of Container for VecContainer
impl<T> Container for VecContainer<T> {
    type Item = T;

    fn add(&mut self, item: Self::Item) {
        self.items.push(item);
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

/// A trait demonstrating associated constants
pub trait Bounded {
    /// The maximum value for this type
    const MAX: Self;
    /// The minimum value for this type
    const MIN: Self;
}

impl Bounded for i32 {
    const MAX: i32 = i32::MAX;
    const MIN: i32 = i32::MIN;
}

/// A trait that extends multiple other traits
pub trait SuperTrait: Debug + Display + Clone {
    /// A method specific to SuperTrait
    fn super_method(&self) -> String;
}

/// A trait with lifetime parameters
pub trait Parser<'a, T> {
    /// Parse a string into type T
    fn parse(&self, input: &'a str) -> Result<T, &'static str>;
}

/// A struct implementing the Parser trait
pub struct NumberParser;

impl<'a> Parser<'a, i32> for NumberParser {
    fn parse(&self, input: &'a str) -> Result<i32, &'static str> {
        input.parse::<i32>().map_err(|_| "Failed to parse number")
    }
}
