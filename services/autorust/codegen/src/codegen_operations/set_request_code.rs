use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

use crate::spec::WebVerb;

use super::{function_params::FunctionParams, set_request_param_code::SetRequestParamsCode, web_operation_gen::WebOperationGen};
/// Set all body and parameters for the request.
pub struct SetRequestCode {
    pub has_param_api_version: bool,
    pub has_param_x_ms_version: bool,
    pub api_version: String,
    consumes: String,
    pub parameters: FunctionParams,
    has_body_parameter: bool,
    is_post: bool,
}

impl SetRequestCode {
    pub fn new(operation: &WebOperationGen, parameters: &FunctionParams, consumes: String) -> Self {
        let is_post = operation.0.verb == WebVerb::Post;
        Self {
            has_param_api_version: parameters.has_api_version(),
            has_param_x_ms_version: parameters.has_x_ms_version(),
            api_version: operation.api_version().to_string(),
            consumes,
            parameters: parameters.clone(),
            has_body_parameter: operation.0.has_body_parameter(),
            is_post,
        }
    }
}

impl ToTokens for SetRequestCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.has_param_x_ms_version {
            let api_version = &self.api_version;
            tokens.extend(quote! {
                req.insert_header(azure_core::headers::VERSION, #api_version);
            });
        }

        // params
        let build_request_params = SetRequestParamsCode {
            content_type: self.consumes.clone(),
            params: self.parameters.clone(),
        };
        tokens.extend(build_request_params.into_token_stream());

        if !self.has_body_parameter {
            tokens.extend(quote! {
                let req_body = azure_core::EMPTY_BODY;
            });
        }

        // if it is a post and there is no body, set the Content-Length to 0
        if self.is_post && !self.has_body_parameter {
            tokens.extend(quote! {
                req.insert_header(azure_core::headers::CONTENT_LENGTH, "0");
            });
        }
    }
}
