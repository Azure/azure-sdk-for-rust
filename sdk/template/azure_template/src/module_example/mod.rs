// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example module demonstrating common Rust patterns
//!
//! This module serves as a reference implementation showing various Rust features
//! and patterns commonly used in the Azure SDK. It includes examples of traits,
//! generic types, and error handling.

/// Module containing lease-related functionality
pub mod lease;

/// A trait defining operations that can be performed on templates
///
/// This trait demonstrates proper trait design patterns including
/// documentation and method signatures.
///
/// # Examples
///
/// ```
/// use azure_template::module_example::{Template, TemplateOperation};
///
/// let template = Template {
///     name: String::from("example"),
///     value: 42
/// };
/// assert!(template.process().contains("example"));
/// ```
pub trait TemplateOperation {
    /// Process the template and return a formatted string representation
    fn process(&self) -> String;
}

/// A generic template type that can hold any displayable value
///
/// This struct demonstrates generic type parameters and trait bounds.
/// The type parameter T must implement Display to ensure the value
/// can be converted to a string representation.
///
/// # Type Parameters
///
/// * `T` - The type of value stored in the template, must implement Display
///
/// # Examples
///
/// ```
/// use azure_template::module_example::Template;
///
/// let string_template = Template {
///     name: String::from("text"),
///     value: String::from("Hello")
/// };
///
/// let number_template = Template {
///     name: String::from("count"),
///     value: 42
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Template<T: std::fmt::Display> {
    /// Name of the template
    pub name: String,
    /// Value stored in the template
    pub value: T,
}

impl<T: std::fmt::Display> TemplateOperation for Template<T> {
    fn process(&self) -> String {
        format!("Template {} with value: {}", self.name, self.value)
    }
}

/// Process a template and return its string representation
///
/// This function demonstrates error handling patterns using Result
/// and generic type constraints.
///
/// # Arguments
///
/// * `template` - The template to process
///
/// # Returns
///
/// * `Ok(String)` - The processed template string
/// * `Err(&str)` - An error message if processing fails
///
/// # Examples
///
/// ```
/// use azure_template::module_example::{Template, process_template};
///
/// let template = Template {
///     name: String::from("example"),
///     value: 42
/// };
/// let result = process_template(&template).unwrap();
/// assert!(result.contains("example"));
/// assert!(result.contains("42"));
/// ```
pub fn process_template<T: std::fmt::Display>(
    template: &Template<T>,
) -> Result<String, &'static str> {
    Ok(template.process())
}
