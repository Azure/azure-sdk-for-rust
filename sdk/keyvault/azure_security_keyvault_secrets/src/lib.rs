// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

mod authorizer;
pub mod clients;
mod generated;
mod resource;

pub use clients::{SecretClient, SecretClientOptions};
pub use generated::*;
pub use resource::*;
