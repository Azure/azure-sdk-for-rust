// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, spanned::Spanned, AngleBracketedGenericArguments, ExprStruct, ItemFn, Result,
};
use tracing::trace;

const INVALID_SERVICE_CLIENT_NEW_MESSAGE: &str =
    "new attribute must be applied to a public function which returns Self, a Result and/or Arc containing Self";

fn parse_struct_expr(
    client_namespace: &str,
    struct_body: &ExprStruct,
    default: TokenStream,
    is_ok: bool,
) -> TokenStream {
    if struct_body.path.is_ident("Self") {
        let fields = struct_body.fields.iter();
        let tracer_init = quote! {
            if let Some(tracer_options) = &options.client_options.request_instrumentation {
                tracer_options
                    .tracer_provider
                    .as_ref()
                    .map(|tracer_provider| {
                        tracer_provider.get_tracer(
                            Some(#client_namespace),
                            option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                            option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                        )
                    })
            } else {
                None
            }
        };
        if is_ok {
            quote! {
                Ok(Self {
                    tracer: #tracer_init,
                    #(#fields),*,
                })
            }
        } else {
            quote! {
                Self {
                    tracer: #tracer_init,
                    #(#fields),*,
                }
            }
        }
    } else {
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

    let ItemFn {
        vis,
        sig,
        block,
        attrs,
    } = syn::parse2(item.clone())?;

    let ident = &sig.ident;
    let inputs = sig.inputs.iter();
    let body =block.stmts.iter().map(|stmt| {
        // Ensure that the body of the new function initializes the `tracer` field.

        if let syn::Stmt::Expr(expr, _) = stmt {
            if let syn::Expr::Call(c) = expr {
                // If the expression is a call, we need to check if it is a struct initialization.
                if c.args.len() != 1 {
                    trace!("Call expression does not have exactly one argument, emitting expression: {stmt:?}");
                    // If the call does not have exactly one argument, just return it as is.
                    stmt.to_token_stream()
                } else if let syn::Expr::Struct(struct_body) = &c.args[0] {
                    parse_struct_expr(namespace_attrs.client_namespace.as_str(), struct_body, stmt.to_token_stream(), true)
                } else {
                    trace!("Call expression is not a struct, emitting expression: {stmt:?}");
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
    let output = &sig.output;
    Ok(quote! {
        #(#attrs)*
        #vis
        fn #ident(#(#inputs),*) #output {
            #(#body)*
        }
    })
}

fn is_arc_of_self(path: &syn::Path) -> bool {
    let segment = path.segments.last().unwrap();
    if segment.ident != "Arc" {
        eprintln!(
            "Invalid return type for new function: Arc must be the first segment, found {:?}",
            segment.ident
        );
        return false;
    }
    if segment.arguments.is_empty() {
        eprintln!(
            "Invalid return type for new function: Arc must have arguments, found {:?}",
            segment.arguments
        );
        return false;
    }
    match &segment.arguments {
        syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            if args.len() != 1 {
                eprintln!(
                    "Invalid return type for new function: Arc must have one argument, found {args:?}",
                );
                return false;
            }
            if let syn::GenericArgument::Type(syn::Type::Path(path)) = &args[0] {
                path.path.is_ident("Self")
            } else {
                eprintln!(
                    "Invalid return type for new function: Arc argument must be Self, found {:?}",
                    args[0]
                );
                false
            }
        }
        _ => {
            eprintln!(
                "Invalid return type for new function: Arc arguments must be angle bracketed"
            );
            false
        }
    }
}
fn is_valid_new_return(return_type: &syn::ReturnType) -> bool {
    match return_type {
        syn::ReturnType::Default => false,
        syn::ReturnType::Type(_, ty) => {
            let syn::Type::Path(p) = ty.as_ref() else {
                println!("Invalid return type for new function, expected path: {ty:?}");
                return false;
            };
            if p.path.segments.is_empty() {
                println!("Invalid return type for new function: Path is empty");
                return false;
            }
            if p.path.is_ident("Self") {
                true
            } else {
                // segments.last to allow for std::arc::Arc or azure_core::Result
                let segment = p.path.segments.last().unwrap();

                if segment.ident == "Result" {
                    match &segment.arguments {
                        syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            args,
                            ..
                        }) => {
                            if args.len() != 1 && args.len() != 2 {
                                eprintln!("Invalid return type for new function: Result must have one or two arguments");
                                return false;
                            }
                            if let syn::GenericArgument::Type(syn::Type::Path(path)) = &args[0] {
                                if path.path.is_ident("Self") {
                                    true
                                } else {
                                    is_arc_of_self(&path.path)
                                }
                            } else {
                                eprintln!("Invalid return type for new function: Result first argument must be Self, found {:?}", args[0]);
                                false
                            }
                        }
                        _ => {
                            eprintln!("Invalid return type for new function: Result arguments must be angle bracketed");
                            false
                        }
                    }
                } else if segment.ident == "Arc" {
                    is_arc_of_self(&p.path)
                } else {
                    false
                }
            }
        }
    }
}

/// Returns true if the item at the head of the token stream is a valid service client declaration.
fn is_new_declaration(item: &TokenStream) -> bool {
    // The item must be a function declaration.
    let item_fn: ItemFn = match syn::parse2(item.clone()) {
        Ok(fn_item) => fn_item,
        Err(e) => {
            eprintln!("could not parse new: {e}");
            return false;
        }
    };

    // Service clients new functions must be public.
    if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
        eprintln!("Service client new function must be public");
        return false;
    }

    // Verify that this function returns a type that is either Self, Result<Self, E>, Arc<Self>, or Result<Arc<Self>, E>.

    if !is_valid_new_return(&item_fn.sig.output) {
        eprintln!(
            "Invalid return type for new function: {:?}",
            item_fn.sig.output
        );
        return false;
    }
    // Look at the function body to ensure that the last statement is a struct initialization.

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::tests::setup_tracing;

    #[test]
    fn is_new_declaration_valid() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Self { Self {}}}
        ));
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Arc<Self> { Arc::new(Self {})}}
        ));
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> std::sync::Arc<Self> { std::sync::Arc::new(Self {})}}
        ));
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<Self> { Ok(Self {})}}
        ));
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> std::result::Result<Self, std::convert::Infallible> { Ok(Self {})}}
        ));
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<Arc<Self>> { Ok(Arc::new(Self {}) )}}
        ));

        assert!(!is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> u64 { Ok(Arc::new(Self {}) )}}
        ));
        assert!(!is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<u64> { Ok(Arc::new(Self {}) )}}
        ));
        assert!(!is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<Arc<u64>> { Ok(Arc::new(Self {}) )}}
        ));
    }

    #[test]
    fn parse_new_function() {
        setup_tracing();
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

                Self {
                    tracer: if let Some(tracer_options) =
                        &options.client_options.request_instrumentation
                    {
                        tracer_options
                            .tracer_provider
                            .as_ref()
                            .map(|tracer_provider| {
                                tracer_provider.get_tracer(
                                    Some("Az.Namespace"),
                                    option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                                    option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                                )
                            })
                    } else {
                        None
                    },
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
        setup_tracing();
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
                Ok(Self {
                    tracer: if let Some(tracer_options) =
                        &options.client_options.request_instrumentation
                    {
                        tracer_options
                            .tracer_provider
                            .as_ref()
                            .map(|tracer_provider| {
                                tracer_provider.get_tracer(
                                    Some("Az.GeneratedNamespace"),
                                    option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                                    option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                                )
                            })
                    } else {
                        None
                    },
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

    #[test]
    fn parse_arc_new() {
        setup_tracing();
        let attr = quote!("Az.GeneratedNamespace");
        let new_function = quote! {
            pub fn new(
                endpoint: &str,
                credential: Arc<dyn TokenCredential>,
                options: Option<SecretClientOptions>,
            ) -> Result<Arc<Self>> {
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
                Ok(Arc::new(Self {
                    endpoint,
                    api_version: options.api_version,
                    pipeline: Pipeline::new(
                        option_env!("CARGO_PKG_NAME"),
                        option_env!("CARGO_PKG_VERSION"),
                        options.client_options,
                        Vec::default(),
                        vec![auth_policy],
                    ),
                }))
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
            ) -> Result<Arc<Self>> {
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
                Ok(Arc::new(Self {
                    tracer: if let Some(tracer_options) =
                        &options.client_options.request_instrumentation
                    {
                        tracer_options
                            .tracer_provider
                            .as_ref()
                            .map(|tracer_provider| {
                                tracer_provider.get_tracer(
                                    Some("Az.GeneratedNamespace"),
                                    option_env!("CARGO_PKG_NAME").unwrap_or("UNKNOWN"),
                                    option_env!("CARGO_PKG_VERSION").unwrap_or("UNKNOWN"),
                                )
                            })
                    } else {
                        None
                    },
                    endpoint,
                    api_version: options.api_version,
                    pipeline: Pipeline::new(
                        option_env!("CARGO_PKG_NAME"),
                        option_env!("CARGO_PKG_VERSION"),
                        options.client_options,
                        Vec::default(),
                        vec![auth_policy],
                    ),
                }))
            }
        };
        assert!(
            crate::tracing::tests::compare_token_stream(actual, expected),
            "Parsed tokens do not match expected tokens"
        );
    }
}
