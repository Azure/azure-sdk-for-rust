// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::parse::OptionsInput;
use crate::Result;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{PathArguments, Type};

/// Returns true if `ty` is a `Vec<T>` type (checking the last path segment).
fn is_vec_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => {
            type_path.path.segments.last().is_some_and(|seg| {
                seg.ident == "Vec" && matches!(seg.arguments, PathArguments::AngleBracketed(_))
            })
        }
        _ => false,
    }
}

/// Returns true if `ty` is the primitive `bool` type.
fn is_bool_type(ty: &Type) -> bool {
    match ty {
        Type::Path(type_path) if type_path.qself.is_none() => type_path.path.is_ident("bool"),
        _ => false,
    }
}

/// Generates the per-field initializer expression that reads `env_var_name`
/// and parses it into the field's type (with comma-splitting for `Vec<T>`).
///
/// Shared by both the base `from_env_vars` and the `from_env_override_vars`
/// constructors so the parse/warn semantics stay identical across the base
/// and `_OVERRIDE` variants.
fn field_init(field_name: &syn::Ident, inner_type: &Type, env_var: &str) -> TokenStream {
    if is_vec_type(inner_type) {
        quote! {
            #field_name: env_var(#env_var)
                .ok()
                .map(|v| v.split(',')
                    .filter_map(|s| {
                        let trimmed = s.trim();
                        match trimmed.parse() {
                            Ok(parsed) => Some(parsed),
                            Err(_) => {
                                ::tracing::warn!(
                                    env_var = #env_var,
                                    value = trimmed,
                                    "failed to parse element from environment variable; skipping",
                                );
                                None
                            }
                        }
                    })
                    .collect())
        }
    } else if is_bool_type(inner_type) {
        // Booleans frequently back operator-facing switches (including the
        // `_OVERRIDE` incident kill switches), so parse them leniently instead
        // of with `bool::from_str` (which accepts only exact lowercase
        // `true`/`false`). Common spellings like `FALSE`, `0`, `off`, and `no`
        // are honored so a kill switch does not silently fail open on a typo.
        quote! {
            #field_name: env_var(#env_var)
                .ok()
                .and_then(|v| match v.trim().to_ascii_lowercase().as_str() {
                    "true" | "1" | "yes" | "on" => Some(true),
                    "false" | "0" | "no" | "off" => Some(false),
                    _ => {
                        ::tracing::warn!(
                            env_var = #env_var,
                            value = %v,
                            "failed to parse boolean environment variable; ignoring",
                        );
                        None
                    }
                })
        }
    } else {
        quote! {
            #field_name: env_var(#env_var)
                .ok()
                .and_then(|v| match v.parse() {
                    Ok(parsed) => Some(parsed),
                    Err(_) => {
                        ::tracing::warn!(
                            env_var = #env_var,
                            value = %v,
                            "failed to parse environment variable; ignoring",
                        );
                        None
                    }
                })
        }
    }
}

/// Generates `from_env()` and `from_env_vars()` methods on the original struct.
///
/// When any field is marked `#[option(env = "...", overridable)]`, also
/// generates `from_env_override()` / `from_env_override_vars()`, which read the
/// `{ENV}_OVERRIDE` kill-switch variant for each overridable field (and leave
/// every other field `None`).
///
/// `from_env_vars` accepts a function `Fn(&str) -> Result<String, VarError>` used
/// to read a single variable, making it testable without touching the real environment.
/// `from_env` delegates to `from_env_vars` with `std::env::var`.
pub fn generate_from_env(input: &OptionsInput) -> Result<TokenStream> {
    if !input.has_env_fields() {
        return Ok(TokenStream::new());
    }

    let struct_name = &input.name;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let field_inits = input.fields.iter().map(|field| {
        let field_name = &field.ident;
        if let Some(ref env_var) = field.env_var {
            field_init(field_name, &field.inner_type, env_var)
        } else {
            quote! { #field_name: None }
        }
    });

    let override_tokens = generate_from_env_override(input);

    Ok(quote! {
        #[automatically_derived]
        impl #impl_generics #struct_name #ty_generics #where_clause {
            /// Creates an instance by reading environment variables.
            ///
            /// Fields with `#[option(env = "...")]` are populated from the
            /// corresponding environment variable. All other fields are `None`.
            pub fn from_env() -> Self {
                Self::from_env_vars(|key| ::std::env::var(key))
            }

            /// Creates an instance using the provided function to read environment variables.
            #[doc(hidden)]
            pub fn from_env_vars(env_var: impl Fn(&str) -> ::std::result::Result<String, ::std::env::VarError>) -> Self {
                Self {
                    #(#field_inits),*
                }
            }

            #override_tokens
        }
    })
}

/// Generates `from_env_override()` / `from_env_override_vars()` when the option
/// group has any `#[option(env = "...", overridable)]` field. Emits nothing
/// otherwise.
fn generate_from_env_override(input: &OptionsInput) -> TokenStream {
    if !input.has_overridable_fields() {
        return TokenStream::new();
    }

    let field_inits = input.fields.iter().map(|field| {
        let field_name = &field.ident;
        if let Some(override_var) = field.override_env_var() {
            field_init(field_name, &field.inner_type, &override_var)
        } else {
            // Non-overridable fields are never sourced from an `_OVERRIDE`
            // variable — the override layer only carries kill-switch values.
            quote! { #field_name: None }
        }
    });

    quote! {
        /// Creates an instance by reading the `{ENV}_OVERRIDE` kill-switch
        /// variables for each `overridable` field.
        ///
        /// Only fields marked `#[option(env = "...", overridable)]` are
        /// populated (from `{ENV}_OVERRIDE`); all other fields are `None`.
        /// The resulting value is intended to seed the highest-priority
        /// override layer of the generated View.
        pub fn from_env_override() -> Self {
            Self::from_env_override_vars(|key| ::std::env::var(key))
        }

        /// Creates an override instance using the provided function to read
        /// environment variables.
        #[doc(hidden)]
        pub fn from_env_override_vars(env_var: impl Fn(&str) -> ::std::result::Result<String, ::std::env::VarError>) -> Self {
            Self {
                #(#field_inits),*
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::OptionsInput;
    use quote::quote;

    #[test]
    fn from_env_generated_when_env_fields_present() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                #[option(env = "MY_VAR_A")]
                pub field_a: Option<String>,
                pub field_b: Option<u32>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_from_env(&parsed).unwrap();

        let expected = quote! {
            #[automatically_derived]
            impl TestOptions {
                /// Creates an instance by reading environment variables.
                ///
                /// Fields with `#[option(env = "...")]` are populated from the
                /// corresponding environment variable. All other fields are `None`.
                pub fn from_env() -> Self {
                    Self::from_env_vars(|key| ::std::env::var(key))
                }

                /// Creates an instance using the provided function to read environment variables.
                #[doc(hidden)]
                pub fn from_env_vars(env_var: impl Fn(&str) -> ::std::result::Result<String, ::std::env::VarError>) -> Self {
                    Self {
                        field_a: env_var("MY_VAR_A")
                            .ok()
                            .and_then(|v| match v.parse() {
                                Ok(parsed) => Some(parsed),
                                Err(_) => {
                                    ::tracing::warn!(
                                        env_var = "MY_VAR_A",
                                        value = %v,
                                        "failed to parse environment variable; ignoring",
                                    );
                                    None
                                }
                            }),
                        field_b: None
                    }
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }

    #[test]
    fn no_output_when_no_env_fields() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                pub field_a: Option<String>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_from_env(&parsed).unwrap();
        assert!(tokens.is_empty());
    }

    #[test]
    fn from_env_override_generated_for_overridable_fields() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account, operation))]
            pub struct TestOptions {
                #[option(env = "MY_VAR_A", overridable)]
                pub field_a: Option<bool>,
                #[option(env = "MY_VAR_B")]
                pub field_b: Option<u32>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_from_env(&parsed).unwrap().to_string();

        // The override constructors are emitted...
        assert!(tokens.contains("from_env_override"));
        assert!(tokens.contains("from_env_override_vars"));
        // ...and read the `_OVERRIDE` variant for the overridable field only.
        assert!(tokens.contains("MY_VAR_A_OVERRIDE"));
        // The non-overridable field's base var must NOT gain an override read.
        assert!(!tokens.contains("MY_VAR_B_OVERRIDE"));
    }

    #[test]
    fn no_from_env_override_without_overridable_fields() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                #[option(env = "MY_VAR_A")]
                pub field_a: Option<String>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_from_env(&parsed).unwrap().to_string();

        assert!(tokens.contains("from_env_vars"));
        assert!(!tokens.contains("from_env_override"));
    }
}
