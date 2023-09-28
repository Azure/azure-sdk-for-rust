use crate::{
    codegen::TypeNameCode,
    identifier::{CamelCaseIdent, SnakeCaseIdent},
    spec::{self, get_schema_array_items, get_type_name_for_schema, get_type_name_for_schema_ref},
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
pub struct PropertyGen {
    name: String,
    schema: SchemaGen,
}

impl PropertyGen {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn xml_attribute(&self) -> bool {
        self.schema.xml_attribute()
    }

    pub fn xml_name(&self) -> Option<&str> {
        self.schema.xml_name()
    }

    pub fn schema(&self) -> &SchemaGen {
        &self.schema
    }
}

#[derive(Clone)]
pub struct SchemaGen {
    ref_key: Option<RefKey>,
    schema: Schema,

    // used for identifying workarounds
    doc_file: Utf8PathBuf,

    // resolved
    properties: Vec<PropertyGen>,
    all_of: Vec<SchemaGen>,
}

#[derive(Clone)]
pub struct EnumValue {
    value: String,
    description: Option<String>,
}

impl EnumValue {
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
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

    /// Replaces the name of the element/attribute used for the described schema property.
    /// When defined within the Items Object (items), it will affect the name of the individual
    /// XML elements within the list. When defined alongside type being array (outside the items),
    /// it will affect the wrapping element and only if wrapped is true. If wrapped is false, it will be ignored.
    ///
    /// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#xmlObject
    fn xml_name(&self) -> Option<&str> {
        self.schema.common.xml.as_ref().and_then(|xml| xml.name.as_deref())
    }

    /// Declares whether the property definition translates to an attribute instead of an element.
    /// Default value is false.
    ///
    /// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#xmlObject
    fn xml_attribute(&self) -> bool {
        self.schema.common.xml.as_ref().and_then(|xml| xml.attribute).unwrap_or_default()
    }

    /// MAY be used only for an array definition. Signifies whether the array is wrapped (for example,
    /// <books><book/><book/></books>) or unwrapped (<book/><book/>). Default value is false.
    /// The definition takes effect only when defined alongside type being array (outside the items).
    ///
    /// https://github.com/OAI/OpenAPI-Specification/blob/main/versions/2.0.md#xmlObject
    fn xml_wrapped(&self) -> bool {
        self.schema.common.xml.as_ref().and_then(|xml| xml.wrapped).unwrap_or_default()
    }

    pub fn name(&self) -> Result<&str> {
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

    pub fn type_name(&self, cg: &CodeGen) -> Result<TypeNameCode> {
        let mut type_name = TypeNameCode::new(&get_type_name_for_schema(&self.schema.common)?)?;
        cg.set_if_union_type(&mut type_name);
        Ok(type_name)
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

    pub fn enum_values(&self) -> Vec<EnumValue> {
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

    pub fn properties(&self) -> Vec<&PropertyGen> {
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

    fn discriminator(&self) -> Option<&str> {
        self.schema.discriminator.as_deref()
    }

    fn discriminator_value(&self) -> Option<&str> {
        self.schema.x_ms_discriminator_value.as_deref()
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

pub fn all_schemas_resolved(spec: &Spec) -> Result<Vec<(RefKey, SchemaGen)>> {
    let schemas = all_schemas(spec)?;
    let schemas = resolve_all_schema_properties(&schemas, spec)?;
    let schemas = resolve_all_all_of(&schemas, spec)?;
    // sort schemas by name
    let mut schemas: Vec<_> = schemas.into_iter().collect();
    schemas.sort_by(|a, b| a.0.name.cmp(&b.0.name));
    Ok(schemas)
}

pub enum ModelCode {
    Struct(StructCode),
    Enum(NamedTypeCode),
    VecAlias(VecAliasCode),
    TypeAlias(TypeAliasCode),
    Union(UnionCode),
}

impl ToTokens for ModelCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            ModelCode::Struct(struct_code) => struct_code.to_tokens(tokens),
            ModelCode::Enum(enum_code) => enum_code.to_tokens(tokens),
            ModelCode::VecAlias(vec_alias_code) => vec_alias_code.to_tokens(tokens),
            ModelCode::TypeAlias(type_alias_code) => type_alias_code.to_tokens(tokens),
            ModelCode::Union(union_code) => union_code.to_tokens(tokens),
        }
    }
}

pub struct ModelsCode {
    pub has_case_workaround: bool,
    pub models: Vec<ModelCode>,
}

impl ToTokens for ModelsCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let has_case_workaround = self.has_case_workaround;
        let models = &self.models;
        tokens.extend(quote! {
            #![allow(non_camel_case_types)]
            #![allow(unused_imports)]
            use std::str::FromStr;
            use serde::{Serialize, Deserialize, Serializer};
            use serde::de::{value, Deserializer, IntoDeserializer};
        });
        if has_case_workaround {
            tokens.extend(quote! {
                use azure_core::util::case_insensitive_deserialize;
            });
        }
        tokens.extend(quote! {
            #(#models)*
        });
    }
}

pub fn create_models(cg: &mut CodeGen) -> Result<ModelsCode> {
    let mut pageable_response_names: HashMap<String, MsPageable> = HashMap::new();
    for operation in cg.spec.operations()? {
        if let Some(pageable) = operation.pageable.as_ref() {
            for response in operation.responses.values() {
                if let Some(schema) = &response.schema {
                    let pageable_name = TypeNameCode::new(&get_type_name_for_schema_ref(schema)?)?.to_string();
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

    let mut models = Vec::new();
    let mut schema_names = IndexMap::new();
    let all_schemas = &all_schemas_resolved(&cg.spec)?;

    // add union types
    for (_ref_key, schema) in all_schemas {
        if schema.discriminator().is_some() {
            let name = schema.name()?.to_camel_case_id();
            cg.add_union_type(name);
        }
    }

    for (ref_key, schema) in all_schemas {
        let doc_file = &ref_key.file_path;
        let schema_name = &ref_key.name;
        // println!("schema_name: {}", schema_name);
        if let Some(_first_doc_file) = schema_names.insert(schema_name, doc_file) {
            // eprintln!(
            //     "WARN schema {} already created from {:?}, duplicate from {:?}",
            //     schema_name, _first_doc_file, doc_file
            // );
        } else if schema.is_array() {
            models.push(ModelCode::VecAlias(create_vec_alias(schema)?));
        } else if schema.is_local_enum() {
            let enum_code = create_enum(None, schema, schema_name, false)?;
            models.push(ModelCode::Enum(enum_code));
        } else if schema.is_basic_type() {
            let alias = create_basic_type_alias(cg, schema_name, schema)?;
            models.push(ModelCode::TypeAlias(alias));
        } else {
            let pageable_name = format!("{}", schema_name.to_camel_case_ident()?);
            models.push(ModelCode::Struct(create_struct(
                cg,
                schema,
                schema_name,
                pageable_response_names.get(&pageable_name),
                HashSet::new(),
            )?));
            // create union if discriminator
            if let Some(tag) = schema.discriminator() {
                models.push(ModelCode::Union(UnionCode::from_schema(tag, schema_name, ref_key, all_schemas)?));
            }
        }
    }
    Ok(ModelsCode {
        has_case_workaround: cg.should_workaround_case(),
        models,
    })
}

pub struct UnionCode {
    pub tag: String,
    pub name: TypeNameCode,
    pub values: Vec<UnionValueCode>,
}

impl UnionCode {
    fn from_schema(tag: &str, schema_name: &str, ref_key: &RefKey, all_schemas: &Vec<(RefKey, SchemaGen)>) -> Result<Self> {
        let mut values = Vec::new();
        for (child_ref_key, child_schema) in all_schemas {
            if child_schema
                .all_of()
                .iter()
                .any(|all_of_schema| all_of_schema.ref_key.as_ref() == Some(ref_key))
            {
                if let Some(tag) = child_schema.discriminator_value() {
                    let name = tag.to_camel_case_ident()?;
                    let type_name = TypeNameCode::from(child_ref_key.name.to_camel_case_ident()?);
                    values.push(UnionValueCode {
                        tag: tag.to_string(),
                        name,
                        type_name,
                    });
                }
            }
        }
        let mut name = TypeNameCode::from(schema_name.to_camel_case_ident()?);
        name.union(true);
        Ok(Self {
            tag: tag.to_string(),
            name,
            values,
        })
    }
}

impl ToTokens for UnionCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let UnionCode { tag, name, values } = self;
        tokens.extend(quote! {
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[serde(tag = #tag)]
            pub enum #name {
                #(#values)*
            }
        });
    }
}

pub struct UnionValueCode {
    pub tag: String,
    pub name: Ident,
    pub type_name: TypeNameCode,
}

impl ToTokens for UnionValueCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let UnionValueCode { tag, name, type_name } = self;
        let serde = if tag != &name.to_string() {
            Some(SerdeCode::rename(tag))
        } else {
            None
        };
        tokens.extend(quote! {
            #serde
            #name(#type_name),
        });
    }
}

pub struct TypeAliasCode {
    pub id: Ident,
    pub value: TypeNameCode,
}

impl ToTokens for TypeAliasCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.id;
        let value = &self.value;
        tokens.extend(quote! {
            pub type #id = #value;
        });
    }
}

fn create_basic_type_alias(cg: &CodeGen, property_name: &str, property: &SchemaGen) -> Result<TypeAliasCode> {
    let id = property_name.to_camel_case_ident()?;
    let value = property.type_name(cg)?;
    Ok(TypeAliasCode { id, value })
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

fn create_enum(namespace: Option<&Ident>, property: &SchemaGen, property_name: &str, lowercase_workaround: bool) -> Result<NamedTypeCode> {
    let enum_values = property.enum_values();
    let id = &property_name.to_camel_case_ident()?;

    let mut values = TokenStream::new();
    for enum_value in &enum_values {
        let value = &enum_value.value;
        let nm = value.to_camel_case_ident()?;
        let doc_comment = DocCommentCode::from(&enum_value.description);
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

    let doc_comment = DocCommentCode::from(&property.schema.common.description);

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

    Ok(NamedTypeCode {
        type_name,
        code: Some(TypeCode::Enum(code)),
    })
}

pub struct VecAliasCode {
    pub id: Ident,
    pub value: TypeNameCode,
}

impl ToTokens for VecAliasCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = &self.id;
        let value = &self.value;
        tokens.extend(quote! {
            pub type #id = Vec<#value>;
        });
    }
}

fn create_vec_alias(schema: &SchemaGen) -> Result<VecAliasCode> {
    let items = schema.array_items()?;
    let id = schema.name()?.to_camel_case_ident()?;
    let value = TypeNameCode::new(&get_type_name_for_schema_ref(items)?)?;
    Ok(VecAliasCode { id, value })
}

pub struct StructCode {
    doc_comment: DocCommentCode,
    struct_name_code: Ident,
    default_code: TokenStream,
    props: Vec<StructPropCode>,
    continuable: Option<ContinuableCode>,
    implement_default: bool,
    new_fn_params: Vec<TokenStream>,
    new_fn_body: TokenStream,
    mod_code: TokenStream,
    ns: Ident,
}

impl ToTokens for StructCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let StructCode {
            doc_comment,
            struct_name_code,
            default_code,
            props,
            continuable,
            implement_default,
            new_fn_params,
            new_fn_body,
            mod_code,
            ns,
        } = self;

        let struct_code = quote! {
            #doc_comment
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
            #default_code
            pub struct #struct_name_code {
                #(#props)*
            }
            #continuable
        };
        tokens.extend(struct_code);

        tokens.extend(if *implement_default {
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
            tokens.extend(quote! {
                pub mod #ns {
                    use super::*;
                    #mod_code
                }
            });
        }
    }
}

fn create_struct(
    cg: &CodeGen,
    schema: &SchemaGen,
    struct_name: &str,
    pageable: Option<&MsPageable>,
    mut needs_boxing: HashSet<String>,
) -> Result<StructCode> {
    let mut mod_code = TokenStream::new();
    let mut props = Vec::new();
    let mut new_fn_params: Vec<TokenStream> = Vec::new();
    let mut new_fn_body = TokenStream::new();
    let ns = struct_name.to_snake_case_ident()?;
    let struct_name_code = struct_name.to_camel_case_ident()?;
    let required = schema.required();

    // println!("struct: {} {:?}", struct_name_code, pageable);
    needs_boxing.insert(struct_name.to_camel_case_ident()?.to_string());

    for schema in schema.all_of() {
        let schema_name = schema.name()?;
        let mut type_name = TypeNameCode::from(schema_name.to_camel_case_ident()?);
        type_name.union(false);
        let field_name = schema_name.to_snake_case_ident()?;
        props.push(StructPropCode {
            doc_comments: Vec::new(),
            serde: SerdeCode::flatten(),
            field_name: field_name.clone(),
            field_type: type_name.clone(),
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
        let property_name = if let Some(xml_name) = property.xml_name() {
            xml_name
        } else {
            property.name()
        };
        let field_name = property_name.to_snake_case_ident()?;
        let prop_nm = &PropertyName {
            file_path: schema.doc_file.clone(),
            schema_name: struct_name.to_owned(),
            property_name: property_name.to_owned(),
        };

        let lowercase_workaround = cg.should_workaround_case();

        let NamedTypeCode {
            mut type_name,
            code: field_code,
        } = create_struct_field_code(
            cg,
            &ns.clone(),
            &property.schema,
            property_name,
            lowercase_workaround,
            needs_boxing.clone(),
        )?;
        mod_code.extend(field_code.into_token_stream());
        let mut doc_comments = Vec::new();
        // uncomment the next two lines to help identify entries that need boxed
        // let prop_nm_str = format!("{} , {} , {}", prop_nm.file_path, prop_nm.schema_name, property_name);
        // doc_comments.push(DocCommentCode::from(&Some(prop_nm_str)));

        let mut boxed = false;
        if needs_boxing.contains(&type_name.to_string().to_camel_case_ident()?.to_string()) {
            boxed = true;
        }

        if cg.should_force_obj(prop_nm) {
            type_name.force_value(true);
        }

        let is_required = required.contains(property_name) && !cg.should_force_optional(prop_nm);

        field_names.insert(format!("{field_name}"), is_required);

        if !type_name.is_vec() && !is_required {
            type_name.optional(true);
        }

        let mut serde = SerdeCode::default();
        if field_name != property_name {
            if property.xml_attribute() {
                let as_attribute = format!("@{}", property_name);
                serde.add_rename(&as_attribute);
            } else {
                serde.add_rename(property_name);
            }
        }
        #[allow(clippy::collapsible_else_if)]
        if is_required {
            if type_name.is_date_time() {
                serde.add_with("azure_core::date::rfc3339");
            } else if type_name.is_date_time_rfc1123() {
                serde.add_with("azure_core::date::rfc1123");
            }
        } else {
            if type_name.is_date_time() {
                // Must specify `default` when using `with` for `Option`
                serde.add_default();
                serde.add_with("azure_core::date::rfc3339::option");
            } else if type_name.is_date_time_rfc1123() {
                // Must specify `default` when using `with` for `Option`
                serde.add_default();
                serde.add_with("azure_core::date::rfc1123::option");
            } else if type_name.is_vec() {
                serde.add_default();
                serde.add_deserialize_with("azure_core::util::deserialize_null_as_default");
                serde.add_skip_serializing_if("Vec::is_empty");
            } else {
                serde.add_default();
                serde.add_skip_serializing_if("Option::is_none");
            }
        }
        if property.schema.is_local_enum() {
            if lowercase_workaround {
                serde.add_deserialize_with("case_insensitive_deserialize");
            } else if cg.has_xml() {
                serde.add_with("azure_core::xml::text_content");
            }
        }

        // see if a field should be wrapped in a Box
        if cg.should_box_property(prop_nm) {
            boxed = true;
        }
        type_name.boxed(boxed);

        doc_comments.push(DocCommentCode::from(&property.schema.schema.common.description));

        props.push(StructPropCode {
            doc_comments,
            serde,
            field_name: field_name.clone(),
            field_type: type_name.clone(),
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
            new_fn_body.extend(quote! { #field_name: None, });
        }
    }

    let default_code = if schema.implement_default() {
        quote! { #[derive(Default)] }
    } else {
        quote! {}
    };

    let doc_comment = DocCommentCode::from(&schema.schema.common.description);

    let continuable = ContinuableCode::from_pageable(struct_name_code.clone(), pageable, field_names)?;

    Ok(StructCode {
        doc_comment,
        struct_name_code,
        default_code,
        props,
        continuable,
        implement_default: schema.implement_default(),
        new_fn_params,
        new_fn_body,
        mod_code,
        ns,
    })
}

pub struct ContinuableCode {
    pub struct_name: Ident,
    pub field_name: Option<Ident>,
    pub is_required: Option<bool>,
}

impl ContinuableCode {
    pub fn new(struct_name: Ident, field_name: Option<Ident>, is_required: Option<bool>) -> Self {
        Self {
            struct_name,
            field_name,
            is_required,
        }
    }

    pub fn from_pageable(struct_name: Ident, pageable: Option<&MsPageable>, field_names: HashMap<String, bool>) -> Result<Option<Self>> {
        if let Some(pageable) = pageable {
            let field_name = if let Some(name) = &pageable.next_link_name {
                let field_name = name.to_snake_case_ident()?;
                Some(field_name)
            } else {
                None
            };
            let is_required = field_name.as_ref().and_then(|field_name| field_names.get(&format!("{field_name}")));
            Ok(Some(Self::new(struct_name, field_name, is_required.cloned())))
        } else {
            Ok(None)
        }
    }
}

impl ToTokens for ContinuableCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            struct_name,
            field_name,
            is_required,
        } = self;

        if let Some(field_name) = field_name {
            // when there are multiple responses, we only add the Continuable
            // for the cases that have the field we care about.
            // println!("checking {} {} {}", struct_name_code, field_name, is_required);
            if let Some(is_required) = is_required {
                if *is_required {
                    tokens.extend(quote! {
                        impl azure_core::Continuable for #struct_name {
                            type Continuation = String;
                            fn continuation(&self) -> Option<Self::Continuation> {
                                if self.#field_name.is_empty() {
                                    None
                                } else {
                                    Some(self.#field_name.clone())
                                }
                            }
                        }
                    });
                } else {
                    tokens.extend(quote! {
                        impl azure_core::Continuable for #struct_name {
                            type Continuation = String;
                            fn continuation(&self) -> Option<Self::Continuation> {
                                self.#field_name.clone().filter(|value| !value.is_empty())
                            }
                        }
                    });
                }
            } else {
                // In a number of cases, such as USqlAssemblyList used in
                // datalake-analytics, the next link name is provided, but the
                // field doesn't exist in the response schema.  Handle that by
                // adding a Continuable that always returns None.
                tokens.extend(quote! {
                    impl azure_core::Continuable for #struct_name {
                        type Continuation = String;
                        fn continuation(&self) -> Option<Self::Continuation> {
                            None
                        }
                    }
                });
            }
        } else {
            // In a number of cases, such as DimensionsListResult used in
            // costmanagement, the next link name is null, and it's not provided
            // via a header or sometimes used in other responses.
            //
            // Handle that by // adding a Continuable that always returns None.
            tokens.extend(quote! {
                impl azure_core::Continuable for #struct_name {
                    type Continuation = String;
                    fn continuation(&self) -> Option<Self::Continuation> {
                        None
                    }
                }
            });
        }
    }
}

pub struct StructPropCode {
    pub doc_comments: Vec<DocCommentCode>,
    pub serde: SerdeCode,
    pub field_name: Ident,
    pub field_type: TypeNameCode,
}

impl StructPropCode {
    pub fn new(field_name: Ident, field_type: TypeNameCode) -> Self {
        Self {
            doc_comments: Vec::new(),
            serde: SerdeCode::default(),
            field_name,
            field_type,
        }
    }
}

impl ToTokens for StructPropCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let doc_comments = &self.doc_comments;
        let serde = &self.serde;
        let field_name = &self.field_name;
        let field_type = &self.field_type;
        tokens.extend(quote! {
            #(#doc_comments)*
            #serde
            pub #field_name: #field_type,
        });
    }
}

#[derive(Default)]
pub struct SerdeCode {
    attributes: Vec<TokenStream>,
}

impl SerdeCode {
    pub fn flatten() -> Self {
        let mut serde = Self::default();
        serde.add_flatten();
        serde
    }
    pub fn tag(tag: &str) -> Self {
        let mut serde = Self::default();
        serde.add_tag(tag);
        serde
    }
    pub fn rename(rename: &str) -> Self {
        let mut serde = Self::default();
        serde.add_rename(rename);
        serde
    }
    pub fn add_tag(&mut self, tag: &str) {
        self.attributes.push(quote! { tag = #tag });
    }
    pub fn add_flatten(&mut self) {
        self.attributes.push(quote! { flatten });
    }
    pub fn add_rename(&mut self, rename: &str) {
        self.attributes.push(quote! { rename = #rename });
    }
    pub fn add_alias(&mut self, alias: &str) {
        self.attributes.push(quote! { alias = #alias });
    }
    pub fn add_skip_serializing_if(&mut self, skip_serializing_if: &str) {
        self.attributes.push(quote! { skip_serializing_if = #skip_serializing_if });
    }
    pub fn add_default(&mut self) {
        self.attributes.push(quote! { default });
    }
    pub fn add_default_value(&mut self, default: &str) {
        self.attributes.push(quote! { default = #default });
    }
    pub fn add_with(&mut self, with: &str) {
        self.attributes.push(quote! { with = #with });
    }
    pub fn add_deserialize_with(&mut self, deserialize_with: &str) {
        self.attributes.push(quote! { deserialize_with = #deserialize_with });
    }
    pub fn add_serialize_with(&mut self, serialize_with: &str) {
        self.attributes.push(quote! { serialize_with = #serialize_with });
    }
    pub fn add_remote(&mut self, remote: &str) {
        self.attributes.push(quote! { remote = #remote });
    }
    pub fn add_skip_deserializing(&mut self) {
        self.attributes.push(quote! { skip_deserializing });
    }
}

impl ToTokens for SerdeCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attributes = &self.attributes;
        if !attributes.is_empty() {
            tokens.extend(quote! {
                #[serde(#(#attributes),*)]
            });
        }
    }
}

#[derive(Default)]
pub struct DocCommentCode {
    description: Option<String>,
}

impl From<&str> for DocCommentCode {
    fn from(description: &str) -> Self {
        Self {
            description: Some(description.to_string()),
        }
    }
}

impl From<&Option<String>> for DocCommentCode {
    fn from(description: &Option<String>) -> Self {
        Self {
            description: description.clone(),
        }
    }
}

impl ToTokens for DocCommentCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(description) = &self.description {
            tokens.extend(quote! {
                #[doc = #description]
            });
        }
    }
}

pub struct NamedTypeCode {
    type_name: TypeNameCode,
    code: Option<TypeCode>,
}

impl ToTokens for NamedTypeCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if let Some(code) = &self.code {
            code.to_tokens(tokens)
        }
    }
}

enum TypeCode {
    Struct(StructCode),
    Enum(TokenStream),
    XmlWrapped(XmlWrappedCode),
}

impl ToTokens for TypeCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            TypeCode::Struct(code) => code.to_tokens(tokens),
            TypeCode::Enum(code) => code.to_tokens(tokens),
            TypeCode::XmlWrapped(code) => code.to_tokens(tokens),
        }
    }
}

struct XmlWrappedCode {
    struct_name: Ident,
    type_name: TypeNameCode,
}

impl ToTokens for XmlWrappedCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { struct_name, type_name } = self;
        tokens.extend(quote! {
            #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
            pub struct #struct_name {
                #[serde(rename = "$value", default, skip_serializing_if = "Vec::is_empty")]
                pub items: #type_name,
            }
        });
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
    needs_boxing: HashSet<String>,
) -> Result<NamedTypeCode> {
    match &property.ref_key {
        Some(ref_key) => {
            let mut type_name = TypeNameCode::from(ref_key.name.to_camel_case_ident()?);
            cg.set_if_union_type(&mut type_name);
            Ok(NamedTypeCode { type_name, code: None })
        }
        None => {
            if property.is_local_enum() {
                create_enum(Some(namespace), property, property_name, lowercase_workaround)
            } else if property.is_local_struct() {
                let id = property_name.to_camel_case_ident()?;
                let type_name = TypeNameCode::from(vec![namespace.clone(), id]);
                let code = create_struct(cg, property, property_name, None, needs_boxing)?;
                Ok(NamedTypeCode {
                    type_name,
                    code: Some(TypeCode::Struct(code)),
                })
            } else if property.xml_wrapped() {
                let id = property_name.to_camel_case_ident()?;
                let struct_name = property
                    .xml_name()
                    .map(|name| name.to_camel_case_ident())
                    .transpose()?
                    .unwrap_or_else(|| id.clone());
                let code = XmlWrappedCode {
                    struct_name: struct_name.clone(),
                    type_name: property.type_name(cg)?,
                };
                Ok(NamedTypeCode {
                    type_name: TypeNameCode::from(vec![namespace.clone(), struct_name]),
                    code: Some(TypeCode::XmlWrapped(code)),
                })
            } else {
                Ok(NamedTypeCode {
                    type_name: property.type_name(cg)?,
                    code: None,
                })
            }
        }
    }
}
