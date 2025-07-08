// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, ItemStruct, Result};

const INVALID_SERVICE_CLIENT_MESSAGE: &str = "client attribute must be applied to a public struct";

/// Parse the token stream for an Azure Service client declaration.
///
/// An Azure Service client is a public struct that represents a client for an Azure service.
///
/// This macro will ensure that the struct is public and has a `tracer` field of type `Option<azure_core::tracing::Tracer>`.
///
pub fn parse_client(_attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    if !is_client_declaration(&item) {
        return Err(syn::Error::new(item.span(), INVALID_SERVICE_CLIENT_MESSAGE));
    }

    let client_struct: ItemStruct = syn::parse2(item.clone())?;

    let vis = &client_struct.vis;
    let ident = &client_struct.ident;
    let fields = client_struct.fields.iter();
    Ok(quote! {
        #vis
        struct #ident {
            #(#fields),*,
            tracer: Option<Arc<dyn azure_core::tracing::Tracer>>,
        }
    })
}

/// Returns true if the item at the head of the token stream is a valid service client declaration.
fn is_client_declaration(item: &TokenStream) -> bool {
    let item_struct: ItemStruct = match syn::parse2(item.clone()) {
        Ok(struct_item) => struct_item,
        Err(_) => return false,
    };

    // Service clients must be public structs.
    if !matches!(item_struct.vis, syn::Visibility::Public(_)) {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    // cspell: ignore punct
    #[test]
    fn parse_service_client() {
        let attr = TokenStream::new();
        let item = quote! {
            pub struct ServiceClient {
                name: &'static str,
                endpoint: Url,
            }
        };
        let actual = parse_client(attr, item).expect("Failed to parse client declaration");
        let expected = quote! {
            pub struct ServiceClient {
                name: &'static str,
                endpoint: Url,
                tracer: Option<Arc<dyn azure_core::tracing::Tracer>>,
            }
        };
        //        println!("Parsed tokens: {:?}", tokens);
        //        println!("Expected tokens: {:?}", expected);

        assert!(
            crate::tracing::tests::compare_token_stream(actual, expected),
            "Parsed tokens do not match expected tokens"
        );
    }

    #[test]
    fn parse_not_service_client() {
        let attr = TokenStream::new();
        let item = quote! {
            fn NotServiceClient(&self, name: &'static str) -> Result<(), Box<dyn std::error::Error>> {
                Ok(())
            }
        };
        assert!(
            parse_client(attr, item).is_err(),
            "Expected error for non-client declaration"
        );
    }
}
