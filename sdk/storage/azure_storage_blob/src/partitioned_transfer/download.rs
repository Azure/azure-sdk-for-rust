// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    cmp::min,
    collections::VecDeque,
    ops::Range,
    sync::{
        atomic::{AtomicUsize, Ordering},
        mpsc::{self, TryRecvError},
        Arc,
    },
};

use async_trait::async_trait;
use azure_core::{
    async_runtime::get_async_runtime,
    error::ErrorKind,
    http::{response::PinnedStream, AsyncRawResponse, StatusCode},
    stream::BytesStream,
    Error,
};
use bytes::{Bytes, BytesMut};
use futures::TryStream;

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
    let max_buffers = parallel * 2;
    let partition_size = partition_size.get();

    // Outer bound estimate of the resource range that will be downloaded. The actual download
    // range will never exceed these bounds, but it may be smaller, based on the actual size
    // of the remote resource.
    let max_download_range = range.unwrap_or(0..usize::MAX);
    if max_download_range.is_empty() {
        let raw_stream: PinnedStream = Box::pin(BytesStream::new_empty());
        return Ok(raw_stream);
    }

    let initial_response = download_handle_invalid_range(
        client.as_ref(),
        max_download_range.start
            ..min(
                max_download_range.end,
                max_download_range.start.saturating_add(partition_size),
            ),
    )
    .await?;

    let stats =
        analyze_initial_response(&initial_response, partition_size, max_download_range.end)?
            .unwrap(); // TODO

    let mut remaining_ranges = stats.remaining_download_ranges;
    let total_chunks = remaining_ranges.len() + 1;
    if remaining_ranges.is_empty() {
        return Ok(Box::pin(initial_response.into_body()));
    }

    let (tx, rx) = mpsc::channel();
    // start with one task: init_download_task
    let active_tasks_counter = Arc::new(AtomicUsize::new(1));

    let tx_init = tx.clone();
    let counter_init = active_tasks_counter.clone();
    let init_download_task = get_async_runtime().spawn(Box::pin(async move {
        let res = collect_into(
            initial_response.into_body(),
            BytesMut::with_capacity(partition_size),
        )
        .await
        .map(|bytes| (0usize, bytes));
        let _ = tx_init.send(res);
        counter_init.fetch_sub(1, Ordering::Relaxed);
    }));

    let mut tasks = vec![init_download_task];
    let mut drain = vec![None; max_buffers];
    let drain_len = drain.len(); // store this to dodge some borrowing issues in try_stream

    let mut drain_cursor = 0;

    let mut tx_opt = Some(tx);
    let stream = async_stream::try_stream! {
        while drain_cursor < total_chunks {
            // If room in the drain and not at max connections
            while tasks.len() - drain_cursor < max_buffers && active_tasks_counter.load(Ordering::Relaxed) < parallel {
                match remaining_ranges.pop_front() {
                    Some(range) => {
                        let i = tasks.len();
                        active_tasks_counter.fetch_add(1, Ordering::Relaxed);

                        let c = client.clone();
                        let t = tx_opt.as_ref().ok_or_else(||Error::with_message(ErrorKind::Other, "Channel closed unexpectedly."))?.clone();
                        let active_tasks = active_tasks_counter.clone();
                        tasks.push(get_async_runtime().spawn(Box::pin(async move {
                            let res = download_range_to_bytes(c, range).await.map(|bytes| (i, bytes));
                            let _ = t.send(res);
                            active_tasks.fetch_sub(1, Ordering::Relaxed);
                        })));
                    }
                    None => {
                        tx_opt = None;
                        break;
                    }
                }
            }

            // return readied bytes, sequentially
            while let Some(bytes) = drain[drain_cursor % drain_len].take() {
                drain_cursor += 1;
                yield bytes;
            }

            // early break if finished
            if drain_cursor >= total_chunks {
                break;
            }

            match rx.try_recv() {
                Ok(Ok((idx, bytes))) => drain[idx % drain_len] = Some(bytes),
                Ok(Err(err)) => Err(err)?,
                Err(TryRecvError::Empty) => {} //get_async_runtime().yield_now().await,
                // Unexpected channel close
                Err(TryRecvError::Disconnected) => {
                    // if we yielded all the chunks, no harm no foul
                    if drain_cursor == total_chunks {
                        break;
                    }
                    // Did a task fail without notifying the channel?
                    while !tasks.is_empty() {
                        let result;
                        (result, _, tasks) = future::select_all(tasks).await;
                        result.map_err(|err| Error::with_message(ErrorKind::Other, format!("A download task failed: {err}")))?;
                    }
                    // No detectable error but we didn't get everything we needed.
                    Err(Error::with_message(ErrorKind::Other, format!("Download incomplete: received {drain_cursor} of {total_chunks} chunks.")))?;
                }
            }
        }
    };

    Ok(Box::pin(stream))
}

async fn download_range_to_bytes(
    client: Arc<impl PartitionedDownloadBehavior>,
    range: Range<usize>,
) -> AzureResult<Bytes> {
    let dst = BytesMut::with_capacity(range.len());
    let response = client.transfer_range(Some(range)).await?;
    collect_into(response.into_body(), dst).await
}

async fn collect_into<S: TryStream<Ok = Bytes> + Unpin>(
    mut stream: S,
    mut destination: BytesMut,
) -> Result<Bytes, S::Error> {
    while let Some(bytes) = stream.try_next().await? {
        destination.extend_from_slice(&bytes);
    }
    Ok(destination.freeze())
}

/// Performs a `transfer_range()` call with the given range. If this results in a
/// RequestedRangeNotSatisfiable error, and if the requested range begins at the
/// start of the blob, retries the operation without a range argument.
async fn download_handle_invalid_range<Behavior>(
    client: &Behavior,
    range: Range<usize>,
) -> AzureResult<AsyncRawResponse>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let range_start = range.start;
    match client.transfer_range(Some(range)).await {
        Ok(response) => Ok(response),
        Err(err) => match (err.http_status(), range_start) {
            (Some(StatusCode::RequestedRangeNotSatisfiable), 0) => {
                client.transfer_range(None).await
            }
            _ => Err(err),
        },
    }
}

/// First attempts to get the length from the Content-Range header. Content range will give the
/// most accurate reading for the content length, unaffected by an encoding such as structured
/// message. It is highly unlikely content-range will be absent.
/// Uses the content-length header if content-range is unavailable.
fn get_body_len(response: &AsyncRawResponse) -> AzureResult<Option<usize>> {
    if let Some(content_range) = response
        .headers()
        .get_optional_as::<ContentRange, _>(&"content-range".into())?
    {
        if let Some(range) = content_range.range {
            return Ok(Some(range.1 - range.0));
        }
    }
    if let Some(content_length) = response
        .headers()
        .get_optional_as::<usize, _>(&"content-length".into())?
    {
        return Ok(Some(content_length));
    }
    Ok(None)
}

struct InitialResponseAnalysis {
    overall_download_range: Range<usize>,
    initial_download_range: Range<usize>,
    remaining_download_ranges: VecDeque<Range<usize>>,
}
/// Reads over the response headers of the initial download response and compiles all relevant
/// information to perform the remaining downloads and arrange all resulting bytes.
///
/// # Returns
///
/// Ok(Some(analysis)) if the appropriate information was available.
///
/// Ok(None) of the appropriate information was not available.
///
/// Err(error) if there was an error parsing the appropriate information.
fn analyze_initial_response(
    initial_response: &AsyncRawResponse,
    partition_len: usize,
    max_range_end: usize,
) -> AzureResult<Option<InitialResponseAnalysis>> {
    let content_range = match initial_response
        .headers()
        .get_optional_as::<ContentRange, _>(&"content-range".into())?
    {
        Some(content_range) => content_range,
        None => return Ok(None),
    };
    match (content_range.range, content_range.total_len) {
        (Some(received_range), Some(resource_len)) => {
            let remainder_start = received_range.1;
            let remainder_end = min(max_range_end, resource_len);
            Ok(Some(InitialResponseAnalysis {
                overall_download_range: received_range.0..remainder_end,
                initial_download_range: received_range.0..received_range.1,
                remaining_download_ranges: (remainder_start..remainder_end)
                    .step_by(partition_len)
                    .map(|i| i..min(i.saturating_add(partition_len), remainder_end))
                    .collect(),
            }))
        }
        _ => Ok(None),
    }
}

trait DownloadRangeFuture: Future + Send {}
impl<T: Future + Send> DownloadRangeFuture for T {}

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

        for parallel in [4, 1] {
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
