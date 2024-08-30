use syn::{parse::ParseStream, DeriveInput, LitStr};

extern crate proc_macro;

mod error;
mod model;

pub(crate) use error::{Error, Result};

// NOTE: Proc macros must appear in the root of the crate. Just re-exporting them with `pub use` is **not sufficient**.
// So, all the top-level entry functions for the proc macros will appear here, but they just call inner "impl" functions in the modules.

/// Defines the function signature expected by run_derive_macro
type DeriveImpl = fn(DeriveInput) -> error::Result<proc_macro2::TokenStream>;

/// Runs the provided derive macro implementation, automatically generating errors if it returns errors.
fn run_derive_macro(input: proc_macro::TokenStream, imp: DeriveImpl) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input.into()).unwrap();
    match imp(ast) {
        Ok(tokens) => tokens.into(),
        Err(mut errs) => {
            let tokens: Vec<proc_macro2::TokenStream> = errs.drain(..).map(|e| e.into()).collect();

            quote::quote! {
                #(#tokens)*
            }
            .into()
        }
    }
}

/// Parses a `syn::parse::ParseStream` that is expected to contain a string literal and extracts the `syn::LitStr`.
fn parse_literal_string(value: ParseStream) -> Result<LitStr> {
    let expr: syn::Expr = value
        .parse()
        .map_err(|_| vec![Error::new(value.span(), "Expected string literal")])?;
    match expr {
        syn::Expr::Lit(lit) => match lit.lit {
            syn::Lit::Str(s) => Ok(s),
            _ => Err(vec![Error::new(value.span(), "Expected string literal")]),
        },
        _ => Err(vec![Error::new(value.span(), "Expected string literal")]),
    }
}

/// Derive macro for implementing `Model` trait.
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
/// # use typespec_derive::Model;
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
/// # use typespec_derive::Model;
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
