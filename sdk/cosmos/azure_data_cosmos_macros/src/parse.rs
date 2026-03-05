// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::Result;
use proc_macro2::Span;
use syn::{
    parse::Parse, punctuated::Punctuated, spanned::Spanned, token::Comma, Data, DataStruct,
    DeriveInput, Error, Fields, GenericArgument, Generics, Ident, PathArguments, Type,
};

/// Known layer names that can appear in `#[options(layers(...))]`.
const KNOWN_LAYERS: &[&str] = &["runtime", "account", "operation"];

/// Parsed representation of a `#[derive(CosmosOptions)]` struct.
pub struct OptionsInput {
    /// The struct name.
    pub name: Ident,
    /// The struct's generic parameters.
    pub generics: Generics,
    /// The visibility of the struct.
    pub vis: syn::Visibility,
    /// Which explicit layers this group participates in.
    pub layers: Vec<Layer>,
    /// Parsed fields.
    pub fields: Vec<OptionField>,
}

/// A configuration layer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    /// Runtime (application-global) layer.
    Runtime,
    /// Account (per-client) layer.
    Account,
    /// Operation (per-request) layer.
    Operation,
}

impl Layer {
    /// Returns the identifier for this layer.
    pub fn ident(&self) -> Ident {
        match self {
            Layer::Runtime => Ident::new("runtime", Span::call_site()),
            Layer::Account => Ident::new("account", Span::call_site()),
            Layer::Operation => Ident::new("operation", Span::call_site()),
        }
    }

    fn from_str(s: &str, span: Span) -> Result<Self> {
        match s {
            "runtime" => Ok(Layer::Runtime),
            "account" => Ok(Layer::Account),
            "operation" => Ok(Layer::Operation),
            _ => Err(Error::new(
                span,
                format!(
                    "unknown layer `{s}`, expected one of: {}",
                    KNOWN_LAYERS.join(", ")
                ),
            )),
        }
    }
}

/// Parsed representation of a single field in the option struct.
#[allow(dead_code)]
pub struct OptionField {
    /// The field name.
    pub ident: Ident,
    /// The inner type (unwrapped from `Option<T>`).
    pub inner_type: Type,
    /// The full `Option<T>` type.
    pub full_type: Type,
    /// Environment variable name, if any.
    pub env_var: Option<String>,
    /// Whether this field uses merge semantics.
    pub merge: Option<String>,
    /// Whether this field is a nested option group.
    pub nested: bool,
    /// The field's visibility.
    pub vis: syn::Visibility,
}

impl OptionsInput {
    /// Parse a `DeriveInput` into an `OptionsInput`.
    pub fn from_derive_input(ast: &DeriveInput) -> Result<Self> {
        let name = ast.ident.clone();
        let generics = ast.generics.clone();
        let vis = ast.vis.clone();

        let layers = parse_layers_attr(&ast.attrs)?;
        if layers.is_empty() {
            return Err(Error::new(
                ast.ident.span(),
                "missing `#[options(layers(...))]` attribute",
            ));
        }

        let data = match &ast.data {
            Data::Struct(data) => data,
            _ => {
                return Err(Error::new(
                    ast.ident.span(),
                    "CosmosOptions can only be derived for structs",
                ))
            }
        };

        let fields = parse_fields(data)?;

        Ok(OptionsInput {
            name,
            generics,
            vis,
            layers,
            fields,
        })
    }

    /// Returns true if any field has an `#[option(env = "...")]` attribute.
    pub fn has_env_fields(&self) -> bool {
        self.fields.iter().any(|f| f.env_var.is_some())
    }
}

fn parse_layers_attr(attrs: &[syn::Attribute]) -> Result<Vec<Layer>> {
    for attr in attrs {
        if !attr.path().is_ident("options") {
            continue;
        }

        let mut layers = Vec::new();
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("layers") {
                let content;
                syn::parenthesized!(content in meta.input);
                let layer_idents: Punctuated<Ident, Comma> =
                    content.parse_terminated(Ident::parse, Comma)?;

                for ident in &layer_idents {
                    let layer = Layer::from_str(&ident.to_string(), ident.span())?;
                    layers.push(layer);
                }
                Ok(())
            } else {
                Err(meta.error("expected `layers(...)`"))
            }
        })?;

        return Ok(layers);
    }

    Ok(Vec::new())
}

fn parse_fields(data: &DataStruct) -> Result<Vec<OptionField>> {
    let named_fields = match &data.fields {
        Fields::Named(fields) => &fields.named,
        _ => {
            return Err(Error::new(
                data.fields.span(),
                "CosmosOptions requires named fields",
            ))
        }
    };

    let mut result = Vec::new();
    for field in named_fields {
        let ident = field
            .ident
            .clone()
            .ok_or_else(|| Error::new(field.span(), "expected named field"))?;

        let inner_type = extract_option_inner_type(&field.ty).ok_or_else(|| {
            Error::new(
                field.ty.span(),
                format!("field `{ident}` must be `Option<T>`"),
            )
        })?;

        let (env_var, merge, nested) = parse_option_attrs(&field.attrs)?;

        result.push(OptionField {
            ident,
            inner_type,
            full_type: field.ty.clone(),
            env_var,
            merge,
            nested,
            vis: field.vis.clone(),
        });
    }

    Ok(result)
}

fn parse_option_attrs(attrs: &[syn::Attribute]) -> Result<(Option<String>, Option<String>, bool)> {
    let mut env_var = None;
    let mut merge = None;
    let mut nested = false;

    for attr in attrs {
        if !attr.path().is_ident("option") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("env") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                env_var = Some(lit.value());
                Ok(())
            } else if meta.path.is_ident("merge") {
                let value = meta.value()?;
                let lit: syn::LitStr = value.parse()?;
                merge = Some(lit.value());
                Ok(())
            } else if meta.path.is_ident("nested") {
                nested = true;
                Ok(())
            } else {
                Err(meta.error("expected `env = \"...\"`, `merge = \"...\"`, or `nested`"))
            }
        })?;
    }

    // Validate attribute combinations.
    if env_var.is_some() && merge.is_some() {
        return Err(Error::new(
            Span::call_site(),
            "`env` and `merge` cannot be combined on the same field",
        ));
    }
    if env_var.is_some() && nested {
        return Err(Error::new(
            Span::call_site(),
            "`env` and `nested` cannot be combined on the same field",
        ));
    }
    if merge.is_some() && nested {
        return Err(Error::new(
            Span::call_site(),
            "`merge` and `nested` cannot be combined on the same field",
        ));
    }

    Ok((env_var, merge, nested))
}

/// Extracts the inner type `T` from `Option<T>`.
fn extract_option_inner_type(ty: &Type) -> Option<Type> {
    let path = match ty {
        Type::Path(type_path) if type_path.qself.is_none() => &type_path.path,
        _ => return None,
    };

    let segment = path.segments.last()?;
    if segment.ident != "Option" {
        return None;
    }

    let args = match &segment.arguments {
        PathArguments::AngleBracketed(args) => &args.args,
        _ => return None,
    };

    if args.len() != 1 {
        return None;
    }

    match &args[0] {
        GenericArgument::Type(inner) => Some(inner.clone()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layers_parsed_correctly() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account, operation))]
            struct TestOptions {
                pub field_a: Option<String>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        assert_eq!(parsed.layers.len(), 3);
        assert_eq!(parsed.layers[0], Layer::Runtime);
        assert_eq!(parsed.layers[1], Layer::Account);
        assert_eq!(parsed.layers[2], Layer::Operation);
    }

    #[test]
    fn two_layers_parsed() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                pub field_a: Option<u32>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        assert_eq!(parsed.layers.len(), 2);
        assert_eq!(parsed.layers[0], Layer::Runtime);
        assert_eq!(parsed.layers[1], Layer::Account);
    }

    #[test]
    fn missing_layers_attr_errors() {
        let input: DeriveInput = syn::parse_quote! {
            struct TestOptions {
                pub field_a: Option<String>,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("missing `#[options(layers(...))]` attribute", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn unknown_layer_errors() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, global))]
            struct TestOptions {
                pub field_a: Option<String>,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("unknown layer `global`, expected one of: runtime, account, operation", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn non_option_field_errors() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                pub field_a: String,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("field `field_a` must be `Option<T>`", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn field_attrs_parsed() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                #[option(env = "MY_VAR")]
                pub field_a: Option<String>,

                #[option(merge = "extend")]
                pub field_b: Option<Vec<String>>,

                #[option(nested)]
                pub field_c: Option<ChildOptions>,

                pub field_d: Option<u32>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        assert_eq!(parsed.fields.len(), 4);

        assert_eq!(parsed.fields[0].env_var.as_deref(), Some("MY_VAR"));
        assert!(!parsed.fields[0].nested);
        assert!(parsed.fields[0].merge.is_none());

        assert_eq!(parsed.fields[1].merge.as_deref(), Some("extend"));
        assert!(parsed.fields[1].env_var.is_none());

        assert!(parsed.fields[2].nested);

        assert!(parsed.fields[3].env_var.is_none());
        assert!(parsed.fields[3].merge.is_none());
        assert!(!parsed.fields[3].nested);
    }

    #[test]
    fn has_env_fields_true() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                #[option(env = "MY_VAR")]
                pub field_a: Option<String>,
                pub field_b: Option<u32>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        assert!(parsed.has_env_fields());
    }

    #[test]
    fn has_env_fields_false() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                pub field_a: Option<String>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        assert!(!parsed.has_env_fields());
    }

    #[test]
    fn enum_input_errors() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime))]
            enum TestOptions {
                A,
                B,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("CosmosOptions can only be derived for structs", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn extract_option_inner() {
        let ty: Type = syn::parse_quote! { Option<String> };
        let inner = extract_option_inner_type(&ty).unwrap();
        assert_eq!(quote::quote!(#inner).to_string(), "String");
    }

    #[test]
    fn extract_non_option_returns_none() {
        let ty: Type = syn::parse_quote! { String };
        assert!(extract_option_inner_type(&ty).is_none());
    }

    #[test]
    fn env_and_merge_cannot_be_combined() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                #[option(env = "MY_VAR", merge = "extend")]
                pub field_a: Option<Vec<String>>,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("`env` and `merge` cannot be combined on the same field", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn env_and_nested_cannot_be_combined() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                #[option(env = "MY_VAR", nested)]
                pub field_a: Option<ChildOptions>,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("`env` and `nested` cannot be combined on the same field", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }

    #[test]
    fn merge_and_nested_cannot_be_combined() {
        let input: DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            struct TestOptions {
                #[option(merge = "extend", nested)]
                pub field_a: Option<ChildOptions>,
            }
        };
        match OptionsInput::from_derive_input(&input) {
            Err(e) => assert_eq!("`merge` and `nested` cannot be combined on the same field", e.to_string()),
            Ok(_) => panic!("expected error"),
        }
    }
}
