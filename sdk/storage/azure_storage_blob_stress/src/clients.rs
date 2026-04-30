// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    sync::{Arc, LazyLock},
    time::Duration,
};

use azure_core::{
    error::{ErrorKind, ResultExt},
    http::{ClientOptions, Transport, Url},
    Result,
};
use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
use azure_storage_blob_test::fault_injection::{FaultInjectionPolicy, FaultInjectionProbabilities};
use rand::rngs::SmallRng;

pub fn get_container_client(
    fault_options: &FaultInjectionProbabilities,
) -> Result<BlobContainerClient> {
    let account_name = std::env::var("AZURE_STORAGE_ACCOUNT_NAME").with_context(
        ErrorKind::Other,
        "Configure `AZURE_STORAGE_ACCOUNT_NAME` environment variable.",
    )?;
    let container_name = uuid::Uuid::new_v4().to_string();

    let client_options = if fault_options.is_zero() {
        None
    } else {
        Some(BlobContainerClientOptions {
            client_options: ClientOptions {
                per_try_policies: vec![Arc::new(FaultInjectionPolicy::<SmallRng>::new(
                    Url::parse("https://127.0.0.1:7778").unwrap(),
                    fault_options.clone(),
                )?)],
                transport: Some(FAULT_HTTP_CLIENT.clone()),
                ..Default::default()
            },
            ..Default::default()
        })
    };

    BlobContainerClient::from_url(
        Url::parse(
            format!(
                "https://{}.blob.core.windows.net/{}",
                account_name, container_name
            )
            .as_str(),
        )?,
        Some(azure_core_test::credentials::from_env(None)?),
        client_options,
    )
}

// Build a custom reqwest client that accepts the fault injector's
// self-signed TLS certificate and disables automatic decompression
// (required for partitioned downloads to work correctly).
static FAULT_HTTP_CLIENT: LazyLock<Transport> = LazyLock::new(|| {
    Transport::new(Arc::new(
        ::reqwest::ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .read_timeout(Duration::from_secs(10))
            .no_gzip()
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .build()
            .expect("build reqwest client"),
    ))
});
