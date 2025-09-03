// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod clients;
#[allow(unused_imports)]
#[expect(deprecated, reason = "requires emitter update")]
mod generated;
pub mod models;
mod resource;

pub use clients::{CertificateClient, CertificateClientOptions};
pub use resource::*;
