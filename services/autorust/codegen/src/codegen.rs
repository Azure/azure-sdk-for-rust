use crate::{
    identifier::{self, ident, CamelCaseIdent},
    spec,
    status_codes::{get_error_responses, get_response_type_name, get_status_code_name, get_success_responses, has_default_response},
    Config, OperationVerb, PropertyName, ResolvedSchema, Spec,
};
use autorust_openapi::{
    CollectionFormat, DataType, Parameter, ParameterType, PathItem, Reference, ReferenceOr, Response, Schema, SchemaCommon,
};
use heck::{CamelCase, SnakeCase};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use regex::Regex;
use serde_json::Value;
use spec::{get_schema_schema_references, openapi, RefKey};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

/// code generation context
pub struct CodeGen {
    config: Config,
    pub spec: Spec,
}

impl CodeGen {
    pub fn new(config: Config) -> Result<Self> {
        let spec = Spec::read_files(&config.input_files).map_err(Error::SpecError)?;
        Ok(Self { config, spec })
    }

    pub fn input_files(&self) -> &[PathBuf] {
        &self.config.input_files
    }

    pub fn output_folder(&self) -> &Path {
        &self.config.output_folder
    }

    pub fn api_version(&self) -> Option<&str> {
        self.config.api_version.as_deref()
    }

    pub fn create_models(&self) -> Result<TokenStream> {
        let mut file = TokenStream::new();
        file.extend(create_generated_by_header());
        file.extend(quote! {
            #![allow(non_camel_case_types)]
            #![allow(unused_imports)]
            use serde::{Deserialize, Serialize};
        });
        let mut all_schemas: IndexMap<RefKey, ResolvedSchema> = IndexMap::new();

        // all definitions from input_files
        for (doc_file, doc) in self.spec.input_docs() {
            let schemas = self.spec.resolve_schema_map(doc_file, &doc.definitions).map_err(Error::SpecError)?;
            for (name, schema) in schemas {
                all_schemas.insert(
                    RefKey {
                        file_path: doc_file.to_owned(),
                        name,
                    },
                    schema,
                );
            }
        }

        // any referenced schemas from other files
        for (doc_file, doc) in self.spec.input_docs() {
            for reference in openapi::get_api_schema_references(doc) {
                self.add_schema_refs(&mut all_schemas, doc_file, reference)?;
            }
        }

        let mut schema_names = IndexMap::new();
        for (ref_key, schema) in &all_schemas {
            let doc_file = &ref_key.file_path;
            let schema_name = &ref_key.name;
            if let Some(_first_doc_file) = schema_names.insert(schema_name, doc_file) {
                // eprintln!(
                //     "WARN schema {} already created from {:?}, duplicate from {:?}",
                //     schema_name, first_doc_file, doc_file
                // );
            } else {
                if is_array(&schema.schema.common) {
                    file.extend(self.create_vec_alias(doc_file, schema_name, schema)?);
                } else if is_local_enum(schema) {
                    let no_namespace = TokenStream::new();
                    let (_tp_name, tp) = create_enum(&no_namespace, schema_name, schema)?;
                    file.extend(tp);
                } else {
                    for stream in self.create_struct(doc_file, schema_name, schema)? {
                        file.extend(stream);
                    }
                }
            }
        }
        Ok(file)
    }

    pub fn create_operations(&self) -> Result<TokenStream> {
        let mut file = TokenStream::new();
        file.extend(create_generated_by_header());
        file.extend(quote! {
            #![allow(unused_mut)]
            #![allow(unused_variables)]
            #![allow(unused_imports)]
            use crate::models::*;

        });
        let param_re = Regex::new(r"\{(\w+)\}").unwrap();
        let mut modules: IndexMap<Option<String>, TokenStream> = IndexMap::new();
        // println!("input_files {:?}", self.input_files());
        for (doc_file, doc) in self.spec.docs() {
            // only operations from listed input files
            // println!("doc_file {:?}", doc_file);
            if self.spec.is_input_file(&doc_file) {
                let paths = self.spec.resolve_path_map(doc_file, &doc.paths).map_err(Error::SpecError)?;
                for (path, item) in &paths {
                    for op in spec::path_item_operations(item) {
                        let (module_name, function_name) = op.function_name(path);
                        let function = create_function(self, doc_file, path, item, &op, &param_re, &function_name)?;
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
                    let name = ident(&module_name).map_err(|source| Error::IdentError {
                        source,
                        file: file!(),
                        line: line!(),
                    })?;
                    file.extend(quote! {
                        pub mod #name {
                            use crate::models::*;

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

    // For create_models. Recursively adds schema refs.
    fn add_schema_refs(&self, schemas: &mut IndexMap<RefKey, ResolvedSchema>, doc_file: &Path, schema_ref: Reference) -> Result<()> {
        let schema = self.spec.resolve_schema_ref(doc_file, schema_ref).map_err(Error::SpecError)?;
        if let Some(ref_key) = schema.ref_key.clone() {
            if !schemas.contains_key(&ref_key) {
                if !self.spec.is_input_file(&ref_key.file_path) {
                    let refs = get_schema_schema_references(&schema.schema);
                    schemas.insert(ref_key.clone(), schema);
                    for reference in refs {
                        self.add_schema_refs(schemas, &ref_key.file_path, reference)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn create_vec_alias(&self, _doc_file: &Path, alias_name: &str, schema: &ResolvedSchema) -> Result<TokenStream> {
        let items = get_schema_array_items(&schema.schema.common)?;
        let typ = ident(&alias_name.to_camel_case()).map_err(|source| Error::IdentError {
            source,
            file: file!(),
            line: line!(),
        })?;
        let items_typ = get_type_name_for_schema_ref(&items, AsReference::False)?;
        Ok(quote! { pub type #typ = Vec<#items_typ>; })
    }

    fn create_struct(&self, doc_file: &Path, struct_name: &str, schema: &ResolvedSchema) -> Result<Vec<TokenStream>> {
        // println!("create_struct {} {}", doc_file.to_str().unwrap(), struct_name);
        let mut streams = Vec::new();
        let mut local_types = Vec::new();
        let mut props = TokenStream::new();
        let ns = ident(&struct_name.to_snake_case()).map_err(|source| Error::IdentError {
            source,
            file: file!(),
            line: line!(),
        })?;
        let nm = ident(&struct_name.to_camel_case()).map_err(|source| Error::IdentError {
            source,
            file: file!(),
            line: line!(),
        })?;
        let required: HashSet<&str> = schema.schema.required.iter().map(String::as_str).collect();

        for schema in &schema.schema.all_of {
            let type_name = get_type_name_for_schema_ref(schema, AsReference::False)?;
            let field_name = ident(&type_name.to_string().to_snake_case()).map_err(|source| Error::IdentError {
                source,
                file: file!(),
                line: line!(),
            })?;
            props.extend(quote! {
                #[serde(flatten)]
                pub #field_name: #type_name,
            });
        }

        let properties = self
            .spec
            .resolve_schema_map(doc_file, &schema.schema.properties)
            .map_err(Error::SpecError)?;
        for (property_name, property) in &properties {
            let nm = ident(&property_name.to_snake_case()).map_err(|source| Error::IdentError {
                source,
                file: file!(),
                line: line!(),
            })?;
            let (mut field_tp_name, field_tp) = self.create_struct_field_type(doc_file, &ns, property_name, property)?;
            let is_required = required.contains(property_name.as_str());
            let is_vec = is_vec(&field_tp_name);
            if !is_vec {
                field_tp_name = require(is_required, field_tp_name);
            }
            local_types.extend(field_tp);
            let mut serde_attrs: Vec<TokenStream> = Vec::new();
            if &nm.to_string() != property_name {
                serde_attrs.push(quote! { rename = #property_name });
            }
            if property.schema.read_only == Some(true) {
                serde_attrs.push(quote! { skip_serializing });
            } else {
                if !is_required {
                    if is_vec {
                        serde_attrs.push(quote! { default, skip_serializing_if = "Vec::is_empty"});
                    } else {
                        serde_attrs.push(quote! { default, skip_serializing_if = "Option::is_none"});
                    }
                }
            }
            let serde = if serde_attrs.len() > 0 {
                quote! { #[serde(#(#serde_attrs),*)] }
            } else {
                quote! {}
            };
            // see if a field shoud be wrapped in a Box
            let prop_nm = &PropertyName {
                file_path: PathBuf::from(doc_file),
                schema_name: struct_name.to_owned(),
                property_name: property_name.to_string(),
            };
            // println!("property {:?}", prop_nm);
            if self.config.box_properties.contains(prop_nm) {
                field_tp_name = quote! { Box<#field_tp_name> };
            }
            props.extend(quote! {
                #serde
                pub #nm: #field_tp_name,
            });
        }

        let st = quote! {
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            pub struct #nm {
                #props
            }
        };
        streams.push(TokenStream::from(st));

        if local_types.len() > 0 {
            let mut types = TokenStream::new();
            local_types.into_iter().for_each(|tp| types.extend(tp));
            streams.push(quote! {
                pub mod #ns {
                    use super::*;
                    #types
                }
            });
        }

        Ok(streams)
    }

    /// Creates the type reference for a struct field from a struct property.
    /// Optionally, creates a type for a local schema.
    fn create_struct_field_type(
        &self,
        doc_file: &Path,
        namespace: &TokenStream,
        property_name: &str,
        property: &ResolvedSchema,
    ) -> Result<(TokenStream, Vec<TokenStream>)> {
        match &property.ref_key {
            Some(ref_key) => {
                let tp = ident(&ref_key.name.to_camel_case()).map_err(|source| Error::IdentError {
                    source,
                    file: file!(),
                    line: line!(),
                })?;
                Ok((tp, Vec::new()))
            }
            None => {
                if is_local_enum(property) {
                    let (tp_name, tp) = create_enum(namespace, property_name, property)?;
                    Ok((tp_name, vec![tp]))
                } else if is_local_struct(property) {
                    let id = ident(&property_name.to_camel_case()).map_err(|source| Error::IdentError {
                        source,
                        file: file!(),
                        line: line!(),
                    })?;
                    let tp_name = quote! {#namespace::#id};
                    let tps = self.create_struct(doc_file, property_name, property)?;
                    // println!("creating local struct {:?} {}", tp_name, tps.len());
                    Ok((tp_name, tps))
                } else {
                    Ok((get_type_name_for_schema(&property.schema.common, AsReference::False)?, Vec::new()))
                }
            }
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("SpecError: {0}")]
    SpecError(spec::Error),
    #[error("ArrayExpectedToHaveItems")]
    ArrayExpectedToHaveItems,
    #[error("NoNameForRef")]
    NoNameForRef,
    #[error("IdentError at {}:{} {} ", file, line, source)]
    IdentError {
        source: crate::identifier::Error,
        file: &'static str,
        line: u32,
    },
    #[error("CreateEnumIdentError {} {}: {}", property_name, enum_value, source)]
    CreateEnumIdentError {
        source: identifier::Error,
        property_name: String,
        enum_value: String,
    },
}

/// Whether or not to pass a type is a reference.
#[derive(Copy, Clone)]
pub enum AsReference {
    True,
    False,
}

fn is_vec(ts: &TokenStream) -> bool {
    ts.to_string().starts_with("Vec <")
}

fn is_array(schema: &SchemaCommon) -> bool {
    matches!(schema.type_, Some(DataType::Array))
}

fn is_string(schema: &SchemaCommon) -> bool {
    matches!(schema.type_, Some(DataType::String))
}

fn get_schema_array_items(schema: &SchemaCommon) -> Result<&ReferenceOr<Schema>> {
    Ok(schema.items.as_ref().as_ref().map_or(Err(Error::ArrayExpectedToHaveItems), Ok)?)
}

pub fn create_generated_by_header() -> TokenStream {
    let version = env!("CARGO_PKG_VERSION");
    let comment = format!("generated by AutoRust {}", &version);
    quote! { #![doc = #comment] }
}

fn is_local_enum(property: &ResolvedSchema) -> bool {
    property.schema.common.enum_.len() > 0
}

fn is_local_struct(property: &ResolvedSchema) -> bool {
    property.schema.properties.len() > 0
}

fn create_enum(namespace: &TokenStream, property_name: &str, property: &ResolvedSchema) -> Result<(TokenStream, TokenStream)> {
    let enum_values = enum_values_as_strings(&property.schema.common.enum_);
    let id = ident(&property_name.to_camel_case()).map_err(|source| Error::IdentError {
        source,
        file: file!(),
        line: line!(),
    })?;
    let mut values = TokenStream::new();
    for name in enum_values {
        let nm = name.to_camel_case_ident().map_err(|source| Error::CreateEnumIdentError {
            source,
            property_name: property_name.to_owned(),
            enum_value: name.to_owned(),
        })?;
        let rename = if &nm.to_string() == name {
            quote! {}
        } else {
            quote! { #[serde(rename = #name)] }
        };
        let value = quote! {
            #rename
            #nm,
        };
        values.extend(value);
    }
    let nm = ident(&property_name.to_camel_case()).map_err(|source| Error::IdentError {
        source,
        file: file!(),
        line: line!(),
    })?;
    let tp = quote! {
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum #nm {
            #values
        }
    };
    let tp_name = quote! {#namespace::#id};
    Ok((tp_name, tp))
}

/// Wraps a type in an Option if is not required.
fn require(is_required: bool, tp: TokenStream) -> TokenStream {
    if is_required {
        tp
    } else {
        quote! { Option<#tp> }
    }
}

fn enum_values_as_strings(values: &Vec<Value>) -> Vec<&str> {
    values
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .collect()
}

fn get_param_type(param: &Parameter) -> Result<TokenStream> {
    let is_required = param.required.unwrap_or(false);
    let is_array = is_array(&param.common);
    let tp = if let Some(_param_type) = &param.common.type_ {
        get_type_name_for_schema(&param.common, AsReference::True)?
    } else if let Some(schema) = &param.schema {
        get_type_name_for_schema_ref(schema, AsReference::True)?
    } else {
        eprintln!("WARN unkown param type for {}", &param.name);
        quote! { &serde_json::Value }
    };
    Ok(require(is_required || is_array, tp))
}

fn get_param_name(param: &Parameter) -> Result<TokenStream> {
    ident(&param.name.to_snake_case()).map_err(|source| Error::IdentError {
        source,
        file: file!(),
        line: line!(),
    })
}

fn parse_params(param_re: &Regex, path: &str) -> Vec<String> {
    // capture 0 is the whole match and 1 is the actual capture like other languages
    param_re.captures_iter(path).into_iter().map(|c| c[1].to_string()).collect()
}

fn format_path(param_re: &Regex, path: &str) -> String {
    param_re.replace_all(path, "{}").to_string()
}

fn create_function_params(_cg: &CodeGen, _doc_file: &Path, parameters: &Vec<Parameter>) -> Result<TokenStream> {
    let mut params: Vec<TokenStream> = Vec::new();
    for param in parameters {
        let name = get_param_name(param)?;
        let tp = get_param_type(param)?;
        params.push(quote! { #name: #tp });
    }
    let slf = quote! { operation_config: &crate::OperationConfig };
    params.insert(0, slf);
    Ok(quote! { #(#params),* })
}

fn get_type_name_for_schema(schema: &SchemaCommon, as_ref: AsReference) -> Result<TokenStream> {
    if let Some(schema_type) = &schema.type_ {
        let format = schema.format.as_deref();
        let ts = match schema_type {
            DataType::Array => {
                let items = get_schema_array_items(&schema)?;
                let vec_items_typ = get_type_name_for_schema_ref(&items, as_ref)?;
                match as_ref {
                    AsReference::True => quote! { &Vec<#vec_items_typ> },
                    AsReference::False => quote! { Vec<#vec_items_typ> },
                }
            }
            DataType::Integer => {
                if format == Some("int32") {
                    quote! { i32 }
                } else {
                    quote! { i64 }
                }
            }
            DataType::Number => {
                if format == Some("float") {
                    quote! { f32 }
                } else {
                    quote! { f64 }
                }
            }
            DataType::String => match as_ref {
                AsReference::True => quote! { &str },
                AsReference::False => quote! { String },
            },
            DataType::Boolean => quote! { bool },
            DataType::Object => match as_ref {
                AsReference::True => quote! { &serde_json::Value },
                AsReference::False => quote! { serde_json::Value },
            },
            DataType::File => todo!("Handle DataType::File"),
        };
        Ok(ts)
    } else {
        // eprintln!(
        //     "WARN unknown type in get_type_name_for_schema, description {:?}",
        //     schema.description
        // );
        match as_ref {
            AsReference::True => Ok(quote! { &serde_json::Value }),
            AsReference::False => Ok(quote! { serde_json::Value }),
        }
    }
}

fn get_type_name_for_schema_ref(schema: &ReferenceOr<Schema>, as_ref: AsReference) -> Result<TokenStream> {
    match schema {
        ReferenceOr::Reference { reference, .. } => {
            let name = &reference.name.as_ref().map_or(Err(Error::NoNameForRef), Ok)?;
            let idt = ident(&name.to_camel_case()).map_err(|source| Error::IdentError {
                source,
                file: file!(),
                line: line!(),
            })?;
            match as_ref {
                AsReference::True => Ok(quote! { &#idt }),
                AsReference::False => Ok(quote! { #idt }),
            }
        }
        ReferenceOr::Item(schema) => get_type_name_for_schema(&schema.common, as_ref),
    }
}

fn create_response_type(rsp: &Response) -> Result<Option<TokenStream>> {
    if let Some(schema) = &rsp.schema {
        Ok(Some(get_type_name_for_schema_ref(schema, AsReference::False)?))
    } else {
        Ok(None)
    }
}

fn create_function(
    cg: &CodeGen,
    doc_file: &Path,
    path: &str,
    _item: &PathItem,
    operation_verb: &OperationVerb,
    param_re: &Regex,
    function_name: &str,
) -> Result<TokenStream> {
    let fname = ident(function_name).map_err(|source| Error::IdentError {
        source,
        file: file!(),
        line: line!(),
    })?;

    let params = parse_params(param_re, path);
    // println!("path params {:#?}", params);
    let params: Result<Vec<_>> = params
        .iter()
        .map(|s| {
            Ok(ident(&s.to_snake_case()).map_err(|source| Error::IdentError {
                source,
                file: file!(),
                line: line!(),
            })?)
        })
        .collect();
    let params = params?;
    let url_str_args = quote! { #(#params),* };

    let fpath = format!("{{}}{}", &format_path(param_re, path));

    let parameters: Vec<Parameter> = cg
        .spec
        .resolve_parameters(doc_file, &operation_verb.operation().parameters)
        .map_err(Error::SpecError)?;
    let param_names: HashSet<_> = parameters.iter().map(|p| p.name.as_str()).collect();
    let has_param_api_version = param_names.contains("api-version");
    let mut skip = HashSet::new();
    if cg.api_version().is_some() {
        skip.insert("api-version");
    }
    let parameters = parameters.into_iter().filter(|p| !skip.contains(p.name.as_str())).collect();

    let fparams = create_function_params(cg, doc_file, &parameters)?;

    // see if there is a body parameter
    // let fresponse = create_function_return(operation_verb)?;

    let mut ts_request_builder = TokenStream::new();

    let mut is_post = false;
    let req_verb = match operation_verb {
        OperationVerb::Get(_) => quote! { req_builder = req_builder.method(http::Method::GET); },
        OperationVerb::Post(_) => {
            is_post = true;
            quote! { req_builder = req_builder.method(http::Method::POST); }
        }
        OperationVerb::Put(_) => quote! { req_builder = req_builder.method(http::Method::PUT); },
        OperationVerb::Patch(_) => quote! { req_builder = req_builder.method(http::Method::PATCH); },
        OperationVerb::Delete(_) => quote! { req_builder = req_builder.method(http::Method::DELETE); },
        OperationVerb::Options(_) => quote! { req_builder = req_builder.method(http::Method::OPTIONS); },
        OperationVerb::Head(_) => quote! { req_builder = req_builder.method(http::Method::HEAD); },
    };
    ts_request_builder.extend(req_verb);

    // auth
    ts_request_builder.extend(quote! {
        if let Some(token_credential) = operation_config.token_credential() {
            let token_response = token_credential
                .get_token(operation_config.token_credential_resource()).await
                .map_err(#fname::Error::GetTokenError)?;
            req_builder = req_builder.header(http::header::AUTHORIZATION, format!("Bearer {}", token_response.token.secret()));
        }
    });

    // api-version param
    if has_param_api_version {
        if let Some(_api_version) = cg.api_version() {
            ts_request_builder.extend(quote! {
                url.query_pairs_mut().append_pair("api-version", operation_config.api_version());
            });
        }
    }

    // params
    let mut has_body_parameter = false;
    for param in &parameters {
        let param_name = &param.name;
        let param_name_var = get_param_name(&param)?;
        let required = param.required.unwrap_or(false);
        match param.in_ {
            ParameterType::Path => {} // handled above
            ParameterType::Query => {
                let is_array = is_array(&param.common);
                let query_body = if is_array {
                    let collection_format = param.collection_format.as_ref().unwrap_or(&CollectionFormat::Csv);
                    match collection_format {
                        CollectionFormat::Multi => Some(
                            if is_string(&param.common){
                                quote! {
                                    for value in #param_name_var {
                                        url.query_pairs_mut().append_pair(#param_name, value);
                                    }
                                }
                            } else {
                                quote! {
                                    for value in #param_name_var {
                                        url.query_pairs_mut().append_pair(#param_name, value.to_string().as_str());
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
                    Some(if is_string(&param.common) {
                        quote! {
                            url.query_pairs_mut().append_pair(#param_name, #param_name_var);
                        }
                    } else {
                        quote! {
                            url.query_pairs_mut().append_pair(#param_name, #param_name_var.to_string().as_str());
                        }
                    })
                };
                if let Some(query_body) = query_body {
                    if required || is_array {
                        ts_request_builder.extend(query_body);
                    } else {
                        ts_request_builder.extend(quote! {
                            if let Some(#param_name_var) = #param_name_var {
                                #query_body
                            }
                        });
                    }
                }
            }
            ParameterType::Header => {
                if required {
                    ts_request_builder.extend(quote! {
                        req_builder = req_builder.header(#param_name, #param_name_var);
                    });
                } else {
                    ts_request_builder.extend(quote! {
                        if let Some(#param_name_var) = #param_name_var {
                            req_builder = req_builder.header(#param_name, #param_name_var);
                        }
                    });
                }
            }
            ParameterType::Body => {
                has_body_parameter = true;
                if required {
                    ts_request_builder.extend(quote! {
                        let req_body = azure_core::to_json(#param_name_var).map_err(#fname::Error::SerializeError)?;
                    });
                } else {
                    ts_request_builder.extend(quote! {
                        let req_body =
                            if let Some(#param_name_var) = #param_name_var {
                                azure_core::to_json(#param_name_var).map_err(#fname::Error::SerializeError)?
                            } else {
                                bytes::Bytes::from_static(azure_core::EMPTY_BODY)
                            };
                    });
                }
            }
            ParameterType::Form => {
                if required {
                    ts_request_builder.extend(quote! {
                        req_builder = req_builder.form(#param_name_var);
                    });
                } else {
                    ts_request_builder.extend(quote! {
                        if let Some(#param_name_var) = #param_name_var {
                            req_builder = req_builder.form(#param_name_var);
                        }
                    });
                }
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

    let responses = &operation_verb.operation().responses;
    let success_responses = get_success_responses(responses);
    let error_responses = get_error_responses(responses);
    let is_single_response = success_responses.len() == 1;
    let has_default_response = has_default_response(responses);

    let fresponse = if is_single_response {
        let tp = create_response_type(&success_responses[0])?.unwrap_or(quote! { () });
        quote! { std::result::Result<#tp, #fname::Error> }
    } else {
        quote! { std::result::Result<#fname::Response, #fname::Error> }
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
            let enum_type_name = ident(&get_response_type_name(status_code)).map_err(|source| Error::IdentError {
                source,
                file: file!(),
                line: line!(),
            })?;
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
            let response_type = ident(response_type).map_err(|source| Error::IdentError {
                source,
                file: file!(),
                line: line!(),
            })?;
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
                let status_code_name = ident(&get_status_code_name(status_code)).map_err(|source| Error::IdentError {
                    source,
                    file: file!(),
                    line: line!(),
                })?;
                let response_type_name = ident(&get_response_type_name(status_code)).map_err(|source| Error::IdentError {
                    source,
                    file: file!(),
                    line: line!(),
                })?;
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
                let status_code_name = ident(&get_status_code_name(status_code)).map_err(|source| Error::IdentError {
                    source,
                    file: file!(),
                    line: line!(),
                })?;
                let response_type_name = ident(&get_response_type_name(status_code)).map_err(|source| Error::IdentError {
                    source,
                    file: file!(),
                    line: line!(),
                })?;
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
    // default must be last
    if has_default_response {
        for (status_code, rsp) in responses {
            match status_code {
                autorust_openapi::StatusCode::Code(_) => {}
                autorust_openapi::StatusCode::Default => {
                    let tp = create_response_type(rsp)?;
                    match tp {
                        Some(tp) => {
                            match_status.extend(quote! {
                                status_code => {
                                    let rsp_body = rsp.body();
                                    let rsp_value: #tp = serde_json::from_slice(rsp_body).map_err(|source| #fname::Error::DeserializeError(source, rsp_body.clone()))?;
                                    Err(#fname::Error::DefaultResponse{status_code, value: rsp_value})
                                }
                            });
                        }
                        None => {
                            match_status.extend(quote! {
                                status_code => {
                                    Err(#fname::Error::DefaultResponse{status_code})
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
                let rsp_body = rsp.body();
                Err(#fname::Error::UnexpectedResponse{status_code, body: rsp_body.clone()})
            }
        });
    }

    let func = quote! {
        pub async fn #fname(#fparams) -> #fresponse {
            let http_client = operation_config.http_client();
            let url_str = &format!(#fpath, operation_config.base_path(), #url_str_args);
            let mut url = url::Url::parse(url_str).map_err(#fname::Error::ParseUrlError)?;
            let mut req_builder = http::request::Builder::new();
            #ts_request_builder
            req_builder = req_builder.uri(url.as_str());
            let req = req_builder.body(req_body).map_err(#fname::Error::BuildRequestError)?;
            let rsp = http_client.execute_request(req).await.map_err(#fname::Error::ExecuteRequestError)?;
            match rsp.status() {
                #match_status
            }
        }
        pub mod #fname {
            use crate::{models, models::*};

            #response_enum

            #[derive(Debug, thiserror::Error)]
            pub enum Error {
                #error_responses_ts
                #[error("Failed to parse request URL: {0}")]
                ParseUrlError(url::ParseError),
                #[error("Failed to build request: {0}")]
                BuildRequestError(http::Error),
                #[error("Failed to execute request: {0}")]
                ExecuteRequestError(azure_core::HttpError),
                #[error("Failed to serialize request body: {0}")]
                SerializeError(serde_json::Error),
                #[error("Failed to deserialize response: {0}, body: {1:?}")]
                DeserializeError(serde_json::Error, bytes::Bytes),
                #[error("Failed to get access token: {0}")]
                GetTokenError(azure_core::Error),
            }
        }
    };
    Ok(TokenStream::from(func))
}

pub fn create_mod(api_version: &str) -> TokenStream {
    quote! {
        pub mod models;
        pub mod operations;
        pub const API_VERSION: &str = #api_version;
    }
}
