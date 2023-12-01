use super::function_params::{FunctionParam, FunctionParams};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Clone)]
pub struct FunctionCallParamsCode(pub FunctionParams);

impl ToTokens for FunctionCallParamsCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        for FunctionParam {
            variable_name, type_name, ..
        } in self.0.required_params()
        {
            let mut type_name = type_name.clone();
            let is_vec = type_name.is_vec();
            type_name.impl_into(!is_vec);
            params.push(quote! { #variable_name: #type_name });
        }
        let slf = quote! { &self };
        params.insert(0, slf);
        tokens.extend(quote! { #(#params),* })
    }
}
