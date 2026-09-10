# azure_canary_core

- **Description**: Canary crate for Azure SDK pipeline testing
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]
pub use azure_canary_core::numeric::NumericCore;
pub fn add(left: u64, right: u64) -> u64;
#[cfg_attr(docsrs, feature(doc_cfg))]
pub mod numeric {
    pub trait NumericCore {
        fn is_valid(&self) -> bool;
        fn to_string(&self) -> String;
    }
}
```
