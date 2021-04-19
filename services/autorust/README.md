# AutoRust [![Software License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)

A command line app similar to [AutoRest](https://github.com/azure/autorest), but is written in Rust to generate Rust code. The goal is to be able to generate code from the documents in [Azure/azure-rest-api-specs/specification](https://github.com/Azure/azure-rest-api-specs/tree/master/specification). If we can figure out the Language Server Protocol for AutoRest, we hope to compile an AutoRest Extension as well.

## Buliding

By default, building the project is very straight forward:
```sh
cargo build
```

### Formatting

The generated code is not formatted. To format the generated code, run [cargo fmt](https://github.com/rust-lang/rustfmt#usage).

## Running
The command line args are a subset of those supported by `autorest`.

``` sh
cargo run -- --help
cargo run -- --input-file=../azure-rest-api-specs/specification/vmware/resource-manager/Microsoft.AVS/stable/2020-03-20/vmware.json
cargo run -- --input-file ../OpenAPI-Specification/examples/v2.0/json/petstore.json
```

## Status

It is early days. The generated code is not finished. No binaries have been published. You will probably get panics trying out other specs. I've posted some status videos to an [AutoRust YouTube playlist](https://www.youtube.com/playlist?list=PL6MfGfZ-qCMq1mYjzTdGhKOHfrMFZjjW_). TODO items are starting to be annotated with the GitHub issue numbers using [GitHub Pull Requests and Issues](https://marketplace.visualstudio.com/items?itemName=GitHub.vscode-pull-request-github) extension for VS Code.