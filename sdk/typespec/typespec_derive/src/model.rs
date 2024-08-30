use proc_macro2::TokenStream;
use syn::spanned::Spanned;
use syn::{Attribute, DeriveInput, Meta, Path};

use crate::{parse_literal_string, Error, Result};

pub fn derive_model_impl(ast: DeriveInput) -> Result<TokenStream> {
    let body = generate_body(ast)?;

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote::quote! {
        #[doc(hidden)]
        const _: () = {
            #body
        };
    };
    Ok(gen.into())
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
    pub fn from_attrs(attrs: &[Attribute]) -> Result<Attrs> {
        let mut typespec_path = None;
        let mut format = None;
        let mut errors = Vec::new();
        for attr in attrs.iter().filter(|a| a.path().is_ident("typespec")) {
            let Meta::List(meta_list) = &attr.meta else {
                errors.push(Error::new(attr.span(), "Invalid typespec attribute, expected attribute in form #[typespec(key = value)]"));
                continue;
            };

            meta_list.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    errors.push(Error::new(meta.path.span(), "Invalid typespec attribute, expected attribute in form #[typespec(key = value)]"));
                    return Ok(());
                };
                match ident.to_string().as_str() {
                    "crate" => {
                        let value = meta.value().unwrap();
                        let value = match parse_literal_string(value) {
                            Ok(v) => v,
                            Err(mut e) => {
                                errors.append(&mut e);

                                // Returning from the _closure_.
                                // We've already set an error that will cause the outer function to return Err
                                return Ok(())
                            }
                        };
                        let path = value.parse().unwrap();
                        typespec_path = Some(path);
                    }
                    "format" => {
                        let value = meta.value().unwrap();
                        let lit = match parse_literal_string(value) {
                            Ok(v) => v,
                            Err(mut e) => {
                                errors.append(&mut e);

                                // Returning from the _closure_.
                                // We've already set an error that will cause the outer function to return Err
                                return Ok(())
                            }
                        };
                        format = Some(match lit.value().as_str() {
                            "json" => Format::Json,
                            "xml" => Format::Xml,
                            x => {
                                errors.push(Error::new(lit.span(), &format!("Unknown format '{}'", x)));

                                // Returning from the _closure_.
                                // We've already set an error that will cause the outer function to return Err
                                return Ok(())
                            }
                        });
                    }
                    x => {
                        errors.push(Error::new(meta.path.span(), &format!("Unknown typespec attribute '#[typespec({})'", x)));
                    }
                };
                Ok(())
            }).unwrap();
        }

        if errors.is_empty() {
            Ok(Attrs {
                typespec_path,
                format,
            })
        } else {
            Err(errors)
        }
    }
}
