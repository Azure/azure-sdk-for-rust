// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Models for the example certificates client.

use azure_core::{
    http::{
        poller::{PollerStatus, StatusMonitor},
        JsonFormat, RequestContent,
    },
    json::to_json,
    Result,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A certificate.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct Certificate {
    /// The certificate id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The DER-encoded certificate bytes.
    #[serde(
        default,
        deserialize_with = "base64_option::deserialize",
        serialize_with = "base64_option::serialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub cer: Option<Vec<u8>>,
}

mod base64_option {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(value: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use base64::Engine as _;
        match value {
            Some(v) => base64::engine::general_purpose::STANDARD
                .encode(v)
                .serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        use base64::Engine as _;
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let bytes = base64::engine::general_purpose::STANDARD
                    .decode(&s)
                    .map_err(serde::de::Error::custom)?;
                Ok(Some(bytes))
            }
            None => Ok(None),
        }
    }
}

/// A certificate operation returned during LROs.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct CertificateOperation {
    /// The status of the operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,

    /// The target URL of the completed certificate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,

    /// The error, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HashMap<String, String>>,

    /// The certificate id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

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

/// Parameters for creating a certificate.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct CreateCertificateParameters {
    /// The management policy for the certificate.
    #[serde(rename = "policy", skip_serializing_if = "Option::is_none")]
    pub certificate_policy: Option<CertificatePolicy>,

    /// Application-specific metadata as key-value pairs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<HashMap<String, String>>,
}

impl TryFrom<CreateCertificateParameters> for RequestContent<CreateCertificateParameters> {
    type Error = azure_core::Error;
    fn try_from(value: CreateCertificateParameters) -> Result<Self> {
        Ok(to_json(&value)?.into())
    }
}

/// Certificate management policy.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct CertificatePolicy {
    /// Properties of the X.509 component of the certificate.
    #[serde(rename = "x509_props", skip_serializing_if = "Option::is_none")]
    pub x509_certificate_properties: Option<X509CertificateProperties>,

    /// Issuer parameters.
    #[serde(rename = "issuer", skip_serializing_if = "Option::is_none")]
    pub issuer_parameters: Option<IssuerParameters>,
}

/// X.509 certificate properties.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct X509CertificateProperties {
    /// The subject name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
}

/// Issuer parameters.
#[derive(Clone, Default, Deserialize, Serialize, Debug)]
pub struct IssuerParameters {
    /// The issuer name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
