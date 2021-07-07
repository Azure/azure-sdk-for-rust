#![allow(dead_code)]
use autorust_openapi::{ParameterType, ReferenceOr, Response, StatusCode};
use heck::ShoutySnakeCase;
use heck::{CamelCase, SnakeCase};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Replacer;
use std::path::Path;

use crate::{
    codegen::{
        create_generated_by_header, get_type_name_for_schema, get_type_name_for_schema_ref, is_array, parse_params, AsReference, Error,
        PARAM_RE,
    },
    identifier::ident,
    path, spec,
    status_codes::{
        get_error_responses, get_response_type, get_response_type_name, get_status_code_name, get_success_responses, has_default_response,
    },
    CodeGen, OperationVerb,
};

/// Create a route call from the function name and to routes
fn add_route(routes: &mut Vec<TokenStream>, module_name: Option<&str>, function_name: &str) -> Result<(), Error> {
    let function_name = ident(function_name).map_err(Error::FunctionName)?;
    match module_name {
        Some(module_name) => {
            let module_name = ident(module_name).map_err(Error::ModuleName)?;
            routes.push(quote! {
                #module_name::#function_name
            });
        }
        None => {
            routes.push(quote! {
                #function_name
            });
        }
    }
    Ok(())
}

pub fn create_routes(cg: &CodeGen) -> Result<TokenStream, Error> {
    let mut file = TokenStream::new();
    file.extend(create_generated_by_header());
    file.extend(quote! {
        #![allow(unused_mut)]
        #![allow(unused_variables)]
        #![allow(unused_imports)]
        use crate::read_example_body;
        use super::models::*;
        use rocket::serde::json::Json;
    });
    let mut modules: IndexMap<Option<String>, TokenStream> = IndexMap::new();
    let mut routes = Vec::new();
    // println!("input_files {:?}", cg.input_files());
    for (doc_file, doc) in cg.spec.docs() {
        // only operations from listed input files
        // println!("doc_file {:?}", doc_file);
        if cg.spec.is_input_file(&doc_file) {
            let paths = cg.spec.resolve_path_map(doc_file, &doc.paths)?;
            for (path, item) in &paths {
                for op in spec::path_item_operations(item) {
                    let (module_name, function_name) = op.function_name(path);
                    add_route(&mut routes, module_name.as_deref(), function_name.as_ref())?;
                    let function = create_function(cg, doc_file, path, &op, &function_name)?;
                    if modules.contains_key(&module_name) {}
                    match modules.get_mut(&module_name) {
                        Some(module) => {
                            module.extend(function);
                        }
                        None => {
                            let mut module = TokenStream::new();
                            module.extend(function);
                            modules.insert(module_name, module);
                        }
                    }
                }
            }
        }
    }
    for (module_name, module) in modules {
        match module_name {
            Some(module_name) => {
                let name = ident(&module_name).map_err(Error::ModuleName)?;
                file.extend(quote! {
                    pub mod #name {
                        use super::*;
                        #module
                    }
                });
            }
            None => {
                file.extend(module);
            }
        }
    }
    file.extend(quote! {
        pub fn routes() -> Vec<rocket::Route> {
            routes![#(#routes),*]
        }
    });
    Ok(file)
}

pub struct OperationParameters(Vec<OperationParameter>);

pub struct OperationParameter {
    name: String,
    in_: ParameterType,
    type_: TokenStream,
    is_required: bool,
    is_array: bool,
}

impl OperationParameter {
    pub fn snake_case_name(&self) -> String {
        self.name.to_snake_case()
    }
    pub fn snake_case_name_ident(&self) -> Result<TokenStream, Error> {
        ident(&self.snake_case_name()).map_err(Error::ParamName)
    }
    pub fn in_body(&self) -> bool {
        self.in_ == ParameterType::Body
    }
    pub fn in_path(&self) -> bool {
        self.in_ == ParameterType::Path
    }
    pub fn in_query(&self) -> bool {
        self.in_ == ParameterType::Query
    }
    pub fn in_header(&self) -> bool {
        self.in_ == ParameterType::Header
    }
    pub fn in_form(&self) -> bool {
        self.in_ == ParameterType::Form
    }
}

impl OperationParameters {
    pub fn create(cg: &CodeGen, doc_file: &Path, operation: &OperationVerb) -> Result<OperationParameters, Error> {
        let parameters = cg.spec.resolve_parameters(doc_file, &operation.operation().parameters)?;
        let mut v = Vec::new();
        for param in &parameters {
            let name = param.name.to_owned();
            let in_ = param.in_.to_owned();
            if name != "api-version" {
                let type_ = {
                    if let Some(_param_type) = &param.common.type_ {
                        get_type_name_for_schema(&param.common, AsReference::True)?
                    } else if let Some(schema) = &param.schema {
                        get_type_name_for_schema_ref(schema, AsReference::False)?
                    } else {
                        eprintln!("WARN unknown param type for {}", &param.name);
                        quote! { &serde_json::Value }
                    }
                };
                let is_required = param.required.unwrap_or(false);
                let is_array = is_array(&param.common);
                v.push(OperationParameter {
                    name,
                    in_,
                    type_,
                    is_required,
                    is_array,
                })
            }
        }
        Ok(OperationParameters(v))
    }

    pub fn iter(&self) -> std::slice::Iter<OperationParameter> {
        self.0.iter()
    }

    pub fn get_body(&self) -> Option<&OperationParameter> {
        self.iter().find(|param| param.in_body())
    }
}

fn create_function(
    cg: &CodeGen,
    doc_file: &Path,
    path: &str,
    operation_verb: &OperationVerb,
    function_name: &str,
) -> Result<TokenStream, Error> {
    let fname = ident(function_name).map_err(Error::FunctionName)?;

    let params = parse_params(path);
    // println!("path params {:#?}", params);
    let params: Result<Vec<_>, Error> = params
        .iter()
        .map(|s| Ok(ident(&s.to_snake_case()).map_err(Error::ParamName)?))
        .collect();
    let params = params?;
    let _url_str_args = quote! { #(#params),* };

    let parameters = OperationParameters::create(cg, doc_file, &operation_verb)?;

    let fparams = create_function_params(&parameters)?;

    let examples_name = ident(&format!("{}_examples", function_name.to_snake_case())).map_err(Error::FunctionName)?;
    let examples = get_operation_examples(operation_verb);
    let base_path = doc_file.clone();
    let examples_mod = create_examples_mod(base_path, &examples_name, &examples)?;
    let first_example = examples.0.first().ok_or_else(|| Error::OperationMissingExample)?;

    let responses = &operation_verb.operation().responses;
    let success_responses = get_success_responses(responses);
    let error_responses = get_error_responses(responses);
    let is_single_response = success_responses.len() == 1;
    let has_default_response = has_default_response(responses);

    let responses = get_operation_responses(&operation_verb)?;
    let responder_name = ident(&format!("{}Responder", function_name.to_camel_case())).map_err(Error::FunctionName)?;
    let responder = create_responder(&responder_name, &responses)?;

    let fresponse = quote! { Result<#responder_name, crate::CloudErrorResponder> };

    let mut response_enum = TokenStream::new();
    if !is_single_response {
        let mut success_responses_ts = TokenStream::new();
        for (status_code, rsp) in &success_responses {
            let tp = create_response_type(rsp)?;
            let tp = match tp {
                Some(tp) => quote! { (#tp) },
                None => quote! {},
            };
            let enum_type_name = ident(&get_response_type_name(status_code)).map_err(Error::ResponseTypeName)?;
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
            Some(tp) => quote! { value: models::#tp, },
            None => quote! {},
        };
        let response_type = &get_response_type_name(status_code);
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
                let status_code_name = ident(&get_status_code_name(status_code)).map_err(Error::StatusCodeName)?;
                let response_type_name = ident(&get_response_type_name(status_code)).map_err(Error::ResponseTypeName)?;
                if is_single_response {
                    match tp {
                        Some(tp) => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    let rsp_body = rsp.body();
                                    let rsp_value: #tp = serde_json::from_slice(rsp_body).map_err(|source| #fname::Error::DeserializeError(source, rsp_body.clone()))?;
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
                        Some(tp) => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    let rsp_body = rsp.body();
                                    let rsp_value: #tp = serde_json::from_slice(rsp_body).map_err(|source| #fname::Error::DeserializeError(source, rsp_body.clone()))?;
                                    Ok(#fname::Response::#response_type_name(rsp_value))
                                }
                            });
                        }
                        None => {
                            match_status.extend(quote! {
                                http::StatusCode::#status_code_name => {
                                    Ok(#fname::Response::#response_type_name)
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
                let status_code_name = ident(&get_status_code_name(status_code)).map_err(Error::StatusCodeName)?;
                let response_type_name = ident(&get_response_type_name(status_code)).map_err(Error::ResponseTypeName)?;
                match tp {
                    Some(tp) => {
                        match_status.extend(quote! {
                            http::StatusCode::#status_code_name => {
                                let rsp_body = rsp.body();
                                let rsp_value: #tp = serde_json::from_slice(rsp_body).map_err(|source| #fname::Error::DeserializeError(source, rsp_body.clone()))?;
                                Err(#fname::Error::#response_type_name{value: rsp_value})
                            }
                        });
                    }
                    None => {
                        match_status.extend(quote! {
                            http::StatusCode::#status_code_name => {
                                Err(#fname::Error::#response_type_name{})
                            }
                        });
                    }
                }
            }
            autorust_openapi::StatusCode::Default => {}
        }
    }

    let verb = match operation_verb {
        OperationVerb::Get(_) => quote! { get },
        OperationVerb::Post(_) => quote! { post },
        OperationVerb::Put(_) => quote! { put },
        OperationVerb::Patch(_) => quote! { patch },
        OperationVerb::Delete(_) => quote! { delete },
        OperationVerb::Options(_) => quote! { options },
        OperationVerb::Head(_) => quote! { head },
    };

    let api_version = cg.api_version().ok_or_else(|| Error::MissingApiVersion)?;
    let route_path = route_path(path);
    let mut verb_parts = Vec::new();
    let path = format!("{}?api-version={}", route_path, api_version);
    verb_parts.push(quote! { #path });
    match parameters.get_body() {
        Some(param) => {
            let data = format!("<{}>", param.snake_case_name());
            verb_parts.push(quote! { data = #data });
        }
        None => {}
    }
    let route = quote! { #verb (#(#verb_parts),*) };

    let first_response = responses.0.first().ok_or_else(|| Error::OperationMissingResponses)?;
    let first_example_name = ident(&first_example.const_name()).map_err(Error::ExamplesName)?;
    let status_code = first_response.status_code.ok_or_else(|| Error::StatusCodeRequired)?;
    let response_type = ident(&get_response_type(status_code).map_err(Error::ResponseType)?).map_err(Error::ResponseTypeName)?;
    let first_responder = match &first_response.body_type_name {
        Some(_body) => quote! {
            #responder_name::#response_type(Json(read_example_body(#examples_name::#first_example_name, 0)?))
        },
        None => quote! {
            #responder_name::#response_type(None)
        },
    };

    let func = quote! {
        #responder
        #examples_mod
        #[#route]
        pub fn #fname(#fparams) -> #fresponse {
            Ok(#first_responder)
        }
    };
    Ok(TokenStream::from(func))
}

fn create_examples_mod(base_path: &Path, name: &TokenStream, examples: &OperationExamples) -> Result<TokenStream, Error> {
    let mut values = TokenStream::new();
    for example in &examples.0 {
        let name = ident(&example.const_name()).map_err(Error::ExamplesName)?;
        let file = path::join(base_path, &example.file).map_err(Error::ExamplePath)?;
        let file = path::join("../", file).map_err(Error::ExamplePath)?; // TODO add to config
        let file = file.to_str().ok_or_else(|| Error::ExamplePathNotUtf8)?;
        let file = file.replace("\\", "/");
        values.extend(quote! {
            pub const #name: &str = #file;
        });
    }
    Ok(quote! {
        pub mod #name {
            #values
        }
    })
}

fn create_function_params(parameters: &OperationParameters) -> Result<TokenStream, Error> {
    let mut params: Vec<TokenStream> = Vec::new();
    for param in parameters.iter() {
        let name = param.snake_case_name_ident()?;
        let mut tp = &param.type_;
        let body_tp = quote! { Json<#tp> };
        if param.in_body() {
            tp = &body_tp;
        }
        params.push(quote! { #name: #tp });
    }
    Ok(quote! { #(#params),* })
}

fn create_response_type(rsp: &Response) -> Result<Option<TokenStream>, Error> {
    if let Some(schema) = &rsp.schema {
        Ok(Some(get_type_name_for_schema_ref(schema, AsReference::False)?))
    } else {
        Ok(None)
    }
}

struct ParamReplacer {}

impl regex::Replacer for ParamReplacer {
    fn replace_append(&mut self, caps: &regex::Captures, dst: &mut String) {
        let name = caps.get(1).unwrap().as_str();
        let name = format!("<{}>", name.to_snake_case());
        dst.push_str(name.as_str())
    }
}

fn route_path(spec_path: &str) -> String {
    let mut rep = ParamReplacer {};
    PARAM_RE.replace_all(spec_path, rep.by_ref()).to_string()
}

#[derive(Debug)]
struct OperationExample {
    name: String,
    file: String,
}

impl OperationExample {
    pub fn const_name(&self) -> String {
        self.name.to_shouty_snake_case()
    }
}

fn get_operation_examples(operation: &OperationVerb) -> OperationExamples {
    let operation = operation.operation();
    let mut examples = Vec::new();
    for (name, example) in &operation.x_ms_examples {
        match example {
            ReferenceOr::Reference { reference, .. } => match &reference.file {
                Some(file) => {
                    let name = name.to_owned();
                    let file = file.to_owned();
                    examples.push(OperationExample { name, file });
                }
                None => {}
            },
            ReferenceOr::Item(_) => {}
        }
    }
    OperationExamples(examples)
}

struct OperationExamples(pub Vec<OperationExample>);

#[derive(Debug)]
struct OperationResponse {
    status_code: Option<u16>,
    body_type_name: Option<TokenStream>,
}

struct OperationRespones(pub Vec<OperationResponse>);

fn get_operation_responses(operation: &OperationVerb) -> Result<OperationRespones, Error> {
    let operation = operation.operation();
    let mut responses = Vec::new();
    for (status_code, response) in get_success_responses(&operation.responses) {
        let body_type_name = response
            .schema
            .map(|ref schema| get_type_name_for_schema_ref(schema, AsReference::False))
            .transpose()?;
        let status_code = match status_code {
            StatusCode::Code(status_code) => Some(status_code),
            StatusCode::Default => None,
        };
        responses.push(OperationResponse {
            status_code,
            body_type_name,
        });
    }
    Ok(OperationRespones(responses))
}

fn create_responder(name: &TokenStream, responses: &OperationRespones) -> Result<TokenStream, Error> {
    let mut values = Vec::new();
    for response in &responses.0 {
        let status_code = &response.status_code;
        let status_code = status_code.ok_or_else(|| Error::StatusCodeRequired)?;
        let response_type = ident(&get_response_type(status_code).map_err(Error::ResponseType)?).map_err(Error::ResponseTypeName)?;
        let body = match &response.body_type_name {
            Some(body) => quote! { (Json<#body>) },
            None => quote! { (Option<serde_json::Value>) },
        };
        values.push(quote! {
            #[response(status = #status_code)]
            #response_type#body
        });
    }
    Ok(quote! {
        #[derive(Responder)]
        pub enum #name {
            #(#values),*
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_path() {
        let spec_path = "/subscriptions/{subscriptionId}/resourceGroups/{resourceGroupName}/providers/Microsoft.AVS/privateClouds";
        assert_eq!(
            route_path(spec_path),
            "/subscriptions/<subscription_id>/resourceGroups/<resource_group_name>/providers/Microsoft.AVS/privateClouds"
        );
    }
}
