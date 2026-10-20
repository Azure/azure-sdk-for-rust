# typespec_macros

- **Description**: Procedural macros for client libraries built on typespec.
- **Edition**: 2021

## Features

```rust
#![crate_name = "typespec_macros"]
#![crate_type = "proc-macro"]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
#[proc_macro_derive(SafeDebug, attributes(safe))]
#[derive(SafeDebug)] {
    // Attributes available to this derive:
    #[safe]
}
```
