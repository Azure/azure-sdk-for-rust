// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::Result;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Attribute, Data, DataEnum, DataStruct,
    DeriveInput, Error, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Meta, Path,
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
            let variants = variants.iter().map(|v| {
                let variant_name = &v.ident;
                let path = to_path(&[name, variant_name]);

                let mut enum_attrs = Attrs::from_attrs(&v.attrs).unwrap_or_default();
                enum_attrs.combine(&type_attrs);

                generate_fields(&path, &enum_attrs, &v.fields)
            });

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        Data::Struct(DataStruct { fields, .. }) => {
            let path = to_path(&[name]);
            let fields = generate_fields(&path, &type_attrs, fields);

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

fn generate_fields(path: &Path, type_attrs: &Attrs, fields: &Fields) -> TokenStream {
    let name = &path.segments.last().expect("expected identifier").ident;
    let name_str = name.to_string();

    match fields {
        Fields::Named(FieldsNamed { ref named, .. }) => {
            let names: Vec<&Ident> = named
                .iter()
                .filter(|f| {
                    if cfg!(feature = "debug") {
                        return true;
                    }

                    let attrs = Attrs::from_attrs(&f.attrs).unwrap_or_default();
                    type_attrs.and(&attrs)
                })
                .map(|f| f.ident.as_ref().expect("expected named field"))
                .collect();
            let fields: Vec<TokenStream> = names
                .iter()
                .map(|field_name| {
                    let field_name_str = field_name.to_string();
                    quote! {.field(#field_name_str, &#field_name)}
                })
                .collect();

            // Use an "and the rest" matcher as needed, along with the appropriate `DebugStruct` finisher.
            let (matcher, finisher) = finish(&fields, named, false);
            quote! {
                #path { #(#names),* #matcher } => f
                    .debug_struct(#name_str)
                        #(#fields)*
                        #finisher
            }
        }
        Fields::Unit => quote! {#path => f.write_str(#name_str)},
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
            let indices: Vec<TokenStream> = unnamed
                .iter()
                .enumerate()
                .filter(|(_, f)| {
                    if cfg!(feature = "debug") {
                        return true;
                    }

                    let attrs = Attrs::from_attrs(&f.attrs).unwrap_or_default();
                    type_attrs.and(&attrs)
                })
                .map(|(i, _)| Ident::new(&format!("f{i}"), Span::call_site()).into_token_stream())
                .collect();

            // Use an "and the rest" matcher as needed, along with the appropriate `DebugTuple` finisher.
            let (matcher, finisher) = finish(&indices, unnamed, true);
            quote! {
                #path(#(#indices),* #matcher) => f
                    .debug_tuple(#name_str)
                        #(.field(&#indices))*
                        #finisher
            }
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

#[derive(Default)]
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

    fn and(&self, other: &Attrs) -> bool {
        match (self.safe, other.safe) {
            (Some(safe), Some(other)) => safe && other,
            (None, Some(other)) => other,
            (Some(safe), None) => safe,
            (None, None) => false,
        }
    }

    fn combine(&mut self, other: &Attrs) {
        match (self.safe, other.safe) {
            (None, Some(other)) => self.safe = Some(other),
            (Some(safe), Some(other)) => self.safe = Some(safe && other),
            _ => {}
        }
    }
}

const INVALID_SAFE_ATTRIBUTE_MESSAGE: &str =
    "invalid safe attribute, expected attribute in form #[safe(true)]";

fn parse_attr(attribute: &Attribute, attrs: &mut Attrs) -> Result<()> {
    let Meta::List(meta_list) = &attribute.meta else {
        return Err(Error::new(attribute.span(), INVALID_SAFE_ATTRIBUTE_MESSAGE));
    };

    let lit: syn::LitBool = meta_list
        .parse_args()
        .map_err(|_| Error::new(meta_list.span(), INVALID_SAFE_ATTRIBUTE_MESSAGE))?;
    attrs.safe = Some(lit.value);

    Ok(())
}

#[test]
fn test_attrs_and() {
    let mut type_attrs = Attrs::default();
    let mut field_attrs = Attrs::default();
    assert!(!type_attrs.and(&field_attrs));

    field_attrs.safe = Some(false);
    assert!(!type_attrs.and(&field_attrs));

    field_attrs.safe = Some(true);
    assert!(type_attrs.and(&field_attrs));

    type_attrs.safe = Some(false);
    assert!(!type_attrs.and(&field_attrs));

    field_attrs.safe = Some(false);
    assert!(!type_attrs.and(&field_attrs));

    type_attrs.safe = Some(true);
    assert!(!type_attrs.and(&field_attrs));

    field_attrs.safe = Some(true);
    assert!(type_attrs.and(&field_attrs));

    field_attrs.safe = None;
    assert!(type_attrs.and(&field_attrs));

    type_attrs.safe = Some(false);
    assert!(!type_attrs.and(&field_attrs));
}
