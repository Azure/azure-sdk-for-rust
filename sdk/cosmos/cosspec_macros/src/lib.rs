// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Procedural macros for `cosspec`.
//!
//! Use these through the re-exports in `cosspec`; the generated code
//! refers to that crate by absolute path.

use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Attribute, Error, Expr, ExprLit, FnArg, Ident, ItemFn, Lit, LitStr, Meta, Path, ReturnType,
    Token,
};

/// Registers an async function as a Cosmos DB spec test.
///
/// The function must:
///
/// * be `async`,
/// * have a `///` doc comment that states the specified behavior,
/// * declare an explicit return type (normally `SpecResult`),
/// * take only injectable parameters (`SharedContainer`, `ExclusiveContainer`,
///   `ExclusiveDatabase`, `ExclusiveAccount`, `HttpTap`).
///
/// These attributes may follow `#[spec_test]` on the same function:
///
/// * `#[service_profiles(all)]` or `#[service_profiles(SingleRegion, MultiRead)]`
/// * `#[requires(TopologyControl, MultiRegion)]`
/// * `#[known_issue("why the spec fails today")]` reports the spec as ignored
///   until the SDK meets it; run it with `--include-ignored`. Name service
///   profiles after the reason, as in `#[known_issue("...", MultiRead)]`, to
///   limit it to those profiles.
///
/// Parameters may carry `#[profile(basic_items)]` to choose a container profile.
/// `#[spec_test(id = "query.continuation.split")]` overrides the stable spec ID.
#[proc_macro_attribute]
pub fn spec_test(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as SpecArgs);
    let function = parse_macro_input!(input as ItemFn);
    expand(args, function)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}

#[derive(Default)]
struct SpecArgs {
    id: Option<LitStr>,
}

impl Parse for SpecArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = SpecArgs::default();
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;
        for meta in metas {
            match meta {
                Meta::NameValue(nv) if nv.path.is_ident("id") => match nv.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(value),
                        ..
                    }) => args.id = Some(value),
                    other => return Err(Error::new(other.span(), "`id` must be a string")),
                },
                other => {
                    return Err(Error::new(
                        other.span(),
                        "unsupported argument; expected `id = \"...\"`",
                    ))
                }
            }
        }
        Ok(args)
    }
}

/// `all`, `a, b`, or `[a, b]`.
struct IdentList {
    idents: Vec<Ident>,
}

struct KnownIssue {
    reason: LitStr,
    profiles: Vec<Ident>,
}

impl Parse for KnownIssue {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let reason = input.parse()?;
        let mut profiles = Vec::new();
        while input.parse::<Option<Token![,]>>()?.is_some() && !input.is_empty() {
            profiles.push(input.parse()?);
        }
        Ok(Self { reason, profiles })
    }
}

impl Parse for IdentList {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        let stream = if input.peek(syn::token::Bracket) {
            syn::bracketed!(content in input);
            &content
        } else {
            input
        };
        let idents = Punctuated::<Ident, Token![,]>::parse_terminated(stream)?;
        Ok(Self {
            idents: idents.into_iter().collect(),
        })
    }
}

fn expand(args: SpecArgs, mut function: ItemFn) -> syn::Result<TokenStream2> {
    let krate = quote!(::cosspec);
    let private = quote!(#krate::__private);
    let fn_name = function.sig.ident.clone();

    if function.sig.asyncness.is_none() {
        return Err(Error::new(
            function.sig.fn_token.span(),
            "spec tests must be `async fn`",
        ));
    }
    if matches!(function.sig.output, ReturnType::Default) {
        return Err(Error::new(
            function.sig.ident.span(),
            "spec tests must declare a return type, normally `-> SpecResult`",
        ));
    }

    let mut summary_lines = Vec::new();
    let mut service_profiles: Option<IdentList> = None;
    let mut requires: Vec<Ident> = Vec::new();
    let mut known_issue: Option<KnownIssue> = None;
    let mut kept: Vec<Attribute> = Vec::new();
    for attr in function.attrs.drain(..) {
        if attr.path().is_ident("doc") {
            if let Meta::NameValue(nv) = &attr.meta {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Str(line),
                    ..
                }) = &nv.value
                {
                    summary_lines.push(line.value().trim().to_string());
                }
            }
            kept.push(attr);
        } else if attr.path().is_ident("service_profiles") {
            if service_profiles.is_some() {
                return Err(Error::new(
                    attr.span(),
                    "`service_profiles` may appear only once",
                ));
            }
            service_profiles = Some(attr.parse_args()?);
        } else if attr.path().is_ident("requires") {
            requires.extend(attr.parse_args::<IdentList>()?.idents);
        } else if attr.path().is_ident("known_issue") {
            if known_issue.is_some() {
                return Err(Error::new(
                    attr.span(),
                    "`known_issue` may appear only once",
                ));
            }
            known_issue = Some(attr.parse_args()?);
        } else {
            kept.push(attr);
        }
    }
    function.attrs = kept;

    let summary = summary_lines
        .iter()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    if summary.is_empty() {
        return Err(Error::new(
            fn_name.span(),
            "spec tests must have a `///` doc comment stating the specified behavior",
        ));
    }

    let profiles_expr = match service_profiles {
        None => quote!(#krate::ServiceProfile::ALL),
        Some(list) if list.idents.len() == 1 && list.idents[0] == "all" => {
            quote!(#krate::ServiceProfile::ALL)
        }
        Some(list) => {
            let variants = list
                .idents
                .iter()
                .map(|ident| quote_spanned!(ident.span()=> #krate::ServiceProfile::#ident));
            quote!(&[#(#variants),*])
        }
    };
    let known_issue_expr = match known_issue {
        Some(KnownIssue { reason, profiles }) => {
            let variants = profiles
                .iter()
                .map(|ident| quote_spanned!(ident.span()=> #krate::ServiceProfile::#ident));
            quote!(::core::option::Option::Some(#private::KnownIssue {
                reason: #reason,
                profiles: &[#(#variants),*],
            }))
        }
        None => quote!(::core::option::Option::None),
    };
    let capabilities = requires
        .iter()
        .map(|ident| quote_spanned!(ident.span()=> #krate::Capability::#ident));

    let mut params = Vec::new();
    let mut injections = Vec::new();
    let mut arg_idents = Vec::new();
    for (index, input) in function.sig.inputs.iter_mut().enumerate() {
        let FnArg::Typed(typed) = input else {
            return Err(Error::new(input.span(), "spec tests cannot take `self`"));
        };
        let mut profile: Option<Path> = None;
        let mut kept = Vec::new();
        for attr in typed.attrs.drain(..) {
            if attr.path().is_ident("profile") {
                profile = Some(attr.parse_args()?);
            } else {
                kept.push(attr);
            }
        }
        typed.attrs = kept;

        let ty = &typed.ty;
        let name = match typed.pat.as_ref() {
            syn::Pat::Ident(pat) => pat.ident.to_string(),
            _ => format!("arg{index}"),
        };
        let profile_expr = match profile {
            None => quote!(::core::option::Option::None),
            Some(path) if path.get_ident().is_some() => {
                let ident = path.get_ident().unwrap();
                quote_spanned!(ident.span()=> ::core::option::Option::Some(#krate::profiles::#ident as fn() -> #krate::ContainerProfile))
            }
            Some(path) => {
                quote_spanned!(path.span()=> ::core::option::Option::Some(#path as fn() -> #krate::ContainerProfile))
            }
        };
        let arg = format_ident!("__spec_arg{}", index, span = Span::mixed_site());
        params.push(quote_spanned! {ty.span()=>
            #private::ParamDescriptor {
                name: #name,
                kind: <#ty as #private::Inject>::KIND,
                profile: #profile_expr,
            }
        });
        injections.push(quote_spanned! {ty.span()=>
            let #arg = <#ty as #private::Inject>::inject(&__spec_ctx, #index).await?;
        });
        arg_idents.push(arg);
    }

    let fn_name_str = fn_name.to_string();
    let id_expr = match args.id {
        Some(id) => quote!(::core::option::Option::Some(#id)),
        None => quote!(::core::option::Option::None),
    };
    let static_name = format_ident!("__SPEC_{}", fn_name_str.to_uppercase());

    Ok(quote! {
        #function

        #[#private::linkme::distributed_slice(#private::SPECS)]
        #[linkme(crate = #private::linkme)]
        #[doc(hidden)]
        static #static_name: #private::SpecDescriptor = #private::SpecDescriptor {
            function: #fn_name_str,
            module: ::core::module_path!(),
            id: #id_expr,
            summary: #summary,
            file: ::core::file!(),
            line: ::core::line!(),
            service_profiles: #profiles_expr,
            requires: &[#(#capabilities),*],
            known_issue: #known_issue_expr,
            params: &[#(#params),*],
            run: |__spec_ctx| ::std::boxed::Box::pin(async move {
                #(#injections)*
                #fn_name(#(#arg_idents),*).await
            }),
        };
    })
}
