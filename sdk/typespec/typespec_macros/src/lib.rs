// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use syn::{parse::ParseStream, parse_macro_input, spanned::Spanned, DeriveInput, Error, LitStr};

extern crate proc_macro;

mod model;
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

/// Parses a `syn::parse::ParseStream` that is expected to contain a string literal and extracts the `syn::LitStr`.
fn parse_literal_string(value: ParseStream) -> Result<LitStr> {
    let expr: syn::Expr = value
        .parse()
        .map_err(|_| Error::new(value.span(), "expected string literal"))?;
    match expr {
        syn::Expr::Lit(lit) => match lit.lit {
            syn::Lit::Str(s) => Ok(s),
            _ => Err(Error::new(lit.span(), "expected string literal")),
        },
        _ => Err(Error::new(expr.span(), "expected string literal")),
    }
}

/// Derive macro for implementing the `Model` trait.
///
/// Deriving this trait allows a type to be deserialized from an HTTP response body.
/// By default, the type must also implement `serde::Deserialize`, or the generated code will not compile.
///
/// ## Attributes
///
/// The following attributes are supported on the struct itself:
///
/// ### `#[typespec(format)]`
///
/// The format attribute specifies the format of the response body. The default is `json`.
/// If compiling with the `xml` feature, the value `xml` is also supported.
///
/// ```rust
/// # use typespec_macros::Model;
/// # use serde::Deserialize;
/// #[derive(Model, Deserialize)]
/// #[typespec(format = "xml")]
/// struct MyModel {
///   value: String
/// }
/// ```
///
/// **NOTE:** Using formats other than JSON may require enabling additional features in `typespec_client_core`.
///
/// ### `#[typespec(crate)]`
///
/// The 'crate' attribute specifies an alternate module path, other than the default of `typespec_client_core`, to reference the typespec client crate.
///
/// ```rust
/// # use typespec_macros::Model;
/// # use serde::Deserialize;
/// extern crate typespec_client_core as my_typespec;
///
/// #[derive(Model, Deserialize)]
/// #[typespec(crate = "my_typespec")]
/// struct MyModel {
///   value: String
/// }
/// ```
#[proc_macro_derive(Model, attributes(typespec))]
pub fn derive_model(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    run_derive_macro(input, model::derive_model_impl)
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
