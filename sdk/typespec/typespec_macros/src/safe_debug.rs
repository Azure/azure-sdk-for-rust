// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    spanned::Spanned, Data, DataEnum, DataStruct, DeriveInput, Error, Fields, FieldsNamed,
    FieldsUnnamed, Ident, Path,
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
        #[cfg(feature = "debug")]
        Fields::Named(FieldsNamed { ref named, .. }) => {
            let names: Vec<&Ident> = named
                .iter()
                .map(|f| f.ident.as_ref().expect("expected named field"))
                .collect();
            let fields = names.iter().map(|field_name| {
                let field_name_str = field_name.to_string();
                quote! {.field(#field_name_str, &#field_name)}
            });
            quote! {
                #path { #(#names),* } => f
                    .debug_struct(#name_str)
                        #(#fields)*
                        .finish()
            }
        }
        #[cfg(not(feature = "debug"))]
        Fields::Named(FieldsNamed { .. }) => {
            quote! {
                #path { .. } => f
                    .debug_struct(#name_str).finish_non_exhaustive()
            }
        }
        Fields::Unit => quote! {#path => f.write_str(#name_str)},
        #[cfg(feature = "debug")]
        Fields::Unnamed(FieldsUnnamed { ref unnamed, .. }) => {
            let indices: Vec<Ident> = unnamed
                .iter()
                .enumerate()
                .map(|(i, _)| Ident::new(&format!("f{i}"), proc_macro2::Span::call_site()))
                .collect();
            quote! {
                #path(#(#indices),*) => f
                    .debug_tuple(#name_str)
                        #(.field(&#indices))*
                        .finish()
            }
        }
        #[cfg(not(feature = "debug"))]
        Fields::Unnamed(FieldsUnnamed { .. }) => {
            quote! {
                #path(..) => f
                    .debug_tuple(#name_str).finish_non_exhaustive()
            }
        }
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
