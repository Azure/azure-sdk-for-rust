use std::time::Duration;

use azure_core::auth::{TokenCredential, TokenResponse};

pub(crate) struct SharedAccessCredential {}

impl SharedAccessCredential {
    /// <summary>The buffer to apply when considering refreshing; signatures that expire less than this duration will be refreshed.</summary>
    const SIGNATURE_REFRESH_BUFFER: Duration = Duration::from_secs(10 * 60); // 10 mins

    /// <summary>The length of time extend signature validity, if a token was requested.</summary>
    const SIGNATURE_EXTENSION_DURATION: Duration = Duration::from_secs(30 * 60); // 30 mins
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SharedAccessCredential {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        todo!()
    }
}
