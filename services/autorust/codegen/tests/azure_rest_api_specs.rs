// cargo test --test azure_rest_api_specs
// These tests require cloning azure-rest-api-specs.
// git clone git@github.com:Azure/azure-rest-api-specs.git ../azure-rest-api-specs

use autorust_codegen::*;
use autorust_openapi::Reference;
use spec::TypedReference;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const COMMON_TYPES_SPEC: &str = "azure-rest-api-specs/specification/security/resource-manager/common/v1/types.json";
const VMWARE_SPEC: &str = "azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json";

/// Get the relative base test directory.
/// CI uses a ../, otherwise ../../
/// https://docs.github.com/en/actions/reference/environment-variables
fn base_dir(path: &str) -> String {
    if std::env::var("CI").is_ok() {
        "../../../".to_owned() + path
    } else {
        "../../../../".to_owned() + path
    }
}

#[test]
fn refs_count_security_common() -> Result<()> {
    let api = &spec::openapi::parse(&base_dir(COMMON_TYPES_SPEC))?;
    let refs = spec::openapi::get_references(api);
    assert_eq!(15, refs.len());
    Ok(())
}

#[test]
fn refs_count_avs() -> Result<()> {
    let api = &spec::openapi::parse(&base_dir(VMWARE_SPEC))?;
    let refs = spec::openapi::get_references(api);
    assert_eq!(199, refs.len());
    Ok(())
}

#[test]
fn ref_files() -> Result<()> {
    let api = &spec::openapi::parse(&base_dir(VMWARE_SPEC))?;
    let files = spec::openapi::get_reference_file_paths(api);
    assert_eq!(1, files.len());
    assert!(files.contains(&base_dir("../common-types/resource-management/v1/types.json")));
    Ok(())
}

#[test]
fn read_spec_avs() -> Result<()> {
    let spec = &Spec::read_files(&[&base_dir(VMWARE_SPEC)])?;
    assert_eq!(2, spec.docs().len());
    assert!(spec.docs().contains_key(std::path::Path::new(&base_dir(
        "azure-rest-api-specs/specification/common-types/resource-management/v1/types.json"
    ))));
    Ok(())
}

#[test]
fn test_resolve_schema_ref() -> Result<()> {
    let file = PathBuf::from(&base_dir(VMWARE_SPEC));
    let spec = &Spec::read_files(&[&file])?;
    spec.resolve_schema_ref(&file, Reference::parse("#/definitions/OperationList")?)?;
    spec.resolve_schema_ref(
        &file,
        Reference::parse(&base_dir(
            "../common-types/resource-management/v1/types.json#/definitions/ErrorResponse",
        ))?,
    )?;
    Ok(())
}

#[test]
fn test_resolve_parameter_ref() -> Result<()> {
    let file = PathBuf::from(&base_dir(VMWARE_SPEC));
    let spec = &Spec::read_files(&[&file])?;
    spec.resolve_parameter_ref(
        &file,
        Reference::parse(&base_dir(
            "../common-types/resource-management/v1/types.json#/parameters/ApiVersionParameter",
        ))?,
    )?;
    Ok(())
}

#[test]
fn test_resolve_all_refs() -> Result<()> {
    let doc_file = PathBuf::from(&base_dir(VMWARE_SPEC));
    let spec = &Spec::read_files(&[&doc_file])?;
    for (doc_file, doc) in spec.docs() {
        let refs = spec::openapi::get_references(doc);
        for rs in refs {
            match rs {
                TypedReference::PathItem(_) => {}
                TypedReference::Example(_) => {}
                TypedReference::Parameter(reference) => {
                    spec.resolve_parameter_ref(&doc_file, reference)?;
                }
                TypedReference::Schema(reference) => {
                    spec.resolve_schema_ref(&doc_file, reference)?;
                }
            }
        }
    }
    Ok(())
}
