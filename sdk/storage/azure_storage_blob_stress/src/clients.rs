// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{ErrorKind, ResultExt},
    http::Url,
    Result,
};
use azure_storage_blob::BlobContainerClient;

pub fn get_container_client() -> Result<BlobContainerClient> {
    let account_name = std::env::var("AZURE_STORAGE_ACCOUNT_NAME").with_context(
        ErrorKind::Other,
        "Configure `AZURE_STORAGE_ACCOUNT_NAME` environment variable.",
    )?;
    let container_name = uuid::Uuid::new_v4().to_string();
    BlobContainerClient::from_url(
        Url::parse(
            format!(
                "https://{}.blob.core.windows.net/{}",
                account_name, container_name
            )
            .as_str(),
        )?,
        Some(azure_core_test::credentials::from_env(None)?),
        None,
    )
}
