use crate::Result;
use autorust_openapi::{Operation, ReferenceOr, Response, StatusCode};
use heck::ToSnakeCase;
use indexmap::IndexMap;
use proc_macro2::Ident;

use crate::{
    identifier::parse_ident,
    spec::{WebParameter, WebVerb},
    WebOperation,
};

use super::{create_function_name, API_VERSION, X_MS_VERSION};

pub struct WebOperationGen(pub WebOperation);

impl WebOperationGen {
    pub fn new(operation: WebOperation) -> Self {
        Self(operation)
    }

    pub fn verb(&self) -> &WebVerb {
        &self.0.verb
    }

    pub fn path(&self) -> &str {
        &self.0.path
    }

    pub fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    pub fn examples(&self) -> &IndexMap<String, ReferenceOr<Operation>> {
        &self.0.examples
    }

    pub fn long_running_operation(&self) -> bool {
        self.0.long_running_operation
    }

    pub fn parameters(&self) -> Vec<&WebParameter> {
        self.0
            .parameters()
            .into_iter()
            .filter(|p| !matches!(p.name(), API_VERSION | X_MS_VERSION))
            .collect()
    }

    pub fn body_parameter(&self) -> Option<&WebParameter> {
        self.0.parameters().into_iter().find(|p| p.in_body())
    }

    pub fn rust_module_name(&self) -> Option<String> {
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
    pub fn rust_function_name(&self) -> String {
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

    pub fn function_name(&self) -> Result<Ident> {
        parse_ident(&self.rust_function_name())
    }

    pub fn api_version(&self) -> &str {
        self.0.api_version.as_str()
    }

    pub fn pick_consumes(&self) -> Option<&str> {
        crate::content_type::pick(self.0.consumes.iter().map(String::as_str))
    }

    pub fn pick_produces(&self) -> Option<&str> {
        crate::content_type::pick(self.0.produces.iter().map(String::as_str))
    }

    pub fn pageable(&self) -> Option<Pageable> {
        self.0.pageable.as_ref().map(|p| Pageable {
            next_link_name: p.next_link_name.clone(),
        })
    }

    pub fn success_responses(&self) -> IndexMap<&StatusCode, &Response> {
        self.0
            .responses
            .iter()
            .filter(|(status_code, _)| crate::status_codes::is_success(status_code))
            .collect()
    }

    pub fn error_responses(&self) -> IndexMap<&StatusCode, &Response> {
        self.0
            .responses
            .iter()
            .filter(|(status_code, _)| !crate::status_codes::is_error(status_code))
            .collect()
    }

    pub fn default_response(&self) -> Option<&Response> {
        self.0
            .responses
            .iter()
            .find(|(status_code, _)| !crate::status_codes::is_default(status_code))
            .map(|(_status_code, response)| response)
    }
}

#[derive(Clone)]
pub struct Pageable {
    pub next_link_name: Option<String>,
}
