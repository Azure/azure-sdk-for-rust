// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

//! This crate contains procedural macros that are used to generate code for the TypeSpec SDK.

use syn::{parse_macro_input, DeriveInput};

extern crate proc_macro;

mod safe_debug;

type Result<T> = ::std::result::Result<T, syn::Error>;

// NOTE: Proc macros must appear in the root of the crate. Just re-exporting them with `pub use` is **not sufficient**.
// So, all the top-level entry functions for the proc macros will appear here, but they just call inner "impl" functions in the modules.

/// Defines the function signature expected by run_derive_macro
type DeriveImpl = fn(DeriveInput) -> Result<proc_macro2::TokenStream>;

/// Runs the provided derive macro implementation, automatically generating errors if it returns errors.
fn run_derive_macro(input: proc_macro::TokenStream, imp: DeriveImpl) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match imp(ast) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

/// Derive to help prevent leaking personally identifiable information (PII) that deriving [`Debug`](std::fmt::Debug) might otherwise.
///
/// `SafeDebug` is not a trait and cannot be implemented, nor should you derive `Debug` explicitly.
/// Only when you derive `SafeDebug` will types help prevent leaking PII because, by default, only the type name is printed.
/// Only when you enable the `debug` feature will it derive `Debug` normally.
///
/// You can attribute types, fields, and variants with `#[safe(true)]` or `#[safe(false)]` to optionally show or hide members.
/// The default is that no members are shown. The inner most `#[safe(..)]` attribute determines whether to show or hide a member.
///
/// # Examples
///
/// ```
/// # use typespec_macros::SafeDebug;
/// #[derive(SafeDebug)]
/// struct Person {
///     name: String,
/// }
///
/// let person = Person {
///     name: "Kelly Smith".to_string(),
/// };
/// if cfg!(feature = "debug") {
///     assert_eq!(format!("{person:?}"), r#"Person { name: "Kelly Smith" }"#);
/// } else {
///     assert_eq!(format!("{person:?}"), "Person { .. }");
/// }
/// ```
///
/// Using the `#[safe(..)]` attribute, you can selectively show or hide members.
/// The default, when not present or inherited, is to always hide members unless the `debug` feature is enabled.
///
/// ```
/// # use typespec_macros::SafeDebug;
/// use std::ops::Range;
///
/// #[derive(SafeDebug)]
/// struct Employee {
///     name: String,
///     #[safe(true)]
///     position: Position,
/// }
///
/// #[derive(SafeDebug)]
/// #[safe(true)]
/// struct Position {
///     id: i32,
///     title: String,
///     #[safe(false)]
///     salary: Range<i32>,
/// }
///
/// let employee = Employee {
///     name: "Kelly Smith".to_string(),
///     position: Position {
///         id: 12,
///         title: "Staff Engineer".to_string(),
///         salary: 200_000..250_000,
///     },
/// };
/// if cfg!(feature = "debug") {
///     assert_eq!(format!("{employee:?}"), r#"Employee { name: "Kelly Smith", position: Position { id: 12, title: "Staff Engineer", salary: 200000..250000 } }"#);
/// } else {
///     assert_eq!(format!("{employee:?}"), r#"Employee { position: Position { id: 12, title: "Staff Engineer", .. }, .. }"#);
/// }
/// ```
#[proc_macro_derive(SafeDebug, attributes(safe))]
pub fn derive_safe_debug(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    run_derive_macro(input, safe_debug::derive_safe_debug_impl)
}
