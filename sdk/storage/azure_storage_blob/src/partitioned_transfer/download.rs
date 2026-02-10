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

use crate::{conditional_send::ConditionalSend, models::content_range::ContentRange};

use super::*;

#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
pub(crate) trait PartitionedDownloadBehavior {
    async fn transfer_range(&self, range: Option<Range<u64>>) -> AzureResult<AsyncRawResponse>;
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
    range: Option<Range<u64>>,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: Arc<Behavior>,
) -> AzureResult<PinnedStream>
where
    Behavior: PartitionedDownloadBehavior + Send + Sync + 'static,
{
    let parallel = parallel.get();
    let partition_size = partition_size.get() as u64;
    let range = range.unwrap_or(0..u64::MAX);
    if range.is_empty() {
        let raw_stream: PinnedStream = Box::pin(BytesStream::new_empty());
        return Ok(raw_stream);
    }

    let initial_response = client
        .transfer_range(Some(
            range.start..min(range.end, range.start.saturating_add(partition_size)),
        ))
        .await;
    let initial_response = match initial_response {
        Ok(response) => response,
        Err(err) => match (err.http_status(), range.start) {
            (Some(StatusCode::RequestedRangeNotSatisfiable), 0) => {
                client.transfer_range(None).await?
            }
            _ => Err(err)?,
        },
    };

    let mut ranges: VecDeque<_> = match initial_response
        .headers()
        .get_as::<ContentRange, _>(&"content-range".into())
    {
        Ok(content_range) => {
            let remainder_start = content_range.end() + 1;
            let remainder_end = min(range.end, content_range.total_length());
            (remainder_start..remainder_end)
                .step_by(partition_size as usize)
                .map(move |i| i..min(i + partition_size, remainder_end))
                .collect()
        }
        Err(_) => VecDeque::new(),
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
    range: Range<u64>,
) -> AzureResult<Bytes> {
    let response = client.transfer_range(Some(range)).await?;
    response.into_body().collect().await
}

trait DownloadRangeFuture: Future + ConditionalSend {}
impl<T: Future + ConditionalSend> DownloadRangeFuture for T {}

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
        TransferRange(Option<Range<u64>>),
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
            requested_range: Option<Range<u64>>,
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
                    if range.start >= data_len as u64 {
                        return Err(ErrorKind::HttpResponse {
                            status: StatusCode::RequestedRangeNotSatisfiable,
                            error_code: Some("InvalidRange".into()),
                            raw_response: None,
                        }
                        .into_error());
                    }
                    let range = range.start..min(range.end, data_len as u64);
                    headers.insert(
                        "content-range",
                        ContentRange::new(range.start, range.end - 1, self.data.len() as u64)
                            .to_string(),
                    );
                    let range = range.start as usize..range.end as usize;
                    Ok(AsyncRawResponse::new(
                        StatusCode::PartialContent,
                        headers,
                        Box::pin(BytesStream::from(self.data.slice(range))),
                    ))
                }
                (None, 0) => Ok(AsyncRawResponse::new(
                    StatusCode::Ok,
                    headers,
                    Box::pin(BytesStream::new_empty()),
                )),
                (None, data_len) => {
                    headers.insert(
                        "content-range",
                        ContentRange::new(0, data_len as u64 - 1, data_len as u64).to_string(),
                    );
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

        // trait not implemented for usize??
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
                download_range.map(|r| r.0 as u64..r.1 as u64),
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
                        download_range.map(|r| r.0 as u64..r.1 as u64),
                        parallel.try_into().unwrap(),
                        partition_len.try_into().unwrap(),
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
