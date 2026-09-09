# azure_security_keyvault_test

- **Description**: Common utilities for Key Vault tests
- **Edition**: 2021
- **Rust version**: 1.88

## Features

- `default`

```rust
#[derive(Debug)]
pub struct Retry(/* private fields */);
impl Retry {
    fn duration(&self) -> Option<Duration>;
    fn immediate() -> Self;
    async fn next(&mut self) -> Option<()>;
    fn progressive(timeout: Option<Duration>) -> Self;
}
pub mod policies {
    #[derive(Debug)]
    pub struct GetLatestResource {
        pub collection: &'static str,
        pub name: &'static str,
        pub version: &'static str,
    }
    impl Policy for GetLatestResource {
        #[allow(elided_named_lifetimes, clippy::async_yields_async, clippy::diverging_sub_expression, clippy::let_unit_value, clippy::needless_arbitrary_self_type, clippy::no_effect_underscore_binding, clippy::shadow_same, clippy::type_complexity, clippy::type_repetition_in_bounds, clippy::used_underscore_binding)]
        fn send(&self, ctx: &Context<'_>, request: &mut Request, next: &[Arc<dyn Policy>]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = PolicyResult> + ::core::marker::Send>>;
    }
}
```
