// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resource types that can be protected by a SAS token.

pub mod blob;

mod queue;

pub use queue::{Queue, QueuePermissions};
