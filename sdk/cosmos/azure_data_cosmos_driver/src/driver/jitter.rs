// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

const JITTER_STEP: u64 = 0x9E37_79B9_7F4A_7C15;

/// Global state for lightweight non-crypto jitter generation.
static JITTER_STATE: AtomicU64 = AtomicU64::new(0xA076_1D64_78BD_642F);

/// Applies symmetric jitter to a delay value.
///
/// `ratio` is clamped to `[0.0, 1.0]` and defines the jitter band around
/// `delay` (for example `ratio = 0.25` yields `delay * [0.75, 1.25]`).
pub(crate) fn with_jitter(delay: f64, ratio: f64) -> f64 {
    let jitter_ratio = ratio.clamp(0.0, 1.0);
    if jitter_ratio == 0.0 || delay == 0.0 {
        return delay;
    }

    let nanos_since_epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);

    // Mix monotonic state with wall-clock time via SplitMix64 to produce a
    // cheap, non-cryptographic pseudo-random value in [0.0, 1.0].
    let seed = JITTER_STATE.fetch_add(JITTER_STEP, Ordering::Relaxed) ^ nanos_since_epoch;
    let unit = splitmix64_unit(seed);
    let jitter_multiplier = (1.0 - jitter_ratio) + (2.0 * jitter_ratio * unit);
    delay * jitter_multiplier
}

fn splitmix64_unit(mut x: u64) -> f64 {
    // SplitMix64 mixing: quickly reduces correlation in nearby seeds into a well-distributed
    // 64-bit value suitable for non-cryptographic jitter generation.
    //
    // We then normalize to [0.0, 1.0] so callers can scale jitter bands directly.
    x = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^= x >> 31;
    (x as f64) / (u64::MAX as f64)
}
