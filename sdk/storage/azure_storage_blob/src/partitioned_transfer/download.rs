// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{cmp::min, ops::Range};

use azure_core::http::AsyncRawResponse;
use bytes::Bytes;
use futures::{stream::FuturesOrdered, StreamExt};

use crate::models::content_range::ContentRange;

use super::*;

#[async_trait::async_trait]
pub(crate) trait PartitionedDownloadBehavior<'a, Options>
where
    Options: 'a,
{
    async fn transfer_range(
        &'a self,
        range: Range<u64>,
        options: Options,
    ) -> AzureResult<AsyncRawResponse>;
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
pub(crate) async fn download<'a, Behavior, Options>(
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: &'a Behavior,
    options: Options,
) -> AzureResult<Pin<Box<dyn Stream<Item = AzureResult<Bytes>> + Unpin + 'a>>>
where
    Behavior: PartitionedDownloadBehavior<'a, Options>,
    Options: Clone + 'a,
{
    let parallel = parallel.get();
    let partition_size = partition_size.get() as u64;
    let initial_response = client
        .transfer_range(0..partition_size, options.clone())
        .await?;
    let content_range: ContentRange = initial_response.headers().get_as(&"content-range".into())?;
    let total_ranges = content_range.total_length().div_ceil(partition_size);

    let mut ranges = (1..total_ranges).map(move |i| {
        i * partition_size
            ..min(
                i * partition_size + partition_size,
                content_range.total_length(),
            )
    });

    // the first operation has a different type from the others.
    // fully type this variable out to specify dyn.
    let fut: Pin<Box<dyn Future<Output = AzureResult<Bytes>>>> =
        Box::pin(initial_response.into_body().collect());
    let mut ops = FuturesOrdered::new();
    ops.push_back(fut);

    let stream = futures::stream::poll_fn(move |cx| {
        // fill to max parallel ops
        while ops.len() < parallel {
            match ranges.next() {
                Some(range) => ops.push_back(Box::pin(download_range_to_bytes(
                    client,
                    options.clone(),
                    range,
                ))),
                None => break,
            }
        }

        ops.poll_next_unpin(cx)
    });

    Ok(Box::pin(stream))
}

async fn download_range_to_bytes<'a, Behavior, Options>(
    client: &'a Behavior,
    options: Options,
    range: Range<u64>,
) -> AzureResult<Bytes>
where
    Behavior: PartitionedDownloadBehavior<'a, Options>,
    Options: Clone + 'a,
{
    let response = client.transfer_range(range, options).await?;
    response.into_body().collect().await
}

#[cfg(test)]
mod tests {
    use std::cmp::{max, min};

    use azure_core::{
        http::{headers::Headers, StatusCode},
        stream::BytesStream,
    };

    use tokio::{
        sync::Mutex,
        time::{sleep, Duration},
    };

    use super::*;

    #[derive(Debug)]
    enum MockPartitionedDownloadBehaviorInvocation {
        TransferRange(Range<u64>),
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
    impl<'a> PartitionedDownloadBehavior<'a, ()> for MockPartitionedDownloadBehavior {
        async fn transfer_range(
            &'a self,
            range: Range<u64>,
            _options: (),
        ) -> AzureResult<AsyncRawResponse> {
            {
                self.invocations.lock().await.push(
                    MockPartitionedDownloadBehaviorInvocation::TransferRange(range.clone()),
                );
            }

            if let Some(delay_millis_range) = self.delay_millis.clone() {
                let millis = rand::random_range(delay_millis_range);
                sleep(Duration::from_millis(millis)).await
            }

            let range = max(range.start, 0)..min(range.end, self.data.len() as u64);
            let mut headers = Headers::new();
            headers.insert(
                "content-range",
                ContentRange::new(range.start, range.end - 1, self.data.len() as u64).to_string(),
            );
            let range = range.start as usize..range.end as usize;
            let raw = AsyncRawResponse::new(
                StatusCode::PartialContent,
                headers,
                Box::pin(BytesStream::from(self.data.slice(range))),
            );
            Ok(raw)
        }
    }

    #[tokio::test]
    async fn download_single_range_oversized() -> AzureResult<()> {
        let data_size: usize = 123;
        let partition_size = NonZero::new(1024).unwrap();
        let parallel = NonZero::new(2).unwrap();

        let data = get_random_data(data_size);
        let mock = MockPartitionedDownloadBehavior::new(data.clone(), None);

        let downloaded_data = download(parallel, partition_size, &mock, ())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn download_single_range_exact() -> AzureResult<()> {
        let data_size: usize = 1024;
        let partition_size = NonZero::new(1024).unwrap();
        let parallel = NonZero::new(2).unwrap();

        let data = get_random_data(data_size);
        let mock = MockPartitionedDownloadBehavior::new(data.clone(), None);

        let downloaded_data = download(parallel, partition_size, &mock, ())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), 1);

        Ok(())
    }

    #[tokio::test]
    async fn download_multi_range_exact() -> AzureResult<()> {
        let segments = 8;
        let data_size: usize = 1024 * segments;
        let partition_size = NonZero::new(1024).unwrap();
        let parallel = NonZero::new(2).unwrap();

        let data = get_random_data(data_size);
        let mock = MockPartitionedDownloadBehavior::new(data.clone(), None);

        let downloaded_data = download(parallel, partition_size, &mock, ())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), segments);

        Ok(())
    }

    #[tokio::test]
    async fn download_multi_range_partial() -> AzureResult<()> {
        let segments = 8;
        let data_size: usize = 1024 * (segments - 1) + 123;
        let partition_size = NonZero::new(1024).unwrap();
        let parallel = NonZero::new(2).unwrap();

        let data = get_random_data(data_size);
        let mock = MockPartitionedDownloadBehavior::new(data.clone(), None);

        let downloaded_data = download(parallel, partition_size, &mock, ())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), segments);

        Ok(())
    }

    #[tokio::test]
    async fn download_ranges_sequential() -> AzureResult<()> {
        let segments: usize = 8;
        let partition_size = NonZero::new(1024).unwrap();
        let parallel = NonZero::new(2).unwrap();
        let data_size: usize = partition_size.get() * segments;

        let data = get_random_data(data_size);
        let mock = MockPartitionedDownloadBehavior::new(data.clone(), None);

        let downloaded_data = download(parallel, partition_size, &mock, ())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), segments);

        Ok(())
    }

    #[tokio::test]
    async fn download_ranges_parallel_maintain_order() -> AzureResult<()> {
        let segments: usize = 20;
        let partition_size = NonZero::new(3).unwrap();
        let parallel = NonZero::new(16).unwrap();
        let data_size: usize = partition_size.get() * segments;

        let data = get_random_data(data_size);
        let mock = MockPartitionedDownloadBehavior::new(data.clone(), Some(1..5));

        let downloaded_data = download(parallel, partition_size, &mock, ())
            .await?
            .buffer_all()
            .await?;

        assert_eq!(downloaded_data[..], data[..]);
        assert_eq!(mock.invocations.lock().await.len(), segments);

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
