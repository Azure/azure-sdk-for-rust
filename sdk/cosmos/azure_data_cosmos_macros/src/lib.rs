// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![warn(missing_docs)]

//! Procedural macros for the Azure Cosmos DB SDK hierarchical configuration model.

mod builder;
mod env;
mod parse;
mod view;

use parse::OptionsInput;
use syn::{parse_macro_input, DeriveInput};

type Result<T> = ::std::result::Result<T, syn::Error>;

/// Derives layered configuration boilerplate for Cosmos DB option group structs.
///
/// Generates a View struct, a Builder type, a `from_env()` constructor, and a
/// `Default` implementation.
///
/// # Struct-Level Attributes
///
/// - `#[options(layers(runtime, account, operation))]` — declares which layers
///   this option group participates in.
///
/// # Field-Level Attributes
///
/// - `#[option(env = "AZURE_COSMOS_...")]` — enables environment variable loading.
/// - `#[option(merge = "extend")]` — uses additive merge instead of shadow semantics.
/// - `#[option(nested)]` — delegates resolution to a child View.
///
/// # Example
///
/// ```ignore
/// #[derive(CosmosOptions)]
/// #[options(layers(runtime, account, operation))]
/// pub struct RequestOptions {
///     #[option(env = "AZURE_COSMOS_CONSISTENCY_LEVEL")]
///     pub consistency_level: Option<ConsistencyLevel>,
///
///     pub throughput_bucket: Option<usize>,
///
///     #[option(merge = "extend")]
///     pub custom_headers: Option<HashMap<HeaderName, HeaderValue>>,
/// }
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
        let field_name = &f.ident;
        quote::quote! { #field_name: None }
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
