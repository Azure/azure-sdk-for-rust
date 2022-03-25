use crate::{
    codegen::{add_option, create_generated_by_header, is_vec, type_name_gen, Error},
    identifier::{CamelCaseIdent, SnakeCaseIdent},
    spec::{self, get_schema_array_items, get_type_name_for_schema, get_type_name_for_schema_ref, TypeName},
    CodeGen, PropertyName, ResolvedSchema, Spec,
};
use autorust_openapi::{DataType, Reference, ReferenceOr, Schema};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use spec::{get_schema_schema_references, openapi, RefKey};
use std::collections::HashSet;

#[derive(Clone)]
struct PropertyGen {
    name: String,
    schema: SchemaGen,
}

#[derive(Clone)]
struct SchemaGen {
    ref_key: Option<RefKey>,
    schema: Schema,

    // used for identifying workarounds
    doc_file: Utf8PathBuf,

    // resolved
    properties: Vec<PropertyGen>,
    all_of: Vec<SchemaGen>,
}

#[derive(Clone)]
struct EnumValue {
    value: String,
    description: Option<String>,
}

impl SchemaGen {
    fn new(ref_key: Option<RefKey>, schema: Schema, doc_file: Utf8PathBuf) -> Self {
        Self {
            ref_key,
            schema,
            doc_file,
            properties: Vec::new(),
            all_of: Vec::new(),
        }
    }

    fn name(&self) -> Result<&str, Error> {
        Ok(&self.ref_key.as_ref().ok_or(Error::NoRefKey)?.name)
    }

    fn is_array(&self) -> bool {
        matches!(self.schema.common.type_, Some(DataType::Array))
    }

    fn is_local_enum(&self) -> bool {
        !self.schema.common.enum_.is_empty()
    }

    fn is_local_struct(&self) -> bool {
        !self.schema.properties.is_empty()
    }

    fn is_basic_type(&self) -> bool {
        matches!(
            self.schema.common.type_,
            Some(DataType::Integer | DataType::String | DataType::Number | DataType::Boolean)
        )
    }

    fn type_name(&self) -> Result<TypeName, Error> {
        get_type_name_for_schema(&self.schema.common).map_err(Error::TypeNameForSchema)
    }

    fn required(&self) -> HashSet<&str> {
        self.schema.required.iter().map(String::as_str).collect()
    }

    fn has_required(&self) -> bool {
        !self.schema.required.is_empty()
    }

    fn all_of(&self) -> Vec<&SchemaGen> {
        self.all_of.iter().collect()
    }

    fn array_items(&self) -> Result<&ReferenceOr<Schema>, Error> {
        get_schema_array_items(&self.schema.common).map_err(Error::ArrayItems)
    }

    fn enum_values(&self) -> Vec<EnumValue> {
        self.schema
            .common
            .enum_
            .iter()
            .filter_map(|v| match v {
                Value::String(s) => Some(EnumValue {
                    value: s.to_owned(),
                    description: None,
                }),
                _ => None,
            })
            .collect()
    }

    fn properties(&self) -> Vec<&PropertyGen> {
        self.properties.iter().collect()
    }

    fn default(&self) -> Option<&str> {
        self.schema.common.default.as_ref().and_then(|v| v.as_str())
    }

    /// If the type should implement Default
    fn implement_default(&self) -> bool {
        if self.has_required() {
            return false;
        }
        for schema in self.all_of() {
            if !schema.implement_default() {
                return false;
            }
        }
        true
    }
}

fn resolve_schema_properties(
    resolved: &mut IndexMap<RefKey, SchemaGen>,
    all_schemas: &IndexMap<RefKey, SchemaGen>,
    spec: &Spec,
    doc_file: &Utf8Path,
    schema: &SchemaGen,
) -> Result<SchemaGen, Error> {
    let mut properties: IndexMap<String, _> = IndexMap::new();
    // add any allOf properties not in schemas, not references
    schema.schema.all_of.iter().for_each(|ref_or_schema| match ref_or_schema {
        ReferenceOr::Item(schema) => {
            for (property_name, property) in &schema.properties {
                properties.insert(property_name.clone(), property.clone());
            }
        }
        ReferenceOr::Reference { reference: _, .. } => (),
    });
    for (property_name, property) in &schema.schema.properties {
        properties.insert(property_name.clone(), property.clone());
    }
    let properties = spec.resolve_schema_map(doc_file, &properties)?;
    let mut schema = schema.clone();
    schema.properties = properties
        .into_iter()
        .map(|(property_name, property)| resolve_schema_property(resolved, all_schemas, spec, doc_file, property_name, &property))
        .collect::<Result<_, Error>>()?;
    Ok(schema)
}

fn resolve_schema_property(
    resolved: &mut IndexMap<RefKey, SchemaGen>,
    all_schemas: &IndexMap<RefKey, SchemaGen>,
    spec: &Spec,
    doc_file: &Utf8Path,
    property_name: String,
    property: &ResolvedSchema,
) -> Result<PropertyGen, Error> {
    let schema = if let Some(ref_key) = &property.ref_key {
        if let Some(schema) = resolved.get(ref_key) {
            schema.clone()
        } else {
            let schema = all_schemas
                .get(ref_key)
                .ok_or_else(|| Error::RefKeyNotFound { ref_key: ref_key.clone() })?;
            // prevent overflow for recursive call
            resolved.insert(ref_key.clone(), schema.clone()); // unresolved properties
            let schema = resolve_schema_properties(resolved, all_schemas, spec, &ref_key.file_path, schema)?;
            resolved.insert(ref_key.clone(), schema.clone()); // resolved properties
            schema
        }
    } else {
        let schema = SchemaGen::new(None, property.schema.clone(), doc_file.to_path_buf());
        resolve_schema_properties(resolved, all_schemas, spec, doc_file, &schema)?
    };
    Ok(PropertyGen {
        name: property_name,
        schema,
    })
}

fn resolve_all_of(all_schemas: &IndexMap<RefKey, SchemaGen>, schema: &SchemaGen, spec: &Spec) -> Result<SchemaGen, Error> {
    // recursively apply to all properties
    let properties: Vec<_> = schema
        .properties
        .iter()
        .map(|property| {
            let schema = resolve_all_of(all_schemas, &property.schema, spec)?;
            Ok(PropertyGen {
                name: property.name.clone(),
                schema,
            })
        })
        .collect::<Result<_, Error>>()?;
    let all_of: Vec<_> = schema
        .schema
        .all_of
        .iter()
        .map(|ref_or_schema| match ref_or_schema {
            ReferenceOr::Item(_schema) => Ok(None),
            ReferenceOr::Reference { reference, .. } => {
                let ref_key = spec.ref_key(&schema.doc_file, reference)?;
                let schema = all_schemas
                    .get(&ref_key)
                    .ok_or_else(|| Error::RefKeyNotFound { ref_key: ref_key.clone() })?
                    .clone();
                let schema = resolve_all_of(all_schemas, &schema, spec)?;
                Ok(Some(schema))
            }
        })
        .collect::<Result<_, Error>>()?;
    let mut schema = schema.clone();
    schema.properties = properties;
    schema.all_of = all_of.into_iter().flatten().collect();
    Ok(schema)
}

fn all_schemas(spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>, Error> {
    let mut all_schemas: IndexMap<RefKey, SchemaGen> = IndexMap::new();

    // all definitions from input_files
    for (doc_file, doc) in spec.input_docs() {
        let schemas = spec.resolve_schema_map(doc_file, &doc.definitions).map_err(Error::Spec)?;
        for (name, resolved_schema) in schemas {
            let ref_key = RefKey {
                file_path: doc_file.clone(),
                name,
            };
            all_schemas.insert(
                ref_key.clone(),
                SchemaGen::new(Some(ref_key.clone()), resolved_schema.schema, doc_file.to_path_buf()),
            );
        }
    }

    // any referenced schemas from other files
    for (doc_file, api) in spec.input_docs() {
        for reference in openapi::get_api_schema_references(doc_file, api) {
            add_schema_refs(&mut all_schemas, spec, doc_file, &reference)?;
        }
    }

    Ok(all_schemas)
}

fn resolve_all_schema_properties(schemas: &IndexMap<RefKey, SchemaGen>, spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>, Error> {
    let mut resolved: IndexMap<RefKey, SchemaGen> = IndexMap::new();
    for (ref_key, schema) in schemas {
        resolved.insert(ref_key.clone(), schema.clone()); // order properties after
        let schema = resolve_schema_properties(&mut resolved, schemas, spec, &ref_key.file_path, schema)?;
        resolved.insert(ref_key.clone(), schema);
    }
    Ok(resolved)
}

fn resolve_all_all_of(schemas: &IndexMap<RefKey, SchemaGen>, spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>, Error> {
    let mut resolved: IndexMap<RefKey, SchemaGen> = IndexMap::new();
    for (ref_key, schema) in schemas {
        let schema = resolve_all_of(schemas, schema, spec)?;
        resolved.insert(ref_key.clone(), schema);
    }
    Ok(resolved)
}

fn add_schema_gen(all_schemas: &mut IndexMap<RefKey, SchemaGen>, resolved_schema: ResolvedSchema) {
    if let Some(ref_key) = resolved_schema.ref_key {
        if !all_schemas.contains_key(&ref_key) {
            all_schemas.insert(
                ref_key.clone(),
                SchemaGen::new(Some(ref_key.clone()), resolved_schema.schema, ref_key.file_path),
            );
        }
    }
}

pub fn create_models(cg: &CodeGen) -> Result<TokenStream, Error> {
    let mut file = TokenStream::new();
    file.extend(create_generated_by_header());

    let has_case_workaround = cg.should_workaround_case();

    file.extend(quote! {
        #![allow(non_camel_case_types)]
        #![allow(unused_imports)]
        use serde::{Deserialize, Serialize};
    });
    if has_case_workaround {
        file.extend(quote! {
        use azure_core::util::case_insensitive_deserialize;
        });
    }

    let mut schema_names = IndexMap::new();
    let schemas = all_schemas(&cg.spec)?;
    let schemas = resolve_all_schema_properties(&schemas, &cg.spec)?;
    let schemas = resolve_all_all_of(&schemas, &cg.spec)?;
    // sort schemas by name
    let mut schemas: Vec<_> = schemas.into_iter().collect();
    schemas.sort_by(|a, b| a.0.name.cmp(&b.0.name));
    for (ref_key, schema) in &schemas {
        let doc_file = &ref_key.file_path;
        let schema_name = &ref_key.name;
        if let Some(_first_doc_file) = schema_names.insert(schema_name, doc_file) {
            // eprintln!(
            //     "WARN schema {} already created from {:?}, duplicate from {:?}",
            //     schema_name, _first_doc_file, doc_file
            // );
        } else if schema.is_array() {
            file.extend(create_vec_alias(schema)?);
        } else if schema.is_local_enum() {
            let no_namespace = TokenStream::new();
            let TypeCode { code: enum_code, .. } = create_enum(&no_namespace, schema, schema_name, false)?;
            file.extend(enum_code);
        } else if schema.is_basic_type() {
            let (id, value) = create_basic_type_alias(schema_name, schema)?;
            file.extend(quote! { pub type #id = #value;});
        } else {
            file.extend(create_struct(cg, schema, schema_name)?);
        }
    }
    Ok(file)
}

fn create_basic_type_alias(property_name: &str, property: &SchemaGen) -> Result<(TokenStream, TokenStream), Error> {
    let id = property_name.to_camel_case_ident().map_err(Error::StructName)?;
    let value = type_name_gen(&property.type_name()?, false, false)?;
    Ok((id, value))
}

// For create_models. Recursively adds schema refs.
fn add_schema_refs(
    resolved: &mut IndexMap<RefKey, SchemaGen>,
    spec: &Spec,
    doc_file: &Utf8Path,
    schema_ref: &Reference,
) -> Result<(), Error> {
    let resolved_schema = spec.resolve_schema_ref(doc_file, schema_ref)?;
    if let Some(ref_key) = &resolved_schema.ref_key {
        if !resolved.contains_key(ref_key) && !spec.is_input_file(&ref_key.file_path) {
            add_schema_gen(resolved, resolved_schema.clone());
            for reference in get_schema_schema_references(&resolved_schema.schema) {
                add_schema_refs(resolved, spec, &ref_key.file_path, &reference)?;
            }
        }
    }
    Ok(())
}

fn create_enum(namespace: &TokenStream, property: &SchemaGen, property_name: &str, lowercase_workaround: bool) -> Result<TypeCode, Error> {
    let enum_values = property.enum_values();
    let id = &property_name.to_camel_case_ident().map_err(|source| Error::EnumName {
        source,
        property: property_name.to_owned(),
    })?;
    let mut values = TokenStream::new();
    for enum_value in enum_values {
        let value = &enum_value.value;
        let nm = value.to_camel_case_ident().map_err(|source| Error::EnumName {
            source,
            property: property_name.to_owned(),
        })?;
        let doc_comment = match enum_value.description {
            Some(description) => {
                quote! { #[doc = #description] }
            }
            None => quote! {},
        };
        let lower = value.to_lowercase();
        let rename = if &nm.to_string() == value {
            quote! {}
        } else if value != &lower && lowercase_workaround {
            quote! { #[serde(rename = #value, alias = #lower)] }
        } else {
            quote! { #[serde(rename = #value)] }
        };
        let value_token = quote! {
            #doc_comment
            #rename
            #nm,
        };
        values.extend(value_token);
    }

    let nm = property_name.to_camel_case_ident().map_err(|source| Error::EnumName {
        source,
        property: property_name.to_owned(),
    })?;
    let default_code = if let Some(default_name) = property.default() {
        let default_name = default_name.to_camel_case_ident().map_err(|source| Error::EnumName {
            source,
            property: default_name.to_owned(),
        })?;
        quote! {
            impl Default for #id {
                fn default() -> Self {
                    Self::#default_name
                }
            }
        }
    } else {
        quote! {}
    };

    let doc_comment = match &property.schema.common.description {
        Some(description) => {
            quote! { #[doc = #description] }
        }
        None => quote! {},
    };

    let code = quote! {
        #doc_comment
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        pub enum #nm {
            #values
        }
        #default_code
    };
    let type_name = quote! {#namespace::#id};
    Ok(TypeCode { type_name, code })
}

fn create_vec_alias(schema: &SchemaGen) -> Result<TokenStream, Error> {
    let items = schema.array_items()?;
    let typ = schema.name()?.to_camel_case_ident().map_err(Error::VecAliasName)?;
    let items_typ = type_name_gen(&get_type_name_for_schema_ref(items)?, false, false)?;
    Ok(quote! { pub type #typ = Vec<#items_typ>; })
}

fn create_struct(cg: &CodeGen, schema: &SchemaGen, struct_name: &str) -> Result<TokenStream, Error> {
    let mut code = TokenStream::new();
    let mut mod_code = TokenStream::new();
    let mut props = TokenStream::new();
    let mut new_fn_params: Vec<TokenStream> = Vec::new();
    let mut new_fn_body = TokenStream::new();
    let ns = struct_name.to_snake_case_ident().map_err(Error::StructName)?;
    let struct_name_code = struct_name.to_camel_case_ident().map_err(Error::StructName)?;
    let required = schema.required();

    for schema in schema.all_of() {
        let schema_name = schema.name()?;
        let type_name = schema_name.to_camel_case_ident().map_err(Error::StructFieldName)?;
        let field_name = schema_name.to_snake_case_ident().map_err(Error::StructFieldName)?;
        props.extend(quote! {
            #[serde(flatten)]
            pub #field_name: #type_name,
        });
        if schema.implement_default() {
            new_fn_body.extend(quote! { #field_name: #type_name::default(), });
        } else {
            new_fn_params.push(quote! { #field_name: #type_name });
            new_fn_body.extend(quote! { #field_name, });
        }
    }

    for property in schema.properties() {
        let property_name = property.name.as_str();
        let field_name = property_name.to_snake_case_ident().map_err(Error::StructName)?;
        let prop_nm = &PropertyName {
            file_path: schema.doc_file.clone(),
            schema_name: struct_name.to_owned(),
            property_name: property_name.to_owned(),
        };

        let lowercase_workaround = cg.should_workaround_case();

        let TypeCode {
            mut type_name,
            code: field_code,
        } = create_struct_field_code(cg, &ns, &property.schema, property_name, lowercase_workaround)?;
        mod_code.extend(field_code);
        // uncomment the next two lines to help identify entries that need boxed
        // let prop_nm_str = format!("{} , {} , {}", prop_nm.file_path.display(), prop_nm.schema_name, property_name);
        // props.extend(quote! { #[doc = #prop_nm_str ]});

        if cg.should_force_obj(prop_nm) {
            type_name = quote! { serde_json::Value };
        }

        let is_required = required.contains(property_name) && !cg.should_force_optional(prop_nm);

        let is_vec = is_vec(&type_name);
        if !is_vec {
            type_name = add_option(!is_required, type_name);
        }

        let mut serde_attrs: Vec<TokenStream> = Vec::new();
        if field_name.to_string() != property_name {
            serde_attrs.push(quote! { rename = #property_name });
        }
        if !is_required {
            if is_vec {
                serde_attrs.push(quote! { default, skip_serializing_if = "Vec::is_empty"});
            } else {
                serde_attrs.push(quote! { default, skip_serializing_if = "Option::is_none"});
            }
        }
        if property.schema.is_local_enum() && lowercase_workaround {
            serde_attrs.push(quote! { deserialize_with = "case_insensitive_deserialize"});
        }
        let serde = if !serde_attrs.is_empty() {
            quote! { #[serde(#(#serde_attrs),*)] }
        } else {
            quote! {}
        };
        // see if a field should be wrapped in a Box
        let should_box = cg.should_box_property(prop_nm);
        if should_box {
            type_name = quote! { Box<#type_name> };
        }

        let doc_comment = match &property.schema.schema.common.description {
            Some(description) => quote! { #[doc = #description] },
            None => quote! {},
        };

        props.extend(quote! {
            #doc_comment
            #serde
            pub #field_name: #type_name,
        });

        if is_required {
            new_fn_params.push(quote! { #field_name: #type_name });
            new_fn_body.extend(quote! { #field_name, });
        } else if is_vec {
            if should_box {
                new_fn_body.extend(quote! { #field_name: Box::new(Vec::new()), });
            } else {
                new_fn_body.extend(quote! { #field_name: Vec::new(), });
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if should_box {
                new_fn_body.extend(quote! { #field_name: Box::new(None), });
            } else {
                new_fn_body.extend(quote! { #field_name: None, });
            }
        }
    }

    let default_code = if schema.implement_default() {
        quote! { #[derive(Default)] }
    } else {
        quote! {}
    };

    let doc_comment = match &schema.schema.common.description {
        Some(description) => quote! { #[doc = #description] },
        None => quote! {},
    };

    let struct_code = quote! {
        #doc_comment
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #default_code
        pub struct #struct_name_code {
            #props
        }
    };
    code.extend(struct_code);

    code.extend(if schema.implement_default() {
        quote! {
            impl #struct_name_code {
                pub fn new() -> Self {
                    Self::default()
                }
            }
        }
    } else {
        quote! {
            impl #struct_name_code {
                pub fn new(#(#new_fn_params),*) -> Self {
                    Self {
                        #new_fn_body
                    }
                }
            }
        }
    });

    if !mod_code.is_empty() {
        code.extend(quote! {
            pub mod #ns {
                use super::*;
                #mod_code
            }
        });
    }

    Ok(code)
}

struct TypeCode {
    type_name: TokenStream,
    code: TokenStream,
}

/// Creates the type reference for a struct field from a struct property.
/// Optionally, creates a type for a local schema.
fn create_struct_field_code(
    cg: &CodeGen,
    namespace: &TokenStream,
    property: &SchemaGen,
    property_name: &str,
    lowercase_workaround: bool,
) -> Result<TypeCode, Error> {
    match &property.ref_key {
        Some(ref_key) => {
            let tp = ref_key.name.to_camel_case_ident().map_err(Error::PropertyName)?;
            Ok(TypeCode {
                type_name: tp,
                code: TokenStream::new(),
            })
        }
        None => {
            if property.is_local_enum() {
                create_enum(namespace, property, property_name, lowercase_workaround)
            } else if property.is_local_struct() {
                let id = property_name.to_camel_case_ident().map_err(Error::PropertyName)?;
                let type_name = quote! {#namespace::#id};
                let code = create_struct(cg, property, property_name)?;
                Ok(TypeCode { type_name, code })
            } else {
                Ok(TypeCode {
                    type_name: type_name_gen(&property.type_name()?, false, false)?,
                    code: TokenStream::new(),
                })
            }
        }
    }
}
