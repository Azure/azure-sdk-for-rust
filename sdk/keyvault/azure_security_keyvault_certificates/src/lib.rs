// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

mod generated;
pub mod models;
mod resource;

pub use generated::{clients, CertificateClient, CertificateClientOptions};
pub use resource::*;
