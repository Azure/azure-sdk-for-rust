// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

#[expect(deprecated, reason = "requires emitter update")]
mod generated;
mod resource;

pub use generated::*;
pub use resource::*;
