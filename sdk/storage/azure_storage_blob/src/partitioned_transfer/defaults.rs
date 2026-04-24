// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Default values for `parallel` and `partition_size` on blob upload and download.
//!
//! These match the Azure Storage SDK for .NET (`Azure.Storage.Blobs`):
//! - Concurrency: `min(max(CPUs * 2, 8), 32)`
//! - Upload partition size: 4 MiB when content is under 100 MiB, otherwise 8 MiB
//! - Download partition size: 4 MiB
//! - Service caps: 4000 MiB per uploaded block, 256 MiB per download range, 50 000 blocks per block blob

use std::num::NonZero;

pub(crate) const DEFAULT_BUFFER_SIZE: u64 = 4 * 1024 * 1024;
pub(crate) const LARGE_BUFFER_SIZE: u64 = 8 * 1024 * 1024;
pub(crate) const LARGE_UPLOAD_THRESHOLD: u64 = 100 * 1024 * 1024;
pub(crate) const MAX_STAGE_BYTES: u64 = 4_000 * 1024 * 1024;
pub(crate) const MAX_DOWNLOAD_BYTES: usize = 256 * 1024 * 1024;
pub(crate) const MAX_BLOCKS: u64 = 50_000;

pub(crate) fn default_concurrency() -> NonZero<usize> {
    let cpus = std::thread::available_parallelism()
        .map(NonZero::get)
        .unwrap_or(1);
    let n = cpus.saturating_mul(2).clamp(8, 32);
    NonZero::new(n).unwrap()
}

pub(crate) fn default_upload_partition_size(content_len: Option<u64>) -> NonZero<u64> {
    let n = match content_len {
        Some(len) if len >= LARGE_UPLOAD_THRESHOLD => LARGE_BUFFER_SIZE,
        _ => DEFAULT_BUFFER_SIZE,
    };
    NonZero::new(n).unwrap()
}

pub(crate) fn default_download_partition_size() -> NonZero<usize> {
    NonZero::new(DEFAULT_BUFFER_SIZE as usize).unwrap()
}

pub(crate) fn clamp_upload_partition_size(p: NonZero<u64>) -> NonZero<u64> {
    NonZero::new(p.get().min(MAX_STAGE_BYTES)).unwrap()
}

pub(crate) fn clamp_download_partition_size(p: NonZero<usize>) -> NonZero<usize> {
    NonZero::new(p.get().min(MAX_DOWNLOAD_BYTES)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concurrency_is_within_8_to_32() {
        let c = default_concurrency().get();
        assert!((8..=32).contains(&c), "expected 8..=32, got {c}");
    }

    #[test]
    fn upload_partition_unknown_length_is_4_mib() {
        assert_eq!(default_upload_partition_size(None).get(), DEFAULT_BUFFER_SIZE);
    }

    #[test]
    fn upload_partition_below_threshold_is_4_mib() {
        let below = LARGE_UPLOAD_THRESHOLD - 1;
        assert_eq!(
            default_upload_partition_size(Some(below)).get(),
            DEFAULT_BUFFER_SIZE,
        );
    }

    #[test]
    fn upload_partition_at_threshold_is_8_mib() {
        assert_eq!(
            default_upload_partition_size(Some(LARGE_UPLOAD_THRESHOLD)).get(),
            LARGE_BUFFER_SIZE,
        );
    }

    #[test]
    fn upload_partition_well_above_threshold_is_8_mib() {
        let large = 1024 * 1024 * 1024;
        assert_eq!(
            default_upload_partition_size(Some(large)).get(),
            LARGE_BUFFER_SIZE,
        );
    }

    #[test]
    fn download_partition_default_is_4_mib() {
        assert_eq!(
            default_download_partition_size().get() as u64,
            DEFAULT_BUFFER_SIZE,
        );
    }

    #[test]
    fn clamp_upload_passes_through_under_max() {
        let under = NonZero::new(LARGE_BUFFER_SIZE).unwrap();
        assert_eq!(clamp_upload_partition_size(under), under);
    }

    #[test]
    fn clamp_upload_caps_at_max() {
        let over = NonZero::new(MAX_STAGE_BYTES + 1).unwrap();
        assert_eq!(clamp_upload_partition_size(over).get(), MAX_STAGE_BYTES);
    }

    #[test]
    fn clamp_download_passes_through_under_max() {
        let under = NonZero::new(DEFAULT_BUFFER_SIZE as usize).unwrap();
        assert_eq!(clamp_download_partition_size(under), under);
    }

    #[test]
    fn clamp_download_caps_at_max() {
        let over = NonZero::new(MAX_DOWNLOAD_BYTES + 1).unwrap();
        assert_eq!(clamp_download_partition_size(over).get(), MAX_DOWNLOAD_BYTES);
    }
}
