// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

mod tracing;
mod tracing_client;
mod tracing_function;
mod tracing_new;

use proc_macro::TokenStream;

/// Attribute client struct declarations to enable distributed tracing.
///
/// # Examples
///
///
///
/// For example, to declare a client that will be traced, you should use the `#[trace::client]` attribute.
///
/// ```
/// use azure_core::tracing;
/// use azure_core::http::Url;
/// use std::sync::Arc;
///
/// #[tracing::client]
/// pub struct MyServiceClient {
///    endpoint: Url,
/// }
/// ```
///
#[proc_macro_attribute]
pub fn client(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_client::parse_client(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}

/// Attribute client struct instantiation to enable distributed tracing.
///
/// # Examples
///
/// To declare a client that will be traced, you should use the `#[traced::client]` attribute.
/// To instantiate a client, use the `[traced::new]` which generates a distributed tracing tracer associated with the client namespace.
///
/// ```
/// use azure_core::{tracing, http::{Url, ClientOptions}};
/// use std::sync::Arc;
///
/// #[tracing::client]
/// pub struct MyServiceClient {
///    endpoint: Url,
/// }
///
/// #[derive(Default)]
/// pub struct MyServiceClientOptions {
///     pub client_options: ClientOptions,
/// }
///
/// impl MyServiceClient {
///
///     #[tracing::new("MyServiceClientNamespace")]
///     pub fn new(endpoint: &str, _credential: Arc<dyn azure_core::credentials::TokenCredential>, options: Option<MyServiceClientOptions>) -> Self {
///         let options = options.unwrap_or_default();
///         let url = Url::parse(endpoint).expect("Invalid endpoint URL");
///         Self {
///             endpoint: url,
///         }
///     }
/// }
/// ```
///
#[proc_macro_attribute]
pub fn new(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_new::parse_new(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}

/// Attribute client struct instantiation to enable distributed tracing.
///
/// # Examples
///
/// To declare a client that will be traced, you should use the `#[traced::client]` attribute.
/// To instantiate a client, use the `[traced::new]` which generates a distributed tracing tracer associated with the client namespace.
///
/// ```
/// use azure_core::{tracing, http::{Url, ClientOptions}, Result};
/// use azure_core::http::ClientMethodOptions;
/// use std::sync::Arc;
///
/// #[tracing::client]
/// pub struct MyServiceClient {
///    endpoint: Url,
/// }
///
/// #[derive(Default)]
/// pub struct MyServiceClientOptions {
///     pub client_options: ClientOptions,
/// }
///
/// #[derive(Default)]
/// pub struct MyServiceClientMethodOptions<'a> {
///     pub method_options: ClientMethodOptions<'a>,
/// }
///
/// impl MyServiceClient {
///
///     #[tracing::function("MyServiceClient.PublicFunction")]
///     pub async fn public_function(&self, param: &str,  options: Option<MyServiceClientMethodOptions<'_>>) -> Result<()> {
///         let options = options.unwrap_or_default();
///         Ok(())
///     }
/// }
/// ```
///
#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_function::parse_function(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}
