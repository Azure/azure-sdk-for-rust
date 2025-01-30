// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::Result;
use azure_core_test::{recorded, TestContext};
use azure_security_keyvault_secrets::{
    models::SecretSetParameters, SecretClient, SecretClientOptions,
};

#[recorded::test(live)]
async fn secret_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    let body = SecretSetParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };
    client
        // TODO: https://github.com/Azure/typespec-rust/issues/223
        .set_secret("secret-name".into(), body.try_into()?, None)
        .await?;

    let secret = client
        // TODO: https://github.com/Azure/typespec-rust/issues/223
        .get_secret("secret-name".into(), "".into(), None)
        .await?
        .into_body()
        .await?;

    assert_eq!("secret-value", secret.value.unwrap());

    Ok(())
}
