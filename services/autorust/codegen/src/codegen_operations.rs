mod auth_code;
mod create_client_and_builder;
mod function_call_params;
mod function_code;
mod function_params;
mod new_request_code;
mod operation_module;
mod operations;
mod request_builder_into_future;
mod request_builder_send;
mod request_builder_setter;
mod request_builder_struct;
mod response_code;
mod response_headers;
mod set_request_code;
mod set_request_param_code;
mod web_operation_gen;

use crate::Result;
use crate::{identifier::parse_ident, spec::WebVerb, CodeGen};
use heck::ToPascalCase;
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use std::collections::BTreeSet;

use self::create_client_and_builder::create_client;
use self::operations::{create_operation_code, OperationCode};
use self::web_operation_gen::WebOperationGen;

pub const API_VERSION: &str = "api-version";
pub const X_MS_VERSION: &str = "x-ms-version";

fn error_variant(operation: &WebOperationGen) -> Result<Ident> {
    let function = operation.rust_function_name().to_pascal_case();
    if let Some(module) = operation.rust_module_name() {
        let module = module.to_pascal_case();
        parse_ident(&format!("{module}_{function}"))
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
                    mut client_functions,
                    mut module_code,
                } = code;
                operation_code.client_functions.append(&mut client_functions);
                operation_code.module_code.append(&mut module_code);
            }
            None => {
                operations_code.insert(module_name, code);
            }
        }
    }

    for (module_name, operation_code) in operations_code {
        let OperationCode {
            client_functions,
            module_code,
        } = operation_code;
        let mut builders = TokenStream::new();
        for builder in client_functions {
            builders.extend(builder.into_token_stream());
        }
        match module_name {
            Some(module_name) => {
                let name = parse_ident(&module_name)?;
                file.extend(quote! {
                    pub mod #name {
                        use super::models;
                        #[cfg(target_arch = "wasm32")]
                        use futures::future::LocalBoxFuture as BoxFuture;
                        #[cfg(not(target_arch = "wasm32"))]
                        use futures::future::BoxFuture as BoxFuture;
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

/// Creating a function name from the path and verb when an operationId is not specified.
/// All azure-rest-api-specs operations should have an operationId.
fn create_function_name(verb: &WebVerb, path: &str) -> String {
    let mut path = path.split('/').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
    path.insert(0, verb.as_str());
    path.join("_")
}

#[cfg(test)]
mod tests {
    use crate::WebOperation;

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
