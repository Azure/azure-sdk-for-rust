// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Contains all the data structures and types used by the client library.
pub use crate::generated::models::*;
use azure_core::{
    fmt::SafeDebug,
    http::{
        poller::{PollerOptions, PollerStatus, StatusMonitor},
        JsonFormat, RequestContent,
    },
    json,
};
impl StatusMonitor for CertificateOperation {
    type Output = Certificate;
    type Format = JsonFormat;
    fn status(&self) -> PollerStatus {
        match self.status.as_deref() {
            Some("completed") => PollerStatus::Succeeded,
            Some("cancelled") => PollerStatus::Canceled,
            Some(_) if self.error.is_some() => PollerStatus::Failed,
            _ => PollerStatus::InProgress,
        }
    }
}

/// Options to be passed to [`CertificateClient::create_certificate()`](crate::clients::CertificateClient::create_certificate())
#[derive(Clone, Default, SafeDebug)]
pub struct CertificateClientCreateCertificateOptions<'a> {
    /// Allows customization of the [`Poller`](azure_core::http::poller::Poller).
    pub method_options: PollerOptions<'a>,
}

impl<'a> CertificateClientCreateCertificateOptions<'a> {
    /// Converts these options into an owned form so they can be used in `'static` contexts.
    #[must_use]
    pub fn into_owned(self) -> CertificateClientCreateCertificateOptions<'static> {
        CertificateClientCreateCertificateOptions {
            method_options: self.method_options.into_owned(),
        }
    }
}

impl TryFrom<CreateCertificateParameters> for RequestContent<CreateCertificateParameters> {
    type Error = azure_core::Error;
    fn try_from(value: CreateCertificateParameters) -> azure_core::Result<Self> {
        Ok(json::to_json(&value)?.into())
    }
}
