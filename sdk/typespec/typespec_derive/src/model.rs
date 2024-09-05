// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Error, Meta, Path};

use crate::{parse_literal_string, Result};

pub fn derive_model_impl(ast: DeriveInput) -> Result<TokenStream> {
    let body = generate_body(ast)?;

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote::quote! {
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

    // Parse attributes
    let attrs = Attrs::from_attrs(&ast.attrs)?;

    let format = attrs.format.unwrap_or(Format::Json);
    let deserialize_body = match format {
        Format::Json => quote::quote! {
            body.json().await
        },
        Format::Xml => quote::quote! {
            body.xml().await
        },
    };

    // If the standard path is used, we need to add 'extern crate', because it's possible the calling code
    // depends on typespec_client_core transitively, which means it's not in scope by default.
    // That's not necessary when using a custom path because we assume the user has done that work.
    let typespec_import = match attrs.typespec_path {
        Some(path) => quote::quote! {
            use #path as _typespec_client_core;
        },
        None => quote::quote! {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate typespec_client_core as _typespec_client_core;
        },
    };

    Ok(quote::quote! {
        #typespec_import

        #[automatically_derived]
        impl #impl_generics _typespec_client_core::http::Model for #name #ty_generics #where_clause {
            async fn from_response_body(body: _typespec_client_core::http::ResponseBody) -> _typespec_client_core::Result<Self> {
                #deserialize_body
            }
        }
    })
}

enum Format {
    Json,
    Xml,
}

struct Attrs {
    pub typespec_path: Option<Path>,
    pub format: Option<Format>,
}

impl Attrs {
    pub fn from_attrs(attributes: &[Attribute]) -> Result<Attrs> {
        let mut attrs = Attrs {
            typespec_path: None,
            format: None,
        };

        let mut result = Ok(());
        for attribute in attributes.iter().filter(|a| a.path().is_ident("typespec")) {
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
}

const INVALID_TYPESPEC_ATTRIBUTE_MESSAGE: &'static str =
    "invalid typespec attribute, expected attribute in form #[typespec(key = value)]";

fn parse_attr(attribute: &Attribute, attrs: &mut Attrs) -> Result<()> {
    let Meta::List(meta_list) = &attribute.meta else {
        return Err(Error::new(
            attribute.span(),
            INVALID_TYPESPEC_ATTRIBUTE_MESSAGE,
        ));
    };

    meta_list.parse_nested_meta(|meta| {
        let ident = meta
            .path
            .get_ident()
            .ok_or_else(|| Error::new(attribute.span(), INVALID_TYPESPEC_ATTRIBUTE_MESSAGE))?;
        let value = meta
            .value()
            .map_err(|_| Error::new(attribute.span(), INVALID_TYPESPEC_ATTRIBUTE_MESSAGE))?;

        match ident.to_string().as_str() {
            "crate" => {
                let lit = parse_literal_string(value)?;
                let path = lit.parse().map_err(|_| {
                    Error::new(lit.span(), format!("invalid module path: {}", lit.value()))
                })?;
                attrs.typespec_path = Some(path);
                Ok(())
            }
            "format" => {
                let lit = parse_literal_string(value)?;
                attrs.format = Some(match lit.value().as_str() {
                    "json" => Format::Json,
                    "xml" => Format::Xml,
                    x => {
                        return Err(Error::new(lit.span(), format!("Unknown format '{}'", x)));
                    }
                });
                Ok(())
            }
            x => Err(Error::new(
                meta.path.span(),
                format!("unknown typespec attribute '#[typespec({})'", x),
            )),
        }
    })
}
