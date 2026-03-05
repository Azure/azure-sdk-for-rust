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
    // All layers are wrapped in Option. Arc layers are Option<Arc<T>>,
    // the last (bottom) layer is Option<&'a T> to avoid cloning.
    let mut struct_fields = Vec::new();
    let mut new_params = Vec::new();

    if has_env {
        struct_fields.push(quote! { env: Option<::std::sync::Arc<#struct_name>> });
        new_params.push(quote! { env: Option<::std::sync::Arc<#struct_name>> });
    }

    for (i, layer) in layers.iter().enumerate() {
        let field_name = layer.ident();
        if i == last_layer_idx {
            // Last layer is borrowed
            struct_fields.push(quote! { #field_name: Option<&'a #struct_name> });
            new_params.push(quote! { #field_name: Option<&'a #struct_name> });
        } else {
            struct_fields.push(quote! { #field_name: Option<::std::sync::Arc<#struct_name>> });
            new_params.push(quote! { #field_name: Option<::std::sync::Arc<#struct_name>> });
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
        #vis struct #view_name<'a> {
            #(#struct_fields),*
        }

        #[automatically_derived]
        impl<'a> #view_name<'a> {
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
    // Each layer is Option<...>, so we unwrap with and_then.
    let mut chain_parts = Vec::new();

    for (i, layer) in layers.iter().enumerate().rev() {
        let layer_name = layer.ident();
        if i == last_layer_idx {
            // Bottom layer is Option<&'a T> — unwrap directly
            chain_parts.push(quote! {
                self.#layer_name.and_then(|l| l.#field_name.as_ref())
            });
        } else {
            // Arc layer is Option<Arc<T>> — unwrap with as_ref first
            chain_parts.push(quote! {
                self.#layer_name.as_ref().and_then(|l| l.#field_name.as_ref())
            });
        }
    }

    if has_env {
        chain_parts.push(quote! {
            self.env.as_ref().and_then(|l| l.#field_name.as_ref())
        });
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
    let last_layer_idx = layers.len() - 1;

    // Merge from lowest to highest priority (env → runtime → account → operation).
    // Each layer is Optional, so we unwrap before checking the field.
    let mut extend_stmts = Vec::new();

    if has_env {
        extend_stmts.push(quote! {
            if let Some(ref env) = self.env {
                if let Some(ref v) = env.#field_name {
                    merged.extend(v.clone());
                }
            }
        });
    }

    for (i, layer) in layers.iter().enumerate() {
        let layer_name = layer.ident();
        if i == last_layer_idx {
            // Bottom layer is Option<&'a T>
            extend_stmts.push(quote! {
                if let Some(#layer_name) = self.#layer_name {
                    if let Some(ref v) = #layer_name.#field_name {
                        merged.extend(v.clone());
                    }
                }
            });
        } else {
            // Arc layer is Option<Arc<T>>
            extend_stmts.push(quote! {
                if let Some(ref #layer_name) = self.#layer_name {
                    if let Some(ref v) = #layer_name.#field_name {
                        merged.extend(v.clone());
                    }
                }
            });
        }
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

    // Build arguments for the child View's new().
    // Arc layers extract the nested field and clone into a new Arc.
    // The bottom layer borrows the nested field (no clone).
    // All results are Option — None when the parent layer or nested field is absent.
    let mut new_args = Vec::new();

    if has_env {
        new_args.push(quote! {
            self.env.as_ref()
                .and_then(|l| l.#field_name.as_ref())
                .map(|v| ::std::sync::Arc::new(v.clone()))
        });
    }

    for (i, layer) in layers.iter().enumerate() {
        let layer_name = layer.ident();
        if i == last_layer_idx {
            // Bottom layer is Option<&'a T> — borrow the nested field
            new_args.push(quote! {
                self.#layer_name.and_then(|l| l.#field_name.as_ref())
            });
        } else {
            // Arc layer — extract nested field and clone into a new Arc
            new_args.push(quote! {
                self.#layer_name.as_ref()
                    .and_then(|l| l.#field_name.as_ref())
                    .map(|v| ::std::sync::Arc::new(v.clone()))
            });
        }
    }

    Ok(quote! {
        /// Returns a child View for the nested option group.
        pub fn #field_name(&self) -> #view_path<'_> {
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
            pub struct RequestOptionsView<'a> {
                runtime: Option<::std::sync::Arc<RequestOptions>>,
                account: Option<::std::sync::Arc<RequestOptions>>,
                operation: Option<&'a RequestOptions>
            }

            #[automatically_derived]
            impl<'a> RequestOptionsView<'a> {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: Option<::std::sync::Arc<RequestOptions>>,
                    account: Option<::std::sync::Arc<RequestOptions>>,
                    operation: Option<&'a RequestOptions>
                ) -> Self {
                    Self { runtime, account, operation }
                }

                /// Resolves this field across layers (highest priority first).
                pub fn consistency_level(&self) -> Option<&String> {
                    self.operation.and_then(|l| l.consistency_level.as_ref())
                        .or(self.account.as_ref().and_then(|l| l.consistency_level.as_ref()))
                        .or(self.runtime.as_ref().and_then(|l| l.consistency_level.as_ref()))
                }

                /// Resolves this field across layers (highest priority first).
                pub fn throughput_bucket(&self) -> Option<&usize> {
                    self.operation.and_then(|l| l.throughput_bucket.as_ref())
                        .or(self.account.as_ref().and_then(|l| l.throughput_bucket.as_ref()))
                        .or(self.runtime.as_ref().and_then(|l| l.throughput_bucket.as_ref()))
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
            pub struct ConnectionOptionsView<'a> {
                runtime: Option<::std::sync::Arc<ConnectionOptions>>,
                account: Option<&'a ConnectionOptions>
            }

            #[automatically_derived]
            impl<'a> ConnectionOptionsView<'a> {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: Option<::std::sync::Arc<ConnectionOptions>>,
                    account: Option<&'a ConnectionOptions>
                ) -> Self {
                    Self { runtime, account }
                }

                /// Resolves this field across layers (highest priority first).
                pub fn request_timeout(&self) -> Option<&u64> {
                    self.account.and_then(|l| l.request_timeout.as_ref())
                        .or(self.runtime.as_ref().and_then(|l| l.request_timeout.as_ref()))
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
            pub struct TestOptionsView<'a> {
                env: Option<::std::sync::Arc<TestOptions>>,
                runtime: Option<::std::sync::Arc<TestOptions>>,
                account: Option<&'a TestOptions>
            }

            #[automatically_derived]
            impl<'a> TestOptionsView<'a> {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    env: Option<::std::sync::Arc<TestOptions>>,
                    runtime: Option<::std::sync::Arc<TestOptions>>,
                    account: Option<&'a TestOptions>
                ) -> Self {
                    Self { env, runtime, account }
                }

                /// Resolves this field across layers (highest priority first).
                pub fn my_field(&self) -> Option<&String> {
                    self.account.and_then(|l| l.my_field.as_ref())
                        .or(self.runtime.as_ref().and_then(|l| l.my_field.as_ref()))
                        .or(self.env.as_ref().and_then(|l| l.my_field.as_ref()))
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
            pub struct TestOptionsView<'a> {
                runtime: Option<::std::sync::Arc<TestOptions>>,
                account: Option<&'a TestOptions>
            }

            #[automatically_derived]
            impl<'a> TestOptionsView<'a> {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: Option<::std::sync::Arc<TestOptions>>,
                    account: Option<&'a TestOptions>
                ) -> Self {
                    Self { runtime, account }
                }

                /// Merges this field across all layers (lowest priority first).
                pub fn headers(&self) -> Vec<String> {
                    let mut merged = <#inner_type>::default();
                    if let Some(ref runtime) = self.runtime {
                        if let Some(ref v) = runtime.headers {
                            merged.extend(v.clone());
                        }
                    }
                    if let Some(account) = self.account {
                        if let Some(ref v) = account.headers {
                            merged.extend(v.clone());
                        }
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
            pub struct TestOptionsView<'a> {
                runtime: Option<::std::sync::Arc<TestOptions>>,
                account: Option<&'a TestOptions>
            }

            #[automatically_derived]
            impl<'a> TestOptionsView<'a> {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: Option<::std::sync::Arc<TestOptions>>,
                    account: Option<&'a TestOptions>
                ) -> Self {
                    Self { runtime, account }
                }

                /// Returns a child View for the nested option group.
                pub fn child(&self) -> inner::ChildOptionsView<'_> {
                    inner::ChildOptionsView::new(
                        self.runtime.as_ref()
                            .and_then(|l| l.child.as_ref())
                            .map(|v| ::std::sync::Arc::new(v.clone())),
                        self.account.and_then(|l| l.child.as_ref())
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
            pub struct TestOptionsView<'a> {
                runtime: Option<::std::sync::Arc<TestOptions>>,
                account: Option<&'a TestOptions>
            }

            #[automatically_derived]
            impl<'a> TestOptionsView<'a> {
                /// Creates a new view from layer snapshots.
                pub fn new(
                    runtime: Option<::std::sync::Arc<TestOptions>>,
                    account: Option<&'a TestOptions>
                ) -> Self {
                    Self { runtime, account }
                }

                /// Returns a child View for the nested option group.
                pub fn child(&self) -> ChildOptionsView<'_> {
                    ChildOptionsView::new(
                        self.runtime.as_ref()
                            .and_then(|l| l.child.as_ref())
                            .map(|v| ::std::sync::Arc::new(v.clone())),
                        self.account.and_then(|l| l.child.as_ref())
                    )
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}
