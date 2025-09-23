// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Examples of structs in Rust

/// A regular struct with named fields
#[derive(Debug, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: Option<String>,
}

impl Person {
    /// Creates a new Person with the specified name and age
    pub fn new(name: String, age: u32) -> Self {
        Self {
            name,
            age,
            email: None,
        }
    }

    /// Sets the email address
    pub fn with_email(mut self, email: String) -> Self {
        self.email = Some(email);
        self
    }
}

/// A tuple struct representing a point in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2D(pub f32, pub f32);

impl Point2D {
    /// Creates a new point at the origin
    pub fn origin() -> Self {
        Self(0.0, 0.0)
    }

    /// Calculates the distance between points
    pub fn distance_to(&self, other: &Self) -> f32 {
        let dx = self.0 - other.0;
        let dy = self.1 - other.1;
        (dx * dx + dy * dy).sqrt()
    }
}

/// A unit struct with no fields
#[derive(Debug, Clone, Copy)]
pub struct Empty;

/// A generic struct with a type parameter
#[derive(Debug, Clone)]
pub struct Container<T> {
    pub value: T,
    pub description: String,
}

/// A struct with documentation and lifetimes
#[derive(Debug)]
pub struct Borrowed<'a> {
    pub text: &'a str,
    pub additional_text: Option<&'a str>,
}

impl<'a> Borrowed<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            additional_text: None,
        }
    }
}

/// A struct that uses another custom struct as a field
#[derive(Debug)]
pub struct Employee {
    pub personal_info: Person,
    pub employee_id: String,
    pub position: String,
}

impl Employee {
    pub fn full_description(&self) -> String {
        format!(
            "{} (age: {}), ID: {}, Position: {}",
            self.personal_info.name, self.personal_info.age, self.employee_id, self.position
        )
    }
}
