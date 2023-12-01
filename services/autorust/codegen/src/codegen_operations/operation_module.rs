use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};

use super::{
    request_builder_into_future::RequestBuilderIntoFutureCode, request_builder_send::RequestBuilderSendCode,
    request_builder_setter::RequestBuilderSettersCode, request_builder_struct::RequestBuilderStructCode, response_code::ResponseCode,
};
pub struct OperationModuleCode {
    pub module_name: Ident,
    pub response_code: ResponseCode,
    pub request_builder_struct_code: RequestBuilderStructCode,
    pub request_builder_setters_code: RequestBuilderSettersCode,
    pub request_builder_send_code: RequestBuilderSendCode,
    pub request_builder_intofuture_code: RequestBuilderIntoFutureCode,
}
impl ToTokens for OperationModuleCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            module_name,
            response_code,
            request_builder_struct_code,
            request_builder_setters_code,
            request_builder_send_code,
            request_builder_intofuture_code,
        } = &self;
        tokens.extend(quote! {
            pub mod #module_name {
                use super::models;
                #[cfg(target_arch = "wasm32")]
                use futures::future::LocalBoxFuture as BoxFuture;
                #[cfg(not(target_arch = "wasm32"))]
                use futures::future::BoxFuture as BoxFuture;

                #response_code

                #request_builder_struct_code

                impl RequestBuilder {
                    #request_builder_setters_code
                    #request_builder_send_code
                }

                #request_builder_intofuture_code
            }
        })
    }
}
