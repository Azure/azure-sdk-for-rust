// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::Body;
use bytes::Bytes;

#[cfg(not(target_arch = "wasm32"))]
use azure_core::stream::SeekableStream;
#[cfg(not(target_arch = "wasm32"))]
use futures::StreamExt;

#[cfg(not(target_arch = "wasm32"))]
use crate::streams::partitioned_stream::PartitionedStream;

use super::*;

#[async_trait::async_trait]
pub(crate) trait PartitionedUploadBehavior {
    async fn transfer_oneshot(&self, content: Body) -> AzureResult<()>;
    async fn transfer_partition(&self, offset: usize, content: Body) -> AzureResult<()>;
    async fn initialize(&self, content_len: usize) -> AzureResult<()>;
    async fn finalize(&self) -> AzureResult<()>;
}

pub(crate) async fn upload(
    content: Body,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: &impl PartitionedUploadBehavior,
) -> AzureResult<()> {
    if content.len() <= partition_size.get() {
        client.transfer_oneshot(content).await?;
        return Ok(());
    }

    client.initialize(content.len()).await?;

    match content {
        Body::Bytes(bytes) => {
            upload_bytes_partitions(bytes, parallel, partition_size, client).await?;
        }
        #[cfg(not(target_arch = "wasm32"))]
        Body::SeekableStream(seekable_stream) => {
            upload_stream_partitions(seekable_stream, parallel, partition_size, client).await?;
        }
    }

    client.finalize().await?;

    Ok(())
}

async fn upload_bytes_partitions(
    content: Bytes,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: &impl PartitionedUploadBehavior,
) -> AzureResult<()> {
    let part_size_actual = partition_size.get();
    let num_partitions = content.len().div_ceil(part_size_actual);
    let partitions = (0..num_partitions).map(|part| {
        let offset = part * part_size_actual;
        let range = offset..std::cmp::min(offset + part_size_actual, content.len());
        (offset, content.slice(range))
    });
    let ops = partitions
        .map(|(offset, bytes)| Ok(move || client.transfer_partition(offset, Body::Bytes(bytes))));
    run_all_with_concurrency_limit(futures::stream::iter(ops), parallel).await?;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
async fn upload_stream_partitions(
    content: Box<dyn SeekableStream>,
    parallel: NonZero<usize>,
    partition_size: NonZero<usize>,
    client: &impl PartitionedUploadBehavior,
) -> AzureResult<()> {
    let partitions =
        PartitionedStream::new(content, partition_size).scan(0, |enumerated_bytes, result| {
            match result {
                Ok(bytes) => {
                    let offset = *enumerated_bytes;
                    *enumerated_bytes += bytes.len();
                    future::ready(Some(Ok((offset, bytes))))
                }
                Err(e) => future::ready(Some(Err(e))),
            }
        });
    let ops = partitions
        .map_ok(|(offset, bytes)| move || client.transfer_partition(offset, Body::Bytes(bytes)));
    run_all_with_concurrency_limit(ops, parallel).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::mem::discriminant;

    use azure_core::{http::Body, stream::BytesStream};
    use azure_storage_blob_test::*;
    use bytes::Bytes;
    use tokio::sync::Mutex;

    use super::*;

    /// The possible body types for a body passed to a PartitionedUploadBehavior.
    /// For call history tracking purposes.
    #[derive(Debug, Clone, Copy)]
    enum BodyType {
        Bytes,
        #[cfg(not(target_arch = "wasm32"))]
        SeekableStream,
    }

    /// Record of a call made to a PartitionedUploadBehavior
    #[derive(Debug)]
    enum MockPartitionedUploadBehaviorInvocation {
        Initialize(usize),
        TransferOneshot(Bytes, BodyType),
        TransferPartition(usize, Bytes, BodyType),
        Finalize(),
    }

    /// Mock of a PartitionedUploadBehavior. Keeps a record of all calls made to it.
    struct MockPartitionedUploadBehavior {
        pub invocations: Mutex<Vec<MockPartitionedUploadBehaviorInvocation>>,
    }

    impl MockPartitionedUploadBehavior {
        pub fn new() -> Self {
            Self {
                invocations: Mutex::new(vec![]),
            }
        }
    }

    #[async_trait::async_trait]
    impl PartitionedUploadBehavior for MockPartitionedUploadBehavior {
        async fn transfer_oneshot(&self, mut content: Body) -> AzureResult<()> {
            let body_type = match content {
                Body::Bytes(_) => BodyType::Bytes,
                #[cfg(not(target_arch = "wasm32"))]
                Body::SeekableStream(_) => BodyType::SeekableStream,
            };
            let bytes = content.collect_bytes().await?;
            self.invocations.lock().await.push(
                MockPartitionedUploadBehaviorInvocation::TransferOneshot(bytes, body_type),
            );
            Ok(())
        }

        async fn transfer_partition(&self, offset: usize, mut content: Body) -> AzureResult<()> {
            let body_type = match content {
                Body::Bytes(_) => BodyType::Bytes,
                #[cfg(not(target_arch = "wasm32"))]
                Body::SeekableStream(_) => BodyType::SeekableStream,
            };
            let bytes = content.collect_bytes().await?;
            self.invocations.lock().await.push(
                MockPartitionedUploadBehaviorInvocation::TransferPartition(
                    offset, bytes, body_type,
                ),
            );
            Ok(())
        }

        async fn initialize(&self, content_len: usize) -> AzureResult<()> {
            self.invocations.lock().await.push(
                MockPartitionedUploadBehaviorInvocation::Initialize(content_len),
            );
            Ok(())
        }

        async fn finalize(&self) -> AzureResult<()> {
            self.invocations
                .lock()
                .await
                .push(MockPartitionedUploadBehaviorInvocation::Finalize());
            Ok(())
        }
    }

    #[tokio::test]
    async fn one_shot_bytes_when_within_partition_size() -> AzureResult<()> {
        let data_size: usize = 1024;
        let partition_size: usize = data_size;
        let concurrency: usize = 2;

        let mock = MockPartitionedUploadBehavior::new();
        let src_data = get_random_data(data_size);

        upload(
            Body::Bytes(Bytes::from(src_data.clone())),
            NonZero::new(concurrency).unwrap(),
            NonZero::new(partition_size).unwrap(),
            &mock,
        )
        .await?;

        assert_upload_oneshot_invocations(&mock, &src_data[..], BodyType::Bytes).await;

        Ok(())
    }

    #[tokio::test]
    async fn partition_bytes_when_over_partition_size() -> AzureResult<()> {
        let data_size: usize = 1024;
        let partition_size: usize = 50;
        let concurrency: usize = 2;

        let mock = MockPartitionedUploadBehavior::new();
        let src_data = get_random_data(data_size);

        upload(
            Body::Bytes(Bytes::from(src_data.clone())),
            NonZero::new(concurrency).unwrap(),
            NonZero::new(partition_size).unwrap(),
            &mock,
        )
        .await?;

        assert_upload_partitioned_invocations(
            &mock,
            &src_data[..],
            partition_size,
            BodyType::Bytes,
        )
        .await;

        Ok(())
    }

    #[tokio::test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn one_shot_stream_when_within_partition_size() -> AzureResult<()> {
        let data_size: usize = 1024;
        let partition_size: usize = data_size;
        let concurrency: usize = 2;

        let mock = MockPartitionedUploadBehavior::new();
        let src_data = get_random_data(data_size);

        upload(
            Body::SeekableStream(Box::new(BytesStream::new(Bytes::from(src_data.clone())))),
            NonZero::new(concurrency).unwrap(),
            NonZero::new(partition_size).unwrap(),
            &mock,
        )
        .await?;

        assert_upload_oneshot_invocations(&mock, &src_data[..], BodyType::SeekableStream).await;

        Ok(())
    }

    #[tokio::test]
    #[cfg(not(target_arch = "wasm32"))]
    async fn partition_stream_when_over_partition_size() -> AzureResult<()> {
        let data_size: usize = 1024;
        let partition_size: usize = 50;
        let concurrency: usize = 2;

        let mock = MockPartitionedUploadBehavior::new();
        let src_data = get_random_data(data_size);

        upload(
            Body::SeekableStream(Box::new(BytesStream::new(Bytes::from(src_data.clone())))),
            NonZero::new(concurrency).unwrap(),
            NonZero::new(partition_size).unwrap(),
            &mock,
        )
        .await?;

        assert_upload_partitioned_invocations(
            &mock,
            &src_data[..],
            partition_size,
            BodyType::Bytes,
        )
        .await;

        Ok(())
    }

    async fn assert_upload_oneshot_invocations(
        mock: &MockPartitionedUploadBehavior,
        original_data: &[u8],
        expected_body_type: BodyType,
    ) {
        let invocations = mock.invocations.lock().await;
        assert_eq!(invocations.len(), 1);
        assert!(matches!(
            &invocations[0],
            MockPartitionedUploadBehaviorInvocation::TransferOneshot(data, body_type)
                if data[..] == *original_data && discriminant(body_type) == discriminant(&expected_body_type)
        ));
    }

    async fn assert_upload_partitioned_invocations(
        mock: &MockPartitionedUploadBehavior,
        original_data: &[u8],
        partition_size: usize,
        expected_body_type: BodyType,
    ) {
        let expected_partitions = original_data.len().div_ceil(partition_size);
        let invocations = mock.invocations.lock().await;

        assert_eq!(invocations.len(), expected_partitions + 2);
        assert!(matches!(
            &invocations[0],
            MockPartitionedUploadBehaviorInvocation::Initialize(size) if *size == original_data.len()
        ));
        assert!(matches!(
            &invocations[invocations.len() - 1],
            MockPartitionedUploadBehaviorInvocation::Finalize()
        ));

        let mut sorted_transfer_partition_invocations: Vec<_> = invocations
            .iter()
            .filter_map(|invocation| match invocation {
                MockPartitionedUploadBehaviorInvocation::TransferPartition(
                    offset,
                    body,
                    body_type,
                ) => Some((*offset, body.clone(), *body_type)),
                _ => None,
            })
            .collect();
        sorted_transfer_partition_invocations
            .sort_by(|(left_offset, _, _), (right_offset, _, _)| left_offset.cmp(right_offset));

        assert_eq!(
            sorted_transfer_partition_invocations.len(),
            invocations.len() - 2
        );

        for (i, (offset, body, body_type)) in
            sorted_transfer_partition_invocations.iter().enumerate()
        {
            assert_eq!(*offset, i * partition_size);
            assert_eq!(body[..], original_data[*offset..*offset + body.len()]);
            assert_eq!(discriminant(body_type), discriminant(&expected_body_type));
        }
    }

    fn get_random_data(len: usize) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; len];
        rand::fill(&mut data[..]);
        data
    }
}
