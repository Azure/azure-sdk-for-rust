// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod authorization_code_flow;
mod azure_pipelines_credential;
mod chained_token_credential;
mod credentials;
mod env;
mod federated_credentials_flow;
mod managed_identity_credential;
mod oauth2_http_client;
mod refresh_token;
mod timeout;

use azure_core::{error::ErrorKind, Error, Result};
pub use azure_pipelines_credential::*;
pub use chained_token_credential::*;
pub use credentials::*;
pub use managed_identity_credential::*;
use serde::Deserialize;
use std::borrow::Cow;
use typespec_client_core::http::Model;

#[derive(Debug, Default, Deserialize, Model)]
#[serde(default)]
struct EntraIdErrorResponse {
    error_description: String,
}

#[derive(Debug, Default, Deserialize, Model)]
#[serde(default)]
struct EntraIdTokenResponse {
    token_type: String,
    expires_in: u64,
    ext_expires_in: u64,
    access_token: String,
}

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

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn validate_scope(scope: &str) -> Result<()> {
    if scope.is_empty()
        || !scope.chars().all(|c| {
            c.is_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ':' || c == '/'
        })
    {
        return Err(Error::message(
            ErrorKind::Credential,
            format!("invalid scope {scope}"),
        ));
    }

    Ok(())
}

#[test]
fn test_validate_scope() {
    assert!(validate_scope("").is_err());
    assert!(validate_scope("invalid_scope@id").is_err());
    assert!(validate_scope("A-1b_2c:3d/4.z").is_ok());
    assert!(validate_scope("http://vault.azure.net").is_ok());
}

#[cfg_attr(target_arch = "wasm32", allow(dead_code))]
fn validate_subscription(subscription: &str) -> Result<()> {
    if subscription.is_empty()
        || !subscription
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-' || c == '_' || c == ' ')
    {
        return Err(Error::message(
            ErrorKind::Credential,
            format!("invalid subscription {subscription}. If this is the name of a subscription, use its ID instead"),
        ));
    }

    Ok(())
}

#[test]
fn test_validate_subscription() {
    assert!(validate_subscription("").is_err());
    assert!(validate_subscription("invalid_subscription@id").is_err());
    assert!(validate_subscription("A-1b_2c 3.z").is_ok());
    assert!(validate_subscription("7b795fb9-09d3-42f4-a494-38864f99ba3c").is_ok());
}

fn validate_tenant_id(tenant_id: &str) -> Result<()> {
    if tenant_id.is_empty()
        || !tenant_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '.' || c == '-')
    {
        return Err(Error::message(
            ErrorKind::Credential,
            format!("invalid tenant ID {tenant_id}. You can locate your tenantID by following the instructions listed here: https://learn.microsoft.com/partner-center/find-ids-and-domain-names"),
        ));
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

#[cfg(test)]
mod tests {
    pub const LIVE_TEST_RESOURCE: &str = "https://management.azure.com";
    pub const LIVE_TEST_SCOPES: &[&str] = &["https://management.azure.com/.default"];
}
