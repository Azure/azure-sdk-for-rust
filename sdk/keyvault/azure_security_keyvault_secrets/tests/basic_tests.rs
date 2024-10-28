// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_identity;
use azure_security_keyvault_secrets::{self, models::SecretBundle};

mod common;

#[tokio::test]
async fn test_get_secret() {
    println!("Starting test_list_secrets");
    let token = azure_identity::DefaultAzureCredential::new().unwrap();

    let url: String = "https://".to_owned();
    url.push_str(
        common::create_random_name(None, 8)
            .as_deref()
            .unwrap_or("INVALID_VALUE"),
    );
    url.push_str(".vault.azure.net/".to_owned());

    let secret_client =
        azure_security_keyvault_secrets::SecretClient::new(&url, token.clone(), None).unwrap();

    let secret: SecretBundle = secret_client
        .get_secret(common::create_random_name(None, 16), "", None)
        .await?
        .deserialize_body_into()
        .await?;

    //println!("Secret: {:?}", secret);
    println!("{:?}", secret.value);

    Ok(())
}
