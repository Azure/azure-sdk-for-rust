// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_identity::{AzureCliCredentialOptions, AzureDeveloperCliCredentialOptions};

#[test]
fn az_credential_options() {
    let _options = AzureCliCredentialOptions {
        ..Default::default()
    };
}

#[test]
fn azd_credential_options() {
    let _options = AzureDeveloperCliCredentialOptions {
        ..Default::default()
    };
}
