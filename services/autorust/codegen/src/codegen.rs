use crate::{identifier::parse_ident, spec::TypeName, CrateConfig, PropertyName, Spec};
use crate::{Error, Result};
use camino::Utf8Path;
use camino::Utf8PathBuf;
use heck::ToPascalCase;
use once_cell::sync::Lazy;
use proc_macro2::{Ident, TokenStream};
use quote::ToTokens;
use regex::Regex;
use std::{collections::HashSet, convert::TryFrom};
use syn::{
    punctuated::Punctuated,
    token::{Gt, Impl, Lt},
    AngleBracketedGenericArguments, GenericArgument, Path, PathArguments, PathSegment, TraitBound, TraitBoundModifier, Type, TypeImplTrait,
    TypeParamBound, TypePath,
};

/// code generation context
pub struct CodeGen<'a> {
    crate_config: &'a CrateConfig<'a>,
    pub spec: Spec,

    // workarounds
    box_properties: HashSet<PropertyName>,
    optional_properties: HashSet<PropertyName>,
    fix_case_properties: HashSet<&'a str>,
    invalid_types: HashSet<PropertyName>,
}

impl<'a> CodeGen<'a> {
    pub fn new(
        crate_config: &'a CrateConfig,
        box_properties: HashSet<PropertyName>,
        optional_properties: HashSet<PropertyName>,
        fix_case_properties: HashSet<&'a str>,
        invalid_types: HashSet<PropertyName>,
    ) -> Result<Self> {
        let spec = Spec::read_files(&crate_config.input_files)?;
        Ok(Self {
            crate_config,
            spec,
            box_properties,
            optional_properties,
            fix_case_properties,
            invalid_types,
        })
    }

    pub fn input_files(&self) -> &[Utf8PathBuf] {
        &self.crate_config.input_files
    }

    pub fn output_folder(&self) -> &Utf8Path {
        &self.crate_config.output_folder
    }

    pub fn should_workaround_case(&self) -> bool {
        if let Some(title) = self.spec.title() {
            self.fix_case_properties.contains(title)
        } else {
            false
        }
    }

    pub fn should_force_optional(&self, prop_nm: &PropertyName) -> bool {
        self.optional_properties.contains(prop_nm)
    }

    pub fn should_force_obj(&self, prop_nm: &PropertyName) -> bool {
        self.invalid_types.contains(prop_nm)
    }

    pub fn should_box_property(&self, prop_nm: &PropertyName) -> bool {
        self.box_properties.contains(prop_nm)
    }
}

fn id_models() -> Ident {
    parse_ident("models").unwrap()
}

pub fn type_name_gen(type_name: &TypeName) -> Result<TypeNameCode> {
    let mut type_name_code = match type_name {
        TypeName::Reference(name) => {
            let idt = parse_ident(&name.to_pascal_case())?;
            TypeNameCode::from(idt).allow_qualify_models(true)
        }
        TypeName::Array(vec_items_typ) => type_name_gen(vec_items_typ)?.incr_vec_count(),
        TypeName::Value => TypeNameCode::from(tp_json_value()),
        TypeName::Bytes => TypeNameCode::from(tp_bytes()),
        TypeName::Int32 => TypeNameCode::from(tp_i32()).allow_impl_into(false),
        TypeName::Int64 => TypeNameCode::from(tp_i64()).allow_impl_into(false),
        TypeName::Float32 => TypeNameCode::from(tp_f32()).allow_impl_into(false),
        TypeName::Float64 => TypeNameCode::from(tp_f64()).allow_impl_into(false),
        TypeName::Boolean => TypeNameCode::from(tp_bool()).allow_impl_into(false),
        TypeName::String => TypeNameCode::from(tp_string()),
        TypeName::DateTime => TypeNameCode::from(tp_date_time()),
        TypeName::DateTimeRfc1123 => TypeNameCode::from(tp_date_time()),
    };
    type_name_code.type_name = Some(type_name.clone());
    Ok(type_name_code)
}

// any word character or `-` between curly braces
pub static PARAM_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\{([\w-]+)\}").unwrap());

/// Get a list of parameter names in the URI path
/// For example: "/storage/{storage-account-name}/sas/{sas-definition-name}"
/// Returns ["storage-account-name", "sas-definition-name"]
pub fn parse_path_params(path: &str) -> Vec<String> {
    // capture 0 is the whole match and 1 is the actual capture like other languages
    PARAM_RE.captures_iter(path).into_iter().map(|c| c[1].to_string()).collect()
}

/// Get a set of parameter names in the URI query
/// For example: "/?restype=service&comp=userdelegationkey"
/// Returns ["restype", "comp"]
pub fn parse_query_params(uri: &str) -> Result<HashSet<String>> {
    if let Some(n) = uri.find('?') {
        let query = &uri[n..];
        let qs = qstring::QString::from(query);
        Ok(qs.into_iter().map(|(k, _)| k).collect())
    } else {
        Ok(HashSet::new())
    }
}

#[derive(Clone)]
pub struct TypeNameCode {
    type_path: TypePath,
    force_value: bool,
    pub optional: bool,
    vec_count: i32,
    impl_into: bool,
    allow_impl_into: bool,
    boxed: bool,
    qualify_models: bool,
    allow_qualify_models: bool,
    type_name: Option<TypeName>,
}

impl TypeNameCode {
    pub fn is_string(&self) -> bool {
        self.type_name == Some(TypeName::String)
    }
    pub fn is_bytes(&self) -> bool {
        self.type_name == Some(TypeName::Bytes)
    }
    pub fn is_date_time(&self) -> bool {
        self.type_name == Some(TypeName::DateTime)
    }
    pub fn is_date_time_rfc1123(&self) -> bool {
        self.type_name == Some(TypeName::DateTimeRfc1123)
    }
    pub fn is_vec(&self) -> bool {
        self.vec_count > 0 && !self.force_value
    }
    /// Forces the type to be `serde_json::Value`
    pub fn force_value(mut self, force_value: bool) -> Self {
        self.force_value = force_value;
        self
    }
    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }
    pub fn incr_vec_count(mut self) -> Self {
        self.vec_count += 1;
        self
    }
    pub fn impl_into(mut self, impl_into: bool) -> Self {
        self.impl_into = impl_into;
        self
    }
    pub fn has_impl_into(&self) -> bool {
        self.allow_impl_into && self.impl_into
    }
    fn allow_impl_into(mut self, allow_impl_into: bool) -> Self {
        self.allow_impl_into = allow_impl_into;
        self
    }
    pub fn boxed(mut self, boxed: bool) -> Self {
        self.boxed = boxed;
        self
    }
    pub fn qualify_models(mut self, qualify_models: bool) -> Self {
        self.qualify_models = qualify_models;
        self
    }
    fn allow_qualify_models(mut self, allow_qualify_models: bool) -> Self {
        self.allow_qualify_models = allow_qualify_models;
        self
    }

    fn to_type(&self) -> Type {
        let mut tp = self.type_path.clone();
        if self.allow_qualify_models && self.qualify_models {
            tp.path.segments.insert(0, id_models().into());
        }
        let mut tp = Type::from(tp);
        for _ in 0..self.vec_count {
            tp = generic_type(tp_vec(), tp);
        }
        if self.force_value {
            tp = Type::from(tp_json_value())
        }
        if self.optional {
            tp = generic_type(tp_option(), tp);
        }
        if self.has_impl_into() {
            if let Type::Path(path) = generic_type(tp_into(), tp.clone()) {
                // prefix with "impl "
                let bound = TraitBound {
                    path: path.path,
                    paren_token: None,
                    modifier: TraitBoundModifier::None,
                    lifetimes: None,
                };
                let bound = TypeParamBound::Trait(bound);
                let mut bounds = Punctuated::new();
                bounds.push(bound);
                tp = Type::ImplTrait(TypeImplTrait {
                    bounds,
                    impl_token: Impl::default(),
                });
            }
        }
        if self.boxed {
            tp = generic_type(tp_box(), tp);
        }
        tp
    }
}

impl ToString for TypeNameCode {
    fn to_string(&self) -> String {
        self.to_type().into_token_stream().to_string()
    }
}

impl ToTokens for TypeNameCode {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.to_type().to_tokens(tokens);
    }
}

/// Creates a generic type
/// Passing to (std::i32, std::option::Option) with result in std::option::Option<std::i32>
fn generic_type(mut wrap_tp: TypePath, tp: Type) -> Type {
    let arg = GenericArgument::Type(tp);
    let mut args = Punctuated::new();
    args.push(arg);
    let arguments = PathArguments::AngleBracketed(AngleBracketedGenericArguments {
        args,
        colon2_token: None,
        lt_token: Lt::default(),
        gt_token: Gt::default(),
    });
    if let Some(v) = wrap_tp.path.segments.last_mut() {
        v.arguments = arguments
    }
    Type::from(wrap_tp)
}

impl From<TypePath> for TypeNameCode {
    fn from(type_path: TypePath) -> Self {
        Self {
            type_path,
            force_value: false,
            optional: false,
            vec_count: 0,
            impl_into: false,
            allow_impl_into: true,
            boxed: false,
            qualify_models: false,
            allow_qualify_models: false,
            type_name: None,
        }
    }
}

impl From<Vec<Ident>> for TypeNameCode {
    fn from(value: Vec<Ident>) -> Self {
        Self::from(idents_to_type_path(value))
    }
}

impl From<Ident> for TypeNameCode {
    fn from(value: Ident) -> Self {
        Self::from(idents_to_type_path(vec![value]))
    }
}

impl From<Vec<Option<&Ident>>> for TypeNameCode {
    fn from(value: Vec<Option<&Ident>>) -> Self {
        Self::from(optional_idents_to_type_path(value))
    }
}

impl TryFrom<&str> for TypeNameCode {
    type Error = Error;
    fn try_from(value: &str) -> Result<Self> {
        Ok(Self::from(parse_type_path(value)?))
    }
}

fn segments_to_type_path(segments: Vec<PathSegment>) -> TypePath {
    let mut punctuated = Punctuated::new();
    for segment in segments {
        punctuated.push(segment);
    }
    let path = Path {
        segments: punctuated,
        leading_colon: None,
    };
    TypePath { path, qself: None }
}

fn idents_to_type_path(idents: Vec<Ident>) -> TypePath {
    segments_to_type_path(idents.into_iter().map(PathSegment::from).collect())
}

fn optional_idents_to_type_path(idents: Vec<Option<&Ident>>) -> TypePath {
    let idents: Vec<Ident> = idents.into_iter().filter_map(|id| id.map(Ident::clone)).collect();
    idents_to_type_path(idents)
}

pub fn parse_type_path(text: &str) -> Result<TypePath> {
    Ok(syn::parse_str::<TypePath>(text)?)
}

fn tp_vec() -> TypePath {
    parse_type_path("Vec").unwrap()
    // compatible with current code
    // probably switch to fully qualified later
    // parse_type_path("std::vec::Vec").unwrap()
}

fn tp_option() -> TypePath {
    parse_type_path("Option").unwrap()
    // compatible with current code
    // parse_type_path("std::option::Option").unwrap()
    // probably switch to fully qualified later
}

fn tp_json_value() -> TypePath {
    parse_type_path("serde_json::Value").unwrap()
}

fn tp_into() -> TypePath {
    parse_type_path("Into").unwrap() // impl std::convert::Into
}

fn tp_bytes() -> TypePath {
    parse_type_path("bytes::Bytes").unwrap()
}

fn tp_i32() -> TypePath {
    parse_type_path("i32").unwrap() // std::i32
}

fn tp_i64() -> TypePath {
    parse_type_path("i64").unwrap()
}

fn tp_f32() -> TypePath {
    parse_type_path("f32").unwrap()
}

fn tp_f64() -> TypePath {
    parse_type_path("f64").unwrap()
}

fn tp_bool() -> TypePath {
    parse_type_path("bool").unwrap()
}

fn tp_string() -> TypePath {
    parse_type_path("String").unwrap() // std::string::String
}

fn tp_box() -> TypePath {
    parse_type_path("Box").unwrap() // std::boxed::Box
}

fn tp_date_time() -> TypePath {
    parse_type_path("time::OffsetDateTime").unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_query_params() -> Result<()> {
        let names = parse_query_params("/?restype=service&comp=userdelegationkey")?;
        assert_eq!(2, names.len());
        assert!(names.contains("restype"));
        assert!(names.contains("comp"));
        Ok(())
    }

    #[test]
    fn test_parse_query_params_no_slash() -> Result<()> {
        let names = parse_query_params("?overload=EventGridEvent")?;
        assert_eq!(1, names.len());
        assert!(names.contains("overload"));
        Ok(())
    }

    #[test]
    fn test_parse_path_params_keyvault() -> Result<()> {
        assert_eq!(
            parse_path_params("/storage/{storage-account-name}/sas/{sas-definition-name}"),
            vec!["storage-account-name".to_string(), "sas-definition-name".to_string()]
        );
        Ok(())
    }

    #[test]
    fn test_type_path_code() -> Result<()> {
        let tp = TypeNameCode::try_from("farm::Goat")?;
        assert_eq!("farm :: Goat", tp.to_string());
        Ok(())
    }

    #[test]
    fn test_type_path_code_vec() -> Result<()> {
        let mut tp = TypeNameCode::try_from("farm::Goat")?.incr_vec_count();
        assert_eq!("Vec < farm :: Goat >", tp.to_string());
        tp = tp.incr_vec_count();
        assert_eq!("Vec < Vec < farm :: Goat > >", tp.to_string());
        Ok(())
    }

    #[test]
    fn test_type_path_code_option() -> Result<()> {
        let tp = TypeNameCode::try_from("farm::Goat")?.optional(true);
        assert_eq!("Option < farm :: Goat >", tp.to_string());
        Ok(())
    }

    #[test]
    fn test_tp_vec() -> Result<()> {
        let tp = tp_vec();
        assert_eq!("Vec", tp.into_token_stream().to_string());
        Ok(())
    }

    #[test]
    fn test_tp_into() -> Result<()> {
        let tp = tp_into();
        assert_eq!("Into", tp.into_token_stream().to_string());
        Ok(())
    }

    #[test]
    fn test_with_add_into() -> Result<()> {
        let tp = TypeNameCode::try_from("farm::Goat")?.impl_into(true);
        assert_eq!("impl Into < farm :: Goat >", tp.to_string());
        Ok(())
    }

    #[test]
    fn test_tp_string() -> Result<()> {
        let mut tp = TypeNameCode::from(tp_string());
        tp.type_name = Some(TypeName::String);
        assert!(tp.is_string());
        Ok(())
    }

    #[test]
    fn test_disallow_impl_into() -> Result<()> {
        let mut tp = type_name_gen(&TypeName::Int32)?;
        tp = tp.impl_into(true);
        assert!(!tp.has_impl_into());
        assert_eq!("i32", tp.to_string());
        Ok(())
    }
}
