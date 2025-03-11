// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Access to token credentials through various means
//!
//! Supported means currently include:
//! * The environment
//! * Azure CLI credentials cache
//! * Managed identity
//! * Client secret
mod app_service_managed_identity_credential;
#[cfg(not(target_arch = "wasm32"))]
mod azure_cli_credentials;
pub(crate) mod cache;
mod client_assertion_credentials;
#[cfg(feature = "client_certificate")]
mod client_certificate_credentials;
mod default_azure_credentials;
mod imds_managed_identity_credentials;
mod options;
mod virtual_machine_managed_identity_credential;
mod workload_identity_credentials;

pub use app_service_managed_identity_credential::*;
#[cfg(not(target_arch = "wasm32"))]
pub use azure_cli_credentials::*;
pub use client_assertion_credentials::*;
#[cfg(feature = "client_certificate")]
pub use client_certificate_credentials::*;
pub use default_azure_credentials::*;
pub use imds_managed_identity_credentials::ImdsId;
pub(crate) use imds_managed_identity_credentials::*;
pub use options::*;
pub use virtual_machine_managed_identity_credential::*;
pub use workload_identity_credentials::*;
