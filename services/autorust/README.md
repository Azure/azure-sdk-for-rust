# AutoRust [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

A code generator similar to [AutoRest](https://github.com/azure/autorest), but is written in Rust to generate Rust code. It supports specifications found in [Azure/azure-rest-api-specs/specification](https://github.com/Azure/azure-rest-api-specs/tree/master/specification).

## Building

```sh
cargo build
```

### Formatting

The generated code is not formatted. To format the generated code, run [cargo fmt](https://github.com/rust-lang/rustfmt#usage).
