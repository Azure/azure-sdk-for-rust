// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Initialization of a shared change feed workload.
//!
//! Bootstrap creates unowned range leases without reading change feed pages.
//! Processing and checkpoint persistence are implemented separately.

mod bootstrap;

pub use bootstrap::bootstrap;
pub(crate) use bootstrap::ChangeFeedProcessorBootstrapCoalescer;
