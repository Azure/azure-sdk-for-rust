// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Handle types exposed across the FFI boundary.
//!
//! Each submodule wraps one driver type as an opaque, heap-allocated handle
//! with `cosmos_*_create` / `cosmos_*_free` lifecycle functions. Most modules
//! are stubs in the current skeleton — see `docs/NATIVE_WRAPPER_SPEC.md` for
//! the phased implementation plan.

pub mod account;
pub mod diagnostics;
pub mod driver;
pub mod operation;
pub mod partition_key;
pub mod references;
pub mod response;
