// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]

mod authorization_code_flow;
mod azure_pipelines_credential;
mod credentials;
mod env;
mod federated_credentials_flow;
mod oauth2_http_client;
mod refresh_token;
mod timeout;

use azure_core::{error::ErrorKind, Error, Result};
pub use azure_pipelines_credential::*;
pub use credentials::*;
use std::borrow::Cow;

fn validate_not_empty<C>(value: &str, message: C) -> Result<()>
where
    C: Into<Cow<'static, str>>,
{
    if value.is_empty() {
        return Err(Error::message(ErrorKind::Credential, message));
    }

    Ok(())
}

#[test]
fn test_validate_not_empty() {
    assert!(validate_not_empty("", "it's empty").is_err());
    assert!(validate_not_empty(" ", "it's not empty").is_ok());
    assert!(validate_not_empty("not empty", "it's not empty").is_ok());
}

fn validate_tenant_id(tenant_id: &str) -> Result<()> {
    if tenant_id.is_empty()
        || !tenant_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
    {
        return Err(Error::message(ErrorKind::Credential, "invalid tenantID. You can locate your tenantID by following the instructions listed here: https://learn.microsoft.com/partner-center/find-ids-and-domain-names"));
    }

    Ok(())
}

#[test]
fn test_validate_tenant_id() {
    assert!(validate_tenant_id("").is_err());
    assert!(validate_tenant_id("invalid_tenant@id").is_err());
    assert!(validate_tenant_id("A-1.z").is_ok());
    assert!(validate_tenant_id("7b795fb9-09d3-42f4-a494-38864f99ba3c").is_ok());
}
