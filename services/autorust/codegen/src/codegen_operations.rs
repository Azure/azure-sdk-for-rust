use crate::{
    codegen::{add_option, create_generated_by_header, get_type_name_for_schema_ref, Error},
    codegen::{parse_params, PARAM_RE},
    identifier::ident,
    identifier::SnakeCaseIdent,
    spec::{WebOperation, WebParameter, WebVerb},
    status_codes::{get_error_responses, get_response_type_name, get_success_responses, has_default_response},
    status_codes::{get_response_type_ident, get_status_code_ident},
    CodeGen,
};
use autorust_openapi::{CollectionFormat, ParameterType, Response};
use heck::CamelCase;
use heck::SnakeCase;
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::{BTreeSet, HashSet};

fn error_variant(operation: &WebOperation) -> Result<TokenStream, Error> {
    let function = operation.rust_function_name().to_camel_case();
    if let Some(module) = operation.rust_module_name() {
        let module = module.to_camel_case();
        ident(&format!("{}_{}", module, function)).map_err(Error::EnumVariantName)
    } else {
        ident(&function).map_err(Error::ModuleName)
    }
}

fn error_fqn(operation: &WebOperation) -> Result<TokenStream, Error> {
    let function = ident(&operation.rust_function_name()).map_err(Error::FunctionName)?;
    if let Some(module) = operation.rust_module_name() {
        let module = ident(&module).map_err(Error::ModuleName)?;
        Ok(quote! { #module::#function::Error })
    } else {
        Ok(quote! { #function::Error })
    }
}

pub fn create_client(modules: &[String]) -> Result<TokenStream, Error> {
    let mut clients = TokenStream::new();
    for md in modules {
        let md = md.to_snake_case_ident().map_err(Error::ModuleName)?;
        clients.extend(quote! {
            pub fn #md(&self) -> #md::Client {
                #md::Client(self.clone())
            }
        });
    }

    let mut code = TokenStream::new();
    code.extend(quote! {

        #[derive(Clone)]
        pub struct Client {
            endpoint: String,
            credential: std::sync::Arc<dyn azure_core::TokenCredential>,
            scopes: Vec<String>,
            pipeline: azure_core::pipeline::Pipeline,
        }

        #[derive(Clone)]
        pub struct ClientBuilder {
            credential: std::sync::Arc<dyn azure_core::TokenCredential>,
            endpoint: Option<String>,
            scopes: Option<Vec<String>>,
        }

        pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;

        impl ClientBuilder {
            pub fn new(credential: std::sync::Arc<dyn azure_core::TokenCredential>) -> Self {
                Self {
                    credential,
                    endpoint: None,
                    scopes: None,
                }
            }

            pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
                self.endpoint = Some(endpoint.into());
                self
            }

            pub fn scopes(mut self, scopes: &[&str]) -> Self {
                self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
                self
            }

            pub fn build(self) -> Client {
                let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
                let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
                Client::new(endpoint, self.credential, scopes)
            }
        }

        impl Client {
            pub fn endpoint(&self) -> &str {
                self.endpoint.as_str()
            }
            pub fn credential(&self) -> &dyn azure_core::TokenCredential {
                self.credential.as_ref()
            }
            pub fn scopes(&self) -> Vec<&str> {
                self.scopes.iter().map(String::as_str).collect()
            }
            pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> Result<azure_core::Response, azure_core::Error> {
                let mut context = azure_core::Context::default();
                let mut request = request.into();
                self.pipeline.send(&mut context, &mut request).await
            }
            pub fn new(endpoint: impl Into<String>, credential: std::sync::Arc<dyn azure_core::TokenCredential>, scopes: Vec<String>) -> Self {
                let endpoint = endpoint.into();
                let pipeline = azure_core::pipeline::Pipeline::new(
                    option_env!("CARGO_PKG_NAME"),
                    option_env!("CARGO_PKG_VERSION"),
                    azure_core::ClientOptions::default(),
                    Vec::new(),
                    Vec::new(),
                );
                Self {
                    endpoint,
                    credential,
                    scopes,
                    pipeline,
                }
            }

            #clients
        }
    });
    Ok(code)
}

pub fn create_operations(cg: &CodeGen) -> Result<TokenStream, Error> {
    let mut file = TokenStream::new();
    file.extend(create_generated_by_header());
    file.extend(quote! {
        #![allow(unused_mut)]
        #![allow(unused_variables)]
        #![allow(unused_imports)]
        use super::{API_VERSION, models};
    });
    let mut operations_code: IndexMap<Option<String>, OperationCode> = IndexMap::new();
    // println!("input_files {:?}", cg.input_files());

    let operations = cg.spec.operations()?;
    let module_names: BTreeSet<_> = operations.iter().flat_map(|op| op.rust_module_name()).collect();
    let module_names: Vec<_> = module_names.into_iter().collect();
    file.extend(create_client(&module_names)?);

    let mut errors = TokenStream::new();
    for operation in &operations {
        let variant = error_variant(operation)?;
        let fqn = error_fqn(operation)?;
        errors.extend(quote! {
            #[error(transparent)]
            #variant(#[from] #fqn),
        });
    }

    file.extend(quote! {
        #[non_exhaustive]
        #[derive(Debug, thiserror::Error)]
        #[allow(non_camel_case_types)]
        pub enum Error {
            #errors
        }
    });

    for operation in &operations {
        let module_name = operation.rust_module_name();
        let code = create_operation_code(cg, operation)?;
        // append code to existing module if it already exists
        match operations_code.get_mut(&module_name) {
            Some(operation_code) => {
                let OperationCode {
                    builder_instance_code,
                    module_code,
                } = code;
                operation_code.builder_instance_code.extend(builder_instance_code);
                operation_code.module_code.extend(module_code);
            }
            None => {
                operations_code.insert(module_name, code);
            }
        }
    }

    for (module_name, operation_code) in operations_code {
        let OperationCode {
            builder_instance_code,
            module_code,
        } = operation_code;
        match module_name {
            Some(module_name) => {
                let name = ident(&module_name).map_err(Error::ModuleName)?;
                file.extend(quote! {
                    pub mod #name {
                        use super::{API_VERSION, models};
                        pub struct Client(pub(crate) super::Client);
                        impl Client {
                            #builder_instance_code
                        }
                        #module_code
                    }
                });
            }
            None => {
                file.extend(quote! {
                    impl Client {
                        #builder_instance_code
                    }
                    #module_code
                });
            }
        }
    }
    Ok(file)
}

struct OperationCode {
    builder_instance_code: TokenStream,
    module_code: TokenStream,
}

// Create code for the web operation
fn create_operation_code(cg: &CodeGen, operation: &WebOperation) -> Result<OperationCode, Error> {
    let fname = ident(&operation.rust_function_name()).map_err(Error::FunctionName)?;

    let params = parse_params(&operation.path);
    // println!("path params {:#?}", params);
    let params: Result<Vec<_>, Error> = params
        .iter()
        .map(|s| {
            let param = s.to_snake_case_ident().map_err(Error::ParamName)?;
            Ok(quote! { &self.#param })
        })
        .collect();
    let params = params?;
    let url_str_args = quote! { #(#params),* };

    let fpath = format!("{{}}{}", &format_path(&operation.path));

    let parameters = operation.parameters();
    let param_names: HashSet<_> = parameters.iter().map(|p| p.name()).collect();
    let has_param_api_version = param_names.contains("api-version");
    let mut skip = HashSet::new();
    if cg.spec.api_version().is_some() {
        skip.insert("api-version");
    }
    let parameters: Vec<_> = parameters.clone().into_iter().filter(|p| !skip.contains(p.name())).collect();

    let mut ts_request_builder = TokenStream::new();

    let mut is_post = false;
    let req_verb = match operation.verb {
        WebVerb::Get => quote! { req_builder = req_builder.method(http::Method::GET); },
        WebVerb::Post => {
            is_post = true;
            quote! { req_builder = req_builder.method(http::Method::POST); }
        }
        WebVerb::Put => quote! { req_builder = req_builder.method(http::Method::PUT); },
        WebVerb::Patch => quote! { req_builder = req_builder.method(http::Method::PATCH); },
        WebVerb::Delete => quote! { req_builder = req_builder.method(http::Method::DELETE); },
        WebVerb::Options => quote! { req_builder = req_builder.method(http::Method::OPTIONS); },
        WebVerb::Head => quote! { req_builder = req_builder.method(http::Method::HEAD); },
    };
    ts_request_builder.extend(req_verb);

    // auth
    ts_request_builder.extend(quote! {
        let credential = self.client.credential();
        let token_response = credential
            .get_token(&self.client.scopes().join(" "))
            .await
            .map_err(Error::GetToken)?;
        req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
    });

    // api-version param
    if has_param_api_version {
        if let Some(_api_version) = cg.spec.api_version() {
            ts_request_builder.extend(quote! {
                url.query_pairs_mut().append_pair("api-version", super::API_VERSION);
            });
        }
    }

    let has_content_type_header = parameters
        .iter()
        .any(|p| p.name().to_snake_case() == "content_type" && p.type_() == &ParameterType::Header);

    // params
    let mut has_body_parameter = false;
    for param in &parameters {
        let param_name = param.name();
        let param_name_var = get_param_name(param)?;
        let required = param.required();
        let is_bool = param.is_bool();
        match param.type_() {
            ParameterType::Path => {} // handled above
            ParameterType::Query => {
                let is_array = param.is_array();
                let query_body = if is_array {
                    let collection_format = param.collection_format();
                    match collection_format {
                        CollectionFormat::Multi => Some(
                            if param.is_string(){
                                quote! {
                                    for value in &self.#param_name_var {
                                        url.query_pairs_mut().append_pair(#param_name, value);
                                    }
                                }
                            } else {
                                quote! {
                                    for value in &self.#param_name_var {
                                        url.query_pairs_mut().append_pair(#param_name, &value.to_string());
                                    }
                                }
                            }
                        ),
                        CollectionFormat::Csv | // TODO #71
                        CollectionFormat::Ssv |
                        CollectionFormat::Tsv |
                        CollectionFormat::Pipes => None,
                    }
                } else {
                    Some(if param.is_string() {
                        quote! {
                            url.query_pairs_mut().append_pair(#param_name, #param_name_var);
                        }
                    } else {
                        quote! {
                            url.query_pairs_mut().append_pair(#param_name, &#param_name_var.to_string());
                        }
                    })
                };
                if let Some(query_body) = query_body {
                    if required || is_array {
                        ts_request_builder.extend(query_body);
                    } else {
                        ts_request_builder.extend(quote! {
                            if let Some(#param_name_var) = &self.#param_name_var {
                                #query_body
                            }
                        });
                    }
                }
            }
            ParameterType::Header => {
                // println!("header builder: {:?}", param);
                if required {
                    if is_bool {
                        ts_request_builder.extend(quote! {
                            req_builder = req_builder.header(#param_name, &self.#param_name_var .to_string());
                        });
                    } else {
                        ts_request_builder.extend(quote! {
                            req_builder = req_builder.header(#param_name, &self.#param_name_var);
                        });
                    }
                } else if is_bool {
                    ts_request_builder.extend(quote! {
                        if let Some(#param_name_var) = &self.#param_name_var {
                            req_builder = req_builder.header(#param_name, #param_name_var .to_string());
                        }
                    });
                } else {
                    ts_request_builder.extend(quote! {
                        if let Some(#param_name_var) = &self.#param_name_var {
                            req_builder = req_builder.header(#param_name, #param_name_var);
                        }
                    });
                }
            }
            ParameterType::Body => {
                has_body_parameter = true;
                let set_content_type = if !has_content_type_header {
                    let json_content_type = cg.get_request_content_type_json();
                    quote! {
                        req_builder = req_builder.header("content-type", #json_content_type);
                    }
                } else {
                    quote! {}
                };

                if required {
                    ts_request_builder.extend(quote! {
                        #set_content_type
                        let req_body = azure_core::to_json(&self.#param_name_var).map_err(Error::Serialize)?;
                    });
                } else {
                    ts_request_builder.extend(quote! {
                        let req_body =
                            if let Some(#param_name_var) = &self.#param_name_var {
                                #set_content_type
                                azure_core::to_json(#param_name_var).map_err(Error::Serialize)?
                            } else {
                                bytes::Bytes::from_static(azure_core::EMPTY_BODY)
                            };
                    });
                }
            }
            ParameterType::FormData => {
                ts_request_builder.extend(quote! {
                    unimplemented!("form data not yet supported");
                });
                // https://github.com/Azure/azure-sdk-for-rust/issues/500
                // if required {
                //     cargo run --example gen_svc --release
                //         req_builder = req_builder.form(&self.#param_name_var);
                //     });
                // } else {
                //     ts_request_builder.extend(quote! {
                //         if let Some(#param_name_var) = &self.#param_name_var {
                //             req_builder = req_builder.form(#param_name_var);
                //         }
                //     });
                // }
            }
        }
    }

    if !has_body_parameter {
        ts_request_builder.extend(quote! {
            let req_body = bytes::Bytes::from_static(azure_core::EMPTY_BODY);
        });
    }

    // if it is a post and there is no body, set the Content-Length to 0
    if is_post && !has_body_parameter {
        ts_request_builder.extend(quote! {
            req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        });
    }

    let responses = &operation.responses;
    let success_responses = get_success_responses(responses);
    let error_responses = get_error_responses(responses);
    let is_single_response = success_responses.len() == 1;
    let has_default_response = has_default_response(responses);

    let fresponse = if is_single_response {
        let tp = create_response_type(&success_responses[0])?.unwrap_or(quote! { () });
        quote! { std::result::Result<#tp, Error> }
    } else {
        quote! { std::result::Result<Response, Error> }
    };

    let mut response_enum = TokenStream::new();
    if !is_single_response {
        let mut success_responses_ts = TokenStream::new();
        for (status_code, rsp) in &success_responses {
            let tp = create_response_type(rsp)?;
            let tp = match tp {
                Some(tp) => quote! { (#tp) },
                None => quote! {},
            };
            let enum_type_name = get_response_type_ident(status_code)?;
            success_responses_ts.extend(quote! { #enum_type_name#tp, })
        }
        response_enum.extend(quote! {
            #[derive(Debug)]
            pub enum Response {
                #success_responses_ts
            }
        });
    }

    let mut error_responses_ts = TokenStream::new();
    for (status_code, rsp) in &error_responses {
        let tp = create_response_type(rsp)?;
        let tp = match tp {
            Some(tp) => quote! { value: #tp, },
            None => quote! {},
        };
        let response_type = &get_response_type_name(status_code)?;
        if response_type == "DefaultResponse" {
            error_responses_ts.extend(quote! {
                #[error("HTTP status code {}", status_code)]
                DefaultResponse { status_code: http::StatusCode, #tp },
            });
        } else {
            let response_type = ident(response_type).map_err(Error::ResponseTypeName)?;
            error_responses_ts.extend(quote! {
                #[error("Error response #response_type")]
                #response_type { #tp },
            });
        }
    }
    if !has_default_response {
        error_responses_ts.extend(quote! {
            #[error("Unexpected HTTP status code {}", status_code)]
            UnexpectedResponse { status_code: http::StatusCode, body: bytes::Bytes },
        });
    }

    let mut match_status = TokenStream::new();
    for (status_code, rsp) in &success_responses {
        match status_code {
            autorust_openapi::StatusCode::Code(_) => {
                let tp = create_response_type(rsp)?;
                let rsp_value = create_rsp_value(tp.as_ref());
                let status_code_name = get_status_code_ident(status_code)?;
                let response_type_name = get_response_type_ident(status_code)?;
                if is_single_response {
                    match tp {
                        Some(_tp) => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                    #rsp_value
                                    Ok(rsp_value)
                                }
                            });
                        }
                        None => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    Ok(())
                                }
                            });
                        }
                    }
                } else {
                    match tp {
                        Some(_tp) => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                    #rsp_value
                                    Ok(Response::#response_type_name(rsp_value))
                                }
                            });
                        }
                        None => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    Ok(Response::#response_type_name)
                                }
                            });
                        }
                    }
                }
            }
            autorust_openapi::StatusCode::Default => {}
        }
    }
    for (status_code, rsp) in &error_responses {
        match status_code {
            autorust_openapi::StatusCode::Code(_) => {
                let tp = create_response_type(rsp)?;
                let rsp_value = create_rsp_value(tp.as_ref());
                let status_code_name = get_status_code_ident(status_code)?;
                let response_type_name = get_response_type_ident(status_code)?;
                match tp {
                    Some(_tp) => {
                        match_status.extend(quote! {
                            http::StatusCode::#status_code_name => {
                                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                #rsp_value
                                Err(Error::#response_type_name{value: rsp_value})
                            }
                        });
                    }
                    None => {
                        match_status.extend(quote! {
                            http::StatusCode::#status_code_name => {
                                Err(Error::#response_type_name{})
                            }
                        });
                    }
                }
            }
            autorust_openapi::StatusCode::Default => {}
        }
    }
    // default must be last
    if has_default_response {
        for (status_code, rsp) in responses {
            match status_code {
                autorust_openapi::StatusCode::Code(_) => {}
                autorust_openapi::StatusCode::Default => {
                    let tp = create_response_type(rsp)?;
                    let rsp_value = create_rsp_value(tp.as_ref());
                    match tp {
                        Some(_tp) => {
                            match_status.extend(quote! {
                                status_code => {
                                    let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                                    #rsp_value
                                    Err(Error::DefaultResponse{status_code, value: rsp_value})
                                }
                            });
                        }
                        None => {
                            match_status.extend(quote! {
                                status_code => {
                                    Err(Error::DefaultResponse{status_code})
                                }
                            });
                        }
                    }
                }
            }
        }
    } else {
        match_status.extend(quote! {
            status_code => {
                let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await.map_err(Error::ResponseBytes)?;
                Err(Error::UnexpectedResponse{status_code, body: rsp_body.clone()})
            }
        });
    }

    let builder_instance_code = create_builder_instance_code(&fname, &parameters)?;
    let builder_struct_code = create_builder_struct_code(&parameters)?;
    let builder_setters_code = create_builder_setters_code(&parameters)?;

    let module_code = quote! {

        pub mod #fname {
            use super::{API_VERSION, models};

            #response_enum

            #[derive(Debug, thiserror::Error)]
            pub enum Error {
                #error_responses_ts
                #[error("Failed to parse request URL: {0}")]
                ParseUrl(url::ParseError),
                #[error("Failed to build request: {0}")]
                BuildRequest(http::Error),
                #[error("Failed to serialize request body: {0}")]
                Serialize(serde_json::Error),
                #[error("Failed to get access token: {0}")]
                GetToken(azure_core::Error),
                #[error("Failed to execute request: {0}")]
                SendRequest(azure_core::Error),
                #[error("Failed to get response bytes: {0}")]
                ResponseBytes(azure_core::StreamError),
                #[error("Failed to deserialize response: {0}, body: {1:?}")]
                Deserialize(serde_json::Error, bytes::Bytes),
            }

            #builder_struct_code

            impl Builder {
                #builder_setters_code
                pub fn into_future(self) -> futures::future::BoxFuture<'static, #fresponse> {
                    Box::pin(async move {
                        let url_str = &format!(#fpath, &self.client.endpoint, #url_str_args);
                        let mut url = url::Url::parse(url_str).map_err(Error::ParseUrl)?;
                        let mut req_builder = http::request::Builder::new();
                        #ts_request_builder
                        req_builder = req_builder.uri(url.as_str());
                        let req = req_builder.body(req_body).map_err(Error::BuildRequest)?;
                        let rsp = self.client.send(req).await.map_err(Error::SendRequest)?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            #match_status
                        }
                    })
                }
            }

        }

    };

    Ok(OperationCode {
        builder_instance_code,
        module_code,
    })
}

fn create_rsp_value(tp: Option<&TokenStream>) -> TokenStream {
    if tp.map(|tp| tp.to_string()) == Some("bytes :: Bytes".to_owned()) {
        quote! {
            let rsp_value = rsp_body.clone();
        }
    } else {
        quote! {
            let rsp_value: #tp = serde_json::from_slice(&rsp_body).map_err(|source| Error::Deserialize(source, rsp_body.clone()))?;
        }
    }
}

fn format_path(path: &str) -> String {
    PARAM_RE.replace_all(path, "{}").to_string()
}

fn create_function_params_code(parameters: &[&WebParameter]) -> Result<TokenStream, Error> {
    let mut params: Vec<TokenStream> = Vec::new();
    for param in parameters.iter().filter(|p| p.required()) {
        let name = get_param_name(param)?;
        let tp = get_param_type(param, true, true)?;
        params.push(quote! { #name: #tp });
    }
    let slf = quote! { &self };
    params.insert(0, slf);
    Ok(quote! { #(#params),* })
}

fn create_builder_instance_code(fname: &TokenStream, parameters: &[&WebParameter]) -> Result<TokenStream, Error> {
    let fparams = create_function_params_code(parameters)?;
    let mut params: Vec<TokenStream> = Vec::new();
    params.push(quote! { client: self.0.clone() });
    for param in parameters.iter().filter(|p| p.required()) {
        let name = get_param_name(param)?;
        if param.type_is_ref()? {
            params.push(quote! { #name: #name.into() });
        } else {
            params.push(quote! { #name });
        }
    }
    for param in parameters.iter().filter(|p| !p.required()) {
        let name = get_param_name(param)?;
        params.push(quote! { #name: None });
    }
    Ok(quote! {
        pub fn #fname(#fparams) -> #fname::Builder {
            #fname::Builder {
                #(#params),*
            }
        }
    })
}

fn create_builder_struct_code(parameters: &[&WebParameter]) -> Result<TokenStream, Error> {
    let mut params: Vec<TokenStream> = Vec::new();
    params.push(quote! { pub(crate) client: crate::operations::Client });
    for param in parameters.iter().filter(|p| p.required()) {
        let name = get_param_name(param)?;
        let tp = get_param_type(param, false, true)?;
        params.push(quote! { pub(crate) #name: #tp });
    }
    for param in parameters.iter().filter(|p| !p.required()) {
        let name = get_param_name(param)?;
        let tp = get_param_type(param, false, true)?;
        params.push(quote! { pub(crate) #name: #tp });
    }
    Ok(quote! {
        #[derive(Clone)]
        pub struct Builder {
            #(#params),*
        }
    })
}

fn create_builder_setters_code(parameters: &[&WebParameter]) -> Result<TokenStream, Error> {
    let mut setters = TokenStream::new();
    for param in parameters.iter().filter(|p| !p.required()) {
        let name = &get_param_name(param)?;
        let tp = get_param_type(param, true, false)?;
        let value =
            if param.type_is_ref()? {
                quote! { #name.into() }
            } else {
                name.clone()
            };
        setters.extend(quote! {
            pub fn #name(mut self, #name: #tp) -> Self {
                self.#name = Some(#value);
                self
            }
        });
    }
    Ok(setters)
}

fn get_param_name(param: &WebParameter) -> Result<TokenStream, Error> {
    param.name().to_snake_case_ident().map_err(Error::ParamName)
}

fn get_param_type(param: &WebParameter, as_ref: bool, may_be_option: bool) -> Result<TokenStream, Error> {
    let is_required = param.required();
    let is_array = param.is_array();
    let is_option = may_be_option && !(is_required || is_array);
    let tp = param.type_name()?.to_token_stream(as_ref, true)?;
    Ok(add_option(is_option, tp))
}

fn create_response_type(rsp: &Response) -> Result<Option<TokenStream>, Error> {
    if let Some(schema) = &rsp.schema {
        Ok(Some(get_type_name_for_schema_ref(schema)?.to_token_stream(false, true)?))
    } else {
        Ok(None)
    }
}
