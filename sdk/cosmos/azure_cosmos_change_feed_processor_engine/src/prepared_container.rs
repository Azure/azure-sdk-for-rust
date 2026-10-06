// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::sync::Arc;

use azure_data_cosmos_driver::{
    models::{ContainerReference, Credential},
    CosmosDriver, CosmosError, Result,
};

pub(crate) fn validate_binding(
    driver: &CosmosDriver,
    container: &ContainerReference,
) -> Result<()> {
    let credentials_match = match (driver.account().auth(), container.account().auth()) {
        (Credential::MasterKey(left), Credential::MasterKey(right)) => left == right,
        (Credential::TokenCredential(left), Credential::TokenCredential(right)) => {
            Arc::ptr_eq(left, right)
        }
        _ => false,
    };
    if driver.account().endpoint() != container.account().endpoint() || !credentials_match {
        return Err(CosmosError::builder()
            .with_status(azure_data_cosmos_driver::error::status_codes::CLIENT_BAD_REQUEST)
            .with_message(
                "resolved container and driver must have the same account and credential binding",
            )
            .build());
    }
    Ok(())
}
