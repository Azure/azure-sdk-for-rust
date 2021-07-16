// cargo test --test openapi_spec_examples
// These tests require cloning OpenAPI-Specification.
// git clone git@github.com:OAI/OpenAPI-Specification.git ../OpenAPI-Specification

mod common;
use common::*;

const PATHS: &[&str] = &[
    "../../../../OpenAPI-Specification/examples/v2.0/json/petstore-minimal.json",
    "../../../../OpenAPI-Specification/examples/v2.0/json/petstore.json",
    "../../../../OpenAPI-Specification/examples/v2.0/json/petstore-simple.json",
    "../../../../OpenAPI-Specification/examples/v2.0/json/petstore-with-external-docs.json",
    "../../../../OpenAPI-Specification/examples/v2.0/json/uber.json",
];

#[test]
fn can_roundtrip_openapi_spec_examples() -> Result<()> {
    assert_roundtrip_eq(PATHS)
}
