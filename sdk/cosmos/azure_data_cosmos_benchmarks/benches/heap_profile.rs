// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Heap allocation trace for a single warm `execute_operation` call.
//!
//! Uses the `dhat` heap profiler to capture every allocation that occurs
//! during exactly one point-read iteration with all caches pre-warmed.
//!
//! # Usage
//!
//! ```text
//! cargo bench -p azure_data_cosmos_benchmarks --bench heap_profile
//! ```
//!
//! Output: `dhat-heap.json` in the current working directory.
//! Load it at <https://nnethercote.github.io/dh_view/dh_view.html> to explore
//! the allocation call graph.
//!
//! # Isolation strategy
//!
//! 1. Tokio runtime creation, driver construction, and cache priming all happen
//!    **before** `dhat::Profiler::new_heap()` so their allocations are excluded.
//! 2. Five warmup iterations run outside the profiled region to ensure all
//!    lazily-initialized state (session token cache, thread-local pools, etc.)
//!    is fully populated before measurement starts.
//! 3. The profiler guard is created immediately before one `execute_operation`
//!    call and dropped immediately after, producing a clean single-iteration
//!    allocation snapshot.

#[path = "common.rs"]
mod common;

use azure_data_cosmos_driver::{models::CosmosOperation, options::OperationOptions};
use tokio::runtime::Builder;

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    let rt = Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("failed to create tokio runtime");

    // Setup and cache priming — outside the profiled region.
    let (driver, item_ref) = rt.block_on(common::setup(std::time::Duration::ZERO));

    // Warmup iterations — ensure all lazy state is initialised before profiling.
    for _ in 0..5 {
        rt.block_on(async {
            driver
                .execute_operation(
                    CosmosOperation::read_item(item_ref.clone()),
                    OperationOptions::default(),
                )
                .await
                .expect("warmup iteration failed")
        });
    }

    // Single profiled iteration.
    // The profiler guard is created here and dropped at the end of the block,
    // which writes `dhat-heap.json` to the current working directory.
    {
        let _profiler = dhat::Profiler::new_heap();
        rt.block_on(async {
            driver
                .execute_operation(
                    CosmosOperation::read_item(item_ref.clone()),
                    OperationOptions::default(),
                )
                .await
                .expect("profiled iteration failed")
        });
    }
}
