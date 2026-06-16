// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub(crate) mod models_serde;

use azure_core::{base64, fmt::SafeDebug, time::OffsetDateTime};
use serde::{Deserialize, Serialize};

/// A user delegation key.
#[derive(Clone, Default, Deserialize, SafeDebug, Serialize)]
#[non_exhaustive]
pub struct UserDelegationKey {
    /// The delegated user tenant ID in Entra ID. Returned if DelegatedUserTid is specified.
    #[serde(
        rename = "SignedDelegatedUserTid",
        skip_serializing_if = "Option::is_none"
    )]
    pub signed_delegated_user_tid: Option<String>,

    /// The date-time the key expires.
    #[serde(
        default,
        rename = "SignedExpiry",
        skip_serializing_if = "Option::is_none",
        with = "models_serde::option_offset_date_time_rfc3339"
    )]
    pub signed_expiry: Option<OffsetDateTime>,

    /// The Entra ID object ID in GUID format.
    #[serde(rename = "SignedOid", skip_serializing_if = "Option::is_none")]
    pub signed_oid: Option<String>,

    /// Abbreviation of the Azure Storage service that accepts the key.
    #[serde(rename = "SignedService", skip_serializing_if = "Option::is_none")]
    pub signed_service: Option<String>,

    /// The date-time the key is active.
    #[serde(
        default,
        rename = "SignedStart",
        skip_serializing_if = "Option::is_none",
        with = "models_serde::option_offset_date_time_rfc3339"
    )]
    pub signed_start: Option<OffsetDateTime>,

    /// The Entra ID tenant ID in GUID format.
    #[serde(rename = "SignedTid", skip_serializing_if = "Option::is_none")]
    pub signed_tid: Option<String>,

    /// The service version that created the key.
    #[serde(rename = "SignedVersion", skip_serializing_if = "Option::is_none")]
    pub signed_version: Option<String>,

    /// The base64 encoded key value.
    #[serde(
        default,
        deserialize_with = "base64::option::deserialize",
        rename = "Value",
        serialize_with = "base64::option::serialize",
        skip_serializing_if = "Option::is_none"
    )]
    pub value: Option<Vec<u8>>,
}
