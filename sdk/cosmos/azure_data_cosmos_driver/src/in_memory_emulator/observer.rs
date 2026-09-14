// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Request observation hook for the in-memory emulator.

use std::fmt::Debug;

use azure_core::http::Request;

/// A side-channel observer notified for every HTTP request that reaches
/// [`InMemoryEmulatorHttpClient`](super::InMemoryEmulatorHttpClient).
///
/// Implementations are intended for tests that need to assert on the shape
/// of outgoing requests (URL, method, headers) without altering emulator
/// behavior. Observers are invoked **before** the emulator parses or
/// dispatches the request, and they must not mutate the request or the
/// emulator state in any way that affects request handling — implementations
/// must be cheap and side-effect-free relative to the emulator pipeline.
///
/// Attach an observer with
/// [`InMemoryEmulatorHttpClient::with_request_observer`](super::InMemoryEmulatorHttpClient::with_request_observer).
/// When no observer is attached the dispatch path pays no overhead.
pub trait RequestObserver: Debug + Send + Sync {
    /// Called once per request, before the emulator routes or handles it.
    fn on_request(&self, request: &Request);
}
