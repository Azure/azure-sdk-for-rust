// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![recursion_limit = "128"]
#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod checkpoint_store;
pub use checkpoint_store::BlobCheckpointStore;
