// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::Result;
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn derive_safe_debug_impl(ast: DeriveInput) -> Result<TokenStream> {
    let body = generate_body(ast)?;

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote::quote! {
        #[doc(hidden)]
        const _: () = {
            #body
        };
    };
    Ok(gen)
}

fn generate_body(ast: DeriveInput) -> Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    Ok(quote::quote! {
        #[automatically_derived]
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.debug_struct(stringify!(#name)).finish_non_exhaustive()
            }
        }

        #[automatically_derived]
        impl #impl_generics ::typespec_client_core::fmt::SafeDebug for #name #ty_generics #where_clause {}
    })
}
