extern crate proc_macro;

use proc_macro2::{Span, TokenStream};
use syn::parse::ParseStream;
use syn::spanned::Spanned;
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
/// # use typespec_derive::Model;
/// # use serde::Deserialize;
/// extern crate typespec_client_core as my_typespec;
///
/// #[derive(Model, Deserialize)]
/// #[typespec(crate = "my_typespec")]
/// struct MyModel {
///   value: String
/// }
/// ```
///
/// ### `#[typespec(format)]`
///
/// The format attribute specifies the format of the response body. The default is `json`.
/// If compiling with the `xml` feature, the value `xml` is also supported.
///
/// ```rust
/// # use typespec_derive::Model;
/// # use serde::Deserialize;
/// #[derive(Model, Deserialize)]
/// #[typespec(format = "xml")]
/// struct MyModel {
///   value: String
/// }
/// ```
///
/// **NOTE:** Using formats other than JSON may require enabling additional features in `typespec_client_core`.
#[proc_macro_derive(Model, attributes(typespec))]
pub fn model_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let body = generate_body(ast);

    // We wrap the generated code in a const block to give it a unique scope.
    let gen = quote::quote! {
        #[doc(hidden)]
        const _: () = {
            #body
        };
    };
    gen.into()
}

fn generate_body(ast: DeriveInput) -> TokenStream {
    let mut context = Context::new();
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();
    let name = &ast.ident;

    // Parse attributes
    let attrs = Attrs::from_attrs(&ast.attrs, &mut context);
    if !context.errors().is_empty() {
        // If there were any errors parsing attrs, emit their compile_error! statements and return.
        let errs = context.errors();
        return quote::quote! {
            #(#errs)*
        }
        .into();
    }

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

    quote::quote! {
        #typespec_import

        #[automatically_derived]
        impl #impl_generics _typespec::http::Model for #name #ty_generics #where_clause {
            async fn from_response_body(body: _typespec::http::ResponseBody) -> _typespec::Result<Self> {
                #deserialize_body
            }
        }
    }
}

enum Format {
    Json,
    Xml,
}

struct Context {
    errors: Vec<TokenStream>,
}

impl Context {
    pub fn new() -> Context {
        Context { errors: Vec::new() }
    }

    pub fn emit_error(&mut self, span: Span, message: &str) {
        let token_stream = quote::quote_spanned!(span.into()=> compile_error!(#message));
        self.errors.push(token_stream.into());
    }

    pub fn errors(&self) -> &[TokenStream] {
        &self.errors
    }
}

struct Attrs {
    pub typespec_path: Option<Path>,
    pub format: Option<Format>,
}

impl Attrs {
    pub fn from_attrs(attrs: &[Attribute], ctx: &mut Context) -> Attrs {
        let mut typespec_path = None;
        let mut format = None;
        for attr in attrs.iter().filter(|a| a.path().is_ident("typespec")) {
            let Meta::List(meta_list) = &attr.meta else {
                ctx.emit_error(attr.span(), "Invalid typespec attribute, expected attribute in form #[typespec(key = value)]");
                continue;
            };

            meta_list.parse_nested_meta(|meta| {
                let Some(ident) = meta.path.get_ident() else {
                    ctx.emit_error(meta.path.span(), "Invalid typespec attribute, expected attribute in form #[typespec(key = value)]");
                    return Ok(());
                };
                match ident.to_string().as_str() {
                    "crate" => {
                        let value = meta.value().unwrap();
                        let Ok(value) = parse_literal_string(value) else {
                            ctx.emit_error(value.span(), "Invalid value for 'crate' attribute, expected string literal");
                            return Ok(());
                        };
                        let path = value.parse().unwrap();
                        typespec_path = Some(path);
                    }
                    "format" => {
                        let value = meta.value().unwrap();
                        let Ok(lit) = parse_literal_string(value) else {
                            ctx.emit_error(value.span(), "Invalid value for 'format' attribute, expected string literal");
                            return Ok(());
                        };
                        format = Some(match lit.value().as_str() {
                            "json" => Format::Json,
                            "xml" => Format::Xml,
                            x => {
                                ctx.emit_error(lit.span(), &format!("Unknown format '{}'", x));
                                return Ok(());
                            }
                        });
                    }
                    x => {
                        ctx.emit_error(meta.path.span(), &format!("Unknown typespec attribute '#[typespec({})'", x));
                    }
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

fn parse_literal_string(value: ParseStream) -> Result<LitStr, ()> {
    let expr: syn::Expr = value.parse().map_err(|_| ())?;
    match expr {
        syn::Expr::Lit(lit) => match lit.lit {
            syn::Lit::Str(s) => Ok(s),
            _ => Err(()),
        },
        _ => Err(()),
    }
}
