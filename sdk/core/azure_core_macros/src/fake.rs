// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::helpers::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn derive_server_factory_impl(ast: DeriveInput) -> Result<TokenStream> {
    let name = &ast.ident;

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote! {
        const _: () = {
            #[cfg(feature = "unstable_fake")]
            #[doc(hidden)]
            impl ::azure_core::http::fake::ServerFactory for #name {}
        };
    };
    Ok(gen)
}
