use crate::{
    codegen::{type_name_gen, TypeNameCode},
    identifier::{CamelCaseIdent, SnakeCaseIdent},
    spec::{self, get_schema_array_items, get_type_name_for_schema, get_type_name_for_schema_ref, TypeName},
    CodeGen, PropertyName, ResolvedSchema, Spec,
};
use crate::{Error, ErrorKind, Result};
use autorust_openapi::{DataType, MsPageable, Reference, ReferenceOr, Schema};
use camino::{Utf8Path, Utf8PathBuf};
use indexmap::IndexMap;
use proc_macro2::{Ident, TokenStream};
use quote::{quote, ToTokens};
use serde_json::Value;
use spec::{get_schema_schema_references, openapi, RefKey};
use std::collections::{HashMap, HashSet};

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

    fn name(&self) -> Result<&str> {
        Ok(&self
            .ref_key
            .as_ref()
            .ok_or_else(|| Error::message(ErrorKind::CodeGen, "no ref key"))?
            .name)
    }

    fn is_array(&self) -> bool {
        matches!(self.schema.common.type_, Some(DataType::Array))
    }

    fn is_local_enum(&self) -> bool {
        !self.schema.common.enum_.is_empty()
    }

    fn is_model_as_string_enum(&self) -> bool {
        match &self.schema.common.x_ms_enum {
            Some(x_ms_enum) => x_ms_enum.model_as_string == Some(true),
            None => false,
        }
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

    fn type_name(&self) -> Result<TypeName> {
        get_type_name_for_schema(&self.schema.common)
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

    fn array_items(&self) -> Result<&ReferenceOr<Schema>> {
        get_schema_array_items(&self.schema.common)
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
) -> Result<SchemaGen> {
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
        .collect::<Result<_>>()?;
    Ok(schema)
}

fn resolve_schema_property(
    resolved: &mut IndexMap<RefKey, SchemaGen>,
    all_schemas: &IndexMap<RefKey, SchemaGen>,
    spec: &Spec,
    doc_file: &Utf8Path,
    property_name: String,
    property: &ResolvedSchema,
) -> Result<PropertyGen> {
    let schema = if let Some(ref_key) = &property.ref_key {
        if let Some(schema) = resolved.get(ref_key) {
            schema.clone()
        } else {
            let schema = all_schemas
                .get(ref_key)
                .ok_or_else(|| Error::with_message(ErrorKind::CodeGen, || format!("ref key not found {ref_key:?}")))?;
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

fn resolve_all_of(all_schemas: &IndexMap<RefKey, SchemaGen>, schema: &SchemaGen, spec: &Spec) -> Result<SchemaGen> {
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
        .collect::<Result<_>>()?;
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
                    .ok_or_else(|| Error::with_message(ErrorKind::CodeGen, || format!("ref key not found {ref_key:?}")))?
                    .clone();
                let schema = resolve_all_of(all_schemas, &schema, spec)?;
                Ok(Some(schema))
            }
        })
        .collect::<Result<_>>()?;
    let mut schema = schema.clone();
    schema.properties = properties;
    schema.all_of = all_of.into_iter().flatten().collect();
    Ok(schema)
}

fn all_schemas(spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>> {
    let mut all_schemas: IndexMap<RefKey, SchemaGen> = IndexMap::new();

    // all definitions from input_files
    for (doc_file, doc) in spec.input_docs() {
        let schemas = spec.resolve_schema_map(doc_file, &doc.definitions)?;
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

fn resolve_all_schema_properties(schemas: &IndexMap<RefKey, SchemaGen>, spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>> {
    let mut resolved: IndexMap<RefKey, SchemaGen> = IndexMap::new();
    for (ref_key, schema) in schemas {
        resolved.insert(ref_key.clone(), schema.clone()); // order properties after
        let schema = resolve_schema_properties(&mut resolved, schemas, spec, &ref_key.file_path, schema)?;
        resolved.insert(ref_key.clone(), schema);
    }
    Ok(resolved)
}

fn resolve_all_all_of(schemas: &IndexMap<RefKey, SchemaGen>, spec: &Spec) -> Result<IndexMap<RefKey, SchemaGen>> {
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

pub fn create_models(cg: &CodeGen) -> Result<TokenStream> {
    let mut file = TokenStream::new();

    let has_case_workaround = cg.should_workaround_case();

    file.extend(quote! {
        #![allow(non_camel_case_types)]
        #![allow(unused_imports)]
        use std::str::FromStr;
        use serde::{Serialize, Deserialize, Serializer};
        use serde::de::{value, Deserializer, IntoDeserializer};
    });
    if has_case_workaround {
        file.extend(quote! {
        use azure_core::util::case_insensitive_deserialize;
        });
    }

    let mut pageable_response_names: HashMap<String, MsPageable> = HashMap::new();
    for operation in cg.spec.operations()? {
        if let Some(pageable) = operation.pageable.as_ref() {
            for response in operation.responses.values() {
                if let Some(schema) = &response.schema {
                    let pageable_name = type_name_gen(&get_type_name_for_schema_ref(schema)?)?.to_string();
                    // in some cases, the same struct is used multiple times for
                    // responses (such as a get and list for a given object
                    // type).  In these cases, what we see is a next_link_name
                    // of null in one response, and a valid next_link_name in
                    // another.  so, only keep the one that has a next_link_name.
                    //
                    // operations that are not pageable won't call the
                    // Continuable trait, which should mean this is workaround
                    // is functional.
                    if let Some(entry) = pageable_response_names.get(&pageable_name) {
                        if entry.next_link_name.is_some() && pageable.next_link_name.is_none() {
                            continue;
                        }
                    }

                    pageable_response_names.insert(pageable_name.clone(), pageable.clone());
                }
            }
        }
    }

    // println!("response_names: {:?}", pageable_response_names);

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

        // println!("schema_name: {}", schema_name);

        // create_response_type()

        if let Some(_first_doc_file) = schema_names.insert(schema_name, doc_file) {
            // eprintln!(
            //     "WARN schema {} already created from {:?}, duplicate from {:?}",
            //     schema_name, _first_doc_file, doc_file
            // );
        } else if schema.is_array() {
            file.extend(create_vec_alias(schema)?);
        } else if schema.is_local_enum() {
            let enum_code = create_enum(None, schema, schema_name, false)?;
            file.extend(enum_code.into_token_stream());
        } else if schema.is_basic_type() {
            let (id, value) = create_basic_type_alias(schema_name, schema)?;
            file.extend(quote! { pub type #id = #value;});
        } else {
            let pageable_name = format!("{}", schema_name.to_camel_case_ident()?);
            file.extend(create_struct(cg, schema, schema_name, pageable_response_names.get(&pageable_name))?);
        }
    }
    Ok(file)
}

fn create_basic_type_alias(property_name: &str, property: &SchemaGen) -> Result<(Ident, TypeNameCode)> {
    let id = property_name.to_camel_case_ident()?;
    let value = type_name_gen(&property.type_name()?)?;
    Ok((id, value))
}

// For create_models. Recursively adds schema refs.
fn add_schema_refs(resolved: &mut IndexMap<RefKey, SchemaGen>, spec: &Spec, doc_file: &Utf8Path, schema_ref: &Reference) -> Result<()> {
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

fn create_enum(
    namespace: Option<&Ident>,
    property: &SchemaGen,
    property_name: &str,
    lowercase_workaround: bool,
) -> Result<StructFieldCode> {
    let enum_values = property.enum_values();
    let id = &property_name.to_camel_case_ident()?;

    let mut values = TokenStream::new();
    for enum_value in &enum_values {
        let value = &enum_value.value;
        let nm = value.to_camel_case_ident()?;
        let doc_comment = match &enum_value.description {
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

    // The x-ms-enum modelAsString enum field indicates that the enum is
    // subject to change, so should be treated as extensible. The document
    // says that if this field is set, "the enum will be modeled as a
    // string. No validation will happen."
    // https://azure.github.io/autorest/extensions/#x-ms-enum
    //
    // With Rust enums we can do better than that - use enum variants
    // for the known values but with an additional `UnknownValue(String)`
    // that can capture and store an unknown value as a `String`.
    // Unfortunately the standard `serde` attributes do not support this,
    // but it can be implemented via a custom deserializer using the
    // workaround suggested in this issue:
    // https://github.com/serde-rs/serde/issues/912

    // If `model_as_string` then add the `UnknownValue(String)` field to the enum variants
    if property.is_model_as_string_enum() {
        let value_token = quote! {
            #[serde(skip_deserializing)]
            UnknownValue(String)
        };
        values.extend(value_token);
    }

    // Need the id as a string as it needs to be quoted in some places in the
    // generated code.
    let id_str = id.to_string();

    // If `model_as_string` then set the `serde` `remote` attribute to indicate
    // that the Serializer/Deserializer will be defined elsewhere.
    let maybe_remote_attr = if property.is_model_as_string_enum() {
        quote! {
            #[serde(remote = #id_str)]
        }
    } else {
        quote! {}
    };

    // If `model_as_string` then provide custom `Deserialize` and `Serialize`
    // implementations.
    let custom_serde_code = if property.is_model_as_string_enum() {
        let mut serialize_fields = TokenStream::new();
        for (index, enum_value) in enum_values.iter().enumerate() {
            let value = &enum_value.value;
            let nm = value.to_camel_case_ident()?;
            let variant_index = index as u32;
            serialize_fields.extend(quote! {
                Self::#nm => serializer.serialize_unit_variant(#id_str, #variant_index, #value),
            });
        }

        quote! {
            impl FromStr for #id {
                type Err = value::Error;

                fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
                    Self::deserialize(s.into_deserializer())
                }
            }

            impl<'de> Deserialize<'de> for #id {
                fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
                    where D: Deserializer<'de>
                {
                    let s = String::deserialize(deserializer)?;
                    let deserialized = Self::from_str(&s).unwrap_or(
                        Self::UnknownValue(s)
                    );
                    Ok(deserialized)
                }
            }

            impl Serialize for #id {
                fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
                where
                    S: Serializer,
                {
                    match self {
                        #serialize_fields
                        Self::UnknownValue(s) => serializer.serialize_str(s.as_str()),
                    }
                }
            }
        }
    } else {
        quote! {}
    };

    let nm = property_name.to_camel_case_ident()?;
    let default_code = if let Some(default_name) = property.default() {
        let default_name = default_name.to_camel_case_ident()?;
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
        #maybe_remote_attr
        pub enum #nm {
            #values
        }
        #custom_serde_code
        #default_code
    };
    let type_name = TypeNameCode::from(vec![namespace, Some(id)]);

    Ok(StructFieldCode {
        type_name,
        code: Some(TypeCode::Enum(code)),
    })
}

fn create_vec_alias(schema: &SchemaGen) -> Result<TokenStream> {
    let items = schema.array_items()?;
    let typ = schema.name()?.to_camel_case_ident()?;
    let items_typ = type_name_gen(&get_type_name_for_schema_ref(items)?)?;
    Ok(quote! { pub type #typ = Vec<#items_typ>; })
}

fn create_struct(cg: &CodeGen, schema: &SchemaGen, struct_name: &str, pageable: Option<&MsPageable>) -> Result<TokenStream> {
    let mut code = TokenStream::new();
    let mut mod_code = TokenStream::new();
    let mut props = TokenStream::new();
    let mut new_fn_params: Vec<TokenStream> = Vec::new();
    let mut new_fn_body = TokenStream::new();
    let ns = struct_name.to_snake_case_ident()?;
    let struct_name_code = struct_name.to_camel_case_ident()?;
    let required = schema.required();

    // println!("struct: {} {:?}", struct_name_code, pageable);

    for schema in schema.all_of() {
        let schema_name = schema.name()?;
        let type_name = schema_name.to_camel_case_ident()?;
        let field_name = schema_name.to_snake_case_ident()?;
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

    let mut field_names = HashMap::new();

    for property in schema.properties() {
        let property_name = property.name.as_str();
        let field_name = property_name.to_snake_case_ident()?;
        let prop_nm = &PropertyName {
            file_path: schema.doc_file.clone(),
            schema_name: struct_name.to_owned(),
            property_name: property_name.to_owned(),
        };

        let lowercase_workaround = cg.should_workaround_case();

        let StructFieldCode {
            mut type_name,
            code: field_code,
        } = create_struct_field_code(cg, &ns.clone(), &property.schema, property_name, lowercase_workaround)?;
        mod_code.extend(field_code.into_token_stream());
        // uncomment the next two lines to help identify entries that need boxed
        // let prop_nm_str = format!("{} , {} , {}", prop_nm.file_path, prop_nm.schema_name, property_name);
        // props.extend(quote! { #[doc = #prop_nm_str ]});

        if cg.should_force_obj(prop_nm) {
            type_name = type_name.force_value(true);
        }

        let is_required = required.contains(property_name) && !cg.should_force_optional(prop_nm);

        field_names.insert(format!("{}", field_name), is_required);

        if !type_name.is_vec() && !is_required {
            type_name = type_name.optional(true);
        }

        let mut serde_attrs: Vec<TokenStream> = Vec::new();
        if field_name != property_name {
            serde_attrs.push(quote! { rename = #property_name });
        }
        #[allow(clippy::collapsible_else_if)]
        if is_required {
            if type_name.is_date_time() {
                serde_attrs.push(quote! { with = "azure_core::date::rfc3339"});
            } else if type_name.is_date_time_rfc1123() {
                serde_attrs.push(quote! { with = "azure_core::date::rfc1123"});
            }
        } else {
            if type_name.is_date_time() {
                // Must specify `default` when using `with` for `Option`
                serde_attrs.push(quote! { default, with = "azure_core::date::rfc3339::option"});
            } else if type_name.is_date_time_rfc1123() {
                // Must specify `default` when using `with` for `Option`
                serde_attrs.push(quote! { default, with = "azure_core::date::rfc1123::option"});
            } else if type_name.is_vec() {
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
        let boxed = cg.should_box_property(prop_nm);
        type_name = type_name.boxed(boxed);

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
        } else if type_name.is_vec() {
            if boxed {
                new_fn_body.extend(quote! { #field_name: Box::new(Vec::new()), });
            } else {
                new_fn_body.extend(quote! { #field_name: Vec::new(), });
            }
        } else {
            #[allow(clippy::collapsible_else_if)]
            if boxed {
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

    let mut continuable = quote! {};
    if let Some(pageable) = pageable {
        if let Some(name) = &pageable.next_link_name {
            let field_name = name.to_snake_case_ident()?;
            // when there are multiple responses, we only add the Continuable
            // for the cases that have the field we care about.
            // println!("checking {} {} {}", struct_name_code, field_name, field_names.contains(&format!("{}", field_name)));
            if let Some(is_required) = field_names.get(&format!("{}", field_name)) {
                if *is_required {
                    continuable = quote! {
                        impl azure_core::Continuable for #struct_name_code {
                            type Continuation = String;
                            fn continuation(&self) -> Option<Self::Continuation> {
                                if self.#field_name.is_empty() {
                                    None
                                } else {
                                    Some(self.#field_name.clone())
                                }
                            }
                        }
                    };
                } else {
                    continuable = quote! {
                        impl azure_core::Continuable for #struct_name_code {
                            type Continuation = String;
                            fn continuation(&self) -> Option<Self::Continuation> {
                                self.#field_name.clone()
                            }
                        }
                    };
                }
            } else {
                // In a number of cases, such as USqlAssemblyList used in
                // datalake-analytics, the next link name is provided, but the
                // field doesn't exist in the response schema.  Handle that by
                // adding a Continuable that always returns None.
                continuable = quote! {
                    impl azure_core::Continuable for #struct_name_code {
                        type Continuation = String;
                        fn continuation(&self) -> Option<Self::Continuation> {
                            None
                        }
                    }
                };
            }
        } else {
            // In a number of cases, such as DimensionsListResult used in
            // costmanagement, the next link name is null, and it's not provided
            // via a header or sometimes used in other responses.
            //
            // Handle that by // adding a Continuable that always returns None.
            continuable = quote! {
                impl azure_core::Continuable for #struct_name_code {
                    type Continuation = String;
                    fn continuation(&self) -> Option<Self::Continuation> {
                        None
                    }
                }
            };
        }
    }

    let struct_code = quote! {
        #doc_comment
        #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
        #default_code
        pub struct #struct_name_code {
            #props
        }
        #continuable
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

struct StructFieldCode {
    type_name: TypeNameCode,
    code: Option<TypeCode>,
}

impl ToTokens for StructFieldCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(code) = &self.code {
            code.to_tokens(tokens)
        }
    }
}

enum TypeCode {
    Struct(TokenStream),
    Enum(TokenStream),
}

impl ToTokens for TypeCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeCode::Struct(code) => tokens.extend(code.clone()),
            TypeCode::Enum(code) => tokens.extend(code.clone()),
        }
    }
}

/// Creates the type reference for a struct field from a struct property.
/// Optionally, creates a type for a local schema.
fn create_struct_field_code(
    cg: &CodeGen,
    namespace: &Ident,
    property: &SchemaGen,
    property_name: &str,
    lowercase_workaround: bool,
) -> Result<StructFieldCode> {
    match &property.ref_key {
        Some(ref_key) => {
            let tp = ref_key.name.to_camel_case_ident()?;
            Ok(StructFieldCode {
                type_name: tp.into(),
                code: None,
            })
        }
        None => {
            if property.is_local_enum() {
                create_enum(Some(namespace), property, property_name, lowercase_workaround)
            } else if property.is_local_struct() {
                let id = property_name.to_camel_case_ident()?;
                let type_name = TypeNameCode::from(vec![namespace.clone(), id]);
                let code = create_struct(cg, property, property_name, None)?;
                Ok(StructFieldCode {
                    type_name,
                    code: Some(TypeCode::Struct(code)),
                })
            } else {
                Ok(StructFieldCode {
                    type_name: type_name_gen(&property.type_name()?)?,
                    code: None,
                })
            }
        }
    }
}
