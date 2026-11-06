// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{error::ErrorKind, http::response::AsyncResponseBody, Error};

#[async_trait]
pub(crate) trait AsyncResponseBodyExt {
    /// Collects the response into the given slice.
    ///
    /// # Returns
    /// Ok if the response body was the exact length of the slice and the contents were
    /// successfully copied into it. Otherwise, returns Err.
    async fn collect_into_exact(mut self, buffer: &mut [u8]) -> Result<(), Error>;
}

#[async_trait]
impl AsyncResponseBodyExt for AsyncResponseBody {
    async fn collect_into_exact(mut self, buffer: &mut [u8]) -> Result<(), Error> {
        let count = self.collect_into(buffer).await?;
        if count == buffer.len() {
            Ok(())
        } else {
            Err(Error::with_message(
                ErrorKind::Other,
                "Received fewer than expected bytes.",
            ))
        }
    }
}
