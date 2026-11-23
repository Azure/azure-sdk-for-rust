// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{cosmos_management_options_init, management_headers, CosmosManagementOptions};
use std::{mem::MaybeUninit, ptr};

fn initialized<T>(init: extern "C" fn(*mut T)) -> T {
    let mut value = MaybeUninit::uninit();
    init(value.as_mut_ptr());
    // SAFETY: typed initializer wrote the complete record.
    unsafe { value.assume_init() }
}

#[test]
fn management_throughput_validates_records_conflicts_and_policy_presence() {
    let mut input: CosmosManagementOptions = initialized(cosmos_management_options_init);
    // SAFETY: every input below is live, aligned, and fully initialized.
    unsafe {
        assert!(management_headers(ptr::null()).unwrap().is_none());
        assert!(management_headers(&input).unwrap().is_none());
        input.throughput_kind = 1;
        input.manual_throughput = u64::MAX;
        assert_eq!(
            management_headers(&input)
                .unwrap()
                .unwrap()
                .offer_throughput,
            Some(u64::MAX)
        );
        input.autoscale_max_throughput = 4000;
        assert!(management_headers(&input).is_err());
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
            let headers = management_headers(&input).unwrap().unwrap();
            assert_eq!(
                serde_json::to_value(headers.offer_autopilot_settings.unwrap()).unwrap(),
                expected
            );
        }
        input.increment_percent = u32::MAX;
        assert_eq!(
            management_headers(&input)
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
        assert!(management_headers(&input).is_err());
        input.has_auto_upgrade_policy = 1;
        input.has_throughput_policy = 2;
        assert!(management_headers(&input).is_err());
        input.has_throughput_policy = 0;
        assert!(management_headers(&input).is_err());
        cosmos_management_options_init(&mut input);
        assert!(management_headers(&input).unwrap().is_none());
        input.throughput_kind = 3;
        assert!(management_headers(&input).is_err());
        input.throughput_kind = 1;
        assert!(management_headers(&input).is_err());
        input.throughput_kind = 2;
        assert!(management_headers(&input).is_err());
        cosmos_management_options_init(&mut input);
        input.abi_version = 2;
        assert!(management_headers(&input).is_err());
        input.abi_version = 1;
        input.struct_size_bytes -= 1;
        assert!(management_headers(&input).is_err());
        let misaligned = (&input as *const CosmosManagementOptions)
            .cast::<u8>()
            .add(1)
            .cast();
        assert!(management_headers(misaligned).is_err());
        cosmos_management_options_init(&mut input);
        #[repr(C)]
        struct Extended {
            options: CosmosManagementOptions,
            extension: u64,
        }
        let mut extended = Extended {
            options: input,
            extension: 0,
        };
        extended.options.struct_size_bytes = std::mem::size_of::<Extended>() as u32;
        assert!(management_headers(&extended.options).unwrap().is_none());
        extended.extension = 1;
        assert!(management_headers(&extended.options).is_err());
    }
}
