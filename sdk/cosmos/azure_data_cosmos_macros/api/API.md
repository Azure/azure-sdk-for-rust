# azure_data_cosmos_macros

- **Description**: Procedural macros for the Azure Cosmos DB SDK for Rust.
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
#![warn(missing_docs)]
#[proc_macro_derive(CosmosOptions, attributes(options, option))]
#[derive(CosmosOptions)] {
    // Attributes available to this derive:
    #[options]
    #[option]
}
```
