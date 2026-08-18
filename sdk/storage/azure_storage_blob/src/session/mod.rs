// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Session token authentication support for blob operations.
//!
//! Session authentication lets eligible OAuth-authenticated GET blob download
//! requests be authorized with a short-lived, container-scoped session token
//! instead of a bearer token, falling back to bearer authentication when a
//! session cannot be used. All types in this module are internal; the customer
//! only ever supplies a [`TokenCredential`](azure_core::credentials::TokenCredential).

pub(crate) mod cache;
pub(crate) mod provider;
pub(crate) mod signer;
