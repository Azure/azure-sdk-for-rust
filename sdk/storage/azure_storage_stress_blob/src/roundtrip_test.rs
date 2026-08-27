// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{num::NonZero, sync::LazyLock, time::Duration};

use async_trait::async_trait;
use azure_core::{
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
use azure_storage_stress::{
    data,
    fault_injection::{self, FaultInjectionProbabilities},
    futures_ext::OptionalTimeoutFutureExt,
    StressRunOutput, StressTest, StressTestOperation,
};
use clap::{Args, ValueEnum};
use crc_fast::{CrcAlgorithm, Digest};
use futures::{channel::mpsc::UnboundedSender, SinkExt, TryStreamExt};
use log::{debug, info};
use serde::Serialize;
use uuid::Uuid;

const CRC_ALGORITHM: CrcAlgorithm = CrcAlgorithm::Crc64Nvme;

#[derive(Args, Debug, Serialize)]
pub(crate) struct RoundtripBlobsTestArgs {
    /// Concurrency value for transfer options.
    #[arg(long, default_value_t = NonZero::new(2).unwrap(), value_name = "NUM WORKERS")]
    concurrency: NonZero<usize>,

    /// Block length for transfer options.
    #[arg(long, default_value_t = NonZero::new(4 << 20).unwrap(), value_name = "BYTES")]
    block_len: NonZero<u64>,

    /// Data length of blob(s) to transfer.
    #[arg(long, value_name = "BYTES")]
    data_len: u64,

    /// Type of data source to upload from.
    #[arg(value_enum)]
    data_source: DataSourceType,
}

#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, ValueEnum)]
enum DataSourceType {
    /// A stream which uploads in-memory data through a stream interface.
    GeneratedStream,

    /// Direct contiguous memory access.
    DirectMemory,
}

impl RoundtripBlobsTestArgs {
    pub fn as_test(
        &self,
        fault_options: &FaultInjectionProbabilities,
    ) -> Result<Box<dyn StressTest>> {
        Ok(Box::new(RoundtripBlobsTest {
            container_client: crate::clients::get_container_client(fault_options)?,
            data_len: self.data_len,
            data_type: self.data_source,
            parallel: self.concurrency,
            chunk_len: self.block_len,
        }))
    }
}

impl std::fmt::Display for RoundtripBlobsTestArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "=== Upload Blobs Configuration ===")?;
        writeln!(f, "block_len: {}", self.block_len)?;
        writeln!(f, "concurrency: {}", self.concurrency)?;
        writeln!(f, "data_len: {}", self.data_len)?;
        Ok(())
    }
}

struct RoundtripBlobsTest {
    /// Container client to operate within.
    /// This container will be created on setup and deleted on cleanup.
    container_client: BlobContainerClient,

    data_len: u64,
    data_type: DataSourceType,

    // Download options.
    parallel: NonZero<usize>,
    chunk_len: NonZero<u64>,
}

static NO_FAULT: LazyLock<Context> = LazyLock::new(|| {
    let mut c = Context::new();
    c.insert(fault_injection::NoFault);
    c
});

#[async_trait]
impl StressTest for RoundtripBlobsTest {
    async fn global_setup(&self) -> Result<()> {
        debug!("Creating container...");
        self.container_client
            .create(Some(BlobContainerClientCreateOptions {
                method_options: ClientMethodOptions {
                    context: NO_FAULT.clone(),
                },
                ..Default::default()
            }))
            .await?;
        info!("Container created.");
        Ok(())
    }

    async fn get_operation(&self) -> Result<Box<dyn StressTestOperation>> {
        let body: Body;
        let crc;
        match self.data_type {
            DataSourceType::GeneratedStream => {
                let (stream, stream_crc) =
                    data::random_data_stream_with_checksum(self.data_len, CRC_ALGORITHM);
                body = Body::SeekableStream(Box::new(stream));
                crc = stream_crc;
            }
            DataSourceType::DirectMemory => {
                let (vec, vec_crc) =
                    data::random_data_memory_with_checksum(self.data_len as usize, CRC_ALGORITHM);
                body = vec.into();
                crc = vec_crc;
            }
        }

        let name = Uuid::new_v4().to_string();
        let client = self.container_client.blob_client(name.as_str());

        Ok(Box::new(RoundtripOperation {
            client,
            parallel: self.parallel,
            chunk_len: self.chunk_len,
            data: body,
            data_crc: crc,
        }))
    }

    async fn global_cleanup(&self) -> Result<()> {
        debug!("Deleting container...");
        self.container_client
            .delete(Some(BlobContainerClientDeleteOptions {
                method_options: ClientMethodOptions {
                    context: NO_FAULT.clone(),
                },
                ..Default::default()
            }))
            .await?;
        info!("Container deleted.");
        Ok(())
    }
}

struct RoundtripOperation {
    client: BlobClient,
    parallel: NonZero<usize>,
    chunk_len: NonZero<u64>,
    data: Body,
    data_crc: u64,
}

#[async_trait]
impl StressTestOperation for RoundtripOperation {
    async fn run(
        &mut self,
        timeout: Option<Duration>,
        mut result_sender: UnboundedSender<StressRunOutput>,
    ) {
        let mut digest = Digest::new(CRC_ALGORITHM);

        let roundtrip_op = async {
            // Upload
            self.client
                .upload(
                    self.data.clone().into(),
                    Some(BlobClientUploadOptions {
                        parallel: Some(self.parallel),
                        partition_size: Some(self.chunk_len),
                        ..Default::default()
                    }),
                )
                .await?;

            // Download
            let mut download_body = self
                .client
                .download(Some(BlobClientDownloadOptions {
                    parallel: Some(self.parallel),
                    ..Default::default()
                }))
                .await?
                .body;
            while let Some(bytes) = download_body.try_next().await? {
                digest.update(&bytes);
            }

            Ok::<_, Error>(())
        };

        let output = match roundtrip_op.timeout(timeout).await {
            Ok(Ok(())) => {
                if digest.finalize() == self.data_crc {
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
