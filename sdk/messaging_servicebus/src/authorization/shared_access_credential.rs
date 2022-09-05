use azure_core::auth::{TokenCredential, TokenResponse};

pub(crate) struct SharedAccessCredential {}

impl SharedAccessCredential {
    /// <summary>The buffer to apply when considering refreshing; signatures that expire less than this duration will be refreshed.</summary>
    private static readonly TimeSpan SignatureRefreshBuffer = TimeSpan.FromMinutes(10);

    /// <summary>The length of time extend signature validity, if a token was requested.</summary>
    private static readonly TimeSpan SignatureExtensionDuration = TimeSpan.FromMinutes(30);
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for SharedAccessCredential {
    /// Gets a `TokenResponse` for the specified resource
    async fn get_token(&self, resource: &str) -> azure_core::Result<TokenResponse> {
        todo!()
    }
}
