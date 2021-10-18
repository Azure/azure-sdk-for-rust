use crate::path;
use autorust_openapi::{AdditionalProperties, OpenAPI, Operation, Parameter, PathItem, Reference, ReferenceOr, Schema};
use heck::SnakeCase;
use indexmap::{IndexMap, IndexSet};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

/// An API specification
#[derive(Clone, Debug)]
pub struct Spec {
    /// A store of all the documents for an API specification keyed on their file paths where the first one is the root document
    docs: IndexMap<PathBuf, OpenAPI>,
    schemas: IndexMap<RefKey, Schema>,
    parameters: IndexMap<RefKey, Parameter>,
    input_files_paths: IndexSet<PathBuf>,
}

impl Spec {
    /// Read a list of input files as OpenApi docs with the first being the root doc
    ///
    /// This eagerly collects all the schemas and parametes for the docs
    pub fn read_files<P: AsRef<Path>>(input_files_paths: &[P]) -> Result<Self> {
        let mut docs: IndexMap<PathBuf, OpenAPI> = IndexMap::new();
        for file_path in input_files_paths {
            Spec::read_file(&mut docs, file_path)?;
        }

        let mut schemas: IndexMap<RefKey, Schema> = IndexMap::new();
        let mut parameters: IndexMap<RefKey, Parameter> = IndexMap::new();
        for (path, doc) in &docs {
            for (name, schema) in &doc.definitions {
                if let ReferenceOr::Item(schema) = schema {
                    let ref_key = RefKey {
                        file_path: path.clone(),
                        name: name.clone(),
                    };
                    schemas.insert(ref_key, schema.clone());
                }
            }

            for (name, param) in &doc.parameters {
                parameters.insert(
                    RefKey {
                        file_path: path.clone(),
                        name: name.clone(),
                    },
                    param.clone(),
                );
            }
        }

        Ok(Self {
            docs,
            schemas,
            parameters,
            input_files_paths: input_files_paths.iter().map(|f| f.as_ref().to_owned()).collect(),
        })
    }

    /// Read a file and references too, recursively into the map
    fn read_file<P: AsRef<Path>>(docs: &mut IndexMap<PathBuf, OpenAPI>, file_path: P) -> Result<()> {
        let file_path = file_path.as_ref();
        if !docs.contains_key(file_path) {
            let doc = openapi::parse(&file_path)?;
            let ref_files = openapi::get_reference_file_paths(&doc);
            docs.insert(PathBuf::from(file_path), doc);
            for ref_file in ref_files {
                let child_path = path::join(&file_path, &ref_file).map_err(|source| Error::PathJoin { source })?;
                Spec::read_file(docs, &child_path)?;
            }
        }
        Ok(())
    }

    pub fn docs(&self) -> &IndexMap<PathBuf, OpenAPI> {
        &self.docs
    }

    /// Look for specs with operations and return the last one sorted alphabetically
    pub fn api_version(&self) -> Option<String> {
        let mut versions: Vec<&str> = self
            .docs()
            .values()
            .filter(|doc| doc.paths.len() > 0)
            .filter_map(|api| api.info.version.as_deref())
            .collect();
        versions.sort();
        versions.last().map(|version| version.to_string())
    }

    pub fn input_docs<'a>(&'a self) -> impl Iterator<Item = (&'a PathBuf, &'a OpenAPI)> {
        self.docs.iter().filter(move |(p, _)| self.is_input_file(p))
    }

    pub fn is_input_file<P: AsRef<Path>>(&self, path: P) -> bool {
        self.input_files_paths.contains(path.as_ref())
    }

    /// Find the schema for a given doc path and reference
    pub fn resolve_schema_ref<P: AsRef<Path>>(&self, doc_path: P, reference: Reference) -> Result<ResolvedSchema> {
        let doc_path = doc_path.as_ref();
        let full_path = match reference.file {
            None => doc_path.to_owned(),
            Some(file) => path::join(doc_path, &file).map_err(|source| Error::PathJoin { source })?,
        };

        let name = reference.name.ok_or_else(|| Error::NoNameInReference)?;
        let ref_key = RefKey {
            file_path: full_path,
            name,
        };
        let schema = self
            .schemas
            .get(&ref_key)
            .ok_or_else(|| Error::SchemaNotFound { ref_key: ref_key.clone() })?
            .clone();
        Ok(ResolvedSchema {
            ref_key: Some(ref_key),
            schema,
        })
    }

    /// Find the parameter for a given doc path and reference
    pub fn resolve_parameter_ref<P: AsRef<Path>>(&self, doc_path: P, reference: Reference) -> Result<Parameter> {
        let doc_path = doc_path.as_ref();
        let full_path = match reference.file {
            None => doc_path.to_owned(),
            Some(file) => path::join(doc_path, &file).map_err(|source| Error::PathJoin { source })?,
        };
        let name = reference.name.ok_or_else(|| Error::NoNameInReference)?;
        let ref_key = RefKey {
            file_path: full_path,
            name,
        };
        Ok(self
            .parameters
            .get(&ref_key)
            .ok_or_else(|| Error::ParameterNotFound { ref_key })?
            .clone())
    }

    /// Resolve a reference or schema to a resolved schema
    pub fn resolve_schema<P: AsRef<Path>>(&self, doc_path: P, ref_or_schema: &ReferenceOr<Schema>) -> Result<ResolvedSchema> {
        match ref_or_schema {
            ReferenceOr::Item(schema) => Ok(ResolvedSchema {
                ref_key: None,
                schema: schema.clone(),
            }),
            ReferenceOr::Reference { reference, .. } => self.resolve_schema_ref(doc_path.as_ref(), reference.clone()),
        }
    }

    /// Resolve a collection of references or schemas to a collection of resolved schemas
    pub fn resolve_schemas<P: AsRef<Path>>(&self, doc_path: P, ref_or_schemas: &[ReferenceOr<Schema>]) -> Result<Vec<ResolvedSchema>> {
        let mut resolved = Vec::new();
        for schema in ref_or_schemas {
            resolved.push(self.resolve_schema(&doc_path, schema)?);
        }
        Ok(resolved)
    }

    /// Resolve a collection of references or schemas to a collection of resolved schemas
    pub fn resolve_schema_map<P: AsRef<Path>>(
        &self,
        doc_path: P,
        ref_or_schemas: &IndexMap<String, ReferenceOr<Schema>>,
    ) -> Result<IndexMap<String, ResolvedSchema>> {
        let mut resolved = IndexMap::new();
        for (name, schema) in ref_or_schemas {
            resolved.insert(name.clone(), self.resolve_schema(&doc_path, schema)?);
        }
        Ok(resolved)
    }

    pub fn resolve_path<P: AsRef<Path>>(&self, _doc_path: P, path: &ReferenceOr<PathItem>) -> Result<PathItem> {
        match path {
            ReferenceOr::Item(path) => Ok(path.clone()),
            ReferenceOr::Reference { .. } => {
                // self.resolve_path_ref(doc_file, reference),
                // TODO
                Err(Error::NotImplemented)
            }
        }
    }

    pub fn resolve_path_map(&self, doc_file: &Path, paths: &IndexMap<String, ReferenceOr<PathItem>>) -> Result<IndexMap<String, PathItem>> {
        let mut resolved = IndexMap::new();
        for (name, path) in paths {
            resolved.insert(name.clone(), self.resolve_path(doc_file, path)?);
        }
        Ok(resolved)
    }

    pub fn resolve_parameter(&self, doc_file: &Path, parameter: &ReferenceOr<Parameter>) -> Result<Parameter> {
        match parameter {
            ReferenceOr::Item(param) => Ok(param.clone()),
            ReferenceOr::Reference { reference, .. } => self.resolve_parameter_ref(doc_file, reference.clone()),
        }
    }

    pub fn resolve_parameters(&self, doc_file: &Path, parameters: &Vec<ReferenceOr<Parameter>>) -> Result<Vec<Parameter>> {
        let mut resolved = Vec::new();
        for param in parameters {
            resolved.push(self.resolve_parameter(doc_file, param)?);
        }
        Ok(resolved)
    }
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("PathJoin")]
    PathJoin { source: path::Error },
    #[error("SchemaNotFound {} {}", ref_key.file_path.display(), ref_key.name)]
    SchemaNotFound { ref_key: RefKey },
    #[error("NoNameInReference")]
    NoNameInReference,
    #[error("ParameterNotFound")]
    ParameterNotFound { ref_key: RefKey },
    #[error("NotImplemented")]
    NotImplemented,
    #[error("ReadFile")]
    ReadFile { source: std::io::Error, path: PathBuf },
    #[error("DeserializeYaml")]
    DeserializeYaml { source: serde_yaml::Error, path: PathBuf },
    #[error("DeserializeJson")]
    DeserializeJson { source: serde_json::Error, path: PathBuf },
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RefKey {
    pub file_path: PathBuf,
    pub name: String,
}

pub struct ResolvedSchema {
    pub ref_key: Option<RefKey>,
    pub schema: Schema,
}

/// Functionality related to Open API definitions
pub mod openapi {
    use super::*;

    /// Parse an OpenAPI object from a file located at `path`
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<OpenAPI> {
        let path = path.as_ref();
        let bytes = fs::read(path).map_err(|source| Error::ReadFile {
            source,
            path: PathBuf::from(path),
        })?;
        let api = if path.extension() == Some(OsStr::new("yaml")) || path.extension() == Some(OsStr::new("yml")) {
            serde_yaml::from_slice(&bytes).map_err(|source| Error::DeserializeYaml {
                source,
                path: PathBuf::from(path),
            })?
        } else {
            serde_json::from_slice(&bytes).map_err(|source| Error::DeserializeJson {
                source,
                path: PathBuf::from(path),
            })?
        };

        Ok(api)
    }

    /// Returns a set of referenced relative file paths from an OpenAPI specficiation
    pub fn get_reference_file_paths(api: &OpenAPI) -> IndexSet<String> {
        get_references(api)
            .into_iter()
            .filter_map(|reference| match reference {
                TypedReference::Example(_) => None,
                reference => {
                    let reference: Reference = reference.into();
                    reference.file
                }
            })
            .collect()
    }

    /// Returns the list of all references contained in an OpenAPI schema
    pub fn get_references(api: &OpenAPI) -> Vec<TypedReference> {
        let mut list = Vec::new();

        // paths and operations
        for (_path, item) in api.paths() {
            match item {
                ReferenceOr::Reference { reference, .. } => list.push(TypedReference::PathItem(reference.clone())),
                ReferenceOr::Item(item) => {
                    for verb in path_item_operations(&item) {
                        let op = verb.operation();
                        // parameters
                        for param in &op.parameters {
                            match param {
                                ReferenceOr::Reference { reference, .. } => list.push(TypedReference::Parameter(reference.clone())),
                                ReferenceOr::Item(parameter) => match &parameter.schema {
                                    Some(ReferenceOr::Reference { reference, .. }) => list.push(TypedReference::Schema(reference.clone())),
                                    Some(ReferenceOr::Item(schema)) => add_references_for_schema(&mut list, schema),
                                    None => {}
                                },
                            }
                        }

                        // responses
                        for (_code, rsp) in &op.responses {
                            match &rsp.schema {
                                Some(ReferenceOr::Reference { reference, .. }) => list.push(TypedReference::Schema(reference.clone())),
                                Some(ReferenceOr::Item(schema)) => add_references_for_schema(&mut list, schema),
                                None => {}
                            }
                        }

                        // examples
                        for (_name, example) in &op.x_ms_examples {
                            if let ReferenceOr::Reference { reference, .. } = example {
                                list.push(TypedReference::Example(reference.clone()));
                            }
                        }
                    }
                }
            }
        }

        // definitions
        for (_name, schema) in &api.definitions {
            match schema {
                ReferenceOr::Reference { reference, .. } => list.push(TypedReference::Schema(reference.clone())),
                ReferenceOr::Item(schema) => add_references_for_schema(&mut list, schema),
            }
        }

        list
    }

    /// Get all references related to schemas for an Open API specification
    pub fn get_api_schema_references(api: &OpenAPI) -> Vec<Reference> {
        get_references(api)
            .into_iter()
            .filter_map(|rf| match rf {
                TypedReference::Schema(rs) => Some(rs),
                _ => None,
            })
            .collect()
    }
}

pub enum OperationVerb<'a> {
    Get(&'a Operation),
    Post(&'a Operation),
    Put(&'a Operation),
    Patch(&'a Operation),
    Delete(&'a Operation),
    Options(&'a Operation),
    Head(&'a Operation),
}

// Hold an operation and remembers the operation verb.
impl<'a> OperationVerb<'a> {
    pub fn operation(&self) -> &'a Operation {
        match self {
            OperationVerb::Get(op) => op,
            OperationVerb::Post(op) => op,
            OperationVerb::Put(op) => op,
            OperationVerb::Patch(op) => op,
            OperationVerb::Delete(op) => op,
            OperationVerb::Options(op) => op,
            OperationVerb::Head(op) => op,
        }
    }

    pub fn verb_name(&self) -> &'static str {
        match self {
            OperationVerb::Get(_) => "get",
            OperationVerb::Post(_) => "post",
            OperationVerb::Put(_) => "put",
            OperationVerb::Patch(_) => "patch",
            OperationVerb::Delete(_) => "delete",
            OperationVerb::Options(_) => "options",
            OperationVerb::Head(_) => "head",
        }
    }

    pub fn function_name(&self, path: &str) -> (Option<String>, String) {
        if let Some(operation_id) = &self.operation().operation_id {
            function_name_from_operation_id(operation_id)
        } else {
            (None, create_function_name(path, self.verb_name()))
        }
    }
}

/// Returns the module name and function name.
/// The module name is optional and is text before an underscore in the operatonId.
fn function_name_from_operation_id(operation_id: &str) -> (Option<String>, String) {
    let parts: Vec<&str> = operation_id.splitn(2, '_').collect();
    if parts.len() == 2 {
        (Some(parts[0].to_snake_case()), parts[1].to_snake_case())
    } else {
        (None, parts[0].to_snake_case())
    }
}

/// Creating a function name from the path and verb when an operationId is not specified.
/// All azure-rest-api-specs operations should have an operationId.
fn create_function_name(path: &str, verb_name: &str) -> String {
    let mut path = path.split('/').filter(|&x| !x.is_empty()).collect::<Vec<_>>();
    path.push(verb_name);
    path.join("_")
}

pub fn path_item_operations(item: &PathItem) -> impl Iterator<Item = OperationVerb> {
    vec![
        item.get.as_ref().map(OperationVerb::Get),
        item.post.as_ref().map(OperationVerb::Post),
        item.put.as_ref().map(OperationVerb::Put),
        item.patch.as_ref().map(OperationVerb::Patch),
        item.delete.as_ref().map(OperationVerb::Delete),
        item.options.as_ref().map(OperationVerb::Options),
        item.head.as_ref().map(OperationVerb::Head),
    ]
    .into_iter()
    .filter_map(|x| x)
}

/// A $ref reference type that knows what type of reference it is
#[derive(Clone, Debug, PartialEq)]
pub enum TypedReference {
    PathItem(Reference),
    Parameter(Reference),
    Schema(Reference),
    Example(Reference),
}

impl Into<Reference> for TypedReference {
    fn into(self) -> Reference {
        match self {
            TypedReference::PathItem(r) => r,
            TypedReference::Parameter(r) => r,
            TypedReference::Schema(r) => r,
            TypedReference::Example(r) => r,
        }
    }
}

/// Get all schema references for a given schema
pub fn get_schema_schema_references(schema: &Schema) -> Vec<Reference> {
    let mut refs = Vec::new();
    add_references_for_schema(&mut refs, schema);
    refs.into_iter()
        .filter_map(|rf| match rf {
            TypedReference::Schema(rs) => Some(rs),
            _ => None,
        })
        .collect()
}

fn add_references_for_schema(list: &mut Vec<TypedReference>, schema: &Schema) {
    for (_, schema) in &schema.properties {
        match schema {
            ReferenceOr::Reference { reference, .. } => list.push(TypedReference::Schema(reference.clone())),
            ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
        }
    }
    match schema.additional_properties.as_ref() {
        Some(ap) => match ap {
            AdditionalProperties::Boolean(_) => {}
            AdditionalProperties::Schema(schema) => match schema {
                ReferenceOr::Reference { reference, .. } => list.push(TypedReference::Schema(reference.clone())),
                ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
            },
        },
        _ => {}
    }
    match schema.common.items.as_ref() {
        Some(schema) => match schema {
            ReferenceOr::Reference { reference, .. } => list.push(TypedReference::Schema(reference.clone())),
            ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
        },
        _ => {}
    }
    for schema in &schema.all_of {
        match schema {
            ReferenceOr::Reference { reference, .. } => list.push(TypedReference::Schema(reference.clone())),
            ReferenceOr::Item(schema) => add_references_for_schema(list, schema),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_function_name() {
        assert_eq!(create_function_name("/pets", "get"), "pets_get");
    }

    #[test]
    fn test_function_name_from_operation_id() {
        assert_eq!(
            function_name_from_operation_id("PrivateClouds_CreateOrUpdate"),
            (Some("private_clouds".to_owned()), "create_or_update".to_owned())
        );
        assert_eq!(function_name_from_operation_id("get"), (None, "get".to_owned()));
    }
}
