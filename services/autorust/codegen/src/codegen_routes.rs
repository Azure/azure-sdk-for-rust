use crate::codegen::TypeNameCode;
use crate::codegen_operations::WebOperationGen;
use crate::identifier::{parse_ident, CamelCaseIdent, SnakeCaseIdent};
use crate::spec::{get_type_name_for_schema_ref, WebParameter, WebVerb};
use crate::{codegen::PARAM_RE, status_codes::get_status_code_name, CodeGen};
use crate::{status_codes, Error, ErrorKind};
use autorust_openapi::{ReferenceOr, Response, StatusCode};
use camino::Utf8Path;
use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use regex::Replacer;

/// Create a route call from the function name and to routes
fn add_route(routes: &mut Vec<TokenStream>, module_name: Option<&str>, function_name: &str) -> crate::Result<()> {
    let function_name = parse_ident(function_name)?;
    match module_name {
        Some(module_name) => {
            let module_name = parse_ident(module_name)?;
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

// TODO return RoutesCode & impl ToTokens
pub fn create_routes(cg: &CodeGen) -> crate::Result<TokenStream> {
    let mut modules: IndexMap<Option<String>, TokenStream> = IndexMap::new();
    let mut routes = Vec::new();
    // println!("input_files {:?}", cg.input_files());
    for (doc_file, _doc) in cg.spec.docs() {
        // only operations from listed input files
        // println!("doc_file {:?}", doc_file);
        if cg.spec.is_input_file(doc_file) {
            let operations: Vec<_> = cg.spec.operations()?.into_iter().map(WebOperationGen::new).collect();
            for op in operations {
                let module_name = op.rust_module_name();
                let function_name = op.rust_function_name();
                add_route(&mut routes, module_name.as_deref(), function_name.as_ref())?;
                let function = create_function(doc_file, &op)?;
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

    let mut file = TokenStream::new();
    file.extend(quote! {
        #![allow(unused_mut)]
        #![allow(unused_variables)]
        #![allow(unused_imports)]
        use crate::read_example_response_body;
        use super::models::*;
        use rocket::serde::json::Json;
    });
    file.extend(quote! {
        pub fn routes() -> Vec<rocket::Route> {
            routes![#(#routes),*]
        }
    });
    for (module_name, module) in modules {
        match module_name {
            Some(module_name) => {
                let name = parse_ident(&module_name)?;
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
    Ok(file)
}

fn create_function(doc_file: &Utf8Path, operation: &WebOperationGen) -> crate::Result<TokenStream> {
    let function_name = operation.rust_function_name();
    let fname = parse_ident(&function_name)?;

    let params = operation.parameters();
    let params: crate::Result<Vec<_>> = params.iter().map(|p| parse_ident(&p.name().to_snake_case())).collect();
    let params = params?;
    let _url_str_args = quote! { #(#params),* };

    let parameters = operation.parameters();

    let fparams = create_function_params(&parameters)?;

    let examples_name = parse_ident(&format!("{}_examples", function_name.to_snake_case()))?;
    let examples = get_operation_examples(operation);
    let base_path = doc_file;
    let examples_mod = create_examples_mod(base_path, &examples_name, &examples)?;
    let first_example = examples.0.first();

    let success_responses = operation.success_responses();
    let error_responses = operation.error_responses();
    let is_single_response = success_responses.len() == 1;
    let default_response = operation.default_response();
    let has_default_response = default_response.is_some();

    let responses = get_operation_responses(operation)?;
    let responder_name = parse_ident(&format!("{}Response", function_name.to_camel_case_id()))?;
    let responder = create_responder(&responder_name, &responses)?;

    let fresponse = quote! { Result<#responder_name, crate::CloudErrorResponse> };

    let mut response_enum = TokenStream::new();
    if !is_single_response {
        let mut success_responses_ts = TokenStream::new();
        for (status_code, rsp) in &success_responses {
            let tp = create_response_type(rsp)?;
            let tp = match tp {
                Some(tp) => quote! { (#tp) },
                None => quote! {},
            };
            let enum_type_name = get_response_type_name(status_code)?;
            success_responses_ts.extend(quote! { #enum_type_name #tp, })
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
        let response_type = &get_response_type_name(status_code)?;
        if response_type == "DefaultResponse" {
            error_responses_ts.extend(quote! {
                #[error("HTTP status code {}", status_code)]
                DefaultResponse { status_code: http::StatusCode, #tp },
            });
        } else {
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
                let status_code_name = get_status_code_ident_camel_case(status_code)?;
                let response_type_name = get_response_type_name(status_code)?;
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
                let status_code_name = parse_ident(get_status_code_name(status_code)?)?;
                let response_type_name = get_response_type_name(status_code)?;
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

    let verb = match operation.verb() {
        WebVerb::Get => quote! { get },
        WebVerb::Post => quote! { post },
        WebVerb::Put => quote! { put },
        WebVerb::Patch => quote! { patch },
        WebVerb::Delete => quote! { delete },
        WebVerb::Options => quote! { options },
        WebVerb::Head => quote! { head },
    };

    let api_version = operation.api_version();
    let route_path = route_path(operation.path());
    let mut verb_parts = Vec::new();
    let path = format!("{}?api-version={}", route_path, api_version);
    verb_parts.push(quote! { #path });
    if let Some(param) = operation.body_parameter() {
        let data = format!("<{}>", param.name().to_snake_case_id());
        verb_parts.push(quote! { data = #data });
    }
    // Use operationId as the route name
    if let Some(operation_id) = operation.id() {
        verb_parts.push(quote! { name = #operation_id });
    }
    let route = quote! { #verb (#(#verb_parts),*) };

    let terminal_responses = responses.terminal_responses();
    let first_response = responses
        .0
        .first()
        .ok_or_else(|| Error::with_message(ErrorKind::CodeGen, || format!("no first response {:?}", operation.id())))?;
    // make sure it is a terminal response
    let first_response = terminal_responses.first().unwrap_or(&first_response);
    let status_code = &StatusCode::Code(first_response.status_code.ok_or_else(|| {
        Error::with_message(ErrorKind::CodeGen, || {
            format!("first response missing status code {:?}", operation.id())
        })
    })?);
    let status_code_name = get_status_code_ident_camel_case(status_code)?;
    let response_type = get_response_type_ident(status_code)?;
    let first_responder = match (&first_example, &first_response.body_type_name) {
        (Some(first_example), Some(_body)) => {
            let first_example_name = parse_ident(&first_example.const_name())?;
            quote! {
                #responder_name::#response_type(read_example_response_body(#examples_name::#first_example_name, &rocket::http::Status::#status_code_name)?)
            }
        }
        _ => quote! {
            #responder_name::#response_type
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
    Ok(func)
}

fn get_response_type_name(status_code: &StatusCode) -> crate::Result<Ident> {
    status_codes::get_status_code_ident(status_code)
}

fn create_examples_mod(base_path: &Utf8Path, name: &Ident, examples: &OperationExamples) -> crate::Result<TokenStream> {
    let mut values = TokenStream::new();
    for example in &examples.0 {
        let name = parse_ident(&example.const_name())?;
        let trim = "../../../azure-rest-api-specs-pr/specification/";
        let file = crate::io::join(&base_path.to_string()[trim.len()..], &example.file)?;
        let file = file.to_string();
        let file = file.replace('\\', "/");
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

fn create_function_params(parameters: &[&WebParameter]) -> crate::Result<TokenStream> {
    let mut params: Vec<TokenStream> = Vec::new();
    for param in parameters.iter() {
        let name = param.name().to_snake_case_ident()?;
        if param.in_body() {
            let tp = TypeNameCode::new(&param.type_name()?)?.into_token_stream();
            let body_tp = quote! { Json<#tp> };
            params.push(quote! { #name: #body_tp });
        } else {
            let tp = TypeNameCode::new_ref(&param.type_name()?)?;
            params.push(quote! { #name: #tp });
        }
    }
    Ok(quote! { #(#params),* })
}

fn create_response_type(rsp: &Response) -> crate::Result<Option<TypeNameCode>> {
    if let Some(schema) = &rsp.schema {
        Ok(Some(TypeNameCode::new(&get_type_name_for_schema_ref(schema)?)?))
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

fn get_operation_examples(operation: &WebOperationGen) -> OperationExamples {
    let mut examples = Vec::new();
    for (name, example) in operation.examples() {
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

struct OperationResponse {
    status_code: Option<u16>,
    body_type_name: Option<TypeNameCode>,
    is_long_running: bool,
}

struct OperationRespones(pub Vec<OperationResponse>);

impl OperationRespones {
    // In long-running operations (LROs) or asynchronous operations,
    // 201 & 202 responses are not terminal responses.
    // https://docs.microsoft.com/en-us/azure/azure-resource-manager/management/async-operations
    pub fn terminal_responses(&self) -> Vec<&OperationResponse> {
        self.0
            .iter()
            .filter(|rsp| {
                if rsp.is_long_running {
                    !matches!(rsp.status_code, Some(201) | Some(202))
                } else {
                    true
                }
            })
            .collect()
    }
}

fn get_operation_responses(operation: &WebOperationGen) -> crate::Result<OperationRespones> {
    let mut responses = Vec::new();
    for (status_code, response) in operation.success_responses() {
        let body_type_name = response.schema.as_ref().map(get_type_name_for_schema_ref).transpose()?;
        let body_type_name = body_type_name.map(|tn| TypeNameCode::new(&tn)).transpose()?;
        let status_code = match status_code {
            StatusCode::Code(status_code) => Some(status_code),
            StatusCode::Default => None,
        };
        responses.push(OperationResponse {
            status_code: status_code.copied(),
            body_type_name,
            is_long_running: operation.long_running_operation(),
        });
    }
    Ok(OperationRespones(responses))
}

fn create_responder(name: &Ident, responses: &OperationRespones) -> crate::Result<TokenStream> {
    let mut values = Vec::new();
    let mut respond_tos = Vec::new();
    for response in &responses.0 {
        let status_code = &response.status_code;
        let status_code = &StatusCode::Code(status_code.ok_or_else(|| Error::message(ErrorKind::CodeGen, "status code required"))?);
        let status_code_name = get_status_code_ident_camel_case(status_code)?;
        let response_type = get_response_type_ident(status_code)?;
        match &response.body_type_name {
            Some(body) => {
                values.push(quote! { #response_type(#body) });
                respond_tos.push(quote! {
                    Self::#response_type(v) => (rocket::http::Status::#status_code_name, Json(v)).respond_to(request)
                });
            }
            None => {
                values.push(quote! { #response_type });
                respond_tos.push(quote! {
                    Self::#response_type => rocket::http::Status::#status_code_name.respond_to(request)
                });
            }
        };
    }
    Ok(quote! {
        pub enum #name {
            #(#values),*
        }
        impl<'r> rocket::response::Responder<'r, 'static> for #name {
            fn respond_to(self, request: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                match self {
                    #(#respond_tos),*
                }
            }
        }
    })
}

pub fn get_status_code_name_with_number(status_code: &StatusCode) -> crate::Result<String> {
    match status_code {
        StatusCode::Code(number) => Ok(format!(
            "{}{}",
            status_codes::get_status_code_name(status_code)?.to_pascal_case(),
            number
        )),
        StatusCode::Default => Ok("Default".to_owned()),
    }
}

fn get_response_type_ident(status_code: &StatusCode) -> crate::Result<Ident> {
    parse_ident(&get_status_code_name_with_number(status_code)?)
}

fn get_status_code_ident_camel_case(status_code: &StatusCode) -> crate::Result<Ident> {
    status_codes::get_status_code_ident(status_code)
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
