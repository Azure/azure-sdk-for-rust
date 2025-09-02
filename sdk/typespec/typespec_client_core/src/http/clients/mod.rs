// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Built-in HTTP clients.

mod noop;
#[cfg(all(
    feature = "reqwest",
    not(all(target_arch = "wasm32", target_os = "wasi"))
))]
mod reqwest;
#[cfg(all(feature = "spin", target_arch = "wasm32", target_os = "wasi"))]
mod spin;

#[cfg(not(any(
    all(feature = "spin", target_arch = "wasm32", target_os = "wasi"),
    all(feature = "reqwest", not(all(target_arch = "wasm32", target_os = "wasi")))
)))]
use self::noop::new_noop_client;
#[cfg(all(
    feature = "reqwest",
    not(all(target_arch = "wasm32", target_os = "wasi"))
))]
use self::reqwest::new_reqwest_client;
#[cfg(all(feature = "spin", target_arch = "wasm32", target_os = "wasi"))]
use self::spin::new_spin_client;

use crate::http::{RawResponse, Request};
use cfg_if::cfg_if;
use async_trait::async_trait;
use std::sync::Arc;
use typespec::error::Result;

/// Create a new [`HttpClient`].
pub fn new_http_client() -> Arc<dyn HttpClient> {
    cfg_if! {
        if #[cfg(all(feature = "spin", target_arch = "wasm32", target_os = "wasi"))] {
            new_spin_client()
        } else if #[cfg(all(feature = "reqwest", not(all(target_arch = "wasm32", target_os = "wasi"))))] {
            new_reqwest_client()
        } else {
            new_noop_client()
        }
    }
}

/// An HTTP client which can send requests.
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
pub trait HttpClient: Send + Sync + std::fmt::Debug {
    /// Send a request to the service.
    ///
    /// It does not consume the request. Implementors are expected to clone the necessary parts
    /// of the request and pass them to the underlying transport.
    async fn execute_request(&self, request: &Request) -> Result<RawResponse>;
}
