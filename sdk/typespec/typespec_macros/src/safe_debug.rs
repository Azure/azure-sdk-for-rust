// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::Result;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Error, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Path,
};

pub fn derive_safe_debug_impl(ast: DeriveInput) -> Result<TokenStream> {
    let body = generate_body(ast)?;

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote! {
        #[doc(hidden)]
        const _: () = {
            #body
        };
    };
    Ok(gen)
}

fn generate_body(ast: DeriveInput) -> Result<TokenStream> {
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    let type_attrs = Attrs::from_attrs(&ast.attrs)?;
    let body = match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let variants = variants
                .iter()
                .map(|v| -> Result<TokenStream> {
                    let variant_name = &v.ident;
                    let path = to_path(&[name, variant_name]);

                    let mut enum_attrs = Attrs::from_attrs(&v.attrs)?;
                    enum_attrs.update(&type_attrs);

                    generate_fields(&path, &enum_attrs, &v.fields)
                })
                .collect::<Result<Vec<_>>>()?;

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        Data::Struct(DataStruct { fields, .. }) => {
            let path = to_path(&[name]);
            let fields = generate_fields(&path, &type_attrs, fields)?;

            quote! {
                match self {
                    #fields
                }
            }
        }
        _ => return Err(Error::new(ast.span(), "type not supported for `SafeDebug`")),
    };

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics ::std::fmt::Debug for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                #body
            }
        }
    })
}

fn generate_fields(path: &Path, type_attrs: &Attrs, fields: &Fields) -> Result<TokenStream> {
    let name = &path.segments.last().expect("expected identifier").ident;
    let name_str = name.to_string();

    match fields {
        Fields::Named(FieldsNamed { ref named, .. }) => {
            let names: Vec<&Ident> = named
                .iter()
                .filter_map(|f| -> Option<Result<&Ident>> {
                    if cfg!(feature = "debug") {
                        return Some(Ok(f.ident.as_ref().expect("expected named field")));
                    }

                    match Attrs::from_attrs(&f.attrs) {
                        Err(err) => Some(Err(err)),
                        Ok(attrs) if type_attrs.is_safe(&attrs) => {
                            Some(Ok(f.ident.as_ref().expect("expected named field")))
                        }
                        Ok(_) => None,
                    }
                })
                .collect::<Result<Vec<_>>>()?;
            let fields: Vec<TokenStream> = names
                .iter()
                .map(|field_name| {
                    let field_name_str = field_name.to_string();
                    quote! {.field(#field_name_str, &#field_name)}
                })
                .collect();

            // Use an "and the rest" matcher as needed, along with the appropriate `DebugStruct` finisher.
            let (matcher, finisher) = finish(&fields, named, false);
            Ok(quote! {
                #path { #(#names),* #matcher } => f
                    .debug_struct(#name_str)
                        #(#fields)*
                        #finisher
            })
        }
        Fields::Unit => Ok(quote! {#path => f.write_str(#name_str)}),
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
            let indices: Vec<TokenStream> = unnamed
                .iter()
                .enumerate()
                .filter_map(|(i, f)| {
                    if cfg!(feature = "debug") {
                        return Some(Ok(
                            Ident::new(&format!("f{i}"), Span::call_site()).into_token_stream()
                        ));
                    }

                    match Attrs::from_attrs(&f.attrs) {
                        Err(err) => Some(Err(err)),
                        Ok(attrs) if type_attrs.is_safe(&attrs) => {
                            Some(Ok(
                                Ident::new(&format!("f{i}"), Span::call_site()).into_token_stream()
                            ))
                        }
                        Ok(_) => None,
                    }
                })
                .collect::<Result<Vec<_>>>()?;

            // Use an "and the rest" matcher as needed, along with the appropriate `DebugTuple` finisher.
            let (matcher, finisher) = finish(&indices, unnamed, true);
            Ok(quote! {
                #path(#(#indices),* #matcher) => f
                    .debug_tuple(#name_str)
                        #(.field(&#indices))*
                        #finisher
            })
        }
    }
}

fn finish(
    remaining: &[TokenStream],
    all: &Punctuated<Field, Comma>,
    tuple: bool,
) -> (TokenStream, TokenStream) {
    const MSRV: rustc_version::Version = rustc_version::Version::new(1, 80, 0);
    const MIN: rustc_version::Version = rustc_version::Version::new(1, 82, 0);

    // DebugTuple::finish_non_exhaustive() wasn't added till 1.82.
    let non_exhaustive_finisher = if tuple && rustc_version::version().unwrap_or(MSRV) < MIN {
        quote! {.finish()}
    } else {
        quote! {.finish_non_exhaustive()}
    };

    if remaining.len() == all.len() {
        (TokenStream::new(), quote! {.finish()})
    } else if !remaining.is_empty() {
        (quote! {, ..}, non_exhaustive_finisher)
    } else {
        (quote! {..}, non_exhaustive_finisher)
    }
}

// cspell:ignore idents
fn to_path(idents: &[&Ident]) -> Path {
    use syn::{punctuated::Punctuated, PathArguments, PathSegment};

    let mut segments = Punctuated::new();
    for ident in idents {
        segments.push(PathSegment {
            ident: (*ident).clone(),
            arguments: PathArguments::None,
        });
    }

    Path {
        leading_colon: None,
        segments,
    }
}

#[derive(Debug, Default)]
struct Attrs {
    safe: Option<bool>,
}

impl Attrs {
    fn from_attrs(attributes: &[Attribute]) -> Result<Attrs> {
        let mut attrs = Attrs::default();
        let mut result = Ok(());
        for attribute in attributes.iter().filter(|a| a.path().is_ident("safe")) {
            result = match (result, parse_attr(attribute, &mut attrs)) {
                (Ok(()), Err(e)) => Err(e),
                (Err(mut e1), Err(e2)) => {
                    e1.combine(e2);
                    Err(e1)
                }
                (e, Ok(())) => e,
            };
        }
        result.map(|_| attrs)
    }

    fn is_safe(&self, other: &Attrs) -> bool {
        match other.safe {
            Some(val) => val,
            None => self.safe.unwrap_or(false),
        }
    }

    fn update(&mut self, other: &Attrs) {
        if let Some(val) = other.safe {
            self.safe = Some(val);
        }
    }
}

const INVALID_SAFE_ATTRIBUTE_MESSAGE: &str =
    "invalid safe attribute, expected attribute in form #[safe(false)] or #[safe(true)]";

fn parse_attr(attribute: &Attribute, attrs: &mut Attrs) -> Result<()> {
    let meta_list = attribute
        .meta
        .require_list()
        .map_err(|_| Error::new(attribute.span(), INVALID_SAFE_ATTRIBUTE_MESSAGE))?;
    let lit: syn::LitBool = meta_list
        .parse_args()
        .map_err(|_| Error::new(meta_list.span(), INVALID_SAFE_ATTRIBUTE_MESSAGE))?;
    attrs.safe = Some(lit.value);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attrs_safe_requires_arg() {
        let attr: Attribute = syn::parse_quote! {
            #[safe]
        };
        assert!(
            matches!(Attrs::from_attrs(&[attr]), Err(err) if err.to_string() == INVALID_SAFE_ATTRIBUTE_MESSAGE)
        );
    }

    #[test]
    fn attrs_safe_requires_bool() {
        let attr: Attribute = syn::parse_quote! {
            #[safe(false)]
        };
        assert_eq!(Attrs::from_attrs(&[attr]).unwrap().safe, Some(false));

        let attr: Attribute = syn::parse_quote! {
            #[safe(true)]
        };
        assert_eq!(Attrs::from_attrs(&[attr]).unwrap().safe, Some(true));

        let attr: Attribute = syn::parse_quote! {
            #[safe(other)]
        };
        assert!(
            matches!(Attrs::from_attrs(&[attr]), Err(err) if err.to_string() == INVALID_SAFE_ATTRIBUTE_MESSAGE)
        );
    }

    #[test]
    fn attrs_is_safe() {
        let mut type_attrs = Attrs::default();
        let mut field_attrs = Attrs::default();

        // Both None
        assert!(!type_attrs.is_safe(&field_attrs));

        // None, Some(false)
        field_attrs.safe = Some(false);
        assert!(!type_attrs.is_safe(&field_attrs));

        // None, Some(true)
        field_attrs.safe = Some(true);
        assert!(type_attrs.is_safe(&field_attrs));

        // Some(false), None
        type_attrs.safe = Some(false);
        field_attrs.safe = None;
        assert!(!type_attrs.is_safe(&field_attrs));

        // Some(true), None
        type_attrs.safe = Some(true);
        field_attrs.safe = None;
        assert!(type_attrs.is_safe(&field_attrs));

        // Some(false), Some(false)
        type_attrs.safe = Some(false);
        field_attrs.safe = Some(false);
        assert!(!type_attrs.is_safe(&field_attrs));

        // Some(true), Some(false)
        type_attrs.safe = Some(true);
        field_attrs.safe = Some(false);
        assert!(!type_attrs.is_safe(&field_attrs));

        // Some(false), Some(true)
        type_attrs.safe = Some(false);
        field_attrs.safe = Some(true);
        assert!(type_attrs.is_safe(&field_attrs));

        // Some(true), Some(true)
        type_attrs.safe = Some(true);
        field_attrs.safe = Some(true);
        assert!(type_attrs.is_safe(&field_attrs));
    }

    #[test]
    fn attrs_update() {
        let mut type_attrs = Attrs::default();
        let mut enum_attrs = Attrs::default();

        // Both None
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, None);

        // None, Some(false)
        type_attrs.safe = None;
        enum_attrs.safe = Some(false);
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(false));

        // None, Some(true)
        type_attrs.safe = None;
        enum_attrs.safe = Some(true);
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(true));

        // Some(false), None
        type_attrs.safe = Some(false);
        enum_attrs.safe = None;
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(false));

        // Some(true), None
        type_attrs.safe = Some(true);
        enum_attrs.safe = None;
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(true));

        // Some(false), Some(false)
        type_attrs.safe = Some(false);
        enum_attrs.safe = Some(false);
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(false));

        // Some(true), Some(false)
        type_attrs.safe = Some(true);
        enum_attrs.safe = Some(false);
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(false));

        // Some(false), Some(true)
        type_attrs.safe = Some(false);
        enum_attrs.safe = Some(true);
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(true));

        // Some(true), Some(true)
        type_attrs.safe = Some(true);
        enum_attrs.safe = Some(true);
        type_attrs.update(&enum_attrs);
        assert_eq!(type_attrs.safe, Some(true));
    }
}
