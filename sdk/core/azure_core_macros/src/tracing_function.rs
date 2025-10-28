// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, spanned::Spanned, ItemFn, Member, Result, Token};

const INVALID_PUBLIC_FUNCTION_MESSAGE: &str =
    "function attribute must be applied to a public function returning a Result";

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
    if let Err(e) = is_function_declaration(&item) {
        println!("Not a function declaration: {item}");
        return Err(syn::Error::new(
            item.span(),
            format!("{INVALID_PUBLIC_FUNCTION_MESSAGE}: {e}"),
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
                    ::typespec_client_core::tracing::Attribute{key: #name.into(), value: #value.into()}
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
            if input.peek(syn::Ident) {
                let ident = input.parse::<syn::Ident>()?;
                if ident == "attributes" {
                    if input.peek(Token![=]) {
                        input.parse::<Token![=]>()?;
                    } else {
                        return Err(syn::Error::new(
                            ident.span(),
                            "Expected '=' after 'attributes'",
                        ));
                    }
                    if input.peek(syn::token::Paren) {
                        let content;
                        let _ = syn::parenthesized!(content in input);
                        let mut arguments: syn::punctuated::Punctuated<
                            syn::ExprAssign,
                            syn::token::Comma,
                        > = syn::punctuated::Punctuated::new();
                        if !content.is_empty() {
                            arguments =
                                content.parse_terminated(syn::ExprAssign::parse, syn::Token![,])?;
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
                    Err(syn::Error::new(
                        ident.span(),
                        "Expected 'attributes' identifier",
                    ))
                }
            } else {
                Err(syn::Error::new(
                    input.span(),
                    "Expected identifier after comma",
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

fn is_function_declaration(item: &TokenStream) -> std::result::Result<(), String> {
    let item_fn: ItemFn = match syn::parse2(item.clone()) {
        Ok(fn_item) => fn_item,
        Err(e) => {
            return Err(format!("Failed to parse function declaration: {e}"));
        }
    };

    // Function must return a Result type.
    if let syn::ReturnType::Type(_, ty) = &item_fn.sig.output {
        if !matches!(ty.as_ref(), syn::Type::Path(_)) {
            return Err("Function must return a Result type".into());
        }
    } else {
        return Err("Function must return a Result type".into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::vec;

    use syn::parse_quote;

    use super::*;

    #[test]
    fn test_parse_function_name_and_attributes() {
        {
            let test_stream = quote! { "Test Function", attributes = (arg1 = 42, arg2 = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(
                parsed.arguments,
                vec![
                    ("arg1".to_string(), parse_quote!(42)),
                    ("arg2".to_string(), parse_quote!("value"))
                ]
            );
        }
    }

    #[test]
    fn parse_function_without_attributes() {
        {
            let test_stream = quote! { "Test Function", () };
            syn::parse2::<FunctionNameAndAttributes>(test_stream).expect_err("Failed to parse");
        }
    }

    #[test]
    fn parse_function_without_equals() {
        {
            let test_stream = quote! { "Test Function", attributes() };
            syn::parse2::<FunctionNameAndAttributes>(test_stream).expect_err("Failed to parse");
        }
    }

    #[test]
    fn test_parse_function_name_and_attributes_with_string_name() {
        {
            let test_stream = quote! { "Test Function", attributes = (az.namespace = "my namespace", az.test_value = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(
                parsed.arguments,
                vec![
                    ("az.namespace".to_string(), parse_quote!("my namespace")),
                    ("az.test_value".to_string(), parse_quote!("value")),
                ]
            );
        }
    }
    #[test]
    fn test_parse_function_name_and_attributes_with_dotted_name() {
        {
            let test_stream = quote! { "Test Function", attributes = (az.namespace = "my namespace", az.test_value = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(
                parsed.arguments,
                vec![
                    ("az.namespace".to_string(), parse_quote!("my namespace")),
                    ("az.test_value".to_string(), parse_quote!("value"))
                ]
            );
        }
    }
    #[test]
    fn test_parse_function_name_and_attributes_with_identifier_argument() {
        {
            let test_stream = quote! {"macros_get_with_tracing", attributes = (az.path = path, az.info = "Test", az.number = 42)};
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "macros_get_with_tracing");
            assert_eq!(
                parsed.arguments,
                vec![
                    ("az.path".to_string(), parse_quote!(path)),
                    ("az.info".to_string(), parse_quote!("Test")),
                    ("az.number".to_string(), parse_quote!(42)),
                ]
            );
        }
    }
    #[test]
    fn test_parse_function_name_and_attributes_with_identifier_name() {
        {
            let test_stream = quote! { "Test Function", attributes = (az.foo.bar.namespace = "my namespace", az.test_value = "value") };
            let parsed: FunctionNameAndAttributes =
                syn::parse2(test_stream).expect("Failed to parse");
            assert_eq!(parsed.function_name, "Test Function");
            assert_eq!(
                parsed.arguments,
                vec![
                    (
                        "az.foo.bar.namespace".to_string(),
                        parse_quote!("my namespace")
                    ),
                    ("az.test_value".to_string(), parse_quote!("value"))
                ]
            );
        }
    }
    #[test]
    fn test_parse_function_name_and_attributes_with_comma_no_attributes() {
        {
            let test_stream = quote! { "Test Function", };

            syn::parse2::<FunctionNameAndAttributes>(test_stream)
                .expect_err("Should fail to parse.");
        }
    }

    #[test]
    fn test_parse_function_name_and_attributes_invalid_attribute_name() {
        {
            let test_stream = quote! { "Test Function", attributes = (23.5= "value") };

            syn::parse2::<FunctionNameAndAttributes>(test_stream)
                .expect_err("Should fail to parse.");
        }
    }
    #[test]
    fn test_parse_function_name_and_attributes_empty_attributes() {
        {
            let test_stream = quote! { "Test Function", attributes  = ()};

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
            pub fn my_function() {
            }
        };

        assert!(is_function_declaration(&valid_fn).is_ok());
        assert!(is_function_declaration(&invalid_fn).is_err());
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
        pub async fn my_function(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
            let options = {
                let mut options = options.unwrap_or_default();
                let public_api_info = azure_core::tracing::PublicApiInstrumentationInformation {
                    api_name: "TestFunction",
                    attributes: Vec::new(),
                };
                let mut ctx = options.method_options.context.with_value(public_api_info);
                if let Some(tracer) = &self.tracer {
                    ctx = ctx.with_value(tracer.clone());
                }
                options.method_options.context = ctx;
                Some(options)
            };
            {
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

    // cspell: ignore deletedsecrets
    #[test]
    fn test_parse_pageable_function() -> std::result::Result<(), syn::Error> {
        let attr = quote! { "TestFunction" };
        let item = quote! {
        pub fn list_deleted_secret_properties(
            &self,
            options: Option<SecretClientListDeletedSecretPropertiesOptions<'_>>,
        ) -> Result<Pager<ListDeletedSecretPropertiesResult>> {
            let options = options.unwrap_or_default().into_owned();
            let pipeline = self.pipeline.clone();
            let mut first_url = self.endpoint.clone();
            first_url = first_url.join("deletedsecrets")?;
            first_url
                .query_pairs_mut()
                .append_pair("api-version", &self.api_version);
            if let Some(maxresults) = options.maxresults {
                first_url
                    .query_pairs_mut()
                    .append_pair("maxresults", &maxresults.to_string());
            }
            let api_version = self.api_version.clone();
            Ok(Pager::from_callback(move |next_link: Option<Url>| {
                let url = match next_link {
                    Some(next_link) => {
                        let qp = next_link
                            .query_pairs()
                            .filter(|(name, _)| name.ne("api-version"));
                        let mut next_link = next_link.clone();
                        next_link
                            .query_pairs_mut()
                            .clear()
                            .extend_pairs(qp)
                            .append_pair("api-version", &api_version);
                        next_link
                    }
                    None => first_url.clone(),
                };
                let mut request = Request::new(url, Method::Get);
                request.insert_header("accept", "application/json");
                let ctx = options.method_options.context.clone();
                let pipeline = pipeline.clone();
                async move {
                    let rsp = pipeline.send(&ctx, &mut request).await?;
                    let (status, headers, body) = rsp.deconstruct();
                    let bytes = body.collect().await?;
                    let res: ListDeletedSecretPropertiesResult = json::from_json(&bytes)?;
                    let rsp = BufResponse::from_bytes(status, headers, bytes).into();
                    Ok(match res.next_link {
                        Some(next_link) if !next_link.is_empty() => PagerResult::More {
                            response: rsp,
                            continuation: next_link.parse()?,
                        },
                        _ => PagerResult::Done { response: rsp },
                    })
                }
            }))
        }
        };

        let actual = parse_function(attr, item)?;
        let expected = quote! {
        pub fn list_deleted_secret_properties(
            &self,
            options: Option<SecretClientListDeletedSecretPropertiesOptions<'_>>,
        ) -> Result<Pager<ListDeletedSecretPropertiesResult>> {
            let options = {
                let mut options = options.unwrap_or_default();
                let public_api_info = azure_core::tracing::PublicApiInstrumentationInformation {
                    api_name: "TestFunction",
                    attributes: Vec::new(),
                };
                let mut ctx = options.method_options.context.with_value(public_api_info);
                if let Some(tracer) = &self.tracer {
                    ctx = ctx.with_value(tracer.clone());
                }
                options.method_options.context = ctx;
                Some(options)
            };
            {
               let options = options.unwrap_or_default().into_owned();
                let pipeline = self.pipeline.clone();
                let mut first_url = self.endpoint.clone();
                first_url = first_url.join("deletedsecrets")?;
                first_url
                    .query_pairs_mut()
                    .append_pair("api-version", &self.api_version);
                if let Some(maxresults) = options.maxresults {
                    first_url
                        .query_pairs_mut()
                        .append_pair("maxresults", &maxresults.to_string());
                }
                let api_version = self.api_version.clone();
                Ok(Pager::from_callback(move |next_link: Option<Url>| {
                    let url = match next_link {
                        Some(next_link) => {
                            let qp = next_link
                                .query_pairs()
                                .filter(|(name, _)| name.ne("api-version"));
                            let mut next_link = next_link.clone();
                            next_link
                                .query_pairs_mut()
                                .clear()
                                .extend_pairs(qp)
                                .append_pair("api-version", &api_version);
                            next_link
                        }
                        None => first_url.clone(),
                    };
                    let mut request = Request::new(url, Method::Get);
                    request.insert_header("accept", "application/json");
                    let ctx = options.method_options.context.clone();
                    let pipeline = pipeline.clone();
                    async move {
                        let rsp = pipeline.send(&ctx, &mut request).await?;
                        let (status, headers, body) = rsp.deconstruct();
                        let bytes = body.collect().await?;
                        let res: ListDeletedSecretPropertiesResult = json::from_json(&bytes)?;
                        let rsp = BufResponse::from_bytes(status, headers, bytes).into();
                        Ok(match res.next_link {
                            Some(next_link) if !next_link.is_empty() => PagerResult::More {
                                response: rsp,
                                continuation: next_link.parse()?,
                            },
                            _ => PagerResult::Done { response: rsp },
                        })
                    }
                }))
            }
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
