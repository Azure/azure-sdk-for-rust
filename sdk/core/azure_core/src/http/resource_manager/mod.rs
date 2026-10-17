// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types and functions for Azure Resource Manager HTTP clients.

mod poller;

pub use poller::{new_poller, FinalStateVia, OperationLocationResultCallback, PollerOptions};
