// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{case::Case, symbol::*};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{punctuated::Punctuated, spanned::Spanned as _, DeriveInput, Token};

pub fn expand_derive_variant(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let case = get_case(&input.attrs, Case::default())?;
    let variants = get_variants(input.span(), &input.data, case)?;

    todo!();
}

fn get_case(attrs: &Vec<syn::Attribute>, default: Case) -> syn::Result<Case> {
    let mut case = default;

    for attr in attrs {
        if attr.path() != VARIANT {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path == RENAME_ALL {
                let value = get_attr_string(&meta)?;
                case = Case::from_str(&value)
                    .map_err(|err| syn::Error::new(attr.span(), err.to_string()))?;
            }

            Ok(())
        })?;
    }

    Ok(case)
}

fn get_variants(
    span: Span,
    data: &syn::Data,
    default: Case,
) -> syn::Result<Vec<(syn::Ident, String)>> {
    let mut variants: Vec<_> = Vec::new();
    match *data {
        syn::Data::Enum(ref data) => {
            for var in &data.variants {
                let mut value = default.rename(&var.ident.to_string());

                for attr in &var.attrs {
                    if attr.path() != VARIANT {
                        continue;
                    }

                    attr.parse_nested_meta(|meta| {
                        if meta.path == RENAME {
                            value = get_attr_string(&meta)?;
                        }
                        Ok(())
                    })?;
                }

                variants.push((var.ident.clone(), value));
            }
        }
        _ => {
            return Err(syn::Error::new(span, "can only derive `Variant` on enums"));
        }
    }

    Ok(variants)
}

fn get_attr_string(meta: &syn::meta::ParseNestedMeta) -> syn::Result<String> {
    let expr: syn::Expr = meta.value()?.parse()?;
    let mut value: &syn::Expr = &expr;
    while let syn::Expr::Group(e) = value {
        value = &e.expr;
    }

    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(lit),
        ..
    }) = value
    else {
        return Err(syn::Error::new(expr.span(), "expected `str` literal"));
    };

    let suffix = lit.suffix();
    if !suffix.is_empty() {
        return Err(syn::Error::new(
            expr.span(),
            format!("unexpected suffix {suffix} on `str` literal"),
        ));
    }

    return Ok(lit.value());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_case_default() {
        let tokens = quote! {
            enum Sut {
                Foo,
                Bar,
            }
        };

        let tokens = syn::parse2::<DeriveInput>(tokens).expect("variant token stream");
        let case = get_case(&tokens.attrs, Case::default()).expect("case attribute");

        assert_eq!(Case::None, case);
    }

    #[test]
    fn get_case_lowercase() {
        let tokens = quote! {
            #[variant(rename_all = "lowercase")]
            enum Sut {
                Foo,
                Bar,
            }
        };

        let tokens = syn::parse2::<DeriveInput>(tokens).expect("variant token stream");
        let case = get_case(&tokens.attrs, Case::default()).expect("case attribute");

        assert_eq!(Case::Lowercase, case);
    }

    #[test]
    fn get_variants_names() {
        let tokens = quote! {
            #[variant(rename_all = "lowercase")]
            enum Sut {
                Foo,
                #[variant(rename = "BAZ")]
                Bar,
            }
        };

        let tokens = syn::parse2::<DeriveInput>(tokens).expect("variant token stream");
        let variants = get_variants(tokens.span(), &tokens.data, Case::Lowercase)
            .expect("case attribute and variants");

        assert_eq!(variants.len(), 2);

        assert_eq!(variants[0].0.to_string(), "Foo");
        assert_eq!(variants[0].1, "foo");

        assert_eq!(variants[1].0.to_string(), "Bar");
        assert_eq!(variants[1].1, "BAZ");
    }
}
