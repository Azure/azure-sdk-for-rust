// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{credentials::TokenCredential, http::Url, Result};
use azure_core_test::{Recording, TestMode};
use azure_identity::ManagedIdentityCredential;
use azure_storage_blob::BlobContainerClient;
use std::sync::Arc;

/// Returns a credential suitable for storage operations.
///
/// In playback mode, returns the recording's mock credential immediately.
/// Otherwise, if the environment variable `AZURE_STORAGE_USE_MANAGED_IDENTITY` is set to
/// `"true"`, returns a [`ManagedIdentityCredential`]. Falls back to the recording's
/// test credential via [`Recording::credential`].
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
fn get_test_credential(recording: &Recording) -> Arc<dyn TokenCredential> {
    if recording.test_mode() == TestMode::Playback {
        return recording.credential();
    }

    let use_managed_identity = std::env::var("AZURE_STORAGE_USE_MANAGED_IDENTITY")
        .map(|v| v.eq_ignore_ascii_case("true"))
        .unwrap_or(false);

    if use_managed_identity {
        ManagedIdentityCredential::new(None).expect("failed to create ManagedIdentityCredential")
    } else {
        recording.credential()
    }
}

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
