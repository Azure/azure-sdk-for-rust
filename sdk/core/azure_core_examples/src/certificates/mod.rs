// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example certificates client backed by azure_core primitives.

pub mod models;

use crate::certificates::models::{CertificateOperation, CreateCertificateParameters};
use azure_core::{
    credentials::TokenCredential,
    fmt::SafeDebug,
    http::{poller::Poller, ClientOptions, RequestContent},
    Result,
};
use std::sync::Arc;

/// Options for configuring a [`CertificateClient`].
#[derive(Clone, Default, SafeDebug)]
pub struct CertificateClientOptions {
    /// The API version to use.
    pub api_version: String,
    /// Common client options including transport and policies.
    pub client_options: ClientOptions,
}

/// A minimal certificates client backed by azure_core.
#[derive(Debug)]
pub struct CertificateClient;

impl CertificateClient {
    /// Creates a new `CertificateClient`.
    pub fn new(
        _endpoint: &str,
        _credential: Arc<dyn TokenCredential>,
        _options: Option<CertificateClientOptions>,
    ) -> Result<Self> {
        unimplemented!()
    }

    /// Begins creating a certificate and returns a poller to track the operation.
    pub fn begin_create_certificate(
        &self,
        _certificate_name: &str,
        _parameters: RequestContent<CreateCertificateParameters>,
        _options: Option<()>,
    ) -> Result<Poller<CertificateOperation>> {
        unimplemented!()
    }
}
