// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::parse::OptionsInput;
use crate::Result;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Generates a `{Name}Builder` struct and its implementation.
pub fn generate_builder(input: &OptionsInput) -> Result<TokenStream> {
    let struct_name = &input.name;
    let builder_name = format_ident!("{}Builder", input.name);
    let vis = &input.vis;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Builder struct fields mirror the original struct (all Option<T>).
    let struct_fields = input.fields.iter().map(|field| {
        let field_name = &field.ident;
        let full_type = &field.full_type;
        quote! { #field_name: #full_type }
    });

    // Setter methods: each takes the inner type and wraps in Some.
    let setters = input.fields.iter().map(|field| {
        let field_name = &field.ident;
        let setter_name = format_ident!("with_{}", field_name);
        let inner_type = &field.inner_type;
        quote! {
            /// Sets this field on the builder.
            pub fn #setter_name(mut self, value: #inner_type) -> Self {
                self.#field_name = Some(value);
                self
            }
        }
    });

    // Build method: constructs the original struct.
    let build_fields = input.fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: self.#field_name }
    });

    // Default fields for new().
    let default_fields = input.fields.iter().map(|field| {
        let field_name = &field.ident;
        quote! { #field_name: None }
    });

    Ok(quote! {
        /// Builder for constructing option group instances.
        #[automatically_derived]
        #vis struct #builder_name #ty_generics #where_clause {
            #(#struct_fields),*
        }

        #[automatically_derived]
        impl #impl_generics #builder_name #ty_generics #where_clause {
            /// Creates a new builder with all fields set to `None`.
            #vis fn new() -> Self {
                Self {
                    #(#default_fields),*
                }
            }

            #(#setters)*

            /// Consumes the builder and returns the constructed option group.
            pub fn build(self) -> #struct_name #ty_generics {
                #struct_name {
                    #(#build_fields),*
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::OptionsInput;
    use quote::quote;

    #[test]
    fn builder_generated() {
        let input: syn::DeriveInput = syn::parse_quote! {
            #[options(layers(runtime, account))]
            pub struct TestOptions {
                pub field_a: Option<String>,
                pub field_b: Option<u32>,
            }
        };
        let parsed = OptionsInput::from_derive_input(&input).unwrap();
        let tokens = generate_builder(&parsed).unwrap();

        let expected = quote! {
            /// Builder for constructing option group instances.
            #[automatically_derived]
            pub struct TestOptionsBuilder {
                field_a: Option<String>,
                field_b: Option<u32>
            }

            #[automatically_derived]
            impl TestOptionsBuilder {
                /// Creates a new builder with all fields set to `None`.
                pub fn new() -> Self {
                    Self {
                        field_a: None,
                        field_b: None
                    }
                }

                /// Sets this field on the builder.
                pub fn with_field_a(mut self, value: String) -> Self {
                    self.field_a = Some(value);
                    self
                }

                /// Sets this field on the builder.
                pub fn with_field_b(mut self, value: u32) -> Self {
                    self.field_b = Some(value);
                    self
                }

                /// Consumes the builder and returns the constructed option group.
                pub fn build(self) -> TestOptions {
                    TestOptions {
                        field_a: self.field_a,
                        field_b: self.field_b
                    }
                }
            }
        };

        assert_eq!(expected.to_string(), tokens.to_string());
    }
}
