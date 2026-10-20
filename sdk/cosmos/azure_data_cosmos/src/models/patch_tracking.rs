// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZeroU16;

pub use azure_data_cosmos_driver::models::PatchTrackingId;

/// Reserved item property used to persist PATCH tracking entries.
pub const PATCH_TRACKING_PROPERTY: &str = "_azsdkPatchTracking";

/// Time PATCH tracking entries remain protected from age-based pruning.
///
/// A matching entry is honored for as long as it remains on the item, but a
/// later PATCH may prune it after this interval has elapsed or evict it earlier
/// when the marker array reaches capacity.
pub const PATCH_TRACKING_RETENTION: std::time::Duration = std::time::Duration::from_secs(5 * 60);

/// Default maximum number of PATCH tracking entries retained on one item.
/// The oldest entry is evicted when this capacity is reached.
pub const DEFAULT_PATCH_TRACKING_CAPACITY: NonZeroU16 =
    NonZeroU16::new(1024).expect("default PATCH tracking capacity is non-zero");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn protocol_constants_match_driver() {
        assert_eq!(
            PATCH_TRACKING_PROPERTY,
            azure_data_cosmos_driver::models::PATCH_TRACKING_PROPERTY
        );
        assert_eq!(
            PATCH_TRACKING_RETENTION,
            azure_data_cosmos_driver::models::PATCH_TRACKING_RETENTION
        );
        assert_eq!(
            DEFAULT_PATCH_TRACKING_CAPACITY,
            azure_data_cosmos_driver::models::DEFAULT_PATCH_TRACKING_CAPACITY
        );
    }
}
