// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::test::TestMode;
use proc_macro2::TokenStream;
use quote::quote;
use std::sync::LazyLock;
use syn::{parse::Parse, spanned::Spanned, FnArg, ItemFn, Meta, PatType, Result, Token};

const INVALID_RECORDED_ATTRIBUTE_MESSAGE: &str =
    "expected `#[recorded::test]` or `#[recorded::test(live)]`";
const INVALID_RECORDED_FUNCTION_MESSAGE: &str =
    "expected `async fn(TestContext)` function signature with optional `Result<T, E>` return";

// cspell:ignore asyncness
pub fn parse_test(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let recorded_attrs: Attributes = syn::parse2(attr)?;
    let ItemFn {
        attrs,
        vis,
        sig: original_sig,
        block,
    } = syn::parse2(item)?;

    let mut test_attr: TokenStream = match original_sig.asyncness {
        Some(_) => quote! { #[::tokio::test] },
        None => {
            return Err(syn::Error::new(
                original_sig.span(),
                INVALID_RECORDED_FUNCTION_MESSAGE,
            ))
        }
    };

    // Ignore live-only tests if not running live tests.
    let test_mode = *TEST_MODE;
    if recorded_attrs.live && test_mode < TestMode::Live {
        test_attr.extend(quote! {
            #[ignore = "skipping live tests"]
        });
    }

    let fn_name = &original_sig.ident;
    let mut inputs = original_sig.inputs.iter();
    let setup = match inputs.next() {
        None if recorded_attrs.live => quote! {
            #fn_name().await
        },
        Some(FnArg::Typed(PatType { ty, .. })) if is_test_context(ty.as_ref()) => {
            let test_mode = test_mode_to_tokens(test_mode);
            quote! {
                #[allow(dead_code)]
                let ctx = ::azure_core_test::TestContext::new(#test_mode, env!("CARGO_MANIFEST_DIR"), stringify!(#fn_name));
                let _session = ::azure_core_test::recorded::start(&ctx, ::std::option::Option::None).await?;
                #fn_name(ctx).await
            }
        }
        _ => {
            return Err(syn::Error::new(
                original_sig.ident.span(),
                INVALID_RECORDED_FUNCTION_MESSAGE,
            ))
        }
    };

    if let Some(arg) = inputs.next() {
        return Err(syn::Error::new(
            arg.span(),
            format!("too many parameters; {INVALID_RECORDED_FUNCTION_MESSAGE}"),
        ));
    }

    // Clear the actual test method parameters.
    let mut outer_sig = original_sig.clone();
    outer_sig.inputs.clear();

    Ok(quote! {
        #[cfg(not(target_arch = "wasm32"))]
        #test_attr
        #(#attrs)*
        #vis #outer_sig {
            #original_sig {
                #block
            }
            #setup
        }
    })
}

static TEST_MODE: LazyLock<TestMode> = LazyLock::new(|| {
    // Okay to panic if AZURE_TEST_MODE is unsupported.
    TestMode::current().unwrap()
});

#[derive(Debug, Default)]
struct Attributes {
    live: bool,
}

impl Parse for Attributes {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut attrs = Self::default();
        for arg in input.parse_terminated(Meta::parse, Token![,])? {
            match &arg {
                Meta::Path(path) => {
                    let ident = path.get_ident().ok_or_else(|| {
                        syn::Error::new(arg.span(), INVALID_RECORDED_ATTRIBUTE_MESSAGE)
                    })?;
                    match ident.to_string().as_str() {
                        "live" => attrs.live = true,
                        _ => {
                            return Err(syn::Error::new(
                                arg.span(),
                                INVALID_RECORDED_ATTRIBUTE_MESSAGE,
                            ))
                        }
                    }
                }
                _ => {
                    return Err(syn::Error::new(
                        arg.span(),
                        INVALID_RECORDED_ATTRIBUTE_MESSAGE,
                    ))
                }
            }
        }

        Ok(attrs)
    }
}

fn is_test_context(arg: &syn::Type) -> bool {
    let path = match arg {
        syn::Type::Path(syn::TypePath { path, .. }) => path,
        _ => return false,
    };

    if path.leading_colon.is_none()
        && path.segments.len() == 1
        && path.segments[0].ident == "TestContext"
    {
        return true;
    }

    path.segments.len() == 2
        && path.segments[0].ident == "azure_core_test"
        && path.segments[1].ident == "TestContext"
}

fn test_mode_to_tokens(test_mode: TestMode) -> TokenStream {
    match test_mode {
        TestMode::Playback => quote! { ::azure_core_test::TestMode::Playback },
        TestMode::Record => quote! { ::azure_core_test::TestMode::Record },
        TestMode::Live => quote! { ::azure_core_test::TestMode::Live },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use syn::Attribute;

    #[test]
    fn attributes_parse_live() {
        let attr: Attribute = syn::parse_quote! {
            #[recorded(live)]
        };
        let attrs: Attributes = attr.parse_args().unwrap();
        assert!(attrs.live);
    }

    #[test]
    fn attributes_parse_other() {
        let attr: Attribute = syn::parse_quote! {
            #[recorded(other)]
        };
        attr.parse_args::<Attributes>().unwrap_err();
    }

    #[test]
    fn attributes_parse_multiple() {
        let attr: Attribute = syn::parse_quote! {
            #[recorded(live, other)]
        };
        attr.parse_args::<Attributes>().unwrap_err();
    }

    #[test]
    fn attributes_parse_live_value() {
        let attr: Attribute = syn::parse_quote! {
            #[recorded(live = true)]
        };
        attr.parse_args::<Attributes>().unwrap_err();
    }

    #[test]
    fn is_test_context() {
        let types: Vec<syn::Type> = vec![
            syn::parse_quote! { ::azure_core_test::TestContext },
            syn::parse_quote! { azure_core_test::TestContext },
            syn::parse_quote! { TestContext },
        ];
        for ty in types {
            assert!(super::is_test_context(&ty));
        }
    }

    #[test]
    fn parse_recorded_playback() {
        let attr = TokenStream::new();
        let item = quote! {
            async fn recorded() {
                todo!()
            }
        };
        parse_test(attr, item).unwrap_err();
    }

    #[test]
    fn parse_recorded_playback_with_context() {
        let attr = TokenStream::new();
        let item = quote! {
            async fn recorded(ctx: TestContext) {
                todo!()
            }
        };
        parse_test(attr, item).unwrap();
    }

    #[test]
    fn parse_recorded_playback_with_multiple() {
        let attr = TokenStream::new();
        let item = quote! {
            async fn recorded(ctx: TestContext, name: &'static str) {
                todo!()
            }
        };
        parse_test(attr, item).unwrap_err();
    }

    #[test]
    fn parse_recorded_live() {
        let attr = quote! { live };
        let item = quote! {
            async fn live_only() {
                todo!()
            }
        };
        parse_test(attr, item).unwrap();
    }

    #[test]
    fn parse_recorded_live_with_context() {
        let attr = quote! { live };
        let item = quote! {
            async fn live_only(ctx: TestContext) {
                todo!()
            }
        };
        parse_test(attr, item).unwrap();
    }
}
