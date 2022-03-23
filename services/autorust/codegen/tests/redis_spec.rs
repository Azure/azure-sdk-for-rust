// cargo test --test redis_specs
// These tests require cloning azure-rest-api-specs.
// git clone git@github.com:Azure/azure-rest-api-specs.git ../azure-rest-api-specs

use autorust_codegen::{
    spec::{self, TypedReference},
    Spec,
};
use camino::{Utf8PathBuf, Utf8Path};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const REDIS_SPEC: &str =
    "../../../../azure-rest-api-specs/specification/redis/resource-manager/Microsoft.Cache/stable/2020-06-01/redis.json";
const LINKS_SPEC: &str = "../../../../azure-rest-api-specs/specification/common-types/resource-management/v1/privatelinks.json";

#[test]
fn test_redis_ref_files() -> Result<()> {
    let doc_file = REDIS_SPEC;
    let api = &spec::openapi::parse(doc_file)?;
    let files = spec::openapi::get_reference_file_paths(doc_file, api);
    println!("{:#?}", files);
    assert_eq!(2, files.len());
    assert!(files.contains("../../../../../common-types/resource-management/v2/types.json"));
    Ok(())
}

#[test]
fn test_redis_read_spec() -> Result<()> {
    let spec = &Spec::read_files(&[REDIS_SPEC])?;
    println!("{:#?}", spec.docs().keys());
    assert_eq!(4, spec.docs().len());
    assert!(spec.docs().contains_key(Utf8Path::new(
        "../../../../azure-rest-api-specs/specification/common-types/resource-management/v2/types.json"
    )));
    Ok(())
}

#[test]
fn test_links_ref_files() -> Result<()> {
    let doc_file = LINKS_SPEC;
    let api = &spec::openapi::parse(doc_file)?;
    let files = spec::openapi::get_reference_file_paths(doc_file, api);
    println!("{:#?}", files);
    assert_eq!(1, files.len());
    assert!(files.contains("./types.json"));
    Ok(())
}

#[test]
fn test_links_refs_count() -> Result<()> {
    let doc_file = LINKS_SPEC;
    let api = &spec::openapi::parse(doc_file)?;
    let refs = spec::openapi::get_references(doc_file, api);
    assert_eq!(10, refs.len());
    Ok(())
}

#[test]
fn test_redis_resolve_all_refs() -> Result<()> {
    let doc_file = Utf8PathBuf::from(REDIS_SPEC);
    let spec = &Spec::read_files(&[&doc_file])?;
    for (doc_file, api) in spec.docs() {
        let refs = spec::openapi::get_references(doc_file, api);
        for rs in refs {
            match rs {
                TypedReference::PathItem(_) => {}
                TypedReference::Example(_) => {}
                TypedReference::Parameter(reference) => {
                    spec.resolve_parameter_ref(&doc_file, reference)?;
                }
                TypedReference::Schema(reference) => {
                    spec.resolve_schema_ref(&doc_file, &reference)?;
                }
            }
        }
    }
    Ok(())
}
