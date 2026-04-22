// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{error::Result, http::StatusCode};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_keys::{KeyClient, KeyClientOptions};
use include_file::include_markdown;

#[recorded::test]
async fn readme(ctx: TestContext) -> Result<()> {
    use azure_security_keyvault_test::Retry;

    let recording = ctx.recording();

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Each macro invocation is in its own block to prevent errors with duplicate imports.
    println!("Create a key");
    include_markdown!("README.md", "create_key", scope);

    println!("Get a key");
    include_markdown!("README.md", "get_key", scope);

    println!("Update a key");
    include_markdown!("README.md", "update_key", scope);

    println!("List keys");
    include_markdown!("README.md", "list_keys", scope);

    println!("Encrypt and decrypt");
    rand::seed(recording.random());
    include_markdown!("README.md", "encrypt_decrypt", scope);

    println!("Handle errors");
    include_markdown!("README.md", "errors", scope);

    println!("Delete a key");
    include_markdown!("README.md", "delete_key", scope);

    println!("Purge a key");
    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_key("key-name", None).await {
            Ok(_) => break,
            Err(err) if matches!(err.http_status(), Some(StatusCode::Conflict)) => {
                if retry.next().await.is_none() {
                    return Err(err);
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

/// Override `use rand::random` import in README.md to use recorded seed.
mod rand {
    // cspell:ignore Seedable
    #![allow(static_mut_refs)]
    use rand::{
        distr::{Distribution, StandardUniform},
        RngExt, SeedableRng,
    };
    use rand_chacha::ChaCha20Rng;
    use std::sync::OnceLock;

    static mut RNG: OnceLock<ChaCha20Rng> = OnceLock::new();

    pub fn random<T>() -> T
    where
        StandardUniform: Distribution<T>,
    {
        unsafe { RNG.get_mut().expect("expected ChaCha20 rng").random() }
    }

    pub fn seed(seed: [u8; 32]) {
        unsafe {
            RNG.set(ChaCha20Rng::from_seed(seed)).expect("set seed");
        }
    }
}
