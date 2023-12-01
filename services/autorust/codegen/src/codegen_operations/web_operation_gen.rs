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

use super::{API_VERSION, X_MS_VERSION};

pub struct WebOperationGen(pub WebOperation);

impl WebOperationGen {
    #[allow(dead_code)]
    pub fn new(operation: WebOperation) -> Self {
        Self(operation)
    }

    #[allow(dead_code)]
    pub fn verb(&self) -> &WebVerb {
        &self.0.verb
    }

    #[allow(dead_code)]
    pub fn path(&self) -> &str {
        &self.0.path
    }

    #[allow(dead_code)]
    pub fn id(&self) -> Option<&str> {
        self.0.id.as_deref()
    }

    #[allow(dead_code)]
    pub fn examples(&self) -> &IndexMap<String, ReferenceOr<Operation>> {
        &self.0.examples
    }

    #[allow(dead_code)]
    pub fn long_running_operation(&self) -> bool {
        self.0.long_running_operation
    }

    #[allow(dead_code)]
    pub fn parameters(&self) -> Vec<&WebParameter> {
        self.0
            .parameters()
            .into_iter()
            .filter(|p| !matches!(p.name(), API_VERSION | X_MS_VERSION))
            .collect()
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
    pub fn error_responses(&self) -> IndexMap<&StatusCode, &Response> {
        self.0
            .responses
            .iter()
            .filter(|(status_code, _)| !crate::status_codes::is_error(status_code))
            .collect()
    }

    #[allow(dead_code)]
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

/// Creating a function name from the path and verb when an operationId is not specified.
/// All azure-rest-api-specs operations should have an operationId.
fn create_function_name(verb: &WebVerb, path: &str) -> String {
    let mut path = path.split('/').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
    path.insert(0, verb.as_str());
    path.join("_")
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
