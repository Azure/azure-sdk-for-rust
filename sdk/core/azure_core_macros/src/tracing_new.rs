// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, spanned::Spanned, AngleBracketedGenericArguments, ExprStruct, ItemFn, Result,
};
use tracing::{trace, warn};

const INVALID_SERVICE_CLIENT_NEW_MESSAGE: &str =
    "new attribute must be applied to a public function which returns Self, a Result and/or Arc containing Self";

struct NamespaceAttribute {
    client_namespace: String,
}

impl Parse for NamespaceAttribute {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let client_namespace = input.parse::<syn::LitStr>()?.value();
        Ok(NamespaceAttribute { client_namespace })
    }
}

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
                            option_env!("CARGO_PKG_VERSION"),
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

fn is_arc_new_call(func: &syn::Expr) -> bool {
    if let syn::Expr::Path(path) = func {
        if path.path.segments.len() < 2 {
            return false;
        }
        if path.path.segments[path.path.segments.len() - 2].ident != "Arc" {
            return false;
        }
        if path.path.segments.last().unwrap().ident != "new" {
            return false;
        }
        return true;
    }
    false
}

// Parse a function call expression statement that initializes a struct with `Arc::new(Self {})` or `Ok(Arc::new(Self {}))`.
fn parse_call_expr(namespace: &str, call: &syn::ExprCall) -> TokenStream {
    debug_assert_eq!(
        call.args.len(),
        1,
        "Call expression must have exactly one argument"
    );
    if let syn::Expr::Path(path) = call.func.as_ref() {
        if path.path.segments.last().unwrap().ident == "Ok" {
            match call.args.first().unwrap() {
                syn::Expr::Struct(struct_body) => {
                    parse_struct_expr(namespace, struct_body, call.to_token_stream(), true)
                }
                syn::Expr::Call(call) => {
                    // Let's make sure that we're doing a call to Arc::new before we recurse.
                    // Arc::new takes only a single argument, so we can check that first.
                    if call.args.len() != 1 {
                        trace!("Call expression does not have exactly one argument, emitting expression: {call:?}");
                        return call.to_token_stream();
                    }
                    if is_arc_new_call(call.func.as_ref()) {
                        let call_expr = parse_call_expr(namespace, call);
                        quote!(Ok(#call_expr))
                    } else {
                        trace!("Call expression is not Arc::new(), emitting expression: {call:?}");
                        call.to_token_stream()
                    }
                }
                _ => {
                    trace!(
                        "Call expression is not a struct or call, emitting expression: {call:?}"
                    );
                    call.to_token_stream()
                }
            }
        } else if is_arc_new_call(call.func.as_ref()) {
            if let syn::Expr::Struct(struct_body) = call.args.first().unwrap() {
                let struct_expr =
                    parse_struct_expr(namespace, struct_body, call.to_token_stream(), false);
                quote! {
                    Arc::new(#struct_expr)
                }
            } else {
                trace!("Call expression is not a struct, emitting expression: {call:?}");
                call.to_token_stream()
            }
        } else {
            trace!("Call expression is not an Arc or Ok, emitting expression: {call:?}");
            call.to_token_stream()
        }
    } else {
        trace!("Call expression is not a path, emitting expression: {call:?}");
        call.to_token_stream()
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
    if let Err(reason) = is_new_declaration(&item) {
        return Err(syn::Error::new(
            item.span(),
            format!("{INVALID_SERVICE_CLIENT_NEW_MESSAGE}: {reason}"),
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

        match stmt {
            syn::Stmt::Expr(expr, _) =>
                match expr {
                    syn::Expr::Call(c) => {
                        // If the expression is a call, we need to check if it is a struct initialization.
                        if c.args.len() != 1 {
                            trace!("Call expression does not have exactly one argument, emitting statement: {stmt:?}");
                            // If the call does not have exactly one argument, just return it as is.
                             stmt.to_token_stream()
                        }
                        else {
                            parse_call_expr(namespace_attrs.client_namespace.as_str(), c)
                        }
                    }
                    syn::Expr::Struct(struct_body) => {
                        // If the expression is a struct, we need to parse it.
                        parse_struct_expr(
                            namespace_attrs.client_namespace.as_str(),
                            struct_body,
                            stmt.to_token_stream(),
                            false,
                        )
                    }
                    _ => {
                        // If the expression is not a struct or call, just return it as is (for
                        // instance an "if" statement is an expression)
                        stmt.to_token_stream()
                    }
                }
            _ => {
                // If the statement is not an expression, just return it as is.
                stmt.to_token_stream()
            }
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

fn is_arc_of_self(path: &syn::Path) -> std::result::Result<(), String> {
    let segment = path.segments.last().unwrap();
    if segment.ident != "Arc" {
        warn!("Arc must be the first segment, found {:?}", segment.ident);
        return Err("Arc must be the first segment".to_string());
    }
    if segment.arguments.is_empty() {
        warn!("Arc must have arguments, found {:?}", segment.arguments);
        return Err("Arc must have arguments".to_string());
    }
    match &segment.arguments {
        syn::PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) => {
            if args.len() != 1 {
                warn!("Arc must have one argument, found {args:?}");
                return Err("Arc must have one argument".to_string());
            }
            if let syn::GenericArgument::Type(syn::Type::Path(path)) = &args[0] {
                if path.path.is_ident("Self") {
                    Ok(())
                } else {
                    warn!("Arc argument must be Self, found {:?}", path.path);
                    Err("Arc argument must be Self".to_string())
                }
            } else {
                warn!("Arc argument must be Self, found {:?}", args[0]);
                Err("Arc argument must be Self".to_string())
            }
        }
        _ => {
            warn!("Arc arguments must be angle bracketed");
            Err("Arc arguments must be angle bracketed".to_string())
        }
    }
}

fn is_valid_arc_of_self_call(expr: &syn::Expr) -> std::result::Result<(), String> {
    if let syn::Expr::Struct(struct_body) = expr {
        if struct_body.path.is_ident("Self") {
            Ok(())
        } else {
            warn!(
                "Expected struct initialization with Self, found {:?}",
                struct_body.path
            );
            Err("expected struct initialization with Self".to_string())
        }
    } else {
        warn!("Expected call to `Arc`, found {:?}", expr);
        Err("expected last parameter to Arc to be Self".to_string())
    }
}
fn is_valid_ok_call(
    args: &syn::punctuated::Punctuated<syn::Expr, syn::token::Comma>,
) -> std::result::Result<(), String> {
    if args.len() != 1 {
        warn!("Expected call to `Ok` with one argument, found {args:?}");
        return Err("Expected call to `Ok` with one argument".to_string());
    }
    match &args[0] {
        syn::Expr::Struct(struct_body) => {
            if struct_body.path.is_ident("Self") {
                Ok(())
            } else {
                warn!(
                    "Expected struct initialization with Self, found {:?}",
                    struct_body.path
                );
                Err("expected struct initialization with Self".to_string())
            }
        }
        syn::Expr::Call(call) => {
            if call.args.len() == 1 {
                if is_arc_new_call(call.func.as_ref()) {
                    is_valid_arc_of_self_call(call.args.last().unwrap())
                } else {
                    warn!(
                        "Expected function named Arc, found {:?}",
                        call.func.as_ref()
                    );
                    Err("expected Arc path".to_string())
                }
            } else {
                warn!(
                    "Expected call to function with one argument, found {:?}",
                    args[0]
                );
                Err("expected call to Arc with one argument".to_string())
            }
        }
        _ => {
            warn!(
                "Expected a structure or call to function, found {:?}",
                args[0]
            );
            Err("expected a structure or call to function".to_string())
        }
    }
}

fn is_valid_new_body(stmts: &[syn::Stmt]) -> std::result::Result<(), String> {
    if stmts.is_empty() {
        return Err("New function body must have at least one statement".to_string());
    }
    let last_stmt = stmts.last().unwrap();
    if let syn::Stmt::Expr(expr, _) = last_stmt {
        match expr {
            syn::Expr::Struct(struct_body) => {
                if struct_body.path.is_ident("Self") {
                    Ok(())
                } else {
                    warn!(
                        "Expected struct initialization with Self, found {:?}",
                        struct_body.path
                    );
                    Err("expected struct initialization with Self".to_string())
                }
            }
            syn::Expr::Call(call) => {
                if let syn::Expr::Path(path) = call.func.as_ref() {
                    if path.path.is_ident("Ok") {
                        is_valid_ok_call(&call.args)
                    } else if is_arc_new_call(call.func.as_ref()) {
                        is_valid_arc_of_self_call(call.args.last().unwrap())
                    } else {
                        warn!("Expected call to `Ok` or `Arc`, found {:?}", path);
                        Err("expected call to `Ok` or `Arc`".to_string())
                    }
                } else {
                    warn!("expected Path, got {:?}", call.func);
                    Err("expected Path".to_string())
                }
            }
            _ => {
                warn!("Expected call or struct statement, found {:?}", last_stmt);
                Err("expected call or struct statement".to_string())
            }
        }
    } else {
        warn!("Expected expression statement, found {:?}", last_stmt);
        Err("expected final statement to be an expression".to_string())
    }
}

fn is_valid_new_return(return_type: &syn::ReturnType) -> std::result::Result<(), String> {
    match return_type {
        syn::ReturnType::Default => Err("Default return type is not allowed".to_string()),
        syn::ReturnType::Type(_, ty) => {
            let syn::Type::Path(p) = ty.as_ref() else {
                warn!("Invalid return type for new function, expected path: {ty:?}");
                return Err("expected path".to_string());
            };
            if p.path.segments.is_empty() {
                warn!("Invalid return type for new function: Path is empty");
                return Err("expected non-empty path".to_string());
            }
            if p.path.is_ident("Self") {
                Ok(())
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
                                warn!("Invalid return type for new function: Result must have one or two arguments");
                                return Err("result must have one or two arguments".to_string());
                            }
                            if let syn::GenericArgument::Type(syn::Type::Path(path)) = &args[0] {
                                if path.path.is_ident("Self") {
                                    Ok(())
                                } else {
                                    is_arc_of_self(&path.path)
                                }
                            } else {
                                warn!("Invalid return type for new function: Result first argument must be Self, found {:?}", args[0]);
                                Err("expected Self".to_string())
                            }
                        }
                        _ => {
                            warn!("Invalid return type for new function: Result arguments must be angle bracketed");
                            Err("expected angle bracketed arguments".to_string())
                        }
                    }
                } else if segment.ident == "Arc" {
                    is_arc_of_self(&p.path)
                } else {
                    Err("expected Self, Result<Self>, or Arc<Self>".to_string())
                }
            }
        }
    }
}

/// Returns true if the item at the head of the token stream is a valid service client declaration.
///
/// # Returns
/// - None if the item is a valid service new declaration.
/// - Some(String) if the item is NOT a valid service new declaration
fn is_new_declaration(item: &TokenStream) -> std::result::Result<(), String> {
    // The item must be a function declaration.
    let item_fn: ItemFn = syn::parse2(item.clone())
        .map_err(|e| format!("Failed to parse item as function declaration: {e}"))?;

    // Service clients new functions must be public.
    if !matches!(item_fn.vis, syn::Visibility::Public(_)) {
        warn!("Service client new function must be public");
        Err("`tracing::new` function must be public".to_string())
    } else {
        // Verify that this function returns a type that is either Self, Result<Self, E>, Arc<Self>, or Result<Arc<Self>, E>.
        is_valid_new_return(&item_fn.sig.output)?;
        // Look at the function body to ensure that the last statement is a struct initialization.
        is_valid_new_body(&item_fn.block.stmts)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tracing::tests::setup_tracing;

    #[test]
    fn is_new_declaration_simple_self() {
        setup_tracing();
        assert!(is_new_declaration(&quote! {pub fn new_client(a:u32)-> Self { Self {}}}).is_ok());
    }
    #[test]
    fn is_new_declaration_arc_self() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Arc<Self> { Arc::new(Self {})}}
        )
        .is_ok());
    }
    #[test]
    fn is_new_declaration_arc_self_long() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> std::sync::Arc<Self> { std::sync::Arc::new(Self {})}}
        ).is_ok());
    }
    #[test]
    fn is_new_declaration_result_self() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<Self> { Ok(Self {})}}
        )
        .is_ok());
    }
    #[test]
    fn is_new_declaration_result_self_std_result() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> std::result::Result<Self, std::convert::Infallible> { Ok(Self {})}}
        ).is_ok());
    }
    #[test]
    fn is_new_declaration_result_arc_self() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<Arc<Self>> { Ok(Arc::new(Self {}) )}}
        )
        .is_ok());
    }
    #[test]
    fn is_new_declaration_result_arc_self_long() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<std::sync::Arc<Self>> { Ok(std::sync::Arc::new(Self {}) )}}
        )
        .is_ok());
    }
    #[test]
    fn is_new_declaration_invalid_return_type() {
        setup_tracing();

        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> u64 { Ok(Arc::new(Self {}) )}}
        )
        .is_err());
    }
    #[test]
    fn is_new_declaration_result_not_self() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<u64> { Ok(Arc::new(Self {}) )}}
        )
        .is_err());
    }
    #[test]
    fn is_new_declaration_result_not_arc_self() {
        setup_tracing();
        assert!(is_new_declaration(
            &quote! {pub fn new_client(a:u32)-> Result<Arc<u64>> { Ok(Arc::new(Self {}) )}}
        )
        .is_err());
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
                                    option_env!("CARGO_PKG_VERSION"),
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
                                    option_env!("CARGO_PKG_VERSION"),
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
                                    option_env!("CARGO_PKG_VERSION"),
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
