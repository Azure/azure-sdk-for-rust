use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use super::function_params::{FunctionParam, FunctionParams};

/// The setter functions for the request builder.
#[derive(Clone)]
pub struct RequestBuilderSettersCode {
    parameters: FunctionParams,
}

impl RequestBuilderSettersCode {
    pub fn new(parameters: &FunctionParams) -> Self {
        Self {
            parameters: parameters.clone(),
        }
    }
}

impl ToTokens for RequestBuilderSettersCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for param in self.parameters.optional_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            let is_vec = type_name.is_vec();
            let mut type_name = type_name.clone();
            type_name.optional(false);
            type_name.impl_into(!is_vec);
            let mut value = if type_name.has_impl_into() {
                quote! { #variable_name.into() }
            } else {
                quote! { #variable_name }
            };
            if !is_vec {
                value = quote! { Some(#value) };
            }
            let doc_comment = match &param.description {
                Some(desc) if !desc.is_empty() => quote! { #[ doc = #desc ] },
                _ => quote! {},
            };
            tokens.extend(quote! {
                #doc_comment
                pub fn #variable_name(mut self, #variable_name: #type_name) -> Self {
                    self.#variable_name = #value;
                    self
                }
            });
        }
    }
}
