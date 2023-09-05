# 0.2.1
* public Response internals when no response type

# 0.2
* forked as autorust_openapi
* removed OpenAPI v3 support. Use openapiv3 crate
* moved v2 module to be the root module
* removed serde_yaml, url, serde_url, semver, error-chain, & failure dependencies
* switched CI from Travis CI to GitHub Actions
* removed pretty_assertions dev-dependencies
* added serde_ignored dev-dependency
* added integration tests for OpenAPI v2 specification examples
* added integration tests for azure-rest-api-specs specifications
* added --example ignored
* add `ReferenceOr<T>` from openapiv3
* replace `ParameterOrRef` with `ReferenceOr<Parameter>`
* rename `Spec` to `OpenAPI` to match openapiv3
* replace `Option<Vec<T>>` with `Vec<T>` and skip if `is_empty` like openapiv3
* add dependency on `indexmap` like openapiv3
* use `is_empty` on `IndexMap` instead of wrapping in `Option<T>` like openapiv3
* added missing Schema Object & Parameter Object fields
* added StatusCode based on openapiv3

* expose security definition as an enum type
* Adds License object
* Adds Contact object
* Derives Default for all structs
* Derives Clone for all structs
* Changes the order of the output to be more similar to OpenAPI examples
* switch to 2018 edition

# 0.1.5

* expose other schema types as public interfaces
* re-export Result and ResultExt as top level interfaces

# 0.1.4

* added operational `parameters` field to `Operations` object

# 0.1.3

* added optional `required` and `enum_values` fields to `Schema` object

# 0.1.2

* added optional `format` fields to `Parameter` object

# 0.1.1

* added optional `summary` field to `Operation` object
* made schemes and tags optional fields on `Operation` object

# 0.1.0

* initial release
