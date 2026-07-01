// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::Url, Result};
use azure_core_test::Recording;
use azure_storage_blob::BlobContainerClient;
use azure_storage_blob_test::get_test_credential;

pub trait OnceLockExt {
    type Output;

    /// Emulate nightly `get_or_try_init()`.
    fn try_get_or_init<F>(&self, init: F) -> Result<&Self::Output>
    where
        F: FnOnce() -> Result<Self::Output>;
}

impl<T> OnceLockExt for std::sync::OnceLock<T> {
    type Output = T;

    fn try_get_or_init<F>(&self, init: F) -> Result<&Self::Output>
    where
        F: FnOnce() -> Result<Self::Output>,
    {
        if let Some(value) = self.get() {
            return Ok(value);
        }
        match init() {
            Ok(value) => {
                // If set fails, another thread beat us to initialization. That's not a problem.
                let _ = self.set(value);
                Ok(self.get().expect("just ensured value is set"))
            }
            // Another thread may have initialized in this time.
            // Try to get from them just in case, otherwise return init error.
            Err(e) => self.get().ok_or(e),
        }
    }
}

pub trait RecordingExt {
    fn get_container_client(&self, endpoint: Option<Url>) -> Result<BlobContainerClient>;
}

impl RecordingExt for Recording {
    fn get_container_client(&self, endpoint: Option<Url>) -> Result<BlobContainerClient> {
        let mut container_url = match endpoint {
            Some(url) => url,
            None => Url::parse(&format!(
                "https://{}.blob.core.windows.net",
                self.var("AZURE_STORAGE_ACCOUNT_NAME", None)
            ))?,
        };
        container_url
            .path_segments_mut()
            .expect("endpoint must be a valid base URL")
            .push(&format!("perf-container-{}", azure_core::Uuid::new_v4()));
        BlobContainerClient::new(container_url, Some(get_test_credential(self)), None)
    }
}
