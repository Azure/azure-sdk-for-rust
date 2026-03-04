// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::parse::OptionsInput;
use crate::Result;
use proc_macro2::TokenStream;
use quote::quote;

/// Generates `from_env()` and `from_env_vars()` methods on the original struct.
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
            // Check if inner type looks like Vec<T> to do comma splitting
            let inner_type = &field.inner_type;
            let inner_type_str = quote!(#inner_type).to_string();
            if inner_type_str.starts_with("Vec") {
                quote! {
                    #field_name: env_var(#env_var)
                        .ok()
                        .map(|v| v.split(',')
                            .filter_map(|s| s.trim().parse().ok())
                            .collect())
                }
            } else {
                quote! {
                    #field_name: env_var(#env_var)
                        .ok()
                        .and_then(|v| v.parse().ok())
                }
            }
        } else {
            quote! { #field_name: None }
        }
    });

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
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::OptionsInput;

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
        let output = tokens.to_string();

        assert!(output.contains("from_env"));
        assert!(output.contains("from_env_vars"));
        assert!(output.contains("MY_VAR_A"));
        assert!(output.contains("field_b : None"));
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
}
