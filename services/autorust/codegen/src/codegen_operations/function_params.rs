use autorust_openapi::CollectionFormat;
use proc_macro2::Ident;

use crate::codegen::{parse_query_params, TypeNameCode};
use crate::identifier::SnakeCaseIdent;
use crate::spec::WebParameter;
use crate::{CodeGen, Result};

use super::web_operation_gen::WebOperationGen;
use super::{ParamKind, API_VERSION, X_MS_VERSION};

#[derive(Clone)]
pub struct FunctionParams {
    params: Vec<FunctionParam>,
    has_api_version: bool,
    has_x_ms_version: bool,
}
impl FunctionParams {
    pub fn new(cg: &CodeGen, operation: &WebOperationGen) -> Result<Self> {
        let parameters = operation.0.parameters();
        let has_api_version = parameters.iter().any(|p| p.name() == API_VERSION);
        let has_x_ms_version = parameters.iter().any(|p| p.name() == X_MS_VERSION);
        let mut skip = parse_query_params(&operation.0.path)?;
        skip.insert(API_VERSION.to_string());
        skip.insert(X_MS_VERSION.to_string());
        let parameters: Vec<&WebParameter> = parameters.clone().into_iter().filter(|p| !skip.contains(p.name())).collect();

        let mut params = Vec::new();
        for param in parameters.iter().filter(|p| !skip.contains(p.name())) {
            let name = param.name().to_owned();
            let description = param.description().clone();
            let variable_name = name.to_snake_case_ident()?;
            let mut type_name = TypeNameCode::new(&param.type_name()?)?;
            type_name.qualify_models(true);
            type_name.optional(!param.required());
            cg.set_if_union_type(&mut type_name);
            let kind = ParamKind::from(param.in_());
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
        Ok(Self {
            params,
            has_api_version,
            has_x_ms_version,
        })
    }

    pub fn params(&self) -> Vec<&FunctionParam> {
        self.params.iter().collect()
    }
    pub fn required_params(&self) -> Vec<&FunctionParam> {
        self.params.iter().filter(|p| !p.type_name.is_optional()).collect()
    }
    pub fn optional_params(&self) -> Vec<&FunctionParam> {
        self.params.iter().filter(|p| p.type_name.is_optional()).collect()
    }
    #[allow(dead_code)]
    fn params_of_kind(&self, kind: &ParamKind) -> Vec<&FunctionParam> {
        self.params.iter().filter(|p| &p.kind == kind).collect()
    }

    pub fn has_content_type_header(&self) -> bool {
        self.params()
            .iter()
            .any(|p| p.name.eq_ignore_ascii_case("content-type") && p.kind == ParamKind::Header)
    }

    pub fn has_api_version(&self) -> bool {
        self.has_api_version
    }

    pub fn has_x_ms_version(&self) -> bool {
        self.has_x_ms_version
    }
}

#[derive(Clone)]
pub struct FunctionParam {
    pub name: String,
    pub description: Option<String>,
    pub variable_name: Ident,
    pub type_name: TypeNameCode,
    pub kind: ParamKind,
    pub collection_format: CollectionFormat,
}

impl FunctionParam {
    pub fn is_vec(&self) -> bool {
        self.type_name.is_vec()
    }
    pub fn is_optional(&self) -> bool {
        self.type_name.is_optional()
    }
    pub fn is_string(&self) -> bool {
        self.type_name.is_string()
    }
}
