// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! Derive support for layered Azure Cosmos DB SDK configuration options.

mod builder;
mod env;
mod parse;
mod view;

use parse::OptionsInput;
use syn::{parse_macro_input, DeriveInput};

type Result<T> = ::std::result::Result<T, syn::Error>;

/// Derives layered configuration boilerplate for Cosmos DB option group structs.
///
/// Derive this macro on a struct with named `Option<T>` fields. It generates
/// `{Name}View` for resolving values across layers, `{Name}Builder` for
/// constructing the struct, and `Default` with every field set to `None`.
/// Fields with `#[option(env = "...")]` also enable `from_env()` and
/// `from_env_vars()` constructors. Explicit layers override environment values
/// in the order given below; the last layer has the highest priority.
///
/// # Struct-level attributes
///
/// - `#[options(layers(runtime, account, operation))]` — declares which layers
///   this option group participates in, in increasing priority. Use any
///   nonempty subset in this order.
/// - `#[options(env_only)]` — generates only `from_env()` and `from_env_vars()`
///   on an existing struct; it does not generate a view, builder, or `Default`.
///   It cannot be combined with `layers(...)` and requires at least one
///   `#[option(env = "...")]` field.
///
/// # Field-level attributes
///
/// - `#[option(env = "AZURE_COSMOS_...")]` — enables environment variable loading.
/// - `#[option(env = "AZURE_COSMOS_...", overridable)]` — additionally recognizes a
///   `{ENV}_OVERRIDE` environment variable that takes precedence over every
///   layer, including operation. Generates `from_env_override()` and
///   `new_with_override()` on the view. Requires `env`.
/// - `#[option(merge = "extend")]` — combines values from all layers rather
///   than taking the highest-priority value.
/// - `#[option(nested)]` — delegates resolution to a child View.
/// - `#[option(env = "...", parser = path::to::fn)]` — parses the env var with a
///   custom `fn(&str) -> Option<T>` (where `T` is the field's inner type)
///   instead of `FromStr`. A `None` result is logged and ignored. Requires `env`.
/// - `#[cfg(feature = "...")]` on a field also gates its generated builder
///   setter, view accessor, default value, and environment-variable loading.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_macros::CosmosOptions;
/// use std::sync::Arc;
///
/// #[derive(CosmosOptions)]
/// #[options(layers(runtime, account, operation))]
/// pub struct RequestOptions {
///     pub max_items: Option<u32>,
/// }
///
/// let runtime = Arc::new(RequestOptionsBuilder::new().with_max_items(10).build());
/// let operation = RequestOptionsBuilder::new().with_max_items(5).build();
/// let view = RequestOptionsView::new(None, Some(runtime), None, Some(&operation));
/// assert_eq!(view.max_items(), Some(&5));
/// ```
#[proc_macro_derive(CosmosOptions, attributes(options, option))]
pub fn derive_cosmos_options(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    match derive_cosmos_options_impl(ast) {
        Ok(tokens) => tokens.into(),
        Err(e) => e.to_compile_error().into(),
    }
}

fn derive_cosmos_options_impl(ast: DeriveInput) -> Result<proc_macro2::TokenStream> {
    let input = OptionsInput::from_derive_input(&ast)?;

    // Env-only mode: the struct is purely an environment-variable source (e.g.
    // an existing builder type reused to read env vars into). Emit only the
    // `from_env`/`from_env_vars` constructors; skip the View, Builder, and
    // `Default` generation so the macro can decorate a hand-written type
    // without colliding with its existing `#[derive(Default)]` or builder
    // methods.
    if input.env_only {
        let env_tokens = env::generate_from_env(&input)?;
        return Ok(quote::quote! {
            #[doc(hidden)]
            const _: () = {
                #env_tokens
            };
        });
    }

    let view_tokens = view::generate_view(&input)?;
    let builder_tokens = builder::generate_builder(&input)?;
    let env_tokens = env::generate_from_env(&input)?;
    let default_tokens = generate_default(&input)?;

    // View and Builder structs must be at module scope so they're visible to callers.
    // Trait impls go in a const block for scope isolation.
    let gen = quote::quote! {
        #view_tokens
        #builder_tokens

        #[doc(hidden)]
        const _: () = {
            #env_tokens
            #default_tokens
        };
    };

    Ok(gen)
}

fn generate_default(input: &OptionsInput) -> Result<proc_macro2::TokenStream> {
    let name = &input.name;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let fields = input.fields.iter().map(|f| {
        let cfg_attrs = &f.cfg_attrs;
        let field_name = &f.ident;
        quote::quote! { #(#cfg_attrs)* #field_name: None }
    });

    Ok(quote::quote! {
        #[automatically_derived]
        impl #impl_generics Default for #name #ty_generics #where_clause {
            fn default() -> Self {
                Self {
                    #(#fields),*
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conditional_field_gates_every_generated_member() {
        let ast: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account, operation))]
            struct TestOptions {
                #[cfg(feature = "preview")]
                #[option(env = "TEST_PREVIEW", overridable)]
                preview: Option<u32>,
            }
        };
        let generated = derive_cosmos_options_impl(ast).unwrap().to_string();
        let cfg = quote::quote!(#[cfg(feature = "preview")]).to_string();
        assert_eq!(
            generated.matches(&cfg).count(),
            8,
            "builder field, setter, build, new, view, env, override env, and Default must all be gated"
        );
    }
}
