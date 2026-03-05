// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::parse::{Layer, OptionField, OptionsInput};
use crate::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::{PathArguments, Type};

/// Generates the `{Name}View` struct and its accessor methods.
pub fn generate_view(input: &OptionsInput) -> Result<TokenStream> {
    let view_name = format_ident!("{}View", input.name);
    let struct_name = &input.name;
    let vis = &input.vis;

    let has_env = input.has_env_fields();
    let layers = &input.layers;
    let last_layer_idx = layers.len() - 1;

    // Build struct fields: env (optional) + explicit layers.
    // The last layer in the list is owned (operation layer per spec), others are Arc.
    let mut struct_fields = Vec::new();
    let mut new_params = Vec::new();

    if has_env {
        struct_fields.push(quote! { env: ::std::sync::Arc<#struct_name> });
        new_params.push(quote! { env: ::std::sync::Arc<#struct_name> });
    }

    for (i, layer) in layers.iter().enumerate() {
        let field_name = layer.ident();
        if i == last_layer_idx {
            // Last layer is owned (typically operation)
            struct_fields.push(quote! { #field_name: #struct_name });
            new_params.push(quote! { #field_name: #struct_name });
        } else {
            struct_fields.push(quote! { #field_name: ::std::sync::Arc<#struct_name> });
            new_params.push(quote! { #field_name: ::std::sync::Arc<#struct_name> });
        }
    }

    // Build accessor methods
    let accessors = input
        .fields
        .iter()
        .map(|field| generate_accessor(field, layers, has_env))
        .collect::<Result<Vec<_>>>()?;

    // Build the new() constructor body
    let mut new_body_fields = Vec::new();
    if has_env {
        new_body_fields.push(quote! { env });
    }
    for layer in layers {
        let field_name = layer.ident();
        new_body_fields.push(quote! { #field_name });
    }

    Ok(quote! {
        /// Snapshot view across all layers for resolution.
        #[automatically_derived]
        #vis struct #view_name {
            #(#struct_fields),*
        }

        #[automatically_derived]
        impl #view_name {
            /// Creates a new view from layer snapshots.
            #vis fn new(#(#new_params),*) -> Self {
                Self {
                    #(#new_body_fields),*
                }
            }

            #(#accessors)*
        }
    })
}

fn generate_accessor(field: &OptionField, layers: &[Layer], has_env: bool) -> Result<TokenStream> {
    let _field_name = &field.ident;

    if field.nested {
        return generate_nested_accessor(field, layers, has_env);
    }

    if field.merge.is_some() {
        return generate_merge_accessor(field, layers, has_env);
    }

    generate_shadow_accessor(field, layers, has_env)
}

/// Generates a shadow accessor: returns `Option<&T>` walking highest → lowest priority.
fn generate_shadow_accessor(
    field: &OptionField,
    layers: &[Layer],
    has_env: bool,
) -> Result<TokenStream> {
    let field_name = &field.ident;
    let inner_type = &field.inner_type;
    let last_layer_idx = layers.len() - 1;

    // Build the chain: operation → account → runtime → env
    // Layers are stored in order [runtime, account, operation], so we walk in reverse.
    let mut chain_parts = Vec::new();

    for (i, layer) in layers.iter().enumerate().rev() {
        let layer_name = layer.ident();
        if i == last_layer_idx {
            // Owned layer
            chain_parts.push(quote! { self.#layer_name.#field_name.as_ref() });
        } else {
            chain_parts.push(quote! { self.#layer_name.#field_name.as_ref() });
        }
    }

    if has_env {
        chain_parts.push(quote! { self.env.#field_name.as_ref() });
    }

    // Build the chain with .or()
    let first = &chain_parts[0];
    let rest = &chain_parts[1..];

    let chain = rest
        .iter()
        .fold(first.clone(), |acc, part| quote! { #acc.or(#part) });

    Ok(quote! {
        /// Resolves this field across layers (highest priority first).
        pub fn #field_name(&self) -> Option<&#inner_type> {
            #chain
        }
    })
}

/// Generates a merge accessor for `#[option(merge = "extend")]` fields.
fn generate_merge_accessor(
    field: &OptionField,
    layers: &[Layer],
    has_env: bool,
) -> Result<TokenStream> {
    let field_name = &field.ident;
    let inner_type = &field.inner_type;

    // Merge from lowest to highest priority (env → runtime → account → operation).
    let mut extend_stmts = Vec::new();

    if has_env {
        extend_stmts.push(quote! {
            if let Some(ref v) = self.env.#field_name {
                merged.extend(v.clone());
            }
        });
    }

    for layer in layers {
        let layer_name = layer.ident();
        extend_stmts.push(quote! {
            if let Some(ref v) = self.#layer_name.#field_name {
                merged.extend(v.clone());
            }
        });
    }

    Ok(quote! {
        /// Merges this field across all layers (lowest priority first).
        pub fn #field_name(&self) -> #inner_type {
            let mut merged = <#inner_type>::default();
            #(#extend_stmts)*
            merged
        }
    })
}

/// Generates a nested accessor that delegates to a child View.
fn generate_nested_accessor(
    field: &OptionField,
    layers: &[Layer],
    has_env: bool,
) -> Result<TokenStream> {
    let field_name = &field.ident;
    let inner_type = &field.inner_type;
    let view_path = nested_view_path(inner_type)?;
    let last_layer_idx = layers.len() - 1;

    let mut new_args = Vec::new();

    if has_env {
        new_args.push(quote! {
            self.env.#field_name.as_ref()
                .map(|v| ::std::sync::Arc::new(v.clone()))
                .unwrap_or_else(|| ::std::sync::Arc::clone(&default))
        });
    }

    for (i, layer) in layers.iter().enumerate() {
        let layer_name = layer.ident();
        if i == last_layer_idx {
            // Last layer is owned — extract the nested value directly
            new_args.push(quote! {
                self.#layer_name.#field_name.clone().unwrap_or_default()
            });
        } else {
            new_args.push(quote! {
                self.#layer_name.#field_name.as_ref()
                    .map(|v| ::std::sync::Arc::new(v.clone()))
                    .unwrap_or_else(|| ::std::sync::Arc::clone(&default))
            });
        }
    }

    Ok(quote! {
        /// Returns a child View for the nested option group.
        pub fn #field_name(&self) -> #view_path {
            let default = ::std::sync::Arc::new(<#inner_type>::default());
            #view_path::new(#(#new_args),*)
        }
    })
}

/// Constructs the View type path from a nested inner type.
///
/// Given `NestedOpts`, returns the path `NestedOptsView`.
/// Given `inner::NestedOpts`, returns the path `inner::NestedOptsView`.
fn nested_view_path(inner_type: &Type) -> Result<syn::Path> {
    match inner_type {
        Type::Path(type_path) if type_path.qself.is_none() => {
            let mut path = type_path.path.clone();
            let last_seg = path.segments.last_mut().ok_or_else(|| {
                syn::Error::new_spanned(inner_type, "nested type must have at least one path segment")
            })?;
            last_seg.ident = format_ident!("{}View", last_seg.ident);
            last_seg.arguments = PathArguments::None;
            Ok(path)
        }
        _ => Err(syn::Error::new_spanned(
            inner_type,
            "nested option type must be a simple path type",
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::OptionsInput;
    use quote::quote;

    #[test]
    fn view_generated_for_three_layers() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account, operation))]
            pub struct RequestOptions {
                pub consistency_level: Option<String>,
                pub throughput_bucket: Option<usize>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_view(&parsed).unwrap();

        let expected = quote! {
            /// Snapshot view across all layers for resolution.
            #[automatically_derived]
            pub struct RequestOptionsView {
                runtime: ::std::sync::Arc<RequestOptions>,
                account: ::std::sync::Arc<RequestOptions>,
                operation: RequestOptions
            }

            #[automatically_derived]
            impl RequestOptionsView {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: ::std::sync::Arc<RequestOptions>,
                    account: ::std::sync::Arc<RequestOptions>,
                    operation: RequestOptions
                ) -> Self {
                    Self { runtime, account, operation }
                }

                /// Resolves this field across layers (highest priority first).
                pub fn consistency_level(&self) -> Option<&String> {
                    self.operation.consistency_level.as_ref()
                        .or(self.account.consistency_level.as_ref())
                        .or(self.runtime.consistency_level.as_ref())
                }

                /// Resolves this field across layers (highest priority first).
                pub fn throughput_bucket(&self) -> Option<&usize> {
                    self.operation.throughput_bucket.as_ref()
                        .or(self.account.throughput_bucket.as_ref())
                        .or(self.runtime.throughput_bucket.as_ref())
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }

    #[test]
    fn view_generated_for_two_layers() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct ConnectionOptions {
                pub request_timeout: Option<u64>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_view(&parsed).unwrap();

        let expected = quote! {
            /// Snapshot view across all layers for resolution.
            #[automatically_derived]
            pub struct ConnectionOptionsView {
                runtime: ::std::sync::Arc<ConnectionOptions>,
                account: ConnectionOptions
            }

            #[automatically_derived]
            impl ConnectionOptionsView {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: ::std::sync::Arc<ConnectionOptions>,
                    account: ConnectionOptions
                ) -> Self {
                    Self { runtime, account }
                }

                /// Resolves this field across layers (highest priority first).
                pub fn request_timeout(&self) -> Option<&u64> {
                    self.account.request_timeout.as_ref()
                        .or(self.runtime.request_timeout.as_ref())
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }

    #[test]
    fn view_includes_env_field_when_env_attr_present() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                #[option(env = "MY_VAR")]
                pub my_field: Option<String>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_view(&parsed).unwrap();

        let expected = quote! {
            /// Snapshot view across all layers for resolution.
            #[automatically_derived]
            pub struct TestOptionsView {
                env: ::std::sync::Arc<TestOptions>,
                runtime: ::std::sync::Arc<TestOptions>,
                account: TestOptions
            }

            #[automatically_derived]
            impl TestOptionsView {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    env: ::std::sync::Arc<TestOptions>,
                    runtime: ::std::sync::Arc<TestOptions>,
                    account: TestOptions
                ) -> Self {
                    Self { env, runtime, account }
                }

                /// Resolves this field across layers (highest priority first).
                pub fn my_field(&self) -> Option<&String> {
                    self.account.my_field.as_ref()
                        .or(self.runtime.my_field.as_ref())
                        .or(self.env.my_field.as_ref())
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }

    #[test]
    fn view_merge_accessor_generated() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                #[option(merge = "extend")]
                pub headers: Option<Vec<String>>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_view(&parsed).unwrap();

        let inner_type: syn::Type = syn::parse_quote!(Vec<String>);
        let expected = quote! {
            /// Snapshot view across all layers for resolution.
            #[automatically_derived]
            pub struct TestOptionsView {
                runtime: ::std::sync::Arc<TestOptions>,
                account: TestOptions
            }

            #[automatically_derived]
            impl TestOptionsView {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: ::std::sync::Arc<TestOptions>,
                    account: TestOptions
                ) -> Self {
                    Self { runtime, account }
                }

                /// Merges this field across all layers (lowest priority first).
                pub fn headers(&self) -> Vec<String> {
                    let mut merged = <#inner_type>::default();
                    if let Some(ref v) = self.runtime.headers {
                        merged.extend(v.clone());
                    }
                    if let Some(ref v) = self.account.headers {
                        merged.extend(v.clone());
                    }
                    merged
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }

    #[test]
    fn view_nested_accessor_with_qualified_path() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                #[option(nested)]
                pub child: Option<inner::ChildOptions>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_view(&parsed).unwrap();

        let expected = quote! {
            /// Snapshot view across all layers for resolution.
            #[automatically_derived]
            pub struct TestOptionsView {
                runtime: ::std::sync::Arc<TestOptions>,
                account: TestOptions
            }

            #[automatically_derived]
            impl TestOptionsView {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: ::std::sync::Arc<TestOptions>,
                    account: TestOptions
                ) -> Self {
                    Self { runtime, account }
                }

                /// Returns a child View for the nested option group.
                pub fn child(&self) -> inner::ChildOptionsView {
                    let default = ::std::sync::Arc::new(<inner::ChildOptions>::default());
                    inner::ChildOptionsView::new(
                        self.runtime.child.as_ref()
                            .map(|v| ::std::sync::Arc::new(v.clone()))
                            .unwrap_or_else(|| ::std::sync::Arc::clone(&default)),
                        self.account.child.clone().unwrap_or_default()
                    )
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }

    #[test]
    fn view_nested_accessor_with_simple_path() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                #[option(nested)]
                pub child: Option<ChildOptions>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_view(&parsed).unwrap();

        let expected = quote! {
            /// Snapshot view across all layers for resolution.
            #[automatically_derived]
            pub struct TestOptionsView {
                runtime: ::std::sync::Arc<TestOptions>,
                account: TestOptions
            }

            #[automatically_derived]
            impl TestOptionsView {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: ::std::sync::Arc<TestOptions>,
                    account: TestOptions
                ) -> Self {
                    Self { runtime, account }
                }

                /// Returns a child View for the nested option group.
                pub fn child(&self) -> ChildOptionsView {
                    let default = ::std::sync::Arc::new(<ChildOptions>::default());
                    ChildOptionsView::new(
                        self.runtime.child.as_ref()
                            .map(|v| ::std::sync::Arc::new(v.clone()))
                            .unwrap_or_else(|| ::std::sync::Arc::clone(&default)),
                        self.account.child.clone().unwrap_or_default()
                    )
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}
