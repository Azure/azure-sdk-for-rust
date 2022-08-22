use crate::{
    codegen::{parse_path_params, type_name_gen, PARAM_RE},
    codegen::{parse_query_params, TypeNameCode},
    identifier::{parse_ident, SnakeCaseIdent},
    spec::{get_type_name_for_schema_ref, WebOperation, WebParameter, WebVerb},
    status_codes::get_success_responses,
    status_codes::{get_response_type_ident, get_status_code_ident},
    CodeGen,
};
use crate::{content_type, Result};
use autorust_openapi::{CollectionFormat, ParameterType, Response};
use heck::ToPascalCase;
use heck::ToSnakeCase;
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::BTreeSet;

pub mod query_param {
    pub const API_VERSION: &str = "api-version";
}

fn error_variant(operation: &WebOperationGen) -> Result<Ident> {
    let function = operation.rust_function_name().to_pascal_case();
    if let Some(module) = operation.rust_module_name() {
        let module = module.to_pascal_case();
        parse_ident(&format!("{}_{}", module, function))
    } else {
        parse_ident(&function)
    }
}

fn error_fqn(operation: &WebOperationGen) -> Result<TokenStream> {
    let function = parse_ident(&operation.rust_function_name())?;
    if let Some(module) = operation.rust_module_name() {
        let module = parse_ident(&module)?;
        Ok(quote! { #module::#function::Error })
    } else {
        Ok(quote! { #function::Error })
    }
}

pub fn create_client(modules: &[String], endpoint: Option<&str>) -> Result<TokenStream> {
    let mut clients = TokenStream::new();
    for md in modules {
        let client = format!("{md}_client").to_snake_case_ident()?;
        let md = md.to_snake_case_ident()?;
        clients.extend(quote! {
            pub fn #client(&self) -> #md::Client {
                #md::Client(self.clone())
            }
        });
    }

    let public_cloud = quote! {
        pub const DEFAULT_ENDPOINT: &str = azure_core::resource_manager_endpoint::AZURE_PUBLIC_CLOUD;
    };
    let default_endpoint_code = if let Some(endpoint) = endpoint {
        if endpoint == "https://management.azure.com" {
            public_cloud
        } else {
            quote! {
                pub const DEFAULT_ENDPOINT: &str = #endpoint;
            }
        }
    } else {
        public_cloud
    };

    let mut code = TokenStream::new();
    code.extend(quote! {

        #[derive(Clone)]
        pub struct Client {
            endpoint: String,
            credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
            scopes: Vec<String>,
            pipeline: azure_core::Pipeline,
        }

        #[derive(Clone)]
        pub struct ClientBuilder {
            credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>,
            endpoint: Option<String>,
            scopes: Option<Vec<String>>,
            options: azure_core::ClientOptions,
        }

        #default_endpoint_code

        impl ClientBuilder {
            #[doc = "Create a new instance of `ClientBuilder`."]
            #[must_use]
            pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
                Self {
                    credential,
                    endpoint: None,
                    scopes: None,
                    options: azure_core::ClientOptions::default(),
                }
            }

            #[doc = "Set the endpoint."]
            #[must_use]
            pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
                self.endpoint = Some(endpoint.into());
                self
            }

            #[doc = "Set the scopes."]
            #[must_use]
            pub fn scopes(mut self, scopes: &[&str]) -> Self {
                self.scopes = Some(scopes.iter().map(|scope| (*scope).to_owned()).collect());
                self
            }

            #[doc = "Set the retry options."]
            #[must_use]
            pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
                self.options = self.options.retry(retry);
                self
            }

            #[doc = "Set the transport options."]
            #[must_use]
            pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
                self.options = self.options.transport(transport);
                self
            }

            #[doc = "Convert the builder into a `Client` instance."]
            #[must_use]
            pub fn build(self) -> Client {
                let endpoint = self.endpoint.unwrap_or_else(|| DEFAULT_ENDPOINT.to_owned());
                let scopes = self.scopes.unwrap_or_else(|| vec![format!("{}/", endpoint)]);
                Client::new(endpoint, self.credential, scopes, self.options)
            }
        }

        impl Client {
            pub(crate) fn endpoint(&self) -> &str {
                self.endpoint.as_str()
            }
            pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
                self.credential.as_ref()
            }
            pub(crate) fn scopes(&self) -> Vec<&str> {
                self.scopes.iter().map(String::as_str).collect()
            }
            pub(crate) async fn send(&self, request: &mut azure_core::Request) -> azure_core::Result<azure_core::Response> {
                let mut context = azure_core::Context::default();
                self.pipeline.send(&mut context, request).await
            }

            #[doc = "Create a new `ClientBuilder`."]
            #[must_use]
            pub fn builder(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> ClientBuilder {
                ClientBuilder::new(credential)
            }

            #[doc = "Create a new `Client`."]
            #[must_use]
            pub fn new(endpoint: impl Into<String>, credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>, scopes: Vec<String>, options: azure_core::ClientOptions) -> Self {
                let endpoint = endpoint.into();
                let pipeline = azure_core::Pipeline::new(
                    option_env!("CARGO_PKG_NAME"),
                    option_env!("CARGO_PKG_VERSION"),
                    options,
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

pub fn create_operations(cg: &CodeGen) -> Result<TokenStream> {
    let mut file = TokenStream::new();
    file.extend(quote! {

        #![allow(unused_mut)]
        #![allow(unused_variables)]
        #![allow(unused_imports)]
        #![allow(clippy::redundant_clone)]
        pub mod models;
    });
    let mut operations_code: IndexMap<Option<String>, OperationCode> = IndexMap::new();
    // println!("input_files {:?}", cg.input_files());

    let operations: Vec<_> = cg.spec.operations()?.into_iter().map(WebOperationGen).collect();
    let module_names: BTreeSet<_> = operations.iter().flat_map(|op| op.rust_module_name()).collect();
    let module_names: Vec<_> = module_names.into_iter().collect();
    file.extend(create_client(&module_names, cg.spec.endpoint().as_deref())?);

    let mut errors = TokenStream::new();
    for operation in &operations {
        let variant = error_variant(operation)?;
        let fqn = error_fqn(operation)?;
        errors.extend(quote! {
            #[error(transparent)]
            #variant(#[from] #fqn),
        });
    }

    for operation in operations {
        let module_name = operation.rust_module_name();
        let code = create_operation_code(cg, &operation)?;
        // append code to existing module if it already exists
        match operations_code.get_mut(&module_name) {
            Some(operation_code) => {
                let OperationCode {
                    mut builder_instances,
                    mut module_code,
                } = code;
                operation_code.builder_instances.append(&mut builder_instances);
                operation_code.module_code.append(&mut module_code);
            }
            None => {
                operations_code.insert(module_name, code);
            }
        }
    }

    for (module_name, operation_code) in operations_code {
        let OperationCode {
            builder_instances,
            module_code,
        } = operation_code;
        let mut builders = TokenStream::new();
        for builder in builder_instances {
            builders.extend(builder.into_token_stream());
        }
        match module_name {
            Some(module_name) => {
                let name = parse_ident(&module_name)?;
                file.extend(quote! {
                    pub mod #name {
                        use super::models;
                        pub struct Client(pub(crate) super::Client);
                        impl Client {
                            #builders
                        }
                        #(#module_code)*
                    }
                });
            }
            None => {
                file.extend(quote! {
                    impl Client {
                        #builders
                    }
                    #(#module_code)*
                });
            }
        }
    }
    Ok(file)
}

struct OperationModuleCode {
    module_name: Ident,
    response_code: ResponseCode,
    builder_struct_code: BuilderStructCode,
    builder_setters_code: BuilderSettersCode,
    builder_future_code: BuilderFutureCode,
}

struct OperationCode {
    builder_instances: Vec<BuilderInstanceCode>,
    module_code: Vec<OperationModuleCode>,
}

struct WebOperationGen(WebOperation);

impl WebOperationGen {
    fn rust_module_name(&self) -> Option<String> {
        match &self.0.id {
            Some(id) => {
                let parts: Vec<&str> = id.splitn(2, '_').collect();
                if parts.len() == 2 {
                    Some(parts[0].to_snake_case())
                } else {
                    None
                }
            }
            None => None,
        }
    }
    fn rust_function_name(&self) -> String {
        match &self.0.id {
            Some(id) => {
                let parts: Vec<&str> = id.splitn(2, '_').collect();
                if parts.len() == 2 {
                    parts[1].to_snake_case()
                } else {
                    parts[0].to_snake_case()
                }
            }
            None => create_function_name(&self.0.verb, &self.0.path),
        }
    }

    fn function_name(&self) -> Result<Ident> {
        parse_ident(&self.rust_function_name())
    }

    fn api_version(&self) -> &str {
        self.0.api_version.as_str()
    }

    fn consumes(&self) -> Vec<&str> {
        self.0.consumes.iter().map(String::as_str).collect()
    }

    fn pick_consumes(&self) -> Option<&str> {
        crate::content_type::pick_consumes(self.consumes())
    }

    #[allow(dead_code)]
    fn produces(&self) -> Vec<&str> {
        self.0.produces.iter().map(String::as_str).collect()
    }

    fn pageable(&self) -> Option<Pageable> {
        self.0.pageable.as_ref().map(|p| Pageable {
            next_link_name: p.next_link_name.clone(),
        })
    }
}

/// Creating a function name from the path and verb when an operationId is not specified.
/// All azure-rest-api-specs operations should have an operationId.
fn create_function_name(verb: &WebVerb, path: &str) -> String {
    let mut path = path.split('/').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
    path.insert(0, verb.as_str());
    path.join("_")
}

/// Calls `azure_core::Request::new` and set the authentication.
struct NewRequestCode {
    auth: AuthCode,
    verb: WebVerb,
    path: String,
}

impl ToTokens for NewRequestCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let auth = &self.auth;
        let verb = verb_to_tokens(&self.verb);
        tokens.extend(quote! {
            let mut req = azure_core::Request::new(url, #verb);
            #auth
        })
    }
}

/// Sets the authentication.
/// Only bearer token authentication is supported right now.
struct AuthCode {}

impl ToTokens for AuthCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            let credential = this.client.token_credential();
            let token_response = credential
                .get_token(&this.client.scopes().join(" "))
                .await?;
            req.insert_header(azure_core::headers::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        })
    }
}

fn verb_to_tokens(verb: &WebVerb) -> TokenStream {
    match verb {
        WebVerb::Get => quote! { azure_core::Method::Get },
        WebVerb::Post => quote! { azure_core::Method::Post },
        WebVerb::Put => quote! { azure_core::Method::Put },
        WebVerb::Patch => quote! { azure_core::Method::Patch },
        WebVerb::Delete => quote! { azure_core::Method::Delete },
        WebVerb::Options => quote! { azure_core::Method::Option },
        WebVerb::Head => quote! { azure_core::Method::Head },
    }
}

/// Sets all of the request parameters.
struct SetRequestParamsCode {
    content_type: String,
    params: FunctionParams,
}

impl ToTokens for SetRequestParamsCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for param in self.params.params() {
            let FunctionParam {
                name: param_name,
                variable_name: param_name_var,
                kind,
                collection_format,
                ..
            } = param;
            let is_vec = param.is_vec();
            match kind {
                ParamKind::Path => {} // handled above
                ParamKind::Query => {
                    let query_body = if is_vec {
                        match collection_format {
                            CollectionFormat::Multi => Some(
                                if param.is_string(){
                                    quote! {
                                        for value in &this.#param_name_var {
                                            req.url_mut().query_pairs_mut().append_pair(#param_name, value);
                                        }
                                    }
                                } else {
                                    quote! {
                                        for value in &this.#param_name_var {
                                            req.url_mut().query_pairs_mut().append_pair(#param_name, &value.to_string());
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
                                req.url_mut().query_pairs_mut().append_pair(#param_name, #param_name_var);
                            }
                        } else {
                            quote! {
                                req.url_mut().query_pairs_mut().append_pair(#param_name, &#param_name_var.to_string());
                            }
                        })
                    };
                    if let Some(query_body) = query_body {
                        if !param.optional() || is_vec {
                            tokens.extend(quote! {
                                let #param_name_var = &this.#param_name_var;
                                #query_body
                            });
                        } else {
                            tokens.extend(quote! {
                                if let Some(#param_name_var) = &this.#param_name_var {
                                    #query_body
                                }
                            });
                        }
                    }
                }
                ParamKind::Header => {
                    // always use lowercase header names
                    let header_name = param_name.to_lowercase();
                    if !param.optional() || is_vec {
                        if param.is_string() {
                            tokens.extend(quote! {
                                req.insert_header(#header_name, &this.#param_name_var);
                            });
                        } else {
                            tokens.extend(quote! {
                                req.insert_header(#header_name, &this.#param_name_var.to_string());
                            });
                        }
                    } else if param.is_string() {
                        tokens.extend(quote! {
                            if let Some(#param_name_var) = &this.#param_name_var {
                                req.insert_header(#header_name, #param_name_var);
                            }
                        });
                    } else {
                        tokens.extend(quote! {
                            if let Some(#param_name_var) = &this.#param_name_var {
                                req.insert_header(#header_name, &#param_name_var.to_string());
                            }
                        });
                    }
                }
                ParamKind::Body => {
                    let set_content_type = if !self.params.has_content_type_header() {
                        let content_type = &self.content_type;
                        quote! {
                            req.insert_header("content-type", #content_type);
                        }
                    } else {
                        quote! {}
                    };

                    if !param.optional() || is_vec {
                        tokens.extend(quote! {
                            #set_content_type
                            let req_body = azure_core::to_json(&this.#param_name_var)?;
                        });
                    } else {
                        tokens.extend(quote! {
                            let req_body =
                                if let Some(#param_name_var) = &this.#param_name_var {
                                    #set_content_type
                                    azure_core::to_json(#param_name_var)?
                                } else {
                                    azure_core::EMPTY_BODY
                                };
                        });
                    }
                }
                ParamKind::FormData => {
                    tokens.extend(quote! {
                        unimplemented!("form data not yet supported");
                    });
                    // https://github.com/Azure/azure-sdk-for-rust/issues/500
                    // if required {
                    //     ts_request_builder.extend(quote! {
                    //         req.set_body_from_form(&self.#param_name_var)?;
                    //     });
                    // } else {
                    //     ts_request_builder.extend(quote! {
                    //         if let Some(#param_name_var) = &self.#param_name_var {
                    //             req.set_body_from_form(#param_name_var)?;
                    //         }
                    //     });
                    // }
                }
            }
        }
    }
}

// Create code for the web operation
fn create_operation_code(cg: &CodeGen, operation: &WebOperationGen) -> Result<OperationCode> {
    let parameters = FunctionParams::new(operation)?;

    let verb = operation.0.verb.clone();
    let is_post = verb == WebVerb::Post;
    let auth = AuthCode {};
    let new_request_code = NewRequestCode {
        verb,
        auth,
        path: operation.0.path.clone(),
    };

    // get the consumes content-type from the operation, else the spec, else default to json
    let consumes = operation
        .pick_consumes()
        .unwrap_or_else(|| cg.spec.pick_consumes().unwrap_or(content_type::APPLICATION_JSON))
        .to_string();
    let request_builder = SetRequestCode {
        has_param_api_version: parameters.has_api_version,
        api_version: operation.api_version().to_string(),
        consumes,
        parameters: parameters.clone(),
        has_body_parameter: operation.0.has_body_parameter(),
        is_post,
    };
    let in_operation_group = operation.0.in_group();
    let builder_instance_code = BuilderInstanceCode::new(operation, &parameters, in_operation_group)?;
    let builder_struct_code = BuilderStructCode::new(&parameters, in_operation_group);
    let builder_setters_code = BuilderSettersCode::new(&parameters);
    let response_code = ResponseCode::new(operation)?;
    let long_running_operation = operation.0.long_running_operation;
    let builder_future_code = BuilderFutureCode::new(new_request_code, request_builder, response_code.clone(), long_running_operation)?;

    let module_code = OperationModuleCode {
        module_name: operation.function_name()?,
        response_code,
        builder_struct_code,
        builder_setters_code,
        builder_future_code,
    };

    Ok(OperationCode {
        builder_instances: vec![builder_instance_code],
        module_code: vec![module_code],
    })
}

/// Set all body and parameters for the request.
struct SetRequestCode {
    has_param_api_version: bool,
    api_version: String,
    consumes: String,
    parameters: FunctionParams,
    has_body_parameter: bool,
    is_post: bool,
}

impl ToTokens for SetRequestCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // api-version param
        if self.has_param_api_version {
            let api_version = &self.api_version;
            tokens.extend(quote! {
                req.url_mut().query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, #api_version);
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

/// The response for an operation.
/// An operation may have multiple valid status codes.
#[derive(Clone)]
struct ResponseCode {
    status_responses: Vec<StatusResponseCode>,
    pageable: Option<Pageable>,
}

#[derive(Clone)]
struct Pageable {
    next_link_name: Option<String>,
}

/// A single status code response of an operation.
#[derive(Clone)]
struct StatusResponseCode {
    name: Ident,
    status_code_name: Ident,
    response_type: Option<TypeNameCode>,
}

impl ResponseCode {
    fn new(operation: &WebOperationGen) -> Result<Self> {
        let mut status_responses = Vec::new();
        let responses = &operation.0.responses;
        for (status_code, rsp) in &get_success_responses(responses) {
            status_responses.push(StatusResponseCode {
                name: get_response_type_ident(status_code)?,
                status_code_name: get_status_code_ident(status_code)?,
                response_type: create_response_type(rsp)?,
            });
        }
        Ok(Self {
            status_responses,
            pageable: operation.pageable(),
        })
    }

    fn is_single_response(&self) -> bool {
        self.status_responses.len() == 1
    }
}

impl ToTokens for ResponseCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_single_response() {
            let tp = self.status_responses[0]
                .response_type
                .as_ref()
                .map(TypeNameCode::to_token_stream)
                .unwrap_or(quote! { () });
            tokens.extend(quote! {
                type Response = #tp;
            });
        } else {
            let mut success_responses_ts = TokenStream::new();
            let mut continuation_response = TokenStream::new();
            for status_response in &self.status_responses {
                let response_type = &status_response.response_type;
                let code = match response_type {
                    Some(response_type) => quote! { (#response_type) },
                    None => quote! {},
                };
                let enum_type_name = &status_response.name;
                success_responses_ts.extend(quote! { #enum_type_name#code, });

                let continuation = if response_type.is_some() {
                    quote! {
                        Self::#enum_type_name(x) => x.continuation(),
                    }
                } else {
                    quote! { Self::#enum_type_name => None, }
                };
                continuation_response.extend(continuation);
            }
            tokens.extend(quote! {
                #[derive(Debug)]
                pub enum Response {
                    #success_responses_ts
                }
            });

            if self.pageable.is_some() {
                tokens.extend(quote! {
                    impl azure_core::Continuable for Response {
                        type Continuation = String;
                        fn continuation(&self) -> Option<Self::Continuation> {
                            match self {
                                #continuation_response
                            }
                        }
                    }
                });
            }
        }
    }
}

/// The `into_future` function of the request builder.
struct BuilderFutureCode {
    new_request_code: NewRequestCode,
    request_builder: SetRequestCode,
    response_code: ResponseCode,
    url_args: Vec<Ident>,
    long_running_operation: bool,
}

impl BuilderFutureCode {
    fn new(
        new_request_code: NewRequestCode,
        request_builder: SetRequestCode,
        response_code: ResponseCode,
        long_running_operation: bool,
    ) -> Result<Self> {
        let params = parse_path_params(&new_request_code.path);
        let url_args: Result<Vec<_>> = params.iter().map(|s| s.to_snake_case_ident()).collect();
        let url_args = url_args?;
        Ok(Self {
            new_request_code,
            request_builder,
            response_code,
            url_args,
            long_running_operation,
        })
    }

    fn is_single_response(&self) -> bool {
        self.response_code.status_responses.len() == 1
    }
}

impl ToTokens for BuilderFutureCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let new_request_code = &self.new_request_code;
        let request_builder = &self.request_builder;

        let url_args = self.url_args.iter().map(|url_arg| {
            quote! { &this.#url_arg }
        });
        let url_str_args = quote! { #(#url_args),* };

        let fpath = format!("{{}}{}", &format_path(&new_request_code.path));

        let mut match_status = TokenStream::new();
        for status_response in &self.response_code.status_responses {
            let response_type = &status_response.response_type;
            let rsp_value = create_rsp_value(response_type.as_ref());
            let status_code_name = &status_response.status_code_name;
            let response_type_name = &status_response.name;

            let status_code_code = if self.is_single_response() {
                match response_type {
                    Some(_) => quote! {
                        let rsp_body = rsp_stream.collect().await?;
                        #rsp_value
                        Ok(rsp_value)
                    },
                    None => quote! { Ok(()) },
                }
            } else {
                match response_type {
                    Some(_) => quote! {
                        let rsp_body = rsp_stream.collect().await?;
                        #rsp_value
                        Ok(Response::#response_type_name(rsp_value))
                    },
                    None => quote! { Ok(Response::#response_type_name) },
                }
            };
            match_status.extend(quote! {
                azure_core::StatusCode::#status_code_name => {
                    #status_code_code
                }
            });
        }
        match_status.extend(quote! {
            status_code => {
                Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse { status: status_code, error_code: None }))
            }
        });

        let basic_future = quote! {
            pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::Result<Response>> {
                Box::pin({
                    let this = self.clone();
                    async move {
                        let url = azure_core::Url::parse(&format!(#fpath, this.client.endpoint(), #url_str_args))?;
                        #new_request_code
                        #request_builder
                        req.set_body(req_body);
                        let rsp = this.client.send(&mut req).await?;
                        let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                        match rsp_status {
                            #match_status
                        }
                    }
                })
            }
        };

        let fut = if let Some(pageable) = &self.response_code.pageable {
            // TODO: Pageable requires the values to be part of the response schema,
            // however, some schemas do this via the header x-ms-continuation rather than
            // provide a next_link_name.  For now, those cases get documented that we don't
            // poll and move on.
            if pageable.next_link_name.is_none() {
                // most often when this happens, the continuation token is provided
                // by an HTTP Header x-ms-continuation, which should be extracted
                // from the response.
                //
                // Note, this is only *sometimes* this is specified in the spec.
                //
                // Ref: https://github.com/Azure/azure-sdk-for-rust/issues/446
                let mut fut = quote! { #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]};
                fut.extend(basic_future);
                fut
            } else {
                let mut stream_api_version = quote! {};

                // per discussion in SDK meeting, we should always set the
                // api-version on the request if we have a version.
                if request_builder.has_param_api_version {
                    let api_version = &request_builder.api_version;
                    stream_api_version = quote! {
                        let has_api_version_already = req.url_mut().query_pairs().any(|(k, _)| k == azure_core::query_param::API_VERSION);
                        if !has_api_version_already {
                            req.url_mut().query_pairs_mut().append_pair(azure_core::query_param::API_VERSION, #api_version);
                        }
                    };
                }

                quote! {
                    pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                        let make_request = move |continuation: Option<String>| {
                            let this = self.clone();
                            async move {
                                let mut url = azure_core::Url::parse(&format!(#fpath, this.client.endpoint(), #url_str_args))?;

                                let rsp = match continuation {
                                    Some(value) => {
                                        url.set_path("");
                                        url = url.join(&value)?;
                                        #new_request_code
                                        #stream_api_version
                                        let req_body = azure_core::EMPTY_BODY;
                                        req.set_body(req_body);
                                        this.client.send(&mut req).await?
                                    }
                                    None => {
                                        #new_request_code
                                        #request_builder
                                        req.set_body(req_body);
                                        this.client.send(&mut req).await?
                                    }
                                };
                                let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                                match rsp_status {
                                    #match_status
                                }
                            }
                        };

                        azure_core::Pageable::new(make_request)
                    }
                }
            }
        } else if self.long_running_operation {
            // TODO:  Long running options should also move to the Pageable stream
            // model, however this is not possible at the moment because the
            // continuation token is often not returned in the response body, but
            // instead a header which we don't include as part of the response
            // model.
            //
            // As is, Pageable requires implementing the Continuable trait on the
            // response object.
            //
            // ref: https://github.com/Azure/azure-sdk-for-rust/issues/741
            let mut fut = quote! {#[doc = "only the first response will be fetched as long running operations are not supported yet"]};
            fut.extend(basic_future);
            fut
        } else {
            basic_future
        };
        tokens.extend(fut);
    }
}

impl ToTokens for OperationModuleCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            module_name,
            response_code,
            builder_struct_code,
            builder_setters_code,
            builder_future_code,
        } = &self;
        tokens.extend(quote! {
            pub mod #module_name {
                use super::models;

                #response_code

                #builder_struct_code

                impl Builder {
                    #builder_setters_code
                    #builder_future_code
                }

            }
        })
    }
}

fn create_rsp_value(tp: Option<&TypeNameCode>) -> TokenStream {
    if tp.map(TypeNameCode::is_bytes) == Some(true) {
        quote! {
            let rsp_value = rsp_body;
        }
    } else {
        quote! {
            let rsp_value: #tp = serde_json::from_slice(&rsp_body)?;
        }
    }
}

fn format_path(path: &str) -> String {
    PARAM_RE.replace_all(path, "{}").to_string()
}

#[derive(PartialEq, Eq, Clone)]
enum ParamKind {
    Path,
    Query,
    Header,
    Body,
    FormData,
}

impl From<&ParameterType> for ParamKind {
    fn from(pt: &ParameterType) -> Self {
        match pt {
            ParameterType::Path => Self::Path,
            ParameterType::Query => Self::Query,
            ParameterType::Header => Self::Header,
            ParameterType::Body => Self::Body,
            ParameterType::FormData => Self::FormData,
        }
    }
}

#[derive(Clone)]
struct FunctionParam {
    name: String,
    description: Option<String>,
    variable_name: Ident,
    type_name: TypeNameCode,
    kind: ParamKind,
    collection_format: CollectionFormat,
}
impl FunctionParam {
    fn is_vec(&self) -> bool {
        self.type_name.is_vec()
    }
    fn optional(&self) -> bool {
        self.type_name.optional
    }
    fn is_string(&self) -> bool {
        self.type_name.is_string()
    }
}

#[derive(Clone)]
struct FunctionParams {
    params: Vec<FunctionParam>,
    has_api_version: bool,
}
impl FunctionParams {
    fn new(operation: &WebOperationGen) -> Result<Self> {
        let parameters = operation.0.parameters();
        let has_api_version = parameters.iter().any(|p| p.name() == query_param::API_VERSION);
        let mut skip = parse_query_params(&operation.0.path)?;
        skip.insert(query_param::API_VERSION.to_string());
        let parameters: Vec<&WebParameter> = parameters.clone().into_iter().filter(|p| !skip.contains(p.name())).collect();

        let mut params = Vec::new();
        for param in parameters.iter().filter(|p| !skip.contains(p.name())) {
            let name = param.name().to_owned();
            let description = param.description().clone();
            let variable_name = name.to_snake_case_ident()?;
            let type_name = type_name_gen(&param.type_name()?)?.qualify_models(true).optional(!param.required());
            let kind = ParamKind::from(param.type_());
            let collection_format = param.collection_format().clone();
            params.push(FunctionParam {
                name,
                description,
                variable_name,
                type_name,
                kind,
                collection_format,
            });
        }
        Ok(Self { params, has_api_version })
    }

    fn params(&self) -> Vec<&FunctionParam> {
        self.params.iter().collect()
    }
    fn required_params(&self) -> Vec<&FunctionParam> {
        self.params.iter().filter(|p| !p.type_name.optional).collect()
    }
    fn optional_params(&self) -> Vec<&FunctionParam> {
        self.params.iter().filter(|p| p.type_name.optional).collect()
    }
    #[allow(dead_code)]
    fn params_of_kind(&self, kind: &ParamKind) -> Vec<&FunctionParam> {
        self.params.iter().filter(|p| &p.kind == kind).collect()
    }

    fn has_content_type_header(&self) -> bool {
        self.params()
            .iter()
            .any(|p| p.name.eq_ignore_ascii_case("content-type") && p.kind == ParamKind::Header)
    }
}

#[derive(Clone)]
struct FunctionCallParamsCode(FunctionParams);

impl ToTokens for FunctionCallParamsCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        for FunctionParam {
            variable_name, type_name, ..
        } in self.0.required_params()
        {
            let mut type_name = type_name.clone();
            let is_vec = type_name.is_vec();
            type_name = type_name.impl_into(!is_vec);
            params.push(quote! { #variable_name: #type_name });
        }
        let slf = quote! { &self };
        params.insert(0, slf);
        tokens.extend(quote! { #(#params),* })
    }
}

/// Create the client function that produces the request builder instance.
#[derive(Clone)]
struct BuilderInstanceCode {
    summary: Option<String>,
    description: Option<String>,
    fname: Ident,
    parameters: FunctionParams,
    in_operation_group: bool,
}

impl BuilderInstanceCode {
    fn new(operation: &WebOperationGen, parameters: &FunctionParams, in_operation_group: bool) -> Result<Self> {
        let fname = operation.function_name()?;
        let summary = operation.0.summary.clone();
        let description = operation.0.description.clone();
        Ok(Self {
            summary,
            description,
            fname,
            parameters: parameters.clone(),
            in_operation_group,
        })
    }
}

impl ToTokens for BuilderInstanceCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        if self.in_operation_group {
            params.push(quote! { client: self.0.clone() });
        } else {
            params.push(quote! { client: self.clone() });
        }
        for param in self.parameters.required_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            let mut type_name = type_name.clone();
            let is_vec = type_name.is_vec();
            type_name = type_name.impl_into(!is_vec);
            if type_name.has_impl_into() {
                params.push(quote! { #variable_name: #variable_name.into() });
            } else {
                params.push(quote! { #variable_name });
            }
        }
        for param in self.parameters.optional_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            if type_name.is_vec() {
                params.push(quote! { #variable_name: Vec::new() });
            } else {
                params.push(quote! { #variable_name: None });
            }
        }

        let summary = match &self.summary {
            Some(summary) if !summary.is_empty() => quote! { #[doc = #summary] },
            _ => quote! {},
        };
        let description = match &self.description {
            Some(desc) if !desc.is_empty() => quote! { #[doc = #desc] },
            _ => quote! {},
        };

        let mut param_descriptions: Vec<TokenStream> = Vec::new();
        if self
            .parameters
            .required_params()
            .into_iter()
            .any(|param| param.description.is_some())
        {
            // Add a blank link before the arguments if there is a summary or description.
            if !summary.is_empty() || !description.is_empty() {
                param_descriptions.push(quote! { #[doc = ""] });
            }
            param_descriptions.push(quote! { #[doc = "Arguments:"] });
            for required_param in self.parameters.required_params().iter() {
                if let Some(desc) = &required_param.description {
                    if !desc.is_empty() {
                        let doc_comment = format!("* `{}`: {}", required_param.variable_name, desc);
                        param_descriptions.push(quote! { #[doc = #doc_comment] });
                    }
                }
            }
        };

        let fname = &self.fname;
        let parameters = FunctionCallParamsCode(self.parameters.clone());
        tokens.extend(quote! {
            #summary
            #description
            #(#param_descriptions)*
            pub fn #fname(#parameters) -> #fname::Builder {
                #fname::Builder {
                    #(#params),*
                }
            }
        });
    }
}

/// The request builder struct type, not the impl.
#[derive(Clone)]
struct BuilderStructCode {
    parameters: FunctionParams,
    in_operation_group: bool,
}

impl BuilderStructCode {
    fn new(parameters: &FunctionParams, in_operation_group: bool) -> Self {
        Self {
            parameters: parameters.clone(),
            in_operation_group,
        }
    }
}

impl ToTokens for BuilderStructCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        if self.in_operation_group {
            params.push(quote! { pub(crate) client: super::super::Client });
        } else {
            params.push(quote! { pub(crate) client: super::Client });
        }
        for param in self.parameters.required_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            params.push(quote! { pub(crate) #variable_name: #type_name });
        }
        for param in self.parameters.optional_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            let mut type_name = type_name.clone();
            if type_name.is_vec() {
                type_name = type_name.optional(false);
            }
            params.push(quote! { pub(crate) #variable_name: #type_name });
        }
        tokens.extend(quote! {
            #[derive(Clone)]
            pub struct Builder {
                #(#params),*
            }
        });
    }
}

/// The setter functions for the request builder.
#[derive(Clone)]
struct BuilderSettersCode {
    parameters: FunctionParams,
}

impl BuilderSettersCode {
    fn new(parameters: &FunctionParams) -> Self {
        Self {
            parameters: parameters.clone(),
        }
    }
}

impl ToTokens for BuilderSettersCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for param in self.parameters.optional_params() {
            let FunctionParam {
                variable_name, type_name, ..
            } = param;
            let is_vec = type_name.is_vec();
            let mut type_name = type_name.clone();
            type_name = type_name.optional(false);
            type_name = type_name.impl_into(!is_vec);
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

pub fn create_response_type(rsp: &Response) -> Result<Option<TypeNameCode>> {
    if let Some(schema) = &rsp.schema {
        Ok(Some(type_name_gen(&get_type_name_for_schema_ref(schema)?)?.qualify_models(true)))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_function_name() {
        assert_eq!(create_function_name(&WebVerb::Get, "/pets"), "get_pets");
    }

    #[test]
    fn test_function_name_from_operation_id() {
        let operation = WebOperationGen(WebOperation {
            id: Some("PrivateClouds_CreateOrUpdate".to_owned()),
            path: "/horse".to_owned(),
            verb: WebVerb::Get,
            ..Default::default()
        });
        assert_eq!(Some("private_clouds".to_owned()), operation.rust_module_name());
        assert_eq!("create_or_update", operation.rust_function_name());
    }

    #[test]
    fn test_function_name_from_verb_and_path() {
        let operation = WebOperationGen(WebOperation {
            path: "/horse".to_owned(),
            verb: WebVerb::Get,
            ..Default::default()
        });
        assert_eq!(None, operation.rust_module_name());
        assert_eq!("get_horse", operation.rust_function_name());
    }

    #[test]
    fn test_function_name_with_no_module_name() {
        let operation = WebOperationGen(WebOperation {
            id: Some("PerformConnectivityCheck".to_owned()),
            path: "/horse".to_owned(),
            verb: WebVerb::Put,
            ..Default::default()
        });
        assert_eq!(None, operation.rust_module_name());
        assert_eq!("perform_connectivity_check", operation.rust_function_name());
    }
}
