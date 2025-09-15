// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{credentials::Secret, error::ErrorKind};
use azure_identity::ClientSecretCredential;
use azure_security_keyvault_secrets::SecretClient;
use std::{env, process::exit};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tenant_id = env::var("AZURE_TENANT_ID").expect("AZURE_TENANT_ID required");
    let client_id = env::var("AZURE_CLIENT_ID").expect("AZURE_CLIENT_ID required");
    let secret: Secret = env::var("AZURE_CLIENT_SECRET")
        .expect("AZURE_CLIENT_SECRET required")
        .into();
    let vault_url = env::var("AZURE_KEYVAULT_URL").expect("AZURE_KEYVAULT_URL is required");

    let credential = ClientSecretCredential::new(&tenant_id, client_id, secret, None)?;
    let client = SecretClient::new(&vault_url, credential.clone(), None)?;
    match client.get_secret("my-secret", "", None).await {
        Ok(resp) => {
            let secret = resp.into_body().await?;
            println!("{}", secret.value.unwrap_or_else(|| "(none)".into()));
        }
        Err(err) => {
            let mut next: Option<&dyn std::error::Error> = Some(&err);
            while let Some(err) = next {
                let Some(inner) = err.downcast_ref::<azure_core::Error>() else {
                    break;
                };

                if let ErrorKind::HttpResponse {
                    status,
                    error_code: Some(message),
                } = inner.kind()
                {
                    eprintln!("HTTP error {status}: {message}");
                    exit(1);
                }

                next = err.source();
            }

            eprintln!("Error: {err:?}");
            exit(1);
        }
    }

    Ok(())
}
