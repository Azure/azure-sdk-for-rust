// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core_test::perf::{PerfTestOption, PerfTestOptionKind};

pub fn endpoint() -> PerfTestOption {
    PerfTestOption {
        name: "endpoint",
        display_message: "The endpoint of the blob storage",
        mandatory: false,
        short_activator: Some('e'),
        long_activator: "endpoint",
        expected_args_len: 1,
        ..Default::default()
    }
}

pub fn size() -> PerfTestOption {
    PerfTestOption {
        name: "size",
        display_message: "The size of each blob in bytes",
        mandatory: true,
        short_activator: Some('s'),
        long_activator: "size",
        expected_args_len: 1,
        option_type: PerfTestOptionKind::Usize,
        ..Default::default()
    }
}

pub fn count() -> PerfTestOption {
    PerfTestOption {
        name: "count",
        display_message: "The number of blobs",
        mandatory: false,
        short_activator: Some('c'),
        long_activator: "count",
        expected_args_len: 1,
        option_type: PerfTestOptionKind::Uint32,
        ..Default::default()
    }
}

pub fn collect() -> PerfTestOption {
    PerfTestOption {
        name: "collect",
        display_message: "Collect the blob contents instead of streaming them",
        mandatory: false,
        short_activator: None,
        long_activator: "collect",
        expected_args_len: 1,
        option_type: PerfTestOptionKind::String,
        ..Default::default()
    }
}

pub fn concurrency() -> PerfTestOption {
    PerfTestOption {
        name: "concurrency",
        display_message: "Number of concurrent network transfers",
        mandatory: false,
        long_activator: "concurrency",
        expected_args_len: 1,
        option_type: PerfTestOptionKind::Usize,
        ..Default::default()
    }
}

pub fn partition_size() -> PerfTestOption {
    PerfTestOption {
        name: "partition-size",
        display_message: "Size in bytes to partition data into for each transfer",
        mandatory: false,
        long_activator: "partition-size",
        expected_args_len: 1,
        option_type: PerfTestOptionKind::Usize,
        ..Default::default()
    }
}
