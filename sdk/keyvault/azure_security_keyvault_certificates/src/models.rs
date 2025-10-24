// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::models::*;
use azure_core::{
    fmt::SafeDebug,
    http::{
        poller::{PollerOptions, PollerStatus, StatusMonitor},
        ClientMethodOptions, JsonFormat,
    },
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

/// Options to be passed to [`CertificateClient::begin_create_certificate()`](crate::clients::CertificateClient::begin_create_certificate())
#[derive(Clone, Default, SafeDebug)]
pub struct CertificateClientBeginCreateCertificateOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,

    /// Allows customization of the [`Poller`](azure_core::http::poller::Poller).
    pub poller_options: PollerOptions,
}

impl CertificateClientBeginCreateCertificateOptions<'_> {
    pub fn into_owned(self) -> CertificateClientBeginCreateCertificateOptions<'static> {
        CertificateClientBeginCreateCertificateOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
            poller_options: self.poller_options,
        }
    }
}

/// Options to be passed to [`CertificateClient::resume_certificate_operation()`](crate::clients::CertificateClient::resume_certificate_operation())
#[derive(Clone, Default, SafeDebug)]
pub struct CertificateClientResumeCertificateOperationOptions<'a> {
    /// Allows customization of the method call.
    pub method_options: ClientMethodOptions<'a>,

    /// Allows customization of the [`Poller`](azure_core::http::poller::Poller).
    pub poller_options: PollerOptions,
}

impl CertificateClientResumeCertificateOperationOptions<'_> {
    pub fn into_owned(self) -> CertificateClientResumeCertificateOperationOptions<'static> {
        CertificateClientResumeCertificateOperationOptions {
            method_options: ClientMethodOptions {
                context: self.method_options.context.into_owned(),
            },
            poller_options: self.poller_options,
        }
    }
}
