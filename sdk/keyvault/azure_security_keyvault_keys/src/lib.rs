// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod authorizer;
pub mod clients;
#[allow(
    unused_imports,
    reason = "Publicly exported generated/clients are instead exported from clients"
)]
mod generated;
mod resource;

pub use clients::{KeyClient, KeyClientOptions};
pub use generated::models;
pub use resource::*;
