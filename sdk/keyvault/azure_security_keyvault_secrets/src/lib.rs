// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[expect(deprecated, reason = "requires emitter update")]
mod generated;
mod resource;

pub use generated::*;
pub use resource::*;
