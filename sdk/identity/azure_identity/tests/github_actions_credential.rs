// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::credentials::TokenCredential;
use azure_core_test::recorded;
use azure_identity::GitHubActionsCredential;
use std::{env, error::Error};

const OIDC_REQUEST_URL: &str = "ACTIONS_ID_TOKEN_REQUEST_URL";
const OIDC_REQUEST_TOKEN: &str = "ACTIONS_ID_TOKEN_REQUEST_TOKEN";

#[recorded::test(live)]
async fn get_token() -> Result<(), Box<dyn Error>> {
    if !applicable() {
        return Ok(());
    }

    let credential = GitHubActionsCredential::new(None)?;
    let scope = env::var("TEST_SCOPE")?;
    let token = credential.get_token(&[&scope], None).await?;
    assert!(!token.token.secret().is_empty());
    Ok(())
}

fn applicable() -> bool {
    if env::var(OIDC_REQUEST_URL).is_ok_and(|ref v| !v.is_empty())
        && env::var(OIDC_REQUEST_TOKEN).is_ok_and(|ref v| !v.is_empty())
    {
        return true;
    }
    eprintln!("Skipping because environment variable {OIDC_REQUEST_URL} or {OIDC_REQUEST_TOKEN} is not set");
    false
}
