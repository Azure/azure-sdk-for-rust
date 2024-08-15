extern crate proc_macro;

use syn::meta::ParseNestedMeta;
use syn::{Attribute, DeriveInput, LitStr, Meta, Path};

/// Derive macro for implementing `Model` trait.
/// Deriving this trait allows a type to be deserialized from an HTTP response body.
/// By default, the type must also implement `serde::Deserialize`, or the generated code will not compile.
///
/// ## Attributes
///
/// The following attributes are supported on the struct itself:
///
/// ### `#[typespec(crate)]`
///
/// The 'crate' attribute specifies an alternate module path, other than the default of `typespec_client_core`, to reference the typespec client crate.
///
/// ```rust
/// use typespec_client_core as my_typespec;
///
/// #[derive(Model, Deserialize)]
/// #[typespec(crate = "my_typespec")]
/// struct MyModel {
///    // ...
/// }
/// ```
///
/// ### `#[typespec(format)]`
///
/// The format attribute specifies the format of the response body. The default is `json`.
/// If compiling with the `xml` feature, the value `xml` is also supported.
///
/// ```rust
/// #[derive(Model, Deserialize)]
/// #[typespec(format = "xml")]
/// struct MyModel {
///   // ...
/// }
/// ```
#[proc_macro_derive(Model, attributes(typespec))]
pub fn model_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    // Parse attributes
    let attrs = Attrs::from_attrs(&ast.attrs);

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
            use #path as _typespec;
        },
        None => quote::quote! {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate typespec_client_core as _typespec;
        },
    };

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote::quote! {
        #[doc(hidden)]
        const _: () = {
            #typespec_import

            #[automatically_derived]
            impl #impl_generics _typespec::http::Model for #name #ty_generics #where_clause {
                async fn from_response_body(body: _typespec::http::ResponseBody) -> _typespec::Result<Self> {
                    #deserialize_body
                }
            }
        };
    };
    gen.into()
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
    pub fn from_attrs(attrs: &[Attribute]) -> Attrs {
        let mut typespec_path = None;
        let mut format = None;
        for attr in attrs.iter().filter(|a| a.path().is_ident("typespec")) {
            let Meta::List(meta_list) = &attr.meta else {
                panic!("Invalid typespec attribute, expected attribute in form #[typespec(key = value)]");
            };

            meta_list.parse_nested_meta(|meta| {
                match meta.path.get_ident().expect("Invalid typespec attribute, expected attribute in form #[typespec(key = value)]").to_string().as_str() {
                    "crate" => {
                        let value = parse_literal_string(&meta);
                        let path = value.parse().unwrap();
                        typespec_path = Some(path);
                    }
                    "format" => {
                        format = Some(match parse_literal_string(&meta).value().as_str() {
                            "json" => Format::Json,
                            "xml" => Format::Xml,
                            x => panic!("Unknown typespec format '{}'", x)
                        });
                    }
                    x => panic!("Unknown typespec attribute '{}'", x)
                };
                Ok(())
            }).unwrap();
        }
        Attrs {
            typespec_path,
            format,
        }
    }
}

fn parse_literal_string(meta: &ParseNestedMeta) -> LitStr {
    let expr: syn::Expr = meta.value().unwrap().parse().unwrap();
    match expr {
        syn::Expr::Lit(lit) => match lit.lit {
            syn::Lit::Str(s) => s,
            _ => panic!("Expected string literal"),
        },
        _ => panic!("Expected string literal"),
    }
}

// #[macro_export]
// macro_rules! json_serializable {
//     ($type:ty) => {
//     };
// }
//
// #[macro_export]
// #[cfg(feature = "xml")]
// macro_rules! xml_serializable {
//     ($type:ty) => {
//         impl $crate::http:Model for $type {
//             async fn from_response_body(body: $crate::http::ResponseBody) -> $crate::Result<Self> {
//                 body.xml().await
//             }
//         }
//     };
// }
