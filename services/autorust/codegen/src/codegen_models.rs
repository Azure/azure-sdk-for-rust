use crate::{
    codegen::{
        create_generated_by_header, enum_values_as_strings, get_schema_array_items, get_type_name_for_schema, get_type_name_for_schema_ref,
        is_array, is_basic_type, is_local_enum, is_local_struct, is_vec, require, AsReference, Error,
    },
    identifier::{ident, CamelCaseIdent},
    spec, CodeGen, PropertyName, ResolvedSchema,
};
use autorust_openapi::Reference;
use heck::{CamelCase, SnakeCase};
use indexmap::IndexMap;
use proc_macro2::TokenStream;
use quote::quote;
use spec::{get_schema_schema_references, openapi, RefKey};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};

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

    let mut all_schemas: IndexMap<RefKey, ResolvedSchema> = IndexMap::new();

    // all definitions from input_files
    for (doc_file, doc) in cg.spec.input_docs() {
        let schemas = cg.spec.resolve_schema_map(doc_file, &doc.definitions).map_err(Error::Spec)?;
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
    for (doc_file, api) in cg.spec.input_docs() {
        for reference in openapi::get_api_schema_references(doc_file, api) {
            add_schema_refs(cg, &mut all_schemas, doc_file, reference)?;
        }
    }

    let mut schema_names = IndexMap::new();
    for (ref_key, schema) in &all_schemas {
        let doc_file = &ref_key.file_path;
        let schema_name = &ref_key.name;
        if let Some(_first_doc_file) = schema_names.insert(schema_name, doc_file) {
            // eprintln!(
            //     "WARN schema {} already created from {:?}, duplicate from {:?}",
            //     schema_name, _first_doc_file, doc_file
            // );
        } else if is_array(&schema.schema.common) {
            file.extend(create_vec_alias(schema_name, schema)?);
        } else if is_local_enum(schema) {
            let no_namespace = TokenStream::new();
            let (_tp_name, tp) = create_enum(&no_namespace, schema_name, schema, false)?;
            file.extend(tp);
        } else if is_basic_type(schema) {
            let (id, value) = create_basic_type_alias(schema_name, schema)?;
            file.extend(quote! { pub type #id = #value;});
        } else {
            for stream in create_struct(cg, doc_file, schema_name, schema)? {
                file.extend(stream);
            }
        }
    }
    Ok(file)
}

fn create_basic_type_alias(property_name: &str, property: &ResolvedSchema) -> Result<(TokenStream, TokenStream), Error> {
    let id = ident(&property_name.to_camel_case()).map_err(Error::StructName)?;
    let value = get_type_name_for_schema(&property.schema.common, AsReference::False)?;
    Ok((id, value))
}

// For create_models. Recursively adds schema refs.
fn add_schema_refs(
    cg: &CodeGen,
    schemas: &mut IndexMap<RefKey, ResolvedSchema>,
    doc_file: &Path,
    schema_ref: Reference,
) -> Result<(), Error> {
    let schema = cg.spec.resolve_schema_ref(doc_file, schema_ref)?;
    if let Some(ref_key) = schema.ref_key.clone() {
        if !schemas.contains_key(&ref_key) && !cg.spec.is_input_file(&ref_key.file_path) {
            let refs = get_schema_schema_references(&schema.schema);
            schemas.insert(ref_key.clone(), schema);
            for reference in refs {
                add_schema_refs(cg, schemas, &ref_key.file_path, reference)?;
            }
        }
    }
    Ok(())
}

fn create_enum(
    namespace: &TokenStream,
    property_name: &str,
    property: &ResolvedSchema,
    lowercase_workaround: bool,
) -> Result<(TokenStream, TokenStream), Error> {
    let enum_values = enum_values_as_strings(&property.schema.common.enum_);
    let id = ident(&property_name.to_camel_case()).map_err(|source| Error::EnumName {
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
    let nm = ident(&property_name.to_camel_case()).map_err(|source| Error::EnumName {
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

fn create_vec_alias(alias_name: &str, schema: &ResolvedSchema) -> Result<TokenStream, Error> {
    let items = get_schema_array_items(&schema.schema.common)?;
    let typ = ident(&alias_name.to_camel_case()).map_err(Error::VecAliasName)?;
    let items_typ = get_type_name_for_schema_ref(items, AsReference::False)?;
    Ok(quote! { pub type #typ = Vec<#items_typ>; })
}

fn create_struct(cg: &CodeGen, doc_file: &Path, struct_name: &str, schema: &ResolvedSchema) -> Result<Vec<TokenStream>, Error> {
    // println!("create_struct {} {}", doc_file.to_str().unwrap(), struct_name);
    let mut streams = Vec::new();
    let mut local_types = Vec::new();
    let mut props = TokenStream::new();
    let ns = ident(&struct_name.to_snake_case()).map_err(Error::StructName)?;
    let nm = ident(&struct_name.to_camel_case()).map_err(Error::StructName)?;
    let required: HashSet<&str> = schema.schema.required.iter().map(String::as_str).collect();

    for schema in &schema.schema.all_of {
        let type_name = get_type_name_for_schema_ref(schema, AsReference::False)?;
        let field_name = ident(&type_name.to_string().to_snake_case()).map_err(Error::StructFieldName)?;
        props.extend(quote! {
            #[serde(flatten)]
            pub #field_name: #type_name,
        });
    }

    let properties = cg.spec.resolve_schema_map(doc_file, &schema.schema.properties)?;
    for (property_name, property) in &properties {
        let nm = ident(&property_name.to_snake_case()).map_err(Error::StructName)?;
        let prop_nm = &PropertyName {
            file_path: PathBuf::from(doc_file),
            schema_name: struct_name.to_owned(),
            property_name: property_name.to_string(),
        };

        let lowercase_workaround = cg.should_workaround_case();

        let (mut field_tp_name, field_tp) = create_struct_field_type(cg, doc_file, &ns, property_name, property, lowercase_workaround)?;
        // uncomment the next two lines to help identify entries that need boxed
        // let prop_nm_str = format!("{} , {} , {}", prop_nm.file_path.display(), prop_nm.schema_name, property_name);
        // props.extend(quote! { #[doc = #prop_nm_str ]});

        if cg.should_force_obj(prop_nm) {
            field_tp_name = quote! { serde_json::Value };
        }

        let is_required = required.contains(property_name.as_str()) && !cg.should_force_optional(prop_nm);

        let is_vec = is_vec(&field_tp_name);
        if !is_vec {
            field_tp_name = require(is_required, field_tp_name);
        }
        local_types.extend(field_tp);
        let mut serde_attrs: Vec<TokenStream> = Vec::new();
        if &nm.to_string() != property_name {
            serde_attrs.push(quote! { rename = #property_name });
        }
        if !is_required {
            if is_vec {
                serde_attrs.push(quote! { default, skip_serializing_if = "Vec::is_empty"});
            } else {
                serde_attrs.push(quote! { default, skip_serializing_if = "Option::is_none"});
            }
        }
        if is_local_enum(property) && lowercase_workaround {
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

/// Creates the type reference for a struct field from a struct property.
/// Optionally, creates a type for a local schema.
fn create_struct_field_type(
    cg: &CodeGen,
    doc_file: &Path,
    namespace: &TokenStream,
    property_name: &str,
    property: &ResolvedSchema,
    lowercase_workaround: bool,
) -> Result<(TokenStream, Vec<TokenStream>), Error> {
    match &property.ref_key {
        Some(ref_key) => {
            let tp = ident(&ref_key.name.to_camel_case()).map_err(Error::PropertyName)?;
            Ok((tp, Vec::new()))
        }
        None => {
            if is_local_enum(property) {
                let (tp_name, tp) = create_enum(namespace, property_name, property, lowercase_workaround)?;
                Ok((tp_name, vec![tp]))
            } else if is_local_struct(property) {
                let id = ident(&property_name.to_camel_case()).map_err(Error::PropertyName)?;
                let tp_name = quote! {#namespace::#id};
                let tps = create_struct(cg, doc_file, property_name, property)?;
                // println!("creating local struct {:?} {}", tp_name, tps.len());
                Ok((tp_name, tps))
            } else {
                Ok((get_type_name_for_schema(&property.schema.common, AsReference::False)?, Vec::new()))
            }
        }
    }
}
