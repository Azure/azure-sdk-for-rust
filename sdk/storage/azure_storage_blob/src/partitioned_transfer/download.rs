// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    cmp::min,
    collections::VecDeque,
    ops::Range,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use crate::models::HttpRange;

use async_trait::async_trait;
use azure_core::{
    async_runtime::{get_async_runtime, SpawnedTask},
    error::ErrorKind,
    http::{AsyncRawResponse, AsyncResponseBody, Etag, StatusCode},
    Error,
};
use bytes::Bytes;
use futures::{
    channel::mpsc::{self, UnboundedReceiver, UnboundedSender},
    future::{self, Either},
    SinkExt, StreamExt,
};

use crate::models::{drains::SequentialBoundedDrain, http_ranges::ContentRange};

use super::*;

#[async_trait]
pub(crate) trait PartitionedDownloadBehavior {
    async fn transfer_range(
        &self,
        range: Option<Range<usize>>,
        etag_lock: Option<Etag>,
    ) -> AzureResult<AsyncRawResponse>;
}

/// Returns a stream that runs up to parallel-many ranged downloads at a time.
///
/// Downloads are stored in-order. The returned stream will produce an item only when the next
/// download in the sequence has been buffered, regardless of the state of any other downloads.
/// This means completed ranged downloads may sit for a while while earlier ones complete.
///
/// This implementation makes an initial download request to gauge the actual size of the remote
/// resource while not wasting a roundtrip just for a HEAD request. It then determines the
/// correct set of additional ranges to download and queues them up. The returned `Stream`
/// executes these downloads, maintaining limits for parallel downloads and buffer count.
pub(crate) async fn download<Behavior>(
    range: Option<HttpRange>,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: Arc<Behavior>,
) -> AzureResult<AsyncRawResponse>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let range: Option<Range<usize>> = range.map(|hr| {
        let start = hr.offset() as usize;
        let end = match hr.length() {
            Some(len) => start + len as usize,
            None => usize::MAX,
        };
        start..end
    });
    let parallel = parallel.get();
    let max_buffers = parallel * 2;
    let partition_size = partition_size.get();

    let (initial_response, stats) =
        get_initial_response_and_analyze(range, partition_size, client.clone()).await?;

    let status = initial_response.status();
    let headers = initial_response.headers().clone();
    let etag_lock = headers.get_optional_str(&"etag".into()).map(Etag::from);

    let mut remaining_ranges = stats
        .map(|s| s.remaining_download_ranges)
        .unwrap_or_default();
    if remaining_ranges.is_empty() {
        return Ok(AsyncRawResponse::new(
            status,
            headers,
            Box::pin(initial_response.into_body()),
        ));
    }
    let total_chunks = remaining_ranges.len() + 1;

    // channel for download workers to send results to their coordinator.
    let (tx, mut rx) = mpsc::unbounded();

    // start with one initial download task at index 0
    let active_tasks_counter = Arc::new(AtomicUsize::new(1));
    let mut next_task_index = 1;
    let mut task_bucket = vec![start_initial_download_task_buffer(
        initial_response,
        tx.clone(),
        active_tasks_counter.clone(),
        partition_size,
    )];

    let mut drain = SequentialBoundedDrain::new(max_buffers);
    let mut tx_opt = Some(tx);

    // This stream maintains up to parallel-many active client downloads at a time while maintaining
    // up to max_buffers-many buffers of length partition_size.
    // It re-sequences these buffers, only yielding the sequentially next buffer when it is ready,
    // regardless of the state of subsequent buffers.
    // Drain serves double duty of holding completed buffers as well as tracking position of the
    // download, indexed by chunk.
    let stream = async_stream::try_stream! {
        while drain.position() < total_chunks {
            // while there is room in the buffer drain and not at max connections, start new range downloads
            while drain.currently_accepting().contains(&next_task_index) && active_tasks_counter.load(Ordering::Relaxed) < parallel {
                match remaining_ranges.pop_front() {
                    Some(range) => {
                        let i = next_task_index;
                        next_task_index += 1;
                        active_tasks_counter.fetch_add(1, Ordering::Relaxed);
                        let t = tx_opt.as_ref().ok_or_else(||Error::with_message(ErrorKind::Other, "Channel closed unexpectedly."))?.clone();
                        task_bucket.push(start_download_task_buffer(client.clone(), range, etag_lock.clone(), t, active_tasks_counter.clone(), i));
                    }
                    None => {
                        // if ranges are finished, we'll never need to clone the transmitter again.
                        // drop this transmitter to ensure channel closes when expected
                        tx_opt = None;
                        break;
                    }
                }
            }

            // await the next completed download
            let channel_message;
            (channel_message, task_bucket) = await_message_while_joining_workers(&mut rx, task_bucket).await?;
            let (idx, bytes) = channel_message?;
            drain.push(idx, bytes)?;

            // return next readied bytes, if any
            while let Some(bytes) = drain.pop() {
                yield bytes;
            }
        }
    };

    Ok(AsyncRawResponse::new(status, headers, Box::pin(stream)))
}

pub(crate) async fn download_into<Behavior>(
    buffer: &mut [u8],
    range: Option<HttpRange>,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: Arc<Behavior>,
) -> AzureResult<usize>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let insufficient_buffer_err =
        || Error::with_message(ErrorKind::Other, "Insufficient buffer length.");

    let range: Option<Range<usize>> = range.map(|hr| {
        let start = hr.offset() as usize;
        let end = match hr.length() {
            Some(len) => start + len as usize,
            None => usize::MAX,
        };
        start..end
    });
    let parallel = parallel.get();
    let partition_size = partition_size.get();

    let (initial_response, response_analysis) =
        get_initial_response_and_analyze(range, partition_size, client.clone()).await?;

    let _status = initial_response.status();
    let headers = initial_response.headers().clone();
    let etag_lock = headers.get_optional_str(&"etag".into()).map(Etag::from);

    let mut response_analysis = match response_analysis {
        Some(a) => a,
        // if no response analysis, no subsequent gets, therefore just copy to buffer and return
        None => return initial_response.into_body().collect_into(buffer).await,
    };

    // fail fast for buffer overflow
    if response_analysis.overall_download_range.len() > buffer.len() {
        return Err(insufficient_buffer_err());
    }

    // if no real parallelism, just sequentially go through the ranges and write to buffer
    if parallel == 1 {
        let mut total_read = initial_response.into_body().collect_into(buffer).await?;
        for range in response_analysis.remaining_download_ranges {
            total_read += client
                .transfer_range(Some(range), etag_lock.clone())
                .await?
                .into_body()
                .collect_into(&mut buffer[total_read..])
                .await?;
        }
        return Ok(total_read);
    }

    /* There's no sound way to have parallel workers operate on a borrowed buffer. Instead, we will
     * parllelize only the reading from download streams, sending the resulting Bytes straight into
     * a channel.
     * Back here, we will asynchronously, but without concurrency, poll that channel and write the
     * Bytes it produces into the borrowed destination buffer.
     * Assuming we can copy bytes in memory faster than the network can produce them, there should
     * be no significant buildup of memory in this channel.
     */
    let (mut tx, mut rx) = mpsc::unbounded();
    // worker that does nothing but monitor active downloads and spawn new ones
    let downloads_manager_handle = get_async_runtime().spawn(Box::pin(async move {
        // the starting offset of the overall download range
        // bytes at this position are written to position 0 of `buffer`
        let src_offset = response_analysis.overall_download_range.start;

        let mut download_workers = Vec::with_capacity(parallel);
        download_workers.push(start_initial_download_task_channel(
            initial_response,
            0,
            tx.clone(),
        ));

        while let Some(range) = response_analysis.remaining_download_ranges.pop_front() {
            while download_workers.len() >= parallel {
                (_, _, download_workers) = future::select_all(download_workers).await;
            }
            download_workers.push(start_download_task_channel(
                client.clone(),
                range.clone(),
                range.start - src_offset,
                etag_lock.clone(),
                tx.clone(),
            ));
        }
        if let Err(error) = future::try_join_all(download_workers).await {
            _ = tx.send(Err(map_spawned_task_error(error))).await;
        }
    }));

    let mut total_read = 0;
    while let Ok(msg) = rx.recv().await {
        let (write_offset, bytes) = msg?;
        if write_offset
            .checked_add(bytes.len())
            .ok_or_else(insufficient_buffer_err)?
            > buffer.len()
        {
            return Err(insufficient_buffer_err());
        }
        buffer[write_offset..write_offset + bytes.len()].copy_from_slice(&bytes);
        total_read += bytes.len();
    }

    let expected_total_read = response_analysis.overall_download_range.len();
    if expected_total_read != total_read {
        return Err(Error::with_message(
            ErrorKind::Other,
            format!(
                "Download incomplete. Expected to read {} bytes, but read {} bytes. Bytes may not have been written to buffer in the correct positions.",
                expected_total_read, total_read
            ),
        ));
    }
    downloads_manager_handle
        .await
        .map_err(map_spawned_task_error)?;
    Ok(total_read)
}

async fn get_initial_response_and_analyze<Behavior>(
    range: Option<Range<usize>>,
    partition_size: usize,
    client: Arc<Behavior>,
) -> AzureResult<(AsyncRawResponse, Option<InitialResponseAnalysis>)>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    // Outer bound estimate of the resource range that will be downloaded. The actual download
    // range will never exceed these bounds, but it may be smaller, based on the actual size
    // of the remote resource.
    let max_download_range = range.unwrap_or(0..usize::MAX);
    if max_download_range.is_empty() {
        return Err(Error::with_message(
            ErrorKind::Other,
            "Provided range must have length > 0.",
        ));
    }

    let initial_response = download_with_empty_blob_safety(
        client.as_ref(),
        max_download_range.start
            ..min(
                max_download_range.end,
                max_download_range.start.saturating_add(partition_size),
            ),
    )
    .await?;

    let stats =
        analyze_initial_response(&initial_response, partition_size, max_download_range.end)?;

    Ok((initial_response, stats))
}

/// Race awaiting a message vs checking if tasks have completed successfully,
/// until either message is received or a task failure is found.
///
/// # Returns
///
/// - Returns Ok with received message and remaining un-joined tasks.
/// - Returns Err if channel closed before a message received.
/// - Returns Err if joined task closed with an error.
async fn await_message_while_joining_workers<T>(
    receiver: &mut UnboundedReceiver<T>,
    mut task_bucket: Vec<SpawnedTask>,
) -> AzureResult<(T, Vec<SpawnedTask>)> {
    let on_recv_err = |_| {
        Error::with_message(
            ErrorKind::Other,
            "Download incomplete. Premature channel close.",
        )
    };

    let mut message_fut = receiver.recv();
    // `task_bucket` may be empty. `select_all` cannot handle that.
    while !task_bucket.is_empty() {
        match future::select(message_fut, future::select_all(task_bucket)).await {
            Either::Left((message, task_select)) => {
                return Ok((message.map_err(on_recv_err)?, task_select.into_inner()));
            }
            Either::Right(((completed_task, _, remaining_tasks), m_fut)) => {
                completed_task.map_err(map_spawned_task_error)?;
                task_bucket = remaining_tasks;
                message_fut = m_fut;
            }
        }
    }

    Ok((message_fut.await.map_err(on_recv_err)?, task_bucket))
}

/// Spawns a worker to take the given raw response and stream it into a buffer.
/// That buffer result is then sent through sender with chunk index 0.
fn start_initial_download_task_buffer(
    initial_response: AsyncRawResponse,
    mut sender: UnboundedSender<Result<(usize, Bytes), Error>>,
    active_tasks_counter: Arc<AtomicUsize>,
    partition_size: usize,
) -> SpawnedTask {
    get_async_runtime().spawn(Box::pin(async move {
        let mut dst = vec![0u8; partition_size];
        let res = initial_response
            .into_body()
            .collect_into(&mut dst)
            .await
            // this is the initial download task, it's chunk index is 0
            .map(|_| (0usize, dst.into()));
        active_tasks_counter.fetch_sub(1, Ordering::Relaxed);
        let _send_res = sender.send(res).await;
    }))
}

/// Spawns a worker to request the given range and stream it into a buffer.
/// That buffer result is then sent through sender with the given chunk index.
fn start_download_task_buffer<Behavior: PartitionedDownloadBehavior + Send + Sync + 'static>(
    client: Arc<Behavior>,
    range: Range<usize>,
    etag_lock: Option<Etag>,
    mut sender: UnboundedSender<Result<(usize, Bytes), Error>>,
    active_tasks_counter: Arc<AtomicUsize>,
    chunk_idx: usize,
) -> SpawnedTask {
    get_async_runtime().spawn(Box::pin(async move {
        let mut dst = vec![0u8; range.len()];
        let res = async {
            client
                .transfer_range(Some(range), etag_lock)
                .await?
                .into_body()
                .collect_into(&mut dst)
                .await
        }
        .await;
        if let Ok(count) = res {
            dst.truncate(count);
        }
        active_tasks_counter.fetch_sub(1, Ordering::Relaxed);
        let _send_res = sender.send(res.map(|_| (chunk_idx, dst.into()))).await;
    }))
}

/// Spawns a worker to take the given raw response and stream it through a channel.
///
/// # Arguments
///
/// - initial_response: the initial download response to stream into a channel.
/// - destination_offset: offset in the final destination buffer that this response is meant to be
///   written into.
/// - sender: channel to send network Bytes through along with the destination buffer offset to
///   write those exact bytes to.
fn start_initial_download_task_channel(
    initial_response: AsyncRawResponse,
    destination_offset: usize,
    sender: UnboundedSender<Result<(usize, Bytes), Error>>,
) -> SpawnedTask {
    get_async_runtime().spawn(Box::pin(body_to_channel(
        initial_response.into_body(),
        destination_offset,
        sender,
    )))
}

/// Spawns a worker to request the given range and stream it through a channel.
///
/// # Arguments
///
/// - client: Client to request a range from.
/// - range: The range to request.
/// - destination_offset: offset in the final destination buffer that the response is meant to be
///   written into.
/// - etag_lock: etag to lock on for this ranged request.
/// - sender: channel to send network Bytes through along with the destination buffer offset to
///   write those exact bytes to.
fn start_download_task_channel<Behavior: PartitionedDownloadBehavior + Send + Sync + 'static>(
    client: Arc<Behavior>,
    range: Range<usize>,
    destination_offset: usize,
    etag_lock: Option<Etag>,
    mut sender: UnboundedSender<Result<(usize, Bytes), Error>>,
) -> SpawnedTask {
    get_async_runtime().spawn(Box::pin(async move {
        let response = match client.transfer_range(Some(range), etag_lock).await {
            Ok(response) => response,
            Err(err) => {
                let _res = sender.send(Err(err)).await;
                return;
            }
        };
        body_to_channel(response.into_body(), destination_offset, sender).await;
    }))
}

async fn body_to_channel(
    mut body: AsyncResponseBody,
    mut destination_offset: usize,
    mut sender: UnboundedSender<Result<(usize, Bytes), Error>>,
) {
    const CHANNEL_BATCH_LEN: usize = 8;
    let mut batch_counter = BatchCounter::<CHANNEL_BATCH_LEN>::new();
    while let Some(result) = body.next().await {
        match result {
            Ok(bytes) => {
                let bytes_len = bytes.len();
                let _res = sender.feed(Ok((destination_offset, bytes))).await;
                destination_offset += bytes_len;
                if batch_counter.eval() {
                    let _res = sender.flush().await;
                }
            }
            Err(err) => {
                let _res = sender.send(Err(err)).await;
                return;
            }
        };
    }
    let _res = sender.flush().await;
}

/// Performs a `transfer_range()` call with the given range. If this results in a
/// RequestedRangeNotSatisfiable error, and if the requested range begins at the
/// start of the blob, retries the operation without a range argument.
/// This handles the service's edge case where a ranged get on an empty blob
/// always fails. Retrying with an empty range gives the correct empty blob data
/// as well as all the header information we expect.
async fn download_with_empty_blob_safety<Behavior>(
    client: &Behavior,
    range: Range<usize>,
) -> AzureResult<AsyncRawResponse>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let range_start = range.start;
    match client.transfer_range(Some(range), None).await {
        Ok(response) => Ok(response),
        Err(err) => match (err.http_status(), range_start) {
            (Some(StatusCode::RequestedRangeNotSatisfiable), 0) => {
                client.transfer_range(None, None).await
            }
            _ => Err(err),
        },
    }
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
/// Ok(None) if the appropriate information was not available.
///
/// Err(error) if there was an error parsing the appropriate information.
fn analyze_initial_response(
    initial_response: &AsyncRawResponse,
    partition_len: usize,
    max_range_end: usize,
) -> AzureResult<Option<InitialResponseAnalysis>> {
    if let Some(content_range) = initial_response
        .headers()
        .get_optional_as::<ContentRange, _>(&"content-range".into())?
    {
        if let (Some(received_range), Some(resource_len)) =
            (content_range.range, content_range.total_len)
        {
            let remainder_start = received_range.1;
            let remainder_end = min(max_range_end, resource_len);
            return Ok(Some(InitialResponseAnalysis {
                overall_download_range: received_range.0..remainder_end,
                initial_download_range: received_range.0..received_range.1,
                remaining_download_ranges: (remainder_start..remainder_end)
                    .step_by(partition_len)
                    .map(|i| i..min(i.saturating_add(partition_len), remainder_end))
                    .collect(),
            }));
        }
    }
    Ok(None)
}

fn map_spawned_task_error(err: Box<dyn std::error::Error + Send>) -> Error {
    Error::with_message(ErrorKind::Other, err.to_string())
}

/// Counter which evaluates to true exactly every n evaluations.
struct BatchCounter<const LEN: usize> {
    count: usize,
}
impl<const LEN: usize> BatchCounter<LEN> {
    fn new() -> Self {
        Self { count: 0 }
    }
    /// Evaluates to true every n evaluations, otherwise false.
    fn eval(&mut self) -> bool {
        self.count = (self.count + 1) % LEN;
        self.count == 0
    }
}

trait DownloadRangeFuture: Future + Send {}
impl<T: Future + Send> DownloadRangeFuture for T {}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use azure_core::{
        http::{
            headers::{Header, Headers},
            Etag, StatusCode,
        },
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

    #[derive(Clone, Debug)]
    enum MockPartitionedDownloadBehaviorInvocation {
        TransferRange(Option<Range<usize>>, Option<Etag>),
    }

    struct MockPartitionedDownloadBehavior {
        pub invocations: Mutex<Vec<MockPartitionedDownloadBehaviorInvocation>>,
        pub data: Bytes,
        pub delay_millis: Option<Range<u64>>,
        pub etag: Mutex<Option<Etag>>,
    }

    #[derive(Clone, Default)]
    struct MockOptions {
        /// When provided, will select a random sleep time in milliseconds within the given range.
        delay_millis_range: Option<Range<u64>>,

        /// Sets the initial ETag to match against and return in responses.
        etag: Option<Etag>,
    }

    impl MockPartitionedDownloadBehavior {
        pub fn new(data: impl Into<Bytes>, options: Option<MockOptions>) -> Self {
            Self {
                invocations: Mutex::new(vec![]),
                data: data.into(),
                delay_millis: options.clone().and_then(|o| o.delay_millis_range),
                etag: Mutex::new(options.clone().and_then(|o| o.etag)),
            }
        }
    }

    #[async_trait::async_trait]
    impl PartitionedDownloadBehavior for MockPartitionedDownloadBehavior {
        async fn transfer_range(
            &self,
            requested_range: Option<Range<usize>>,
            etag_lock: Option<Etag>,
        ) -> AzureResult<AsyncRawResponse> {
            {
                self.invocations.lock().await.push(
                    MockPartitionedDownloadBehaviorInvocation::TransferRange(
                        requested_range.clone(),
                        etag_lock.clone(),
                    ),
                );
            }

            // if etag lock AND if the mock is configured to use it, error if mismatch
            if let Some(etag) = etag_lock {
                if let Some(mock_etag) = self.etag.lock().await.as_ref() {
                    if etag != *mock_etag {
                        return Err(Error::with_message(ErrorKind::Other, "Mismatched etag"));
                    }
                }
            }

            if let Some(delay_millis_range) = self.delay_millis.clone() {
                let millis = rand::random_range(delay_millis_range);
                sleep(Duration::from_millis(millis)).await
            }

            struct ContentLength(usize);
            impl Header for ContentLength {
                fn name(&self) -> azure_core::http::headers::HeaderName {
                    "content-length".into()
                }
                fn value(&self) -> azure_core::http::headers::HeaderValue {
                    self.0.to_string().into()
                }
            }
            let mut headers = Headers::new();
            if let Some(etag) = self.etag.lock().await.as_ref() {
                headers.insert("etag", etag.to_string());
            }
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
                    if !range.is_empty() {
                        headers.add(ContentRange {
                            range: Some((range.start, range.end - 1)),
                            total_len: Some(self.data.len()),
                        })?
                    };
                    headers.add(ContentLength(range.len()))?;
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
                    headers.add(ContentLength(0))?;
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
                    headers.add(ContentLength(data_len))?;
                    Ok(AsyncRawResponse::new(
                        StatusCode::Ok,
                        headers,
                        Box::pin(BytesStream::from(self.data.clone())),
                    ))
                }
            }
        }
    }

    struct SingleRangeArgSet {
        partition_len: usize,
        download_range: Option<(usize, usize)>,
    }
    fn single_range_args(data_len: usize) -> impl IntoIterator<Item = SingleRangeArgSet> {
        // trait not implemented for usize
        let part_len = data_len / 5;
        let extra = data_len / 5;
        let offset = data_len / 5;

        let start_range = (0, part_len);
        let mid_range = (offset, offset + part_len);
        let end_range = (data_len - part_len, data_len);
        [
            // exact len
            SingleRangeArgSet {
                partition_len: data_len,
                download_range: None,
            },
            // oversize len
            SingleRangeArgSet {
                partition_len: data_len + extra,
                download_range: None,
            },
            // exact range len (start)
            SingleRangeArgSet {
                partition_len: part_len,
                download_range: Some(start_range),
            },
            // oversize range len (start)
            SingleRangeArgSet {
                partition_len: part_len + extra,
                download_range: Some(start_range),
            },
            // exact range len (mid))
            SingleRangeArgSet {
                partition_len: part_len,
                download_range: Some(mid_range),
            },
            // oversize range len (mid))
            SingleRangeArgSet {
                partition_len: part_len + extra,
                download_range: Some(mid_range),
            },
            // exact range len (end)
            SingleRangeArgSet {
                partition_len: part_len,
                download_range: Some(end_range),
            },
            // oversize range len (end)
            SingleRangeArgSet {
                partition_len: part_len + extra,
                download_range: Some(end_range),
            },
        ]
    }

    #[tokio::test]
    async fn download_single_range() -> AzureResult<()> {
        const DATA_LEN: usize = 1024;
        const PARALLEL: usize = 2;

        let data = get_random_data(DATA_LEN);

        for args in single_range_args(DATA_LEN) {
            let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

            let mut body = download(
                args.download_range.map(|r| (r.0..r.1).into()),
                PARALLEL.try_into().unwrap(),
                args.partition_len.try_into().unwrap(),
                mock.clone(),
            )
            .await?
            .into_body();
            let downloaded_data = body.buffer_all().await?;

            assert_eq!(
                &downloaded_data[..],
                match args.download_range {
                    Some(r) => &data[r.0..r.1],
                    None => &data[..],
                }
            );
            assert_eq!(mock.invocations.lock().await.len(), 1);
        }

        Ok(())
    }

    #[tokio::test]
    async fn download_into_single_range() -> AzureResult<()> {
        const DATA_LEN: usize = 1024;
        const PARALLEL: usize = 2;

        let data = get_random_data(DATA_LEN);
        let mut buffer = [0; DATA_LEN];

        for SingleRangeArgSet {
            partition_len,
            download_range,
        } in single_range_args(DATA_LEN)
        {
            let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));
            for elem in buffer.iter_mut() {
                *elem = 0;
            }
            let download_len = download_range.map_or(DATA_LEN, |r| r.1 - r.0);
            let dst = &mut buffer[..download_len];

            let copied = download_into(
                dst,
                download_range.map(|r| (r.0..r.1).into()),
                PARALLEL.try_into().unwrap(),
                partition_len.try_into().unwrap(),
                mock.clone(),
            )
            .await?;

            assert_eq!(copied, download_len);
            assert_eq!(dst, download_range.map_or(&data[..], |r| &data[r.0..r.1]));
            assert_eq!(mock.invocations.lock().await.len(), 1);
        }

        Ok(())
    }

    struct MultiRangeArgSet {
        pub parallel: usize,
        pub partition_len: usize,
        pub download_range: Option<(usize, usize)>,
        pub expected_parts: usize,
    }
    fn multi_range_args(data_len: usize) -> impl IntoIterator<Item = MultiRangeArgSet> {
        let offset = data_len / 9;
        let range_len = data_len / 9;

        let mut combos = Vec::new();
        for parallel in [1, 4] {
            for blob_range in [
                (0, range_len),
                (offset, offset + range_len),
                (data_len - range_len, data_len),
            ] {
                for (partition_len, download_range) in [
                    (data_len - 1, None),              // barely smaller
                    (data_len / 2, None),              // half size
                    (data_len / 41, None),             // oddball size
                    (range_len - 1, Some(blob_range)), // barely smaller, range
                    (range_len / 2, Some(blob_range)), // half size, range
                    (data_len / 41, Some(blob_range)), // oddball size, range
                ] {
                    let expected_parts = match download_range {
                        Some((start, end)) => (end - start).div_ceil(partition_len),
                        None => data_len.div_ceil(partition_len),
                    };
                    combos.push(MultiRangeArgSet {
                        parallel,
                        partition_len,
                        download_range,
                        expected_parts,
                    });
                }
            }
        }

        combos
    }

    #[tokio::test]
    async fn download_multi_range() -> AzureResult<()> {
        const DATA_LEN: usize = 4096;

        let data = get_random_data(DATA_LEN);

        for args in multi_range_args(DATA_LEN) {
            let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

            let mut body = download(
                args.download_range.map(|r| (r.0..r.1).into()),
                args.parallel.try_into().unwrap(),
                args.partition_len.try_into().unwrap(),
                mock.clone(),
            )
            .await?
            .into_body();
            let downloaded_data = body.buffer_all().await?;

            assert_eq!(
                downloaded_data.len(),
                args.download_range
                    .map_or(DATA_LEN, |range| range.1 - range.0),
                "Data mismatch. partition_len={}. download_range={:?}, expected_parts={}",
                args.partition_len,
                args.download_range,
                args.expected_parts
            );
            assert_eq!(
                &downloaded_data[..],
                match args.download_range {
                    Some(r) => &data[r.0..r.1],
                    None => &data[..],
                },
                "Data mismatch. partition_len={}. download_range={:?}, expected_parts={}",
                args.partition_len,
                args.download_range,
                args.expected_parts
            );
            assert_eq!(
                mock.invocations.lock().await.len(),
                args.expected_parts,
                "Unexpected invocation count. partition_len={}. download_range={:?}, expected_parts={}",
                args.partition_len,
                args.download_range,
                args.expected_parts);
        }

        Ok(())
    }

    #[tokio::test]
    async fn download_into_multi_range() -> AzureResult<()> {
        const DATA_LEN: usize = 4096;

        let data = get_random_data(DATA_LEN);
        let mut buffer = [0; DATA_LEN];

        for MultiRangeArgSet {
            parallel,
            partition_len,
            download_range,
            expected_parts,
        } in multi_range_args(DATA_LEN)
        {
            let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));
            for elem in buffer.iter_mut() {
                *elem = 0;
            }
            let download_len = download_range.map_or(DATA_LEN, |r| r.1 - r.0);
            let dst = &mut buffer[..download_len];

            let copied = download_into(
                dst,
                download_range.map(|r| (r.0..r.1).into()),
                parallel.try_into().unwrap(),
                partition_len.try_into().unwrap(),
                mock.clone(),
            )
            .await?;

            assert_eq!(
                copied, download_len,
                "Data mismatch. partition_len={}. download_range={:?}, expected_parts={}",
                partition_len, download_range, expected_parts
            );
            assert_eq!(
                dst,
                download_range.map_or(&data[..], |r| &data[r.0..r.1]),
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
            Some(MockOptions {
                delay_millis_range: Some(1..5),
                ..Default::default()
            }),
        ));

        let mut body = download(None, parallel, partition_size, mock.clone())
            .await?
            .into_body();
        let downloaded_data = body.buffer_all().await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), segments);

        Ok(())
    }

    #[tokio::test]
    async fn download_empty_resource() -> AzureResult<()> {
        let parallel = NonZero::new(1).unwrap();
        let partition_len = NonZero::new(MB).unwrap();
        let data = get_random_data(0);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

        let mut body = download(None, parallel, partition_len, mock.clone())
            .await?
            .into_body();
        let downloaded_data = body.buffer_all().await?;

        assert_eq!(downloaded_data.len(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn download_into_empty_resource() -> AzureResult<()> {
        let parallel = NonZero::new(1).unwrap();
        let partition_len = NonZero::new(MB).unwrap();
        let data = get_random_data(0);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None));

        let copied =
            download_into(&mut [0; 1024], None, parallel, partition_len, mock.clone()).await?;

        assert_eq!(copied, 0);

        Ok(())
    }

    #[tokio::test]
    async fn download_etag_lock() -> AzureResult<()> {
        let configured_etag = Some(Etag::from("some_etag"));
        let data_len: usize = 1024;
        let partition_len = NonZero::new(data_len / 4).unwrap();
        let parallel = NonZero::new(2).unwrap();

        let data = get_random_data(data_len);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(
            data.clone(),
            Some(MockOptions {
                etag: configured_etag.clone(),
                ..Default::default()
            }),
        ));

        download(None, parallel, partition_len, mock.clone())
            .await?
            .into_body()
            .collect()
            .await?;

        let mut invocations: VecDeque<_> = mock.invocations.lock().await.iter().cloned().collect();

        // Assert first request doesn't supply a lock, as it hasn't received a tag to lock on yet.
        match invocations.pop_front().unwrap() {
            MockPartitionedDownloadBehaviorInvocation::TransferRange(_, received_etag) => {
                assert_eq!(received_etag, None)
            }
        };

        // Assert subsequent requests supply the correct lock.
        assert!(!invocations.is_empty());
        match invocations.pop_front().unwrap() {
            MockPartitionedDownloadBehaviorInvocation::TransferRange(_, received_etag) => {
                assert_eq!(received_etag, configured_etag)
            }
        };

        Ok(())
    }

    #[tokio::test]
    async fn download_into_etag_lock() -> AzureResult<()> {
        let configured_etag = Some(Etag::from("some_etag"));
        const DATA_LEN: usize = 1024;
        let partition_len = NonZero::new(DATA_LEN / 4).unwrap();
        let parallel = NonZero::new(2).unwrap();

        let data = get_random_data(DATA_LEN);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(
            data.clone(),
            Some(MockOptions {
                etag: configured_etag.clone(),
                ..Default::default()
            }),
        ));

        download_into(
            &mut [0; DATA_LEN],
            None,
            parallel,
            partition_len,
            mock.clone(),
        )
        .await?;

        let mut invocations: VecDeque<_> = mock.invocations.lock().await.iter().cloned().collect();

        // Assert first request doesn't supply a lock, as it hasn't received a tag to lock on yet.
        match invocations.pop_front().unwrap() {
            MockPartitionedDownloadBehaviorInvocation::TransferRange(_, received_etag) => {
                assert_eq!(received_etag, None)
            }
        };

        // Assert subsequent requests supply the correct lock.
        assert!(!invocations.is_empty());
        match invocations.pop_front().unwrap() {
            MockPartitionedDownloadBehaviorInvocation::TransferRange(_, received_etag) => {
                assert_eq!(received_etag, configured_etag)
            }
        };

        Ok(())
    }

    #[tokio::test]
    async fn download_fails_on_etag_update() -> AzureResult<()> {
        let configured_etag_1 = Some(Etag::from("some_etag"));
        let configured_etag_2 = Some(Etag::from("another_etag"));
        let data_len: usize = 2048;
        let total_partitions = 8;
        let partition_len = NonZero::new(data_len / total_partitions).unwrap();
        let parallel = NonZero::new(1).unwrap();

        let individual_request_delay_ms = 5;
        let etag_edit_delay_ms = individual_request_delay_ms * total_partitions as u64 / 2;

        let data = get_random_data(data_len);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(
            data.clone(),
            Some(MockOptions {
                etag: configured_etag_1.clone(),
                delay_millis_range: Some(
                    individual_request_delay_ms..individual_request_delay_ms + 1,
                ),
            }),
        ));

        let (download_result, _) = futures::future::join(
            async {
                download(None, parallel, partition_len, mock.clone())
                    .await?
                    .into_body()
                    .collect()
                    .await
            },
            async {
                sleep(Duration::from_millis(etag_edit_delay_ms)).await;
                *mock.clone().etag.lock().await = configured_etag_2;
            },
        )
        .await;

        assert!(download_result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn download_into_fails_on_etag_update() -> AzureResult<()> {
        let configured_etag_1 = Some(Etag::from("some_etag"));
        let configured_etag_2 = Some(Etag::from("another_etag"));
        const DATA_LEN: usize = 2048;
        let total_partitions = 8;
        let partition_len = NonZero::new(DATA_LEN / total_partitions).unwrap();
        let parallel = NonZero::new(1).unwrap();

        let individual_request_delay_ms = 5;
        let etag_edit_delay_ms = individual_request_delay_ms * total_partitions as u64 / 2;

        let data = get_random_data(DATA_LEN);
        let mock = Arc::new(MockPartitionedDownloadBehavior::new(
            data.clone(),
            Some(MockOptions {
                etag: configured_etag_1.clone(),
                delay_millis_range: Some(
                    individual_request_delay_ms..individual_request_delay_ms + 1,
                ),
            }),
        ));

        let (download_result, _) = futures::future::join(
            download_into(
                &mut [0; DATA_LEN],
                None,
                parallel,
                partition_len,
                mock.clone(),
            ),
            async {
                sleep(Duration::from_millis(etag_edit_delay_ms)).await;
                *mock.clone().etag.lock().await = configured_etag_2;
            },
        )
        .await;

        assert!(download_result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn download_into_insufficient_buffer() -> AzureResult<()> {
        const DATA_LEN: usize = 1024;
        let data = get_random_data(DATA_LEN);
        for (buffer_len, range) in [
            (0, None),
            (DATA_LEN - 1, None),
            (DATA_LEN / 2, None),
            (0, Some(0..1)),
            (0, Some(101..102)),
            (0, Some(0..DATA_LEN)),
            (DATA_LEN - 1, Some(0..DATA_LEN)),
            (DATA_LEN / 2, Some(0..DATA_LEN / 2 + 1)),
        ] {
            let http_range = range.map(HttpRange::from);
            for parallel in [1, 8] {
                for partition_len in [DATA_LEN * 2, DATA_LEN / 8] {
                    let download_result = download_into(
                        &mut vec![0; buffer_len],
                        http_range.clone(),
                        NonZero::new(parallel).unwrap(),
                        NonZero::new(partition_len).unwrap(),
                        Arc::new(MockPartitionedDownloadBehavior::new(data.clone(), None)),
                    )
                    .await;
                    assert!(download_result.is_err(), "Expected error for buffer_len: {}, range: {:?}, parallel: {}, partition_len: {}", buffer_len, http_range, parallel, partition_len);
                }
            }
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
