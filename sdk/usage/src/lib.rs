/*!
This crate is from the [Azure SDK for Rust](https://github.com/azure/azure-sdk-for-rust).
It supports [Azure Usage Billing](https://review.learn.microsoft.com/en-us/azure-usage-billing).

# Example
```no_run

// TODO: example

```
*/

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub mod prelude;

mod clients;
mod models;
mod operations;
