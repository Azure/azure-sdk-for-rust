// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

mod tracing;
mod tracing_client;
mod tracing_function;
mod tracing_new;
mod tracing_subclient;

use proc_macro::TokenStream;

/// Attribute client struct declarations to enable distributed tracing.
///
/// To declare a client that will be traced, you should use the `#[tracing::client]` attribute
/// exported from azure_core.
///
#[proc_macro_attribute]
pub fn client(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_client::parse_client(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}

/// Attribute client struct instantiation to enable distributed tracing.
///
/// To enable tracing for a client instantiation, you should use the `#[tracing::new]` attribute
/// exported from azure_core.
///
/// This macro will automatically instrument the client instantiation with tracing information.
/// It will also ensure that the client is created with the necessary tracing context.
///
/// The `#[tracing::new]` attribute takes a single argument, which is a string
/// representing the Azure Namespace name for the service being traced.
///
/// The list of Azure Namespaces can be found [on this page](https://learn.microsoft.com/azure/azure-resource-manager/management/azure-services-resource-providers)
///
#[proc_macro_attribute]
pub fn new(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_new::parse_new(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}

/// Attribute client subclient struct declarations to enable distributed tracing.
///
/// To declare a subclient that will be traced, you should use the `#[tracing::subclient]` attribute
/// exported from azure_core.
///
/// This macro will automatically instrument the subclient declaration with tracing information. It will also ensure that the subclient is created with the necessary tracing context.
/// The `#[tracing::subclient]` attribute takes a single argument, which is a string representing the Azure Namespace name for the service being traced.
#[proc_macro_attribute]
pub fn subclient(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_subclient::parse_subclient(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}

/// Attribute client public APIs to enable distributed tracing.
///
/// To declare a public API function that will be traced, you should use the `#[tracing::function]` attribute
/// exported from azure_core.
///
/// This macro will automatically instrument the public API function with tracing information. It will also ensure that the function is executed with the necessary tracing context.
///
/// The `function` attribute takes one required argument, which is a string representing the name of the operation being traced.
/// This name will be used in the tracing spans to identify the operation being performed. The name should be unique and match the
/// typespec name for the operation being traced if possible.
///
#[proc_macro_attribute]
pub fn function(attr: TokenStream, item: TokenStream) -> TokenStream {
    tracing_function::parse_function(attr.into(), item.into())
        .map_or_else(|e| e.into_compile_error().into(), |v| v.into())
}
