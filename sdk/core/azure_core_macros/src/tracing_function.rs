// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::quote;
use syn::{spanned::Spanned, ItemFn, Result};

const INVALID_PUBLIC_FUNCTION_MESSAGE: &str =
    "function attribute must be applied to a public function returning a Result.";

// cspell: ignore asyncness

/// Parse the token stream for an Azure Service client "new" declaration.
///
/// An Azure Service client "new" declaration is a public function whose name starts with
/// `new` and returns either a new client instance or an error.
///
/// This macro will ensure that the fn is public and returns one of the following:
/// 1) `Self`
/// 1) `Arc<Self>`
/// 1) `Result<Self, E>`
/// 1) `Result<Arc<Self>, E>`
///
pub fn parse_function(_attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    if !is_function_declaration(&item) {
        return Err(syn::Error::new(
            item.span(),
            INVALID_PUBLIC_FUNCTION_MESSAGE,
        ));
    }

    let client_fn: ItemFn = syn::parse2(item.clone())?;

    let vis = &client_fn.vis;
    let asyncness = &client_fn.sig.asyncness;
    let ident = &client_fn.sig.ident;
    let inputs = client_fn.sig.inputs.iter();
    let body = client_fn.block.stmts.iter();
    let output = &client_fn.sig.output;

    Ok(quote! {
        #vis #asyncness
        fn #ident(#(#inputs),*) #output {
            let mut options = options.unwrap_or_default();
            let mut ctx = options.method_options.context.clone();
            let span =  if ctx.value::<std::sync::Arc<dyn azure_core::tracing::Span>>().is_none() {
            if let Some(tracer) = &self.tracer {
                let mut attributes = Vec::new();
                if let Some(namespace) = tracer.namespace() {
                    // If the tracer has a namespace, we set it as an attribute.
                    attributes.push(azure_core::tracing::Attribute {
                        key: "az.namespace",
                        value: namespace.into(),
                    });
                }
                let span = tracer.start_span(
                    stringify!(#ident),
                    azure_core::tracing::SpanKind::Internal,
                    attributes,
                );
                ctx = ctx.with_value(span.clone());
                ctx = ctx.with_value(tracer.clone());
                Some(span)
            } else {
                None
            }
        } else {
            None
        };
        options.method_options.context = ctx;
        let options = Some(options);
        #(#body)*
    }
    })
}

fn is_function_declaration(item: &TokenStream) -> bool {
    let item_fn: ItemFn = match syn::parse2(item.clone()) {
        Ok(fn_item) => fn_item,
        Err(_) => return false,
    };

    // Function must be public.
    if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
        return false;
    }

    // Function must return a Result type.
    if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
        if !matches!(ty.as_ref(), syn::Type::Path(_)) {
            return false;
        }
    } else {
        return false;
    }

    true
}
