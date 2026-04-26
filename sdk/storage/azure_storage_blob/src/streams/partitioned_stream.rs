// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_stream::try_stream;
use std::{cmp::min, mem, num::NonZero, slice};

use azure_core::stream::SeekableStream;
use bytes::{BufMut, Bytes, BytesMut};
use futures::{AsyncReadExt, Stream};

type Result<T> = azure_core::Result<T>;

pub(crate) const MAX_CONTIGUOUS_ELEMENTS: usize = isize::MAX as usize;

/// Converts the given AsyncRead into a Stream<Item = Bytes>, where each item is a chunk of data
/// that is exactly `partition_len` bytes. The last item may be smaller.
pub(crate) fn stream_single_buffer_partitions(
    mut inner: Box<dyn SeekableStream>,
    partition_len: NonZero<usize>,
) -> impl Stream<Item = Result<Bytes>> {
    let partition_len = partition_len.get();
    try_stream! {
        let mut len_hint = inner.len();
        let mut total_read = 0;
        let new_partition = |len_hint: Option<u64>, total_read: u64| BytesMut::with_capacity(len_hint
            .map(|len| min(len.saturating_sub(total_read), partition_len as u64) as usize)
            .unwrap_or(partition_len));
        let mut partition = new_partition(len_hint, total_read);
        loop {
            // If partition is at partition_len, yield it
            // Do NOT consider the len_hint. We handle that later.
            let mut remaining_partition_space = partition_len.saturating_sub(partition.len());
            if remaining_partition_space == 0 {
                yield mem::replace(&mut partition, new_partition(len_hint, total_read)).freeze();
                remaining_partition_space = partition_len;
            }
            let remaining_partition_space = remaining_partition_space; // un-mut this variable

            // Edge-case: remaining_partition_space is non-zero but buffer has no spare capacity.
            // This means it was trimmed to respect len_hint. That's just a hint. Test that.
            // Don't reallocate just to test, as it may be a large reallocation.
            // Only reallocate once len_hint proven inaccurate.
            if partition.spare_capacity_mut().is_empty() {
                const SMALL_ALLOC_SIZE: usize = 4 * 1024;
                let mut small_buf = vec![0; min(remaining_partition_space, SMALL_ALLOC_SIZE)];
                let count = inner.read(&mut small_buf).await?;
                if count == 0 {
                    if !partition.is_empty() {
                        yield mem::take(&mut partition).freeze();
                    }
                    break;
                }
                let remaining = &small_buf[..count];
                // len_hint proven inaccurate. discard and reallocate
                len_hint = None;
                partition.reserve(partition_len.saturating_sub(partition.capacity()));
                partition.put(remaining);
                total_read += count as u64;
                // after special-case read, loop back around to recalculate next steps
                continue;
            }

            // Read from inner stream
            // SAFETY: spare_capacity_mut() returns allocated memory without initialized value.
            // We want to read directly into that memory. We are responsible for correctly
            // updating `buf` with its new length using the memory we've now initialized.
            unsafe {
                let spare_capacity = partition.spare_capacity_mut();
                let mut spare_capacity = slice::from_raw_parts_mut(
                    spare_capacity.as_mut_ptr() as *mut u8,
                    spare_capacity.len(),
                );
                // Reserved capacity may go beyond the partition length. Cap it.
                // The resulting capped slice may go beyond what len_hint suggests. Good.
                if spare_capacity.len() > remaining_partition_space {
                    spare_capacity = &mut spare_capacity[..remaining_partition_space];
                }
                let count = inner.read(spare_capacity).await?;
                if count == 0 {
                    // stream finished! yield existing bytes and complete
                    if !partition.is_empty() {
                        yield mem::take(&mut partition).freeze();
                    }
                    break;
                }
                partition.set_len(partition.len() + count);
                total_read += count as u64;
            }

            // If len_hint has been exceeded, discard as inaccurate
            if let Some(expected_total_len) = len_hint {
                if total_read > expected_total_len {
                    len_hint = None;
                }
            }
        }
    }
}

#[derive(Default)]
struct MultiBufferPartition {
    completed_buffers: Vec<Bytes>,
    completed_buffers_total_bytes: u64,
    current_buffer: BytesMut,
    buffer_len: usize,
}

impl MultiBufferPartition {
    fn new(buffer_len: usize, expected_buffers: usize) -> Self {
        Self {
            completed_buffers: Vec::with_capacity(expected_buffers),
            completed_buffers_total_bytes: 0,
            current_buffer: BytesMut::with_capacity(buffer_len),
            buffer_len,
        }
    }
    fn len(&self) -> u64 {
        self.current_buffer.len() as u64 + self.completed_buffers_total_bytes
    }
    fn buf(&mut self) -> &mut BytesMut {
        if self.current_buffer.spare_capacity_mut().is_empty() {
            let bytes = mem::replace(
                &mut self.current_buffer,
                BytesMut::with_capacity(self.buffer_len),
            )
            .freeze();
            self.completed_buffers_total_bytes += bytes.len() as u64;
            self.completed_buffers.push(bytes);
        }
        &mut self.current_buffer
    }
    fn freeze(mut self) -> Vec<Bytes> {
        if !self.current_buffer.is_empty() {
            self.completed_buffers
                .push(mem::take(&mut self.current_buffer).freeze());
        }
        self.completed_buffers
    }
}

const MULTI_BUF_PARTITION_BUF_LEN: usize = 4 * 1024 * 1024;

/// Converts the given AsyncRead into a Stream<Item = Vec<Bytes>>, where each item is a chunk of
/// data that is exactly `partition_len` bytes. The last item may be smaller.
pub(crate) fn stream_multi_buffer_partitions(
    mut inner: Box<dyn SeekableStream>,
    partition_len: NonZero<u64>,
) -> impl Stream<Item = Result<Vec<Bytes>>> {
    let partition_len = partition_len.get();
    // supports a partition_len up to MAX_CONTIGUOUS_ELEMENT * MULTI_BUF_PARTITION_BUF_LEN (= 8 petabytes)
    let vec_len = partition_len
        .div_ceil(MULTI_BUF_PARTITION_BUF_LEN as u64)
        .try_into()
        .unwrap_or(MAX_CONTIGUOUS_ELEMENTS);
    try_stream! {
        let mut partition = MultiBufferPartition::new(MULTI_BUF_PARTITION_BUF_LEN, vec_len);
        loop {
            // If partition is at partition_len, yield it
            let mut remaining_partition_space = partition_len.saturating_sub(partition.len());
            if remaining_partition_space == 0 {
                yield mem::replace(&mut partition, MultiBufferPartition::new(MULTI_BUF_PARTITION_BUF_LEN, vec_len)).freeze();
                remaining_partition_space = partition_len;
            }
            let remaining_partition_space = remaining_partition_space; // un-mut this variable

            // Read from inner stream
            // SAFETY: spare_capacity_mut() returns allocated memory without initialized value.
            // We want to read directly into that memory. We are responsible for correctly
            // updating `buf` with its new length using the memory we've now initialized.
            unsafe {
                let buf = partition.buf();
                let spare_capacity = buf.spare_capacity_mut();
                let mut spare_capacity = slice::from_raw_parts_mut(
                    spare_capacity.as_mut_ptr() as *mut u8,
                    spare_capacity.len(),
                );
                // Reserved capacity may go beyond the partition length. Cap it.
                if let Ok(remaining_usize) = remaining_partition_space.try_into() {
                    if spare_capacity.len() > remaining_usize {
                        spare_capacity = &mut spare_capacity[..remaining_usize];
                    }
                }
                let count = inner.read(spare_capacity).await?;
                if count == 0 {
                    // stream finished! yield existing bytes and complete
                    if partition.len() > 0 {
                        yield mem::take(&mut partition).freeze();
                    }
                    break;
                }
                buf.set_len(buf.len() + count);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use azure_core::stream::BytesStream;
    use futures::TryStreamExt;

    use super::*;

    fn get_random_data(len: usize) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; len];
        rand::fill(&mut data[..]);
        data
    }

    #[tokio::test]
    async fn single_partitions_exact_len() -> Result<()> {
        for part_count in [2, 3, 11, 16] {
            for part_len in [1024, 1000, 9999, 1] {
                let data = get_random_data(part_len * part_count);
                let stream = Box::pin(stream_single_buffer_partitions(
                    Box::new(BytesStream::new(data.clone())),
                    NonZero::new(part_len).unwrap(),
                ));

                let parts: Vec<_> = stream.try_collect().await?;

                assert_eq!(parts.len(), part_count);
                for (i, bytes) in parts.iter().enumerate() {
                    assert_eq!(bytes.len(), part_len);
                    assert_eq!(bytes[..], data[i * part_len..i * part_len + part_len]);
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_partitions_exact_len() -> Result<()> {
        for part_count in [2, 3, 11, 16] {
            for part_len in [1024, 1000, 9999, 1] {
                let data = get_random_data(part_len * part_count);
                let stream = Box::pin(stream_multi_buffer_partitions(
                    Box::new(BytesStream::new(data.clone())),
                    NonZero::new(part_len as u64).unwrap(),
                ));

                let parts: Vec<_> = stream.try_collect().await?;

                assert_eq!(parts.len(), part_count);
                for (i, vec) in parts.iter().enumerate() {
                    let bytes = vec.first().unwrap();
                    assert_eq!(bytes.len(), part_len);
                    assert_eq!(bytes[..], data[i * part_len..i * part_len + part_len]);
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn single_partitions_with_remainder() -> Result<()> {
        for part_count in [2, 3, 11, 16] {
            for part_len in [1024, 1000, 9999] {
                for dangling_len in [part_len / 2, 100, 128, 99] {
                    let data = get_random_data(part_len * (part_count - 1) + dangling_len);
                    let stream = Box::pin(stream_single_buffer_partitions(
                        Box::new(BytesStream::new(data.clone())),
                        NonZero::new(part_len).unwrap(),
                    ));

                    let parts: Vec<_> = stream.try_collect().await?;

                    assert_eq!(parts.len(), part_count);
                    for (i, bytes) in parts[..parts.len()].iter().enumerate() {
                        if i == parts.len() - 1 {
                            assert_eq!(bytes.len(), dangling_len);
                            assert_eq!(bytes[..], data[i * part_len..]);
                        } else {
                            assert_eq!(bytes.len(), part_len);
                            assert_eq!(bytes[..], data[i * part_len..i * part_len + part_len]);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_partitions_with_remainder() -> Result<()> {
        for part_count in [2, 3, 11, 16] {
            for part_len in [1024, 1000, 9999] {
                for dangling_len in [part_len / 2, 100, 128, 99] {
                    let data = get_random_data(part_len * (part_count - 1) + dangling_len);
                    let stream = Box::pin(stream_multi_buffer_partitions(
                        Box::new(BytesStream::new(data.clone())),
                        NonZero::new(part_len as u64).unwrap(),
                    ));

                    let parts: Vec<_> = stream.try_collect().await?;

                    assert_eq!(parts.len(), part_count);
                    for (i, vec) in parts[..parts.len()].iter().enumerate() {
                        let bytes = vec.first().unwrap();
                        if i == parts.len() - 1 {
                            assert_eq!(bytes.len(), dangling_len);
                            assert_eq!(bytes[..], data[i * part_len..]);
                        } else {
                            assert_eq!(bytes.len(), part_len);
                            assert_eq!(bytes[..], data[i * part_len..i * part_len + part_len]);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn single_exactly_one_partition() -> Result<()> {
        for len in [1024, 1000, 9999, 1] {
            let data = get_random_data(len);
            let mut stream = Box::pin(stream_single_buffer_partitions(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(len).unwrap(),
            ));

            let single_partition = stream.try_next().await?.unwrap();

            assert_eq!(stream.try_next().await?, None);
            assert_eq!(single_partition[..], data[..]);
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_exactly_one_partition() -> Result<()> {
        for len in [1024, 1000, 9999, 1] {
            let data = get_random_data(len);
            let mut stream = Box::pin(stream_multi_buffer_partitions(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(len as u64).unwrap(),
            ));

            let single_partition = stream.try_next().await?.unwrap().pop().unwrap();

            assert_eq!(stream.try_next().await?, None);
            assert_eq!(single_partition[..], data[..]);
        }
        Ok(())
    }

    #[tokio::test]
    async fn single_less_than_one_partition() -> Result<()> {
        let part_len = 99999;
        for len in [1024, 1000, 9999, 1] {
            let data = get_random_data(len);
            let mut stream = Box::pin(stream_single_buffer_partitions(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(part_len).unwrap(),
            ));

            let single_partition = stream.try_next().await?.unwrap();

            assert!(stream.try_next().await?.is_none());
            assert_eq!(single_partition[..], data[..]);
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_less_than_one_partition() -> Result<()> {
        let part_len = 99999;
        for len in [1024, 1000, 9999, 1] {
            let data = get_random_data(len);
            let mut stream = Box::pin(stream_multi_buffer_partitions(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(part_len as u64).unwrap(),
            ));

            let single_partition = stream.try_next().await?.unwrap().pop().unwrap();

            assert!(stream.try_next().await?.is_none());
            assert_eq!(single_partition[..], data[..]);
        }
        Ok(())
    }

    #[tokio::test]
    async fn single_successful_empty_stream_when_empty_source_stream() -> Result<()> {
        for part_len in [1024, 1000, 9999, 1] {
            let data = get_random_data(0);
            let mut stream = Box::pin(stream_single_buffer_partitions(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(part_len).unwrap(),
            ));

            assert!(stream.try_next().await?.is_none());
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_successful_empty_stream_when_empty_source_stream() -> Result<()> {
        for part_len in [1024, 1000, 9999, 1] {
            let data = get_random_data(0);
            let mut stream = Box::pin(stream_multi_buffer_partitions(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(part_len).unwrap(),
            ));

            assert!(stream.try_next().await?.is_none());
        }
        Ok(())
    }

    #[tokio::test]
    async fn multi_buffer_partitions() -> Result<()> {
        for part_len in [
            MULTI_BUF_PARTITION_BUF_LEN + 1,
            MULTI_BUF_PARTITION_BUF_LEN * 2,
            MULTI_BUF_PARTITION_BUF_LEN * 2 + 1,
        ] {
            for part_count in [1, 2, 5] {
                let data = get_random_data(part_len * part_count);
                let stream = Box::pin(stream_multi_buffer_partitions(
                    Box::new(BytesStream::new(data.clone())),
                    NonZero::new(part_len as u64).unwrap(),
                ));

                let parts: Vec<_> = stream.try_collect().await?;
                assert_eq!(parts.len(), part_count);
                for part in &parts {
                    assert!(part.len() > 1)
                }
                let mut data_slice = &data[..];
                for vec in parts {
                    for bytes in vec {
                        assert_eq!(bytes, data_slice[..bytes.len()]);
                        data_slice = &data_slice[bytes.len()..];
                    }
                }
            }
        }
        Ok(())
    }
}
