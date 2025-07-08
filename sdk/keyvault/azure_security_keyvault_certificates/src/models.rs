// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::models::*;
use std::sync::LazyLock;

/// Gets the default self-signed [`CertificatePolicy`].
pub static DEFAULT_POLICY: LazyLock<CertificatePolicy> = LazyLock::new(|| CertificatePolicy {
    x509_certificate_properties: Some(X509CertificateProperties {
        subject: Some("CN=DefaultPolicy".into()),
        ..Default::default()
    }),
    issuer_parameters: Some(IssuerParameters {
        name: Some("Self".into()),
        ..Default::default()
    }),
    ..Default::default()
});
