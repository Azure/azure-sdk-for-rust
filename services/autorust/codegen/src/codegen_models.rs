use crate::{
    codegen::{add_option, create_generated_by_header, is_vec, type_name_gen, Error},
    identifier::{CamelCaseIdent, SnakeCaseIdent},
    spec::{self, get_schema_array_items, get_type_name_for_schema, get_type_name_for_schema_ref, TypeName},
    CodeGen, PropertyName, Spec,
};
use autorust_openapi::{DataType, Reference, ReferenceOr, Schema};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use serde_json::Value;
use spec::{get_schema_schema_references, openapi, RefKey};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

struct PropertyGen {
    name: String,
    schema: SchemaGen,
}

struct SchemaGen {
    ref_key: Option<RefKey>,
    schema: Schema,
    // resolved properties
    properties: Vec<PropertyGen>,
}

impl SchemaGen {
    fn name(&self) -> Result<&str, Error> {
        Ok(&self.ref_key.as_ref().ok_or(Error::NoRefKey)?.name)
    }

    fn doc_file(&self) -> Result<PathBuf, Error> {
        Ok(self.ref_key.as_ref().map(|ref_key| ref_key.doc_file.clone()).unwrap_or_default())
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

    fn all_of(&self) -> Vec<&ReferenceOr<Schema>> {
        self.schema.all_of.iter().collect()
    }

    fn array_items(&self) -> Result<&ReferenceOr<Schema>, Error> {
        get_schema_array_items(&self.schema.common).map_err(Error::ArrayItems)
    }

    fn enum_values_as_strings(&self) -> Vec<&str> {
        enum_values_as_strings(&self.schema.common.enum_)
    }

    fn properties(&self) -> Vec<&PropertyGen> {
        self.properties.iter().collect()
    }
}

fn resolve_properties(schema: &mut SchemaGen, spec: &Spec, doc_file: impl Into<PathBuf>) -> Result<(), Error> {
    let properties = spec.resolve_schema_map(doc_file, &schema.schema.properties)?;
    schema.properties = properties
        .into_iter()
        .map(|(ref_key, resolved_schema)| {
            let mut schema = SchemaGen {
                ref_key: resolved_schema.ref_key,
                schema: resolved_schema.schema,
                properties: Vec::new(),
            };
            // resolve_properties(&mut schema, spec, ref_key.doc_file)?; // recursive
            let property = PropertyGen {
                name: ref_key.name,
                schema,
            };
            Ok(property)
        })
        .collect::<Result<_, Error>>()?;
    Ok(())
}

fn enum_values_as_strings(values: &[Value]) -> Vec<&str> {
    values
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s.as_str()),
            _ => None,
        })
        .collect()
}

fn all_schemas(spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>, Error> {
    let mut all_schemas: IndexMap<RefKey, SchemaGen> = IndexMap::new();

    // all definitions from input_files
    for (doc_file, doc) in spec.input_docs() {
        let schemas = spec.resolve_schema_map(doc_file, &doc.definitions).map_err(Error::Spec)?;
        for (ref_key, resolved_schema) in schemas {
            all_schemas.insert(
                ref_key.clone(),
                SchemaGen {
                    ref_key: Some(ref_key),
                    schema: resolved_schema.schema,
                    properties: Vec::new(),
                },
            );
        }
    }

    // any referenced schemas from other files
    for (doc_file, api) in spec.input_docs() {
        for reference in openapi::get_api_schema_references(doc_file, api) {
            add_schema_refs(spec, &mut all_schemas, doc_file, reference)?;
        }
    }

    // resolve properties before codegen
    for (ref_key, schema) in &mut all_schemas {
        resolve_properties(schema, spec, &ref_key.doc_file)?;
    }

    Ok(all_schemas)
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
    for (ref_key, schema) in &all_schemas(&cg.spec)? {
        let doc_file = &ref_key.doc_file;
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
            let (_tp_name, tp) = create_enum(&no_namespace, schema, schema_name, false)?;
            file.extend(tp);
        } else if schema.is_basic_type() {
            let (id, value) = create_basic_type_alias(schema_name, schema)?;
            file.extend(quote! { pub type #id = #value;});
        } else {
            for stream in create_struct(cg, schema, schema_name)? {
                file.extend(stream);
            }
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
fn add_schema_refs(spec: &Spec, schemas: &mut IndexMap<RefKey, SchemaGen>, doc_file: &Path, schema_ref: Reference) -> Result<(), Error> {
    let resolved_schema = spec.resolve_schema_ref(doc_file, schema_ref)?;
    if let Some(ref_key) = resolved_schema.ref_key {
        if !schemas.contains_key(&ref_key) && !spec.is_input_file(&ref_key.doc_file) {
            let schema = resolved_schema.schema;
            let refs = get_schema_schema_references(&schema);
            let schema = SchemaGen {
                // doc_file: ref_key.doc_file.clone(),
                // name: ref_key.name.clone(),
                ref_key: Some(ref_key.clone()),
                schema,
                properties: Vec::new(),
            };
            schemas.insert(ref_key.clone(), schema);
            for reference in refs {
                add_schema_refs(spec, schemas, &ref_key.doc_file, reference)?;
            }
        }
    }
    Ok(())
}

fn create_enum(
    namespace: &TokenStream,
    property: &SchemaGen,
    property_name: &str,
    lowercase_workaround: bool,
) -> Result<(TokenStream, TokenStream), Error> {
    let enum_values = property.enum_values_as_strings();
    let id = &property_name.to_camel_case_ident().map_err(|source| Error::EnumName {
        source,
        property: property_name.to_owned(),
    })?;
    let mut values = TokenStream::new();
    for name in enum_values {
        let nm = name.to_camel_case_ident().map_err(|source| Error::EnumName {
            source,
            property: property_name.to_owned(),
        })?;
        let lower = name.to_lowercase();
        let rename = if nm.to_string() == name {
            quote! {}
        } else if name != lower && lowercase_workaround {
            quote! { #[serde(rename = #name, alias = #lower)] }
        } else {
            quote! { #[serde(rename = #name)] }
        };
        let value = quote! {
            #rename
            #nm,
        };
        values.extend(value);
    }
    let nm = property_name.to_camel_case_ident().map_err(|source| Error::EnumName {
        source,
        property: property_name.to_owned(),
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

fn create_vec_alias(schema: &SchemaGen) -> Result<TokenStream, Error> {
    let items = schema.array_items()?;
    let typ = schema.name()?.to_camel_case_ident().map_err(Error::VecAliasName)?;
    let items_typ = type_name_gen(&get_type_name_for_schema_ref(items)?, false, false)?;
    Ok(quote! { pub type #typ = Vec<#items_typ>; })
}

fn create_struct(cg: &CodeGen, schema: &SchemaGen, struct_name: &str) -> Result<Vec<TokenStream>, Error> {
    // let struct_name = schema.name().map_err(|_| Error::SchemaNameStruct)?;
    // println!("create_struct {} {}", doc_file.to_str().unwrap(), struct_name);
    let mut streams = Vec::new();
    let mut local_types = Vec::new();
    let mut props = TokenStream::new();
    let ns = struct_name.to_snake_case_ident().map_err(Error::StructName)?;
    let nm = struct_name.to_camel_case_ident().map_err(Error::StructName)?;
    let required = schema.required();

    for schema in schema.all_of() {
        let type_name = type_name_gen(&get_type_name_for_schema_ref(schema)?, false, false)?;
        let field_name = type_name.to_string().to_snake_case_ident().map_err(Error::StructFieldName)?;
        props.extend(quote! {
            #[serde(flatten)]
            pub #field_name: #type_name,
        });
    }

    for property in schema.properties() {
        // let property_name = property.name().map_err(|_| Error::SchemaNameProperty)?;
        let property_name = property.name.as_str();
        let nm = property_name.to_snake_case_ident().map_err(Error::StructName)?;
        let prop_nm = &PropertyName {
            doc_file: schema.doc_file()?.to_owned(),
            schema_name: struct_name.to_owned(),
            property_name: property_name.to_owned(),
        };

        let lowercase_workaround = cg.should_workaround_case();

        let TypeCode {
            type_name: mut field_tp_name,
            inner_types: field_tp,
        } = create_struct_field_code(cg, &ns, &property.schema, property_name, lowercase_workaround)?;
        // uncomment the next two lines to help identify entries that need boxed
        // let prop_nm_str = format!("{} , {} , {}", prop_nm.file_path.display(), prop_nm.schema_name, property_name);
        // props.extend(quote! { #[doc = #prop_nm_str ]});

        if cg.should_force_obj(prop_nm) {
            field_tp_name = quote! { serde_json::Value };
        }

        let is_required = required.contains(property_name) && !cg.should_force_optional(prop_nm);

        let is_vec = is_vec(&field_tp_name);
        if !is_vec {
            field_tp_name = add_option(!is_required, field_tp_name);
        }
        local_types.extend(field_tp);
        let mut serde_attrs: Vec<TokenStream> = Vec::new();
        if nm.to_string() != property_name {
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
        // see if a field shoud be wrapped in a Box
        // println!("property {:?}", prop_nm);
        if cg.should_box_property(prop_nm) {
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
    streams.push(st);

    if !local_types.is_empty() {
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

struct TypeCode {
    type_name: TokenStream,
    inner_types: Vec<TokenStream>,
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
                inner_types: Vec::new(),
            })
        }
        None => {
            if property.is_local_enum() {
                let (tp_name, tp) = create_enum(namespace, property, property_name, lowercase_workaround)?;
                Ok(TypeCode {
                    type_name: tp_name,
                    inner_types: vec![tp],
                })
            } else if property.is_local_struct() {
                let id = property_name.to_camel_case_ident().map_err(Error::PropertyName)?;
                let tp_name = quote! {#namespace::#id};
                let tps = create_struct(cg, property, property_name)?;
                // println!("creating local struct {:?} {}", tp_name, tps.len());
                Ok(TypeCode {
                    type_name: tp_name,
                    inner_types: tps,
                })
            } else {
                Ok(TypeCode {
                    type_name: type_name_gen(&property.type_name()?, false, false)?,
                    inner_types: Vec::new(),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_enum_values_as_strings() {
        let values = vec![json!("/"), json!("/keys")];
        assert_eq!(enum_values_as_strings(&values), vec!["/", "/keys"]);
    }
}
