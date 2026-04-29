// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZero;

/// Default partition size for partitioned downloads (4 MiB).
// unwrap evaluated at compile time
pub(crate) const DEFAULT_DOWNLOAD_PARTITION_SIZE: NonZero<usize> =
    NonZero::new(4 * 1024 * 1024).unwrap();

/// Default partition size for partitioned uploads (4 MiB).
// unwrap evaluated at compile time
pub(crate) const DEFAULT_UPLOAD_PARTITION_SIZE: NonZero<u64> =
    NonZero::new(4 * 1024 * 1024).unwrap();

/// Returns the default concurrency for partitioned uploads and downloads.
///
/// Formula matches the .NET Azure Storage SDK:
/// `min(max(available_parallelism * 2, 8), 32)`
pub(crate) fn default_concurrency() -> NonZero<usize> {
    let cpus = std::thread::available_parallelism()
        .map(NonZero::get)
        .unwrap_or(1);
    let n = cpus.saturating_mul(2).clamp(8, 32);
    // SAFETY: clamp lower-bound is 8, always non-zero.
    NonZero::new(n).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_concurrency_is_within_bounds() {
        let c = default_concurrency().get();
        assert!(c >= 8, "concurrency {c} should be at least 8");
        assert!(c <= 32, "concurrency {c} should be at most 32");
    }

    #[test]
    fn default_concurrency_is_even() {
        // CPUs * 2 is always even (or clamped to 8 which is even)
        let c = default_concurrency().get();
        assert_eq!(c % 2, 0, "concurrency {c} should be even");
    }
}
