// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! System monitoring for CPU, memory, and Azure VM metadata.
//!
//! This module provides process-wide singletons for:
//! - CPU and memory usage monitoring with historical snapshots
//! - Azure VM metadata from the Instance Metadata Service (IMDS)

mod cpu_memory;
mod vm_metadata;

pub use cpu_memory::{CpuLoad, CpuMemoryHistory, CpuMemoryMonitor, CpuUsage, MemoryUsage};
pub use vm_metadata::{AzureVmMetadata, VmMetadataService};
