// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, ExprStruct, ItemFn, Result};

const INVALID_SERVICE_CLIENT_NEW_MESSAGE: &str =
    "new attribute must be applied to a public function with a name starting with `new`";

fn parse_struct_expr(
    client_namespace: &str,
    struct_body: &ExprStruct,
    default: TokenStream,
    is_ok: bool,
) -> TokenStream {
    if struct_body.path.is_ident("Self") {
        let fields = struct_body.fields.iter();
        let before_self = quote! {
            let tracer =
            if let Some(tracer_options) = &options.client_options.request_instrumentation {
                tracer_options
                    .tracing_provider
                    .as_ref()
                    .map(|tracing_provider| {
                        tracing_provider.get_tracer(
                            Some(#client_namespace),
                            option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                            option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                        )
                    })
            } else {
                None
            };
        };
        if is_ok {
            quote! {
                #before_self
                Ok(Self {
                    tracer,
                    #(#fields),*,
                })
            }
        } else {
            quote! {
                #before_self
                Self {
                    tracer,
                    #(#fields),*,
                }
            }
        }
    } else {
        println!("ident is not Self, emitting expression: {default:?}");
        default
    }
}

struct NamespaceAttribute {
    client_namespace: String,
}

impl Parse for NamespaceAttribute {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let client_namespace = input.parse::<syn::LitStr>()?.value();
        Ok(NamespaceAttribute { client_namespace })
    }
}

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
pub fn parse_new(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    if !is_new_declaration(&item) {
        return Err(syn::Error::new(
            item.span(),
            INVALID_SERVICE_CLIENT_NEW_MESSAGE,
        ));
    }
    let namespace_attrs: NamespaceAttribute = syn::parse2(attr)?;

    let client_fn: ItemFn = syn::parse2(item.clone())?;

    let vis = &client_fn.vis;
    let ident = &client_fn.sig.ident;
    let inputs = client_fn.sig.inputs.iter();
    let body = client_fn.block.stmts.iter().map(|stmt| {
        // Ensure that the body of the new function initializes the `tracer` field.

        if let syn::Stmt::Expr(expr, _) = stmt {
            if let syn::Expr::Call(c) = expr {
                // If the expression is a call, we need to check if it is a struct initialization.
                if c.args.len() != 1 {
                    println!("Call expression does not have exactly one argument, emitting expression: {stmt:?}");
                    // If the call does not have exactly one argument, just return it as is.
                    quote! {#stmt}
                } else if let syn::Expr::Struct(struct_body) = &c.args[0] {
                    parse_struct_expr(namespace_attrs.client_namespace.as_str(), struct_body, stmt.to_token_stream(), true)
                } else {
                    println!("Call expression is not a struct, emitting expression: {stmt:?}");
                    // If the expression is not a struct, just return it as is.
                    stmt.to_token_stream()
                }
            } else if let syn::Expr::Struct(struct_body) = expr {
                parse_struct_expr(namespace_attrs.client_namespace.as_str(), struct_body, stmt.to_token_stream(), false)
            } else {
                // If the expression is not a struct, just return it as is.
                stmt.to_token_stream()
            }
        } else {
            stmt.to_token_stream()
        }
    });
    let output = &client_fn.sig.output;
    Ok(quote! {
        #vis
        fn #ident(#(#inputs),*) #output {
            #(#body)*
        }
    })
}

/// Returns true if the item at the head of the token stream is a valid service client declaration.
fn is_new_declaration(item: &TokenStream) -> bool {
    let item_fn: ItemFn = match syn::parse2(item.clone()) {
        Ok(fn_item) => fn_item,
        Err(_) => return false,
    };

    // Service clients new functions must be public.
    if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
        return false;
    }

    // Service clients new functions must have a name that starts with `new_`.
    if !item_fn.sig.ident.to_string().starts_with("new") {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_new_function() {
        let attr = quote!("Az.Namespace");
        let item = quote! {
            pub fn new_service_client(name: &'static str, endpoint: Url) -> Self {
                let function = newtype::new();
                println!("Function: {:?}", function);
                i = i + 1;
                let this = Self {
                    name,
                    endpoint,
                };
                Self {
                    name,
                    endpoint,
                }
            }
        };
        let actual = parse_new(attr, item).expect("Failed to parse new function declaration");
        println!("Parsed tokens: {actual}");

        let expected = quote! {
            pub fn new_service_client(name: &'static str, endpoint: Url) -> Self {
                let function = newtype::new();
                println!("Function: {:?}", function);
                i = i + 1;
                let this = Self {
                    name,
                    endpoint,
                };
                let tracer = if let Some(tracer_options) =
                    &options.client_options.request_instrumentation
                {
                    tracer_options
                        .tracing_provider
                        .as_ref()
                        .map(|tracing_provider| {
                            tracing_provider.get_tracer(
                                Some("Az.Namespace"),
                                option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                                option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                            )
                        })
                } else {
                    None
                };
                Self {
                    tracer,
                    name,
                    endpoint,
                }
            }
        };
        assert!(
            crate::tracing::tests::compare_token_stream(actual, expected),
            "Parsed tokens do not match expected tokens"
        );
    }

    #[test]
    fn parse_generated_new() {
        let attr = quote!("Az.GeneratedNamespace");
        let new_function = quote! {
            pub fn new(
                endpoint: &str,
                credential: Arc<dyn TokenCredential>,
                options: Option<SecretClientOptions>,
            ) -> Result<Self> {
                let options = options.unwrap_or_default();
                let mut endpoint = Url::parse(endpoint)?;
                if !endpoint.scheme().starts_with("http") {
                    return Err(azure_core::Error::message(
                        azure_core::error::ErrorKind::Other,
                        format!("{endpoint} must use http(s)"),
                    ));
                }
                endpoint.set_query(None);
                let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
                    credential,
                    vec!["https://vault.azure.net/.default"],
                ));
                Ok(Self {
                    endpoint,
                    api_version: options.api_version,
                    pipeline: Pipeline::new(
                        option_env!("CARGO_PKG_NAME"),
                        option_env!("CARGO_PKG_VERSION"),
                        options.client_options,
                        Vec::default(),
                        vec![auth_policy],
                    ),
                })
            }
        };
        let actual =
            parse_new(attr, new_function).expect("Failed to parse new function declaration");

        println!("Parsed tokens: {actual}");

        // I am not at all sure why the parameters to `new` are not being parsed correctly -
        // the trailing comma in the `new_function` token stream is not present.
        let expected = quote! {
            pub fn new(
                endpoint: &str,
                credential: Arc<dyn TokenCredential>,
                options: Option<SecretClientOptions>
            ) -> Result<Self> {
                let options = options.unwrap_or_default();
                let mut endpoint = Url::parse(endpoint)?;
                if !endpoint.scheme().starts_with("http") {
                    return Err(azure_core::Error::message(
                        azure_core::error::ErrorKind::Other,
                        format!("{endpoint} must use http(s)"),
                    ));
                }
                endpoint.set_query(None);
                let auth_policy: Arc<dyn Policy> = Arc::new(BearerTokenCredentialPolicy::new(
                    credential,
                    vec!["https://vault.azure.net/.default"],
                ));
                let tracer = if let Some(tracer_options) =
                    &options.client_options.request_instrumentation
                {
                    tracer_options
                        .tracing_provider
                        .as_ref()
                        .map(|tracing_provider| {
                            tracing_provider.get_tracer(
                                Some("Az.GeneratedNamespace"),
                                option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                                option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                            )
                        })
                } else {
                    None
                };
                Ok(Self {
                    tracer,
                    endpoint,
                    api_version: options.api_version,
                    pipeline: Pipeline::new(
                        option_env!("CARGO_PKG_NAME"),
                        option_env!("CARGO_PKG_VERSION"),
                        options.client_options,
                        Vec::default(),
                        vec![auth_policy],
                    ),
                })
            }
        };
        assert!(
            crate::tracing::tests::compare_token_stream(actual, expected),
            "Parsed tokens do not match expected tokens"
        );
    }
}
