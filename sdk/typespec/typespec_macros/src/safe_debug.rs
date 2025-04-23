// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::Result;
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use syn::{
    punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DataEnum, DataStruct,
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

    let body = match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => {
            let variants = variants.iter().map(|v| {
                let variant_name = &v.ident;
                let path = to_path(&[name, variant_name]);

                generate_fields(&path, &v.fields)
            });

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        Data::Struct(DataStruct { fields, .. }) => {
            let path = to_path(&[name]);
            let fields = generate_fields(&path, fields);

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

fn generate_fields(path: &Path, fields: &Fields) -> TokenStream {
    let name = &path.segments.last().expect("expected identifier").ident;
    let name_str = name.to_string();

    match fields {
        Fields::Named(FieldsNamed { ref named, .. }) => {
            let names: Vec<&Ident> = if cfg!(feature = "debug") {
                named
                    .iter()
                    .map(|f| f.ident.as_ref().expect("expected named field"))
                    .collect()
            } else {
                // Should we ever add a `#[safe(bool)]` helper attribute to denote which fields we can safely include,
                // filter the fields to match and emit based on the inherited value or field attribute value.
                Vec::new()
            };
            let fields: Vec<TokenStream> = names
                .iter()
                .map(|field_name| {
                    let field_name_str = field_name.to_string();
                    quote! {.field(#field_name_str, &#field_name)}
                })
                .collect();

            // Use an "and the rest" matcher as needed, along with the appropriate `DebugStruct` finisher.
            let (matcher, finisher) = finish(&fields, named);
            quote! {
                #path { #(#names),* #matcher } => f
                    .debug_struct(#name_str)
                        #(#fields)*
                        #finisher
            }
        }
        Fields::Unit => quote! {#path => f.write_str(#name_str)},
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
            let indices: Vec<TokenStream> = if cfg!(feature = "debug") {
                unnamed
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        Ident::new(&format!("f{i}"), Span::call_site()).into_token_stream()
                    })
                    .collect()
            } else {
                // Should we ever add a `#[safe(bool)]` helper attribute to denote which fields we can safely include,
                // filter the fields to match and emit based on the inherited value or field attribute value.
                Vec::new()
            };

            // Use an "and the rest" matcher as needed, along with the appropriate `DebugTuple` finisher.
            let (matcher, finisher) = finish(&indices, unnamed);
            quote! {
                #path(#(#indices),* #matcher) => f
                    .debug_tuple(#name_str)
                        #(.field(&#indices))*
                        #finisher
            }
        }
    }
}

fn finish(remaining: &[TokenStream], all: &Punctuated<Field, Comma>) -> (TokenStream, TokenStream) {
    if remaining.len() == all.len() {
        (TokenStream::new(), quote! {.finish()})
    } else if !remaining.is_empty() {
        (quote! {, ..}, quote! {.finish_non_exhaustive()})
    } else {
        (quote! {..}, quote!(.finish_non_exhaustive()))
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
