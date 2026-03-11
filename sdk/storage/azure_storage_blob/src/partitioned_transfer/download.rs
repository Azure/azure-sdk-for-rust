// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{cmp::min, collections::VecDeque, ops::Range, sync::Arc};

use async_trait::async_trait;
use azure_core::{
    http::{response::PinnedStream, AsyncRawResponse, StatusCode},
    stream::BytesStream,
};
use bytes::Bytes;
use futures::{stream::FuturesOrdered, StreamExt};

use crate::models::http_ranges::ContentRange;

use super::*;

#[async_trait]
pub(crate) trait PartitionedDownloadBehavior {
    async fn transfer_range(&self, range: Option<Range<usize>>) -> AzureResult<AsyncRawResponse>;
}

/// Returns a stream that runs up to parallel-many ranged downloads at a time.
///
/// Downloads are stored in-order. The returned stream will produce an item only when the next
/// download in the sequence has been buffered, regardless of the state of any other downloads.
/// This means completed ranged downloads may sit for a while while earlier ones complete.
///
/// A download that has completed buffering but has not yet returned its buffer in the resulting
/// stream will still count when determining current running operations. This is so the stream can
/// promise its buffered bytes do not exceed parallel * partition_size.
pub(crate) async fn download<Behavior>(
    range: Option<Range<usize>>,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: Arc<Behavior>,
) -> AzureResult<PinnedStream>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let parallel = parallel.get();
    let partition_size = partition_size.get();

    // Outer bound estimate of the resource range that will be downloaded. The actual download
    // range will never exceed these bounds, but it may be smaller, based on the actual size
    // of the remote resource.
    let max_download_range = range.unwrap_or(0..usize::MAX);
    if max_download_range.is_empty() {
        let raw_stream: PinnedStream = Box::pin(BytesStream::new_empty());
        return Ok(raw_stream);
    }

    let initial_response = match client
        .transfer_range(Some(
            max_download_range.start
                ..min(
                    max_download_range.end,
                    max_download_range.start.saturating_add(partition_size),
                ),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => match (err.http_status(), max_download_range.start) {
            (Some(StatusCode::RequestedRangeNotSatisfiable), 0) => {
                client.transfer_range(None).await?
            }
            _ => Err(err)?,
        },
    };

    let mut ranges: VecDeque<_> = match initial_response
        .headers()
        .get_optional_as::<ContentRange, _>(&"content-range".into())?
    {
        Some(content_range) => match (content_range.range, content_range.total_len) {
            (Some(received_range), Some(resource_len)) => {
                let remainder_start = received_range.1;
                let remainder_end = min(max_download_range.end, resource_len);
                (remainder_start..remainder_end)
                    .step_by(partition_size)
                    .map(|i| i..min(i.saturating_add(partition_size), remainder_end))
                    .collect()
            }
            _ => VecDeque::new(),
        },
        None => VecDeque::new(),
    };

    // the first operation has a different type from the others.
    // fully type this variable out to specify dyn.
    let fut: Pin<Box<dyn DownloadRangeFuture<Output = AzureResult<Bytes>>>> =
        Box::pin(initial_response.into_body().collect());
    let mut ops = FuturesOrdered::new();
    ops.push_back(fut);

    let stream = futures::stream::poll_fn(move |cx| {
        // fill to max parallel ops
        while ops.len() < parallel {
            match ranges.pop_front() {
                Some(range) => {
                    ops.push_back(Box::pin(download_range_to_bytes(client.clone(), range)))
                }
                None => break,
            }
        }

        ops.poll_next_unpin(cx)
    });

    Ok(Box::pin(stream))
}

async fn download_range_to_bytes(
    client: Arc<impl PartitionedDownloadBehavior>,
    range: Range<usize>,
) -> AzureResult<Bytes> {
    let response = client.transfer_range(Some(range)).await?;
    response.into_body().collect().await
}

trait DownloadRangeFuture: Future + Send {}
impl<T: Future + Send> DownloadRangeFuture for T {}

/// A wrapper around a raw pointer to a mutable byte slice that can be sent across threads.
///
/// # Safety
/// The caller must ensure that:
/// - The underlying buffer outlives all `SendSlice` instances
/// - No two `SendSlice` instances cover overlapping memory regions when used concurrently
pub(crate) struct SendSlice {
    ptr: *mut u8,
    len: usize,
}

// SAFETY: SendSlice is only created from non-overlapping sub-regions of a buffer
// that outlives the spawned tasks. The caller guarantees exclusive access to each region.
unsafe impl Send for SendSlice {}

impl SendSlice {
    /// Creates a SendSlice from a raw pointer and length.
    ///
    /// # Safety
    /// The caller must ensure the pointer is valid for `len` bytes and that no other
    /// `SendSlice` or reference accesses overlapping memory concurrently.
    pub(crate) unsafe fn from_raw(ptr: *mut u8, len: usize) -> Self {
        Self { ptr, len }
    }

    /// Returns a mutable slice from the raw pointer.
    ///
    /// # Safety
    /// The caller must ensure no other references to this memory region exist concurrently.
    pub(crate) unsafe fn as_mut_slice(&mut self) -> &mut [u8] {
        std::slice::from_raw_parts_mut(self.ptr, self.len)
    }
}

/// Downloads a blob directly into a caller-provided buffer with true parallel chunk writes.
///
/// Uses a large `initial_partition_size` for the first request (so small/medium blobs complete
/// in a single HTTP request), then fills the remainder with `partition_size` chunks using
/// `tokio::spawn` for real OS-thread-level parallelism. Each spawned task streams its response
/// body directly into a non-overlapping sub-slice of the buffer via unsafe `SendSlice`, avoiding
/// intermediate `Bytes` allocations.
///
/// Returns the number of bytes written to the buffer.
pub(crate) async fn download_to<Behavior>(
    buffer: &mut [u8],
    range: Option<Range<usize>>,
    parallel: NonZero<usize>,
    initial_partition_size: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: Arc<Behavior>,
) -> AzureResult<usize>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let parallel = parallel.get();
    let initial_partition_size = initial_partition_size.get();
    let partition_size = partition_size.get();

    let max_download_range = range.unwrap_or(0..usize::MAX);
    if max_download_range.is_empty() {
        return Ok(0);
    }

    // Initial request uses the large initial_partition_size to probe blob size
    // and download small/medium blobs in a single request.
    let initial_response = match client
        .transfer_range(Some(
            max_download_range.start
                ..min(
                    max_download_range.end,
                    max_download_range.start.saturating_add(initial_partition_size),
                ),
        ))
        .await
    {
        Ok(response) => response,
        Err(err) => match (err.http_status(), max_download_range.start) {
            (Some(StatusCode::RequestedRangeNotSatisfiable), 0) => {
                client.transfer_range(None).await?
            }
            _ => Err(err)?,
        },
    };

    // Parse Content-Range to determine total blob size and compute remaining ranges.
    let ranges: Vec<_> = match initial_response
        .headers()
        .get_optional_as::<ContentRange, _>(&"content-range".into())?
    {
        Some(content_range) => match (content_range.range, content_range.total_len) {
            (Some(received_range), Some(resource_len)) => {
                let remainder_start = received_range.1;
                let remainder_end = min(max_download_range.end, resource_len);
                (remainder_start..remainder_end)
                    .step_by(partition_size)
                    .map(|i| i..min(i.saturating_add(partition_size), remainder_end))
                    .collect()
            }
            _ => Vec::new(),
        },
        None => Vec::new(),
    };

    // Stream initial response body directly into buffer (no intermediate Bytes allocation).
    let mut body = initial_response.into_body();
    let mut write_offset = 0usize;
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        let end = write_offset + chunk.len();
        if end > buffer.len() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Io,
                format!(
                    "Buffer too small: size {} but need at least {} bytes",
                    buffer.len(),
                    end
                ),
            ));
        }
        buffer[write_offset..end].copy_from_slice(&chunk);
        write_offset += chunk.len();
    }
    let mut total_written = write_offset;

    if ranges.is_empty() {
        return Ok(total_written);
    }

    // Use raw pointer to create non-overlapping SendSlice instances for true parallel writes.
    let buffer_ptr = buffer.as_mut_ptr();
    let buffer_len = buffer.len();

    let semaphore = Arc::new(tokio::sync::Semaphore::new(parallel));
    let mut join_set = tokio::task::JoinSet::new();

    for r in ranges {
        let c = client.clone();
        let sem = semaphore.clone();
        let buf_offset = r.start - max_download_range.start;
        let chunk_max_len = r.end - r.start;

        if buf_offset + chunk_max_len > buffer_len {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Io,
                format!(
                    "Buffer too small: size {} but download requires {} bytes",
                    buffer_len,
                    buf_offset + chunk_max_len
                ),
            ));
        }

        // SAFETY: Each range is non-overlapping and within buffer bounds.
        // The buffer outlives all spawned tasks because we join/shutdown before returning.
        // SAFETY: buf_offset + chunk_max_len is within buffer bounds (checked above).
        let mut send_slice = unsafe { SendSlice::from_raw(buffer_ptr.add(buf_offset), chunk_max_len) };

        // tokio::spawn for real OS-thread-level parallelism.
        join_set.spawn(async move {
            let _permit = sem.acquire().await.map_err(|e| {
                azure_core::Error::new(azure_core::error::ErrorKind::Other, e)
            })?;

            let response = c.transfer_range(Some(r)).await?;
            let mut body = response.into_body();
            let mut written = 0usize;
            // SAFETY: No other task accesses this slice region concurrently.
            let slice = unsafe { send_slice.as_mut_slice() };
            while let Some(chunk) = body.next().await {
                let chunk = chunk?;
                slice[written..written + chunk.len()].copy_from_slice(&chunk);
                written += chunk.len();
            }
            Ok::<usize, azure_core::Error>(written)
        });
    }

    while let Some(result) = join_set.join_next().await {
        match result {
            Ok(Ok(written)) => total_written += written,
            Ok(Err(e)) => {
                join_set.shutdown().await;
                return Err(e);
            }
            Err(join_err) => {
                join_set.shutdown().await;
                return Err(azure_core::Error::new(
                    azure_core::error::ErrorKind::Other,
                    join_err,
                ));
            }
        }
    }

    Ok(total_written)
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use azure_core::{
        http::{headers::Headers, StatusCode},
        stream::BytesStream,
    };

    use azure_core_test::ErrorKind;
    use tokio::{
        sync::Mutex,
        time::{sleep, Duration},
    };

    use super::*;

    pub const KB: usize = 1024;
    pub const MB: usize = KB * 1024;
    pub const GB: usize = MB * 1024;

    #[derive(Debug)]
    enum MockPartitionedDownloadBehaviorInvocation {
        TransferRange(Option<Range<usize>>),
    }

    struct MockPartitionedDownloadBehavior {
        pub invocations: Mutex<Vec<MockPartitionedDownloadBehaviorInvocation>>,
        pub data: Bytes,
        pub delay_millis: Option<Range<u64>>,
    }

    impl MockPartitionedDownloadBehavior {
        pub fn new(data: impl Into<Bytes>, delay_millis: Option<Range<u64>>) -> Self {
            Self {
                invocations: Mutex::new(vec![]),
                data: data.into(),
                delay_millis,
            }
        }
    }

    #[async_trait::async_trait]
    impl PartitionedDownloadBehavior for MockPartitionedDownloadBehavior {
        async fn transfer_range(
            &self,
            requested_range: Option<Range<usize>>,
        ) -> AzureResult<AsyncRawResponse> {
            {
                self.invocations.lock().await.push(
                    MockPartitionedDownloadBehaviorInvocation::TransferRange(
                        requested_range.clone(),
                    ),
                );
            }

            if let Some(delay_millis_range) = self.delay_millis.clone() {
                let millis = rand::random_range(delay_millis_range);
                sleep(Duration::from_millis(millis)).await
            }

            let mut headers = Headers::new();
            match (requested_range, self.data.len()) {
                (Some(range), data_len) => {
                    if range.start >= data_len {
                        return Err(ErrorKind::HttpResponse {
                            status: StatusCode::RequestedRangeNotSatisfiable,
                            error_code: Some("InvalidRange".into()),
                            raw_response: None,
                        }
                        .into_error());
                    }
                    let range = range.start..min(range.end, data_len);
                    headers.add(ContentRange {
                        range: Some((range.start, range.end - 1)),
                        total_len: Some(self.data.len()),
                    })?;
                    let range = range.start..range.end;
                    Ok(AsyncRawResponse::new(
                        StatusCode::PartialContent,
                        headers,
                        Box::pin(BytesStream::from(self.data.slice(range))),
                    ))
                }
                (None, 0) => {
                    headers.add(ContentRange {
                        range: None,
                        total_len: None,
                    })?;
                    Ok(AsyncRawResponse::new(
                        StatusCode::Ok,
                        headers,
                        Box::pin(BytesStream::new_empty()),
                    ))
                }
                (None, data_len) => {
                    headers.add(ContentRange {
                        range: Some((0, data_len - 1)),
                        total_len: Some(data_len),
                    })?;
                    Ok(AsyncRawResponse::new(
                        StatusCode::Ok,
                        headers,
                        Box::pin(BytesStream::from(self.data.clone())),
                    ))
                }
            }
        }
    }

    #[tokio::test]
    async fn download_single_range() -> AzureResult<()> {
        const DATA_LEN: usize = 1024;
        const PARALLEL: usize = 2;

        let data = get_random_data(DATA_LEN);

        // trait not implemented for usize
        let part_len = (rand::random::<u64>() as usize % 100) + 100;
        let extra = (rand::random::<u64>() as usize % 100) + 100;
        let offset = (rand::random::<u64>() as usize % 100) + 100;

        let start_range = (0, part_len);
        let mid_range = (offset, offset + part_len);
        let end_range = (DATA_LEN - part_len, DATA_LEN);

        for (partition_size, download_range) in [
            (DATA_LEN, None),                      // exact len
            (DATA_LEN + extra, None),              // oversize len
            (part_len, Some(start_range)),         // exact range len (start)
            (part_len + extra, Some(start_range)), // oversize range len (start)
            (part_len, Some(mid_range)),           // exact range len (mid))
            (part_len + extra, Some(mid_range)),   // oversize range len (mid))
            (part_len, Some(end_range)),           // exact range len (end)
            (part_len + extra, Some(end_range)),   // oversize range len (end)
        ] {
            let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

            let downloaded_data = download(
                download_range.map(|r| r.0..r.1),
                PARALLEL.try_into().unwrap(),
                partition_size.try_into().unwrap(),
                mock.clone(),
            )
            .await?
            .buffer_all()
            .await?;

            assert_eq!(
                &downloaded_data[..],
                match download_range {
                    Some(r) => &data[r.0..r.1],
                    None => &data[..],
                }
            );
            assert_eq!(mock.invocations.lock().await.len(), 1);
        }

        Ok(())
    }

    #[tokio::test]
    async fn download_multi_range() -> AzureResult<()> {
        const DATA_LEN: usize = 4096;

        let data = get_random_data(DATA_LEN);

        // trait not implemented for usize??
        let part_len = (rand::random::<u64>() as usize % 100) + 100;
        let expected_whole_blob_parts = DATA_LEN / part_len
            + if DATA_LEN.is_multiple_of(part_len) {
                0
            } else {
                1
            };

        let range_len = (rand::random::<u64>() as usize % 500) + 500;
        let expected_range_parts = range_len / part_len
            + if range_len.is_multiple_of(part_len) {
                0
            } else {
                1
            };
        let offset = (rand::random::<u64>() as usize % 500) + 500;

        for parallel in [1, 4] {
            for blob_range in [
                (0, range_len),                   // start of blob
                (offset, offset + range_len),     // middle of blob
                (DATA_LEN - range_len, DATA_LEN), // end of blob
            ] {
                for (partition_len, download_range, expected_parts) in [
                    (DATA_LEN - 1, None, 2),                              // barely smaller
                    (DATA_LEN / 2, None, 2),                              // half size
                    (part_len, None, expected_whole_blob_parts),          // oddball size
                    (range_len - 1, Some(blob_range), 2),                 // barely smaller, range
                    (range_len / 2, Some(blob_range), 2 + range_len % 2), // half size, range
                    (part_len, Some(blob_range), expected_range_parts),   // oddball size, range
                ] {
                    let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

                    let downloaded_data = download(
                        download_range.map(|r| r.0..r.1),
                        parallel.try_into().unwrap(),
                        partition_len.try_into().unwrap(),
                        mock.clone(),
                    )
                    .await?
                    .buffer_all()
                    .await?;

                    assert_eq!(
                        downloaded_data.len(),
                        download_range.map_or(DATA_LEN, |range| range.1 - range.0),
                        "Data mismatch. partition_len={}. download_range={:?}, expected_parts={}",
                        partition_len,
                        download_range,
                        expected_parts
                    );
                    assert_eq!(
                        &downloaded_data[..],
                        match download_range {
                            Some(r) => &data[r.0..r.1],
                            None => &data[..],
                        },
                        "Data mismatch. partition_len={}. download_range={:?}, expected_parts={}",
                        partition_len,
                        download_range,
                        expected_parts
                    );
                    assert_eq!(
                        mock.invocations.lock().await.len(),
                        expected_parts,
                        "Unexpected invocation count. partition_len={}. download_range={:?}, expected_parts={}",
                        partition_len,
                        download_range,
                        expected_parts);
                }
            }
        }

        Ok(())
    }

    #[tokio::test]
    async fn download_ranges_parallel_maintain_order() -> AzureResult<()> {
        let segments: usize = 20;
        let partition_size = NonZero::new(3).unwrap();
        let parallel = NonZero::new(16).unwrap();
        let data_size: usize = partition_size.get() * segments;

        let data = get_random_data(data_size);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(
            data.clone(),
            Some(1..5),
        ));

        let downloaded_data = download(None, parallel, partition_size, mock.clone())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), segments);

        Ok(())
    }

    #[tokio::test]
    async fn download_empty() -> AzureResult<()> {
        let parallel = NonZero::new(1).unwrap();
        let partition_len = NonZero::new(MB).unwrap();
        for (empty_source, empty_range) in [(true, false), (false, true), (true, true)] {
            let data = get_random_data(if empty_source { 0 } else { KB });
            let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

            let downloaded_data = download(
                if empty_range { Some(0..0) } else { None },
                parallel,
                partition_len,
                mock.clone(),
            )
            .await?
            .buffer_all()
            .await?;

            assert_eq!(
                downloaded_data.len(),
                0,
                "empty_source={}. empty_range={}.",
                empty_source,
                empty_range
            );
        }

        Ok(())
    }

    trait BytesTryStreamExt {
        async fn buffer_all(&mut self) -> AzureResult<Vec<u8>>;
    }
    impl<S> BytesTryStreamExt for S
    where
        S: ?Sized + Stream<Item = AzureResult<Bytes>> + Unpin,
    {
        async fn buffer_all(&mut self) -> AzureResult<Vec<u8>> {
            let mut buffer = Vec::<u8>::new();
            while let Some(bytes) = self.try_next().await? {
                buffer.extend_from_slice(&bytes);
            }

            Ok(buffer)
        }
    }

    fn get_random_data(len: usize) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; len];
        rand::fill(&mut data[..]);
        data
    }
}
