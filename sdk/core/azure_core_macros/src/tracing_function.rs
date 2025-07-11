// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, ItemFn, Member, Result, Token};

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
pub fn parse_function(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    if !is_function_declaration(&item) {
        println!("Not a function declaration: {item}");
        return Err(syn::Error::new(
            item.span(),
            INVALID_PUBLIC_FUNCTION_MESSAGE,
        ));
    }

    let function_name_and_attributes: FunctionNameAndAttributes = syn::parse2(attr)?;

    let api_name = function_name_and_attributes.function_name;

    let ItemFn {
        attrs,
        vis,
        sig,
        block,
    } = syn::parse2(item)?;

    let attributes: TokenStream = if function_name_and_attributes.arguments.is_empty() {
        quote! {Vec::new()}
    } else {
        let attribute_vec = function_name_and_attributes
            .arguments
            .into_iter()
            .map(|(name, value)| {
                quote! {
                    ::typespec_client_core::tracing::Attribute{key: #name, value: #value.into()}
                }
            })
            .collect::<Vec<_>>();
        quote! { vec![#(#attribute_vec),*] }
    };

    let preamble = quote! {
        let options = {
            let mut options = options.unwrap_or_default();

            let public_api_info = azure_core::tracing::PublicApiInstrumentationInformation {
                api_name: #api_name,
                attributes: #attributes,
            };
            // Add the span to the tracer.
            let mut ctx = options.method_options.context.with_value(public_api_info);
            // If the service has a tracer, we add it to the context.
            if let Some(tracer) = &self.tracer {
                ctx = ctx.with_value(tracer.clone());
            }
            options.method_options.context = ctx;
            Some(options)
        };
    };

    // Clear the actual test method parameters.
    Ok(quote! {
        #(#attrs)*
        #vis #sig {
            #preamble
            #block
        }
    })
}

#[derive(Debug)]
struct FunctionNameAndAttributes {
    function_name: String,
    arguments: Vec<(String, syn::Expr)>,
}

fn name_from_expr(expr: &syn::Expr) -> Result<String> {
    match expr {
        syn::Expr::Lit(lit) => match &lit.lit {
            syn::Lit::Str(lit_str) => Ok(lit_str.value()),
            _ => Err(syn::Error::new(lit.span(), "Unsupported literal type")),
        },
        syn::Expr::Path(expr_path) => expr_path
            .path
            .get_ident()
            .ok_or_else(|| {
                syn::Error::new(
                    expr_path.span(),
                    "Expected an identifier in path expression",
                )
            })
            .map(|ident| ident.to_string()),
        syn::Expr::Field(expr_field) => {
            // If it's a field, we can extract the base and path
            // This assumes the field is a path like `az.foo.bar.namespace`
            // and we want to extract `az.foo.bar.namespace`
            let base = name_from_expr(expr_field.base.as_ref())?;
            let member = match &expr_field.member {
                Member::Named(ident) => ident.to_string(),
                Member::Unnamed(_) => {
                    println!("Anonymous member");
                    // If it's an unnamed member, we can use the index or some other identifier
                    // Here we assume it's a named member for simplicity
                    format!("{:?}", expr_field.member.to_token_stream())
                }
            };
            Ok(format!("{base}.{member}"))
        }
        _ => Err(syn::Error::new(expr.span(), "Unsupported expression type")),
    }
}

impl Parse for FunctionNameAndAttributes {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let function_name = input.parse::<syn::LitStr>()?.value();
        // If the next character is a comma, we expect a list of attributes.
        if input.peek(Token!(,)) {
            input.parse::<Token![,]>()?;
            if input.peek(syn::token::Paren) {
                let content;
                let _ = syn::parenthesized!(content in input);
                let mut arguments: syn::punctuated::Punctuated<syn::ExprAssign, syn::token::Comma> =
                    syn::punctuated::Punctuated::new();
                if !content.is_empty() {
                    arguments = content.parse_terminated(syn::ExprAssign::parse, syn::Token![,])?;
                }
                let arguments_result = arguments
                    .into_iter()
                    .map(|arg| {
                        let syn::ExprAssign { left, right, .. } = arg;
                        let (left, right) = (left, right);
                        let name = name_from_expr(left.as_ref())?;
                        Ok((name, *right))
                    })
                    .collect::<Result<Vec<_>>>()?;

                Ok(FunctionNameAndAttributes {
                    function_name,
                    arguments: arguments_result,
                })
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected parentheses after function name.",
                ))
            }
        } else {
            Ok(FunctionNameAndAttributes {
                function_name,
                arguments: vec![],
            })
        }
    }
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

    // Function must be public.
    if item_fn.sig.asyncness.is_none() {
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

#[cfg(test)]
mod tests {
    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_parse_function_name_and_attributes() {
        let types = [
            quote! { "Text String" },
            quote! { "Text String", (a = 1, b = 2) },
        ];

        for stream in types.iter() {
            let parsed: FunctionNameAndAttributes =
                syn::parse2(stream.clone()).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Text String");
            if !parsed.arguments.is_empty() {
                assert_eq!(parsed.arguments.len(), 2);
                assert_eq!(parsed.arguments[0].0, "a");
                assert_eq!(parsed.arguments[1].0, "b");
            }
        }

        {
            let test_stream = quote! { "Test Function", (arg1 = 42, arg2 = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(parsed.arguments.len(), 2);
            assert_eq!(parsed.arguments[0].0, "arg1");
            assert_eq!(parsed.arguments[0].1, parse_quote!(42));
            assert_eq!(parsed.arguments[1].0, "arg2");
            assert_eq!(parsed.arguments[1].1, parse_quote!("value"));
        }
        {
            let test_stream = quote! { "Test Function", ("az.namespace" = "my namespace", az.test_value = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(parsed.arguments.len(), 2);
            assert_eq!(parsed.arguments[0].0, "az.namespace");
            assert_eq!(parsed.arguments[0].1, parse_quote!("my namespace"));
            assert_eq!(parsed.arguments[1].0, "az.test_value");
            assert_eq!(parsed.arguments[1].1, parse_quote!("value"));
        }
        {
            let test_stream = quote! { "Test Function", (az.namespace = "my namespace", az.test_value = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(parsed.arguments.len(), 2);
            assert_eq!(parsed.arguments[0].0, "az.namespace");
            assert_eq!(parsed.arguments[0].1, parse_quote!("my namespace"));
            assert_eq!(parsed.arguments[1].0, "az.test_value");
            assert_eq!(parsed.arguments[1].1, parse_quote!("value"));
        }
        {
            let test_stream = quote! {"macros_get_with_tracing", (az.path = path, az.info = "Test", az.number = 42)};
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "macros_get_with_tracing");
            assert_eq!(parsed.arguments.len(), 3);
            assert_eq!(parsed.arguments[0].0, "az.path");
            assert_eq!(parsed.arguments[0].1, parse_quote!(path));

            assert_eq!(parsed.arguments[1].0, "az.info");
            assert_eq!(parsed.arguments[1].1, parse_quote!("Test"));

            assert_eq!(parsed.arguments[2].0, "az.number");
            assert_eq!(parsed.arguments[2].1, parse_quote!(42));
        }
        {
            let test_stream = quote! { "Test Function", (az.foo.bar.namespace = "my namespace", az.test_value = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(parsed.arguments.len(), 2);
            assert_eq!(parsed.arguments[0].0, "az.foo.bar.namespace");
            assert_eq!(parsed.arguments[0].1, parse_quote!("my namespace"));
            assert_eq!(parsed.arguments[1].0, "az.test_value");
            assert_eq!(parsed.arguments[1].1, parse_quote!("value"));
        }

        {
            let test_stream = quote! { "Test Function", };

            syn::parse2::<FunctionNameAndAttributes>(test_stream)
                .expect_err("Should fail to parse.");
        }
        {
            let test_stream = quote! { "Test Function",(23.5= "value") };

            syn::parse2::<FunctionNameAndAttributes>(test_stream)
                .expect_err("Should fail to parse.");
        }
        {
            let test_stream = quote! { "Test Function", ()};

            syn::parse2::<FunctionNameAndAttributes>(test_stream).expect("No attributes are ok.");
        }
    }

    #[test]
    fn test_is_function_declaration() {
        let valid_fn = quote! {
            pub async fn my_function() -> Result<(), Box<dyn std::error::Error>> {
            }
        };
        let invalid_fn = quote! {
            pub fn my_function() -> Result<(), Box<dyn std::error::Error>> {
            }
        };

        assert!(is_function_declaration(&valid_fn));
        assert!(!is_function_declaration(&invalid_fn));
    }

    #[test]
    fn test_parse_function() -> std::result::Result<(), syn::Error> {
        let attr = quote! { "TestFunction" };
        let item = quote! {
            pub async fn my_function(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
                let options = options.unwrap_or_default();

                let mut url = self.endpoint.clone();
                url.set_path(path);
                url.query_pairs_mut()
                    .append_pair("api-version", &self.api_version);

                let mut request = Request::new(url, azure_core::http::Method::Get);

                let response = self
                    .pipeline
                    .send(&options.method_options.context, &mut request)
                    .await?;
                if !response.status().is_success() {
                    return Err(azure_core::Error::message(
                        azure_core::error::ErrorKind::HttpResponse {
                            status: response.status(),
                            error_code: None,
                        },
                        format!("Failed to GET {}: {}", request.url(), response.status()),
                    ));
                }
                Ok(response)
            }
        };

        let actual = parse_function(attr, item)?;
        let expected = quote! {
            pub async fn my_function() -> Result<(), Box<dyn std::error::Error>> {
                let mut options = None;
                let result = tracing::function("TestFunction", options);
                Ok(())
            }
        };

        println!("Parsed tokens: {:?}", actual.to_string());
        println!("Expected tokens: {:?}", expected.to_string());

        assert!(
            crate::tracing::tests::compare_token_stream(actual, expected),
            "Parsed tokens do not match expected tokens"
        );
        Ok(())
    }
}
