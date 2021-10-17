# autorust_openapi [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

Rust crate for deserializing [OpenAPI](http://swagger.io/specification/) documents as needed by [autorust](https://github.com/ctaggart/autorust/), an AutoRest extension. The goal is to be able to deserialize all of the documents found in [Azure/azure-rest-api-specs/specification](https://github.com/Azure/azure-rest-api-specs/tree/master/specification). They follow [OpenAPI Specification Version 2.0](https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#parameter-object) and use several [extensions](https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md).

## Install

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
autorust_openapi = { git = "https://github.com/ctaggart/autorust_openapi" }
```

## Similar Crates

- This is a fork of the [openapi crate](https://crates.io/crates/openapi), maintained at [softprops/openapi](https://github.com/softprops/openapi). It was created by Doug Tangren (softprops) in 2017.
- The [openapiv3 crate](https://github.com/glademiller/openapiv3) was created by Glade Miller in 2019 and is maintained at [glademiller/openapiv3](https://github.com/glademiller/openapiv3).
