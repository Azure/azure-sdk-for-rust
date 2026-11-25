// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{throughput_headers, CosmosThroughputOptions};
use std::{mem::size_of, ptr};

fn unset() -> CosmosThroughputOptions {
    CosmosThroughputOptions {
        struct_size_bytes: size_of::<CosmosThroughputOptions>() as u32,
        abi_version: 1,
        throughput_kind: 0,
        manual_throughput: 0,
        autoscale_max_throughput: 0,
        has_auto_upgrade_policy: 0,
        has_throughput_policy: 0,
        increment_percent: 0,
    }
}

#[test]
fn throughput_validates_records_conflicts_and_policy_presence() {
    let mut input = unset();
    // SAFETY: every input below is live, aligned, and fully initialized.
    unsafe {
        assert!(throughput_headers(ptr::null()).unwrap().is_none());
        assert!(throughput_headers(&input).unwrap().is_none());
        input.throughput_kind = 1;
        input.manual_throughput = u64::MAX;
        assert_eq!(
            throughput_headers(&input)
                .unwrap()
                .unwrap()
                .offer_throughput,
            Some(u64::MAX)
        );
        input.autoscale_max_throughput = 4000;
        assert!(throughput_headers(&input).is_err());
        input.manual_throughput = 0;
        input.throughput_kind = 2;
        for (parent, child, expected) in [
            (0, 0, serde_json::json!({"maxThroughput":4000})),
            (
                1,
                0,
                serde_json::json!({"maxThroughput":4000,"autoUpgradePolicy":{}}),
            ),
            (
                1,
                1,
                serde_json::json!({"maxThroughput":4000,"autoUpgradePolicy":{"throughputPolicy":{"incrementPercent":0}}}),
            ),
        ] {
            input.has_auto_upgrade_policy = parent;
            input.has_throughput_policy = child;
            let headers = throughput_headers(&input).unwrap().unwrap();
            assert_eq!(
                serde_json::to_value(headers.offer_autopilot_settings.unwrap()).unwrap(),
                expected
            );
        }
        input.increment_percent = u32::MAX;
        assert_eq!(
            throughput_headers(&input)
                .unwrap()
                .unwrap()
                .offer_autopilot_settings
                .unwrap()
                .auto_upgrade_policy
                .unwrap()
                .throughput_policy
                .unwrap()
                .increment_percent,
            u32::MAX
        );
        input.has_auto_upgrade_policy = 0;
        assert!(throughput_headers(&input).is_err());
        input.has_auto_upgrade_policy = 1;
        input.has_throughput_policy = 2;
        assert!(throughput_headers(&input).is_err());
        input.has_throughput_policy = 0;
        assert!(throughput_headers(&input).is_err());
        input = unset();
        assert!(throughput_headers(&input).unwrap().is_none());
        input.throughput_kind = 3;
        assert!(throughput_headers(&input).is_err());
        input.throughput_kind = 1;
        assert!(throughput_headers(&input).is_err());
        input.throughput_kind = 2;
        assert!(throughput_headers(&input).is_err());
        input = unset();
        for version in [0, 2] {
            input.abi_version = version;
            assert!(throughput_headers(&input).is_err());
        }
        input.abi_version = 1;
        for size in [0, size_of::<CosmosThroughputOptions>() as u32 - 1] {
            input.struct_size_bytes = size;
            assert!(throughput_headers(&input).is_err());
        }
        let misaligned = (&input as *const CosmosThroughputOptions)
            .cast::<u8>()
            .add(1)
            .cast();
        assert!(throughput_headers(misaligned).is_err());
        input = unset();
        #[repr(C)]
        struct Extended {
            options: CosmosThroughputOptions,
            extension: u64,
        }
        let mut extended = Extended {
            options: input,
            extension: 0,
        };
        extended.options.struct_size_bytes = std::mem::size_of::<Extended>() as u32;
        assert!(throughput_headers(&extended.options).unwrap().is_none());
        extended.extension = 1;
        assert!(throughput_headers(&extended.options).is_err());
    }
}
