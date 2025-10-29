// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod clients;
#[allow(
    unused_imports,
    reason = "Publicly exported generated/clients are instead exported from clients"
)]
mod generated;
pub mod models;
mod resource;

pub use clients::{CertificateClient, CertificateClientOptions};
pub use resource::*;
