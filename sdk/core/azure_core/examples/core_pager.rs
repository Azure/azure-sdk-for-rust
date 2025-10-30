// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::TokenCredential,
    http::{headers::Headers, BufResponse, HttpClient, Method, StatusCode, Transport},
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
use azure_security_keyvault_secrets::{ResourceExt, SecretClient, SecretClientOptions};
use futures::{FutureExt, TryStreamExt};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// This example demonstrates using a [`Pager`] to list secret properties from Key Vault.
async fn test_pager() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = SecretClientOptions::default();

    // Ignore: this is only set up for testing.
    // You normally would create credentials from `azure_identity` and
    // use the default transport in production.
    let (credential, transport) = setup()?;
    options.client_options.transport = Some(Transport::new(transport));

    let client = SecretClient::new(
        "https://my-vault.vault.azure.net",
        credential,
        Some(options),
    )?;

    // List secret properties using a Pager.
    let mut pager = client.list_secret_properties(None)?;
    let mut names = Vec::new();
    while let Some(secret) = pager.try_next().await? {
        names.push(secret.resource_id()?.name);
    }
    assert_eq!(names, vec!["secret-a", "secret-b", "secret-c"]);

    Ok(())
}

/// This example demonstrates using a [`PageIterator`] to list pages of secret properties from Key Vault.
///
/// Some clients may return a `PageIterator` if there are no items to iterate or multiple items to iterate.
/// The following example shows how you can also get a `PageIterator` from a [`Pager`] to iterate over pages instead of items.
/// The pattern for iterating pages is otherwise the same:
async fn test_page_iterator() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = SecretClientOptions::default();

    // Ignore: this is only set up for testing.
    // You normally would create credentials from `azure_identity` and
    // use the default transport in production.
    let (credential, transport) = setup()?;
    options.client_options.transport = Some(Transport::new(transport));

    let client = SecretClient::new(
        "https://my-vault.vault.azure.net",
        credential,
        Some(options),
    )?;

    // List secret properties using a Pager.
    let mut pager = client.list_secret_properties(None)?.into_pages();
    let mut names = Vec::new();
    while let Some(page) = pager.try_next().await? {
        let page = page.into_body()?;
        for secret in page.value {
            names.push(secret.resource_id()?.name);
        }
    }
    assert_eq!(names, vec!["secret-a", "secret-b", "secret-c"]);

    Ok(())
}

// ----- BEGIN TEST SETUP -----
#[tokio::test]
async fn test_core_pager() -> Result<(), Box<dyn std::error::Error>> {
    test_pager().await
}

#[tokio::test]
async fn test_core_page_iterator() -> Result<(), Box<dyn std::error::Error>> {
    test_page_iterator().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_pager().await?;
    test_page_iterator().await?;

    Ok(())
}

#[allow(clippy::type_complexity)]
fn setup() -> Result<(Arc<dyn TokenCredential>, Arc<dyn HttpClient>), Box<dyn std::error::Error>> {
    let credential: Arc<dyn TokenCredential> = MockCredential::new()?;
    let calls = Arc::new(AtomicUsize::new(0));
    let transport = {
        let calls = calls.clone();
        MockHttpClient::new(move |request| {
            let calls = calls.clone();
            async move {
                let idx = calls.fetch_add(1, Ordering::SeqCst);
                assert_eq!(request.method(), Method::Get);
                assert_eq!(request.url().path(), "/secrets");
                match idx {
                    0 => Ok(BufResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        // First page with continuation (nextLink)
                        r#"{"value":[
                            {"id":"https://my-vault.vault.azure.net/secrets/secret-a"},
                            {"id":"https://my-vault.vault.azure.net/secrets/secret-b"}
                          ],
                          "nextLink":"https://my-vault.vault.azure.net/secrets?api-version=7.4&$skiptoken=page2"}"#,
                    )),
                    1 => Ok(BufResponse::from_bytes(
                        StatusCode::Ok,
                        Headers::new(),
                        // Second (final) page without nextLink
                        r#"{"value":[{"id":"https://my-vault.vault.azure.net/secrets/secret-c"}]}"#,
                    )),
                    _ => panic!("unexpected request count {idx}"),
                }
            }
            .boxed()
        })
    };
    Ok((credential, Arc::new(transport)))
}
// ----- END TEST SETUP -----
