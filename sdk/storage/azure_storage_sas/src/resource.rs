// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resource types that can be protected by a SAS token.

pub mod blob;

mod queue;

pub(crate) use queue::{queue_udk_query_parameters, queue_udk_string_to_sign};
pub use queue::{QueuePermissions, QueueResource};
