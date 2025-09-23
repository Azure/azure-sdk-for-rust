// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    credentials::TokenCredential,
    http::{
        headers::Headers,
        policies::{Policy, PolicyResult},
        BufResponse, Context, HttpClient, Method, Request, StatusCode, Transport,
    },
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use futures::FutureExt;
use std::sync::Arc;

// Define a policy that will remove the User-Agent header.
#[derive(Debug)]
struct RemoveUserAgent;

#[cfg_attr(target_arch = "wasm32", async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait)]
impl Policy for RemoveUserAgent {
    async fn send(
        &self,
        ctx: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let headers = request.headers_mut();

        // Note: HTTP headers are case-insensitive but client-added headers are normalized to lowercase.
        headers.remove("user-agent");

        next[0].send(ctx, request, &next[1..]).await
    }
}

async fn test_remove_user_agent() -> Result<(), Box<dyn std::error::Error>> {
    // Policies are created in an Arc to be generally shared.
    let remove_user_agent = Arc::new(RemoveUserAgent);

    // Construct client options with your policy that runs after the built-in per-call UserAgentPolicy.
    let mut options = SecretClientOptions::default();
    options
        .client_options
        .per_call_policies
        .push(remove_user_agent);

    // Ignore: this is only set up for testing.
    // You normally would create credentials from `azure_identity` and
    // use the default transport in production.
    let (credential, transport) = setup()?;
    options.client_options.transport = Some(Transport::new(transport));

    // Construct the client with these options and a shared credential.
    let client = SecretClient::new(
        "https://my-vault.vault.azure.net",
        credential.clone(),
        Some(options),
    )?;

    // We'll fetch a secret and let the mock client assert the User-Agent header was removed.
    let secret = client
        .get_secret("my-secret", None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.value.as_deref(), Some("secret-value"));

    Ok(())
}

// ----- BEGIN TEST SETUP -----
#[tokio::test]
async fn test_core_remove_user_agent() -> Result<(), Box<dyn std::error::Error>> {
    test_remove_user_agent().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_remove_user_agent().await
}

#[allow(clippy::type_complexity)]
fn setup() -> Result<(Arc<dyn TokenCredential>, Arc<dyn HttpClient>), Box<dyn std::error::Error>> {
    let client = MockHttpClient::new(|request| {
        async move {
            assert!(request.url().path().starts_with("/secrets/my-secret"));
            assert_eq!(request.method(), Method::Get);
            assert!(
                !request
                    .headers()
                    .iter()
                    .any(|(name, _)| name.as_str().eq_ignore_ascii_case("user-agent")),
                "user-agent header should be absent"
            );
            Ok(BufResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"value":"secret-value"}"#,
            ))
        }
        .boxed()
    });

    Ok((MockCredential::new()?, Arc::new(client)))
}
// ----- END TEST SETUP -----
