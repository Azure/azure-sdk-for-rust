// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    collections::VecDeque,
    num::NonZero,
    sync::{Arc, LazyLock},
    time::Duration,
};

use async_trait::async_trait;
use azure_core::{
    error::ErrorKind,
    http::{Body, ClientMethodOptions, Context},
    Error, Result,
};
use azure_storage_blob::{
    models::{
        BlobClientDownloadOptions, BlobClientUploadOptions, BlobContainerClientCreateOptions,
        BlobContainerClientDeleteOptions,
    },
    BlobClient, BlobContainerClient,
};
use azure_storage_blob_test::{
    fault_injection::{self, FaultInjectionProbabilities},
    stress::{
        data,
        value_parsers::{non_zero_usize, simple_non_zero_len_u64, simple_non_zero_len_usize},
        StressRunOutput, StressTest, StressTestOperation,
    },
    OptionalTimeoutFutureExt,
};
use clap::Args;
use crc_fast::{CrcAlgorithm, Digest};
use futures::{channel::mpsc::UnboundedSender, future, lock::Mutex, SinkExt, TryStreamExt};
use uuid::Uuid;

const CRC_ALGORITHM: CrcAlgorithm = CrcAlgorithm::Crc64Nvme;

#[derive(Args, Debug)]
pub(crate) struct DownloadBlobsTestArgs {
    /// Number of blobs used across download operations.
    #[arg(long, default_value_t = 1, value_parser = non_zero_usize, value_name = "COUNT")]
    targets: usize,

    /// Concurrency value for download options.
    #[arg(long, default_value_t = 2, value_parser = non_zero_usize, value_name = "NUM WORKERS")]
    concurrency: usize,

    /// Block length for download options.
    #[arg(long, default_value_t = 4 << 20, value_parser = simple_non_zero_len_usize)]
    block_len: usize,

    /// Data length of blob(s) to download.
    #[arg(long, value_parser = simple_non_zero_len_u64)]
    data_len: u64,
}

impl DownloadBlobsTestArgs {
    pub fn as_test(
        &self,
        fault_options: &FaultInjectionProbabilities,
    ) -> Result<Box<dyn StressTest>> {
        Ok(Box::new(DownloadBlobsTest {
            container_client: crate::clients::get_container_client(fault_options)?,
            download_targets: self.targets,
            download_targets_len: self.data_len,
            download_target_name_queue: Arc::new(Mutex::new(VecDeque::with_capacity(self.targets))),
            parallel: NonZero::new(self.concurrency).ok_or_else(non_zero_err)?,
            chunk_len: NonZero::new(self.block_len).ok_or_else(non_zero_err)?,
        }))
    }
}

impl std::fmt::Display for DownloadBlobsTestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Download Blobs Configuration ===")?;
        writeln!(f, "block_len: {}", self.block_len)?;
        writeln!(f, "concurrency: {}", self.concurrency)?;
        writeln!(f, "data_len: {}", self.data_len)?;
        writeln!(f, "targets: {}", self.targets)?;
        Ok(())
    }
}

struct DownloadBlobsTest {
    /// Container client to operate within.
    /// This container will be created on setup and deleted on cleanup.
    container_client: BlobContainerClient,
    /// Number of blobs to setup for test.
    download_targets: usize,
    /// Length for the blobs being setup in bytes.
    download_targets_len: u64,

    /// Rotating queue of download target names to configure for the next operation.
    download_target_name_queue: Arc<Mutex<VecDeque<BlobInfo>>>,

    // Download options.
    parallel: NonZero<usize>,
    chunk_len: NonZero<usize>,
}

static NO_FAULT: LazyLock<Context> = LazyLock::new(|| {
    let mut c = Context::new();
    c.insert(fault_injection::NoFault);
    c
});

#[async_trait]
impl StressTest for DownloadBlobsTest {
    async fn global_setup(&self) -> Result<()> {
        println!("Creating container...");
        self.container_client
            .create(Some(BlobContainerClientCreateOptions {
                method_options: ClientMethodOptions {
                    context: NO_FAULT.clone(),
                },
                ..Default::default()
            }))
            .await?;
        println!("Container created.");

        let mut create_blob_tasks = Vec::with_capacity(self.download_targets);
        for i in 0..self.download_targets {
            println!("Creating blob {i}...");
            let (stream, data_crc) =
                data::random_data_stream_with_checksum(self.download_targets_len, CRC_ALGORITHM);

            let name = Uuid::new_v4().to_string();
            let client = self.container_client.blob_client(name.as_str());
            self.download_target_name_queue
                .lock()
                .await
                .push_back(BlobInfo { name, data_crc });

            create_blob_tasks.push(async move {
                client
                    .upload(
                        Body::SeekableStream(Box::new(stream)).into(),
                        Some(BlobClientUploadOptions {
                            method_options: ClientMethodOptions {
                                context: NO_FAULT.clone(),
                            },
                            ..Default::default()
                        }),
                    )
                    .await
            });
        }
        for (i, res) in future::join_all(create_blob_tasks)
            .await
            .into_iter()
            .enumerate()
        {
            res?;
            println!("Created blob {i}.");
        }
        Ok(())
    }

    async fn get_operation(&self) -> Result<Box<dyn StressTestOperation>> {
        let mut queue = self.download_target_name_queue.lock().await;
        let next_blob = queue
            .pop_front()
            .ok_or_else(|| Error::with_message(ErrorKind::Other, "No configured blobs."))?;
        let op = DownloadOperation {
            client: self.container_client.blob_client(&next_blob.name),
            parallel: self.parallel,
            chunk_len: self.chunk_len,
            expected_content_crc: next_blob.data_crc,
        };
        queue.push_back(next_blob);
        Ok(Box::new(op))
    }

    async fn global_cleanup(&self) -> Result<()> {
        println!("Deleting container...");
        self.container_client
            .delete(Some(BlobContainerClientDeleteOptions {
                method_options: ClientMethodOptions {
                    context: NO_FAULT.clone(),
                },
                ..Default::default()
            }))
            .await?;
        println!("Deleting created.");
        Ok(())
    }
}

struct BlobInfo {
    name: String,
    data_crc: u64,
}

struct DownloadOperation {
    client: BlobClient,
    parallel: NonZero<usize>,
    chunk_len: NonZero<usize>,
    expected_content_crc: u64,
}

#[async_trait]
impl StressTestOperation for DownloadOperation {
    async fn run(
        &mut self,
        timeout: Option<Duration>,
        mut result_sender: UnboundedSender<StressRunOutput>,
    ) {
        let mut digest = Digest::new(CRC_ALGORITHM);

        let download_op = async {
            let mut download_body = self
                .client
                .download(Some(BlobClientDownloadOptions {
                    parallel: Some(self.parallel),
                    partition_size: Some(self.chunk_len),
                    ..Default::default()
                }))
                .await?
                .body;
            while let Some(bytes) = download_body.try_next().await? {
                digest.update(&bytes);
            }
            Ok::<_, Error>(())
        };

        let output = match download_op.timeout(timeout).await {
            Ok(Ok(())) => {
                if digest.finalize() == self.expected_content_crc {
                    StressRunOutput::Success
                } else {
                    StressRunOutput::DataCorruption
                }
            }
            Ok(Err(op_error)) => StressRunOutput::GracefulError(op_error),
            Err(_timeout) => StressRunOutput::Timeout,
        };
        let _ = result_sender.send(output).await;
    }
}

fn non_zero_err() -> Error {
    Error::with_message(ErrorKind::DataConversion, "Tried to wrap 0 as a NonZero.")
}
