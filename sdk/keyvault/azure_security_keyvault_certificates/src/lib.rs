// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

pub mod clients;
#[allow(unused_imports)]
mod generated;
pub mod models;
mod resource;

pub use clients::{CertificateClient, CertificateClientExt, CertificateClientOptions};
pub use resource::*;
