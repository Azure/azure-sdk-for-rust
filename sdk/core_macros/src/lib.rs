// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod case;
mod symbol;
mod variant;

#[proc_macro_derive(Variant, attributes(variant))]
pub fn variant_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    variant::expand_derive_variant(&input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
