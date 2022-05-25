use crate::{
    codegen::{create_generated_by_header, Error, TypeNameCode},
    codegen::{parse_params, type_name_gen, PARAM_RE},
    identifier::{parse_ident, SnakeCaseIdent},
    spec::{get_type_name_for_schema_ref, WebOperation, WebParameter, WebVerb},
    status_codes::get_success_responses,
    status_codes::{get_response_type_ident, get_status_code_ident},
    CodeGen,
};
use autorust_openapi::{CollectionFormat, ParameterType, Response};
use heck::ToPascalCase;
use heck::ToSnakeCase;
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::{BTreeSet, HashSet};

fn error_variant(operation: &WebOperationGen) -> Result<Ident, Error> {
    let function = operation.rust_function_name().to_pascal_case();
    if let Some(module) = operation.rust_module_name() {
        let module = module.to_pascal_case();
        parse_ident(&format!("{}_{}", module, function)).map_err(Error::EnumVariantName)
    } else {
        parse_ident(&function).map_err(Error::ModuleName)
    }
}

fn error_fqn(operation: &WebOperationGen) -> Result<TokenStream, Error> {
    let function = parse_ident(&operation.rust_function_name()).map_err(Error::FunctionName)?;
    if let Some(module) = operation.rust_module_name() {
        let module = parse_ident(&module).map_err(Error::ModuleName)?;
        Ok(quote! { #module::#function::Error })
    } else {
        Ok(quote! { #function::Error })
    }
}

pub fn create_client(modules: &[String], endpoint: Option<&str>) -> Result<TokenStream, Error> {
    let mut clients = TokenStream::new();
    for md in modules {
        let md = md.to_snake_case_ident().map_err(Error::ModuleName)?;
        clients.extend(quote! {
            pub fn #md(&self) -> #md::Client {
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
        }

        #default_endpoint_code

        impl ClientBuilder {
            pub fn new(credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>) -> Self {
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
            pub(crate) fn endpoint(&self) -> &str {
                self.endpoint.as_str()
            }
            pub(crate) fn token_credential(&self) -> &dyn azure_core::auth::TokenCredential {
                self.credential.as_ref()
            }
            pub(crate) fn scopes(&self) -> Vec<&str> {
                self.scopes.iter().map(String::as_str).collect()
            }
            pub(crate) async fn send(&self, request: impl Into<azure_core::Request>) -> azure_core::error::Result<azure_core::Response> {
                let mut context = azure_core::Context::default();
                let mut request = request.into();
                self.pipeline.send(&mut context, &mut request).await
            }
            pub fn new(endpoint: impl Into<String>, credential: std::sync::Arc<dyn azure_core::auth::TokenCredential>, scopes: Vec<String>) -> Self {
                let endpoint = endpoint.into();
                let pipeline = azure_core::Pipeline::new(
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
        #![allow(clippy::redundant_clone)]
        use super::models;
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
                let name = parse_ident(&module_name).map_err(Error::ModuleName)?;
                file.extend(quote! {
                    pub mod #name {
                        use super::models;
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

    pub fn function_name(&self) -> Result<Ident, Error> {
        parse_ident(&self.rust_function_name()).map_err(Error::FunctionName)
    }

    fn api_version(&self) -> &str {
        self.0.api_version.as_str()
    }
}

/// Creating a function name from the path and verb when an operationId is not specified.
/// All azure-rest-api-specs operations should have an operationId.
fn create_function_name(verb: &WebVerb, path: &str) -> String {
    let mut path = path.split('/').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
    path.insert(0, verb.as_str());
    path.join("_")
}

struct RequestCode {
    auth: AuthCode,
    verb: WebVerb,
}

impl ToTokens for RequestCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let auth = &self.auth;
        let verb = verb_to_tokens(&self.verb);
        tokens.extend(quote! {
            req_builder = req_builder.method(#verb);
            #auth
        })
    }
}

// Only bearer token authentication is supported right now.
struct AuthCode {}
impl ToTokens for AuthCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(quote! {
            let credential = this.client.token_credential();
            let token_response = credential
                .get_token(&this.client.scopes().join(" "))
                .await
                .context(azure_core::error::ErrorKind::Other, "get bearer token")?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        })
    }
}

fn verb_to_tokens(verb: &WebVerb) -> TokenStream {
    match verb {
        WebVerb::Get => quote! { http::Method::GET },
        WebVerb::Post => quote! { http::Method::POST },
        WebVerb::Put => quote! { http::Method::PUT },
        WebVerb::Patch => quote! { http::Method::PATCH },
        WebVerb::Delete => quote! { http::Method::DELETE },
        WebVerb::Options => quote! { http::Method::OPTIONS },
        WebVerb::Head => quote! { http::Method::HEAD },
    }
}

// Create code for the web operation
fn create_operation_code(cg: &CodeGen, operation: &WebOperationGen) -> Result<OperationCode, Error> {
    let params = parse_params(&operation.0.path);
    let params: Result<Vec<_>, Error> = params
        .iter()
        .map(|s| {
            let param = s.to_snake_case_ident().map_err(Error::ParamName)?;
            Ok(quote! { &this.#param })
        })
        .collect();
    let params = params?;
    let url_str_args = quote! { #(#params),* };

    let fpath = format!("{{}}{}", &format_path(&operation.0.path));

    let parameters = operation.0.parameters();
    let param_names: HashSet<_> = parameters.iter().map(|p| p.name()).collect();
    let has_param_api_version = param_names.contains("api-version");
    let mut skip = HashSet::new();
    skip.insert("api-version");
    let parameters: Vec<&WebParameter> = parameters.clone().into_iter().filter(|p| !skip.contains(p.name())).collect();
    let parameters = create_function_params_code(&parameters)?;

    let verb = operation.0.verb.clone();
    let is_post = verb == WebVerb::Post;
    let auth = AuthCode {};
    let request_code = RequestCode { verb, auth };

    let mut ts_request_builder = TokenStream::new(); // TODO change to type
    ts_request_builder.extend(request_code.to_token_stream());

    // api-version param
    if has_param_api_version {
        let api_version = operation.api_version();
        ts_request_builder.extend(quote! {
            url.query_pairs_mut().append_pair("api-version", #api_version);
        });
    }

    let has_content_type_header = parameters
        .params()
        .iter()
        .any(|p| p.name.eq_ignore_ascii_case("content-type") && p.kind == ParamKind::Header);

    // params
    for param in parameters.params() {
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
                                        url.query_pairs_mut().append_pair(#param_name, value);
                                    }
                                }
                            } else {
                                quote! {
                                    for value in &this.#param_name_var {
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
                    if !param.optional() || is_vec {
                        ts_request_builder.extend(quote! {
                            let #param_name_var = &this.#param_name_var;
                            #query_body
                        });
                    } else {
                        ts_request_builder.extend(quote! {
                            if let Some(#param_name_var) = &this.#param_name_var {
                                #query_body
                            }
                        });
                    }
                }
            }
            ParamKind::Header => {
                if !param.optional() || is_vec {
                    if param.is_string() {
                        ts_request_builder.extend(quote! {
                            req_builder = req_builder.header(#param_name, &this.#param_name_var);
                        });
                    } else {
                        ts_request_builder.extend(quote! {
                            req_builder = req_builder.header(#param_name, &this.#param_name_var.to_string());
                        });
                    }
                } else if param.is_string() {
                    ts_request_builder.extend(quote! {
                        if let Some(#param_name_var) = &this.#param_name_var {
                            req_builder = req_builder.header(#param_name, #param_name_var);
                        }
                    });
                } else {
                    ts_request_builder.extend(quote! {
                        if let Some(#param_name_var) = &this.#param_name_var {
                            req_builder = req_builder.header(#param_name, &#param_name_var.to_string());
                        }
                    });
                }
            }
            ParamKind::Body => {
                let set_content_type = if !has_content_type_header {
                    let json_content_type = cg.get_request_content_type_json();
                    quote! {
                        req_builder = req_builder.header("content-type", #json_content_type);
                    }
                } else {
                    quote! {}
                };

                if !param.optional() || is_vec {
                    ts_request_builder.extend(quote! {
                        #set_content_type
                        let req_body = azure_core::to_json(&this.#param_name_var)?;
                    });
                } else {
                    ts_request_builder.extend(quote! {
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

    let has_body_parameter = operation.0.has_body_parameter();
    if !has_body_parameter {
        ts_request_builder.extend(quote! {
            let req_body = azure_core::EMPTY_BODY;
        });
    }

    // if it is a post and there is no body, set the Content-Length to 0
    if is_post && !has_body_parameter {
        ts_request_builder.extend(quote! {
            req_builder = req_builder.header(http::header::CONTENT_LENGTH, 0);
        });
    }

    let responses = &operation.0.responses;
    let success_responses = get_success_responses(responses);
    let is_single_response = success_responses.len() == 1;

    /*
    let fresponse = if is_single_response {
        let tp = create_response_type(&success_responses[0])?.unwrap_or(quote! { () });
        quote! { std::result::Result<#tp, Error> }
    } else {
        quote! { std::result::Result<Response, Error> }
    };
     */

    let mut response_enum = TokenStream::new();
    if is_single_response {
        let tp = create_response_type(&success_responses[0])?
            .map(TypeNameCode::into_token_stream)
            .unwrap_or(quote! { () });
        response_enum.extend(quote! {
            type Response = #tp;
        });
    } else {
        let mut success_responses_ts = TokenStream::new();
        let mut continuation_response = TokenStream::new();
        for (status_code, rsp) in &success_responses {
            let tp = create_response_type(rsp)?;
            let has_tp = tp.is_some();

            let tp = match tp {
                Some(tp) => quote! { (#tp) },
                None => quote! {},
            };
            let enum_type_name = get_response_type_ident(status_code)?;
            success_responses_ts.extend(quote! { #enum_type_name#tp, });

            if has_tp {
                continuation_response.extend(quote! {
                    Self::#enum_type_name(x) => x.continuation(),
                });
            } else {
                continuation_response.extend(quote! { Self::#enum_type_name => None, });
            }
        }
        response_enum.extend(quote! {
            #[derive(Debug)]
            pub enum Response {
                #success_responses_ts
            }
        });

        if let Some(_pageable) = &operation.0.pageable {
            response_enum.extend(quote! {
                impl azure_core::Continuable for Response {
                    fn continuation(&self) -> Option<String> {
                        match self {
                            #continuation_response
                        }
                    }
                }
            });
        }
    }

    let mut match_status = TokenStream::new();
    for (status_code, rsp) in &success_responses {
        match status_code {
            autorust_openapi::StatusCode::Code(_) => {
                let tp = create_response_type(rsp)?.map(TypeNameCode::into_token_stream);
                let rsp_value = create_rsp_value(tp.as_ref());
                let status_code_name = get_status_code_ident(status_code)?;
                let response_type_name = get_response_type_ident(status_code)?;

                if is_single_response {
                    match tp {
                        Some(_) => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
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
                        Some(_) => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    let rsp_body = azure_core::collect_pinned_stream(rsp_stream).await?;
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
    match_status.extend(quote! {
        status_code => {
            Err(azure_core::error::Error::from(azure_core::error::ErrorKind::HttpResponse { status: status_code.as_u16(), error_code: None }))
        }
    });

    let in_group = operation.0.in_group();
    let builder_instance_code = create_builder_instance_code(operation, &parameters, in_group)?;
    let builder_struct_code = create_builder_struct_code(&parameters, in_group)?;
    let builder_setters_code = create_builder_setters_code(&parameters)?;

    let basic_future = quote! {
        pub fn into_future(self) -> futures::future::BoxFuture<'static, azure_core::error::Result<Response>> {
            Box::pin({
                let this = self.clone();
                async move {
                    let url_str = &format!(#fpath, this.client.endpoint(), #url_str_args);
                    let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                    let mut req_builder = http::request::Builder::new();
                    #ts_request_builder
                    req_builder = req_builder.uri(url.as_str());
                    let req = req_builder.body(req_body).context(azure_core::error::ErrorKind::Other, "build request")?;
                    let rsp = this.client.send(req).await.context(azure_core::error::ErrorKind::Io, "execute request")?;
                    let (rsp_status, rsp_headers, rsp_stream) = rsp.deconstruct();
                    match rsp_status {
                        #match_status
                    }
                }
            })
        }
    };

    let fut = if let Some(pageable) = &operation.0.pageable {
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
            let mut fut =
                quote! { #[doc = "only the first response will be fetched as the continuation token is not part of the response schema"]};
            fut.extend(basic_future);
            fut
        } else {
            let mut stream_api_version = quote! {};

            // per discussion in SDK meeting, we should always set the
            // api-version on the request if we have a version.
            if has_param_api_version {
                let api_version = operation.api_version();
                stream_api_version = quote! {
                    let has_api_version_already = url.query_pairs().any(|(k, _)| k == "api-version");
                    if !has_api_version_already {
                        url.query_pairs_mut().append_pair("api-version", #api_version);
                    }
                };
            }

            quote! {
                pub fn into_stream(self) -> azure_core::Pageable<Response, azure_core::error::Error> {
                    let make_request = move |continuation: Option<azure_core::prelude::Continuation>| {
                        let this = self.clone();
                        async move {
                            let url_str = &format!(#fpath, this.client.endpoint(), #url_str_args);
                            let mut url = url::Url::parse(url_str).context(azure_core::error::ErrorKind::Other, "build request")?;

                            let mut req_builder = http::request::Builder::new();
                            let rsp = match continuation {
                                Some(token) => {
                                    url.set_path("");
                                    url = url.join(&token.into_raw()).context(azure_core::error::ErrorKind::DataConversion, "parse url")?;
                                    #stream_api_version
                                    req_builder = req_builder.uri(url.as_str());
                                    #request_code
                                    let req_body = azure_core::EMPTY_BODY;
                                    let req = req_builder.body(req_body).context(azure_core::error::ErrorKind::Other, "build request")?;
                                    this.client.send(req).await.context(azure_core::error::ErrorKind::Io, "execute request")?
                                }
                                None => {
                                    #ts_request_builder
                                    req_builder = req_builder.uri(url.as_str());
                                    let req = req_builder.body(req_body).context(azure_core::error::ErrorKind::Other, "build request")?;
                                    this.client.send(req).await.context(azure_core::error::ErrorKind::Io, "execute request")?
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
    } else if operation.0.long_running_operation {
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

    let fname = operation.function_name()?;
    let module_code = quote! {

        pub mod #fname {
            use super::models;
            use azure_core::error::ResultExt;

            #response_enum

            #builder_struct_code

            impl Builder {
                #builder_setters_code
                #fut
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

#[derive(PartialEq)]
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

struct FunctionParam {
    name: String,
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

struct FunctionParamsCode {
    params: Vec<FunctionParam>,
}
impl FunctionParamsCode {
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
}

impl ToTokens for FunctionParamsCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut params: Vec<TokenStream> = Vec::new();
        for FunctionParam {
            variable_name, type_name, ..
        } in self.required_params()
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

fn create_function_params_code(parameters: &[&WebParameter]) -> Result<FunctionParamsCode, Error> {
    let mut params = Vec::new();
    for param in parameters.iter() {
        let name = param.name().to_owned();
        let variable_name = name.to_snake_case_ident().map_err(Error::ParamName)?;
        let type_name = type_name_gen(&param.type_name()?)?.qualify_models(true).optional(!param.required());
        let kind = ParamKind::from(param.type_());
        let collection_format = param.collection_format().clone();
        params.push(FunctionParam {
            name,
            variable_name,
            type_name,
            kind,
            collection_format,
        });
    }
    Ok(FunctionParamsCode { params })
}

fn create_builder_instance_code(
    operation: &WebOperationGen,
    parameters: &FunctionParamsCode,
    in_group: bool,
) -> Result<TokenStream, Error> {
    let mut params: Vec<TokenStream> = Vec::new();
    if in_group {
        params.push(quote! { client: self.0.clone() });
    } else {
        params.push(quote! { client: self.clone() });
    }
    for param in parameters.required_params() {
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
    for param in parameters.optional_params() {
        let FunctionParam {
            variable_name, type_name, ..
        } = param;
        if type_name.is_vec() {
            params.push(quote! { #variable_name: Vec::new() });
        } else {
            params.push(quote! { #variable_name: None });
        }
    }
    let summary = if let Some(summary) = &operation.0.summary {
        quote! {
            #[doc = #summary]
        }
    } else {
        quote! {}
    };
    let fname = operation.function_name()?;
    Ok(quote! {
        #summary
        pub fn #fname(#parameters) -> #fname::Builder {
            #fname::Builder {
                #(#params),*
            }
        }
    })
}

fn create_builder_struct_code(parameters: &FunctionParamsCode, in_group: bool) -> Result<TokenStream, Error> {
    let mut params: Vec<TokenStream> = Vec::new();
    if in_group {
        params.push(quote! { pub(crate) client: super::super::Client });
    } else {
        params.push(quote! { pub(crate) client: super::Client });
    }
    for param in parameters.required_params() {
        let FunctionParam {
            variable_name, type_name, ..
        } = param;
        params.push(quote! { pub(crate) #variable_name: #type_name });
    }
    for param in parameters.optional_params() {
        let FunctionParam {
            variable_name, type_name, ..
        } = param;
        let mut type_name = type_name.clone();
        if type_name.is_vec() {
            type_name = type_name.optional(false);
        }
        params.push(quote! { pub(crate) #variable_name: #type_name });
    }
    Ok(quote! {
        #[derive(Clone)]
        pub struct Builder {
            #(#params),*
        }
    })
}

fn create_builder_setters_code(parameters: &FunctionParamsCode) -> Result<TokenStream, Error> {
    let mut setters = TokenStream::new();
    for param in parameters.optional_params() {
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
        setters.extend(quote! {
            pub fn #variable_name(mut self, #variable_name: #type_name) -> Self {
                self.#variable_name = #value;
                self
            }
        });
    }
    Ok(setters)
}

pub fn create_response_type(rsp: &Response) -> Result<Option<TypeNameCode>, Error> {
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
