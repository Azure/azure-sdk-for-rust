// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{request::RequestContent, StatusCode},
    json::to_json,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Error {
    #[serde(skip, default = "Error::default_status")]
    pub status_code: StatusCode,
    pub status: String,
    pub message: String,
}

impl Error {
    fn default_status() -> StatusCode {
        StatusCode::Ok
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "HTTP {} ({}): {}",
            self.status_code, self.status, self.message
        )
    }
}

impl std::error::Error for Error {}

impl From<Error> for azure_core::Error {
    fn from(error: Error) -> Self {
        azure_core::Error::new(
            azure_core::error::ErrorKind::HttpResponse {
                status: error.status_code,
                error_code: Some(error.status.clone()),
            },
            error,
        )
    }
}

#[derive(Debug, Default, Serialize)]
pub struct StartPayload {
    /// Path to the recording file relative to the repository root.
    ///
    /// Example: "sdk/keyvault/azure_security_keyvault_secrets/tests/recordings/SecretClient/get_secret.json".
    ///
    /// Note: this is not actually required by test-proxy, but is optional only for performance testing
    /// and meant to be required for normal client testing, which is reflected by this definition.
    #[serde(rename = "x-recording-file")]
    pub recording_file: String,

    /// Path to the assets.json file relative to the repository root.
    #[serde(
        rename = "x-recording-assets-file",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_assets_file: Option<String>,
}

impl TryFrom<StartPayload> for RequestContent<StartPayload> {
    type Error = azure_core::Error;
    fn try_from(value: StartPayload) -> Result<Self, Self::Error> {
        Ok(to_json(&value)?.into())
    }
}

#[derive(Debug, Default, Deserialize)]
pub struct RecordStartResult {
    #[serde(skip)]
    pub recording_id: String,
}

#[derive(Debug, Default, Serialize)]
pub struct VariablePayload {
    #[serde(flatten)]
    pub variables: HashMap<String, String>,
}

impl TryFrom<VariablePayload> for RequestContent<VariablePayload> {
    type Error = azure_core::Error;
    fn try_from(value: VariablePayload) -> Result<Self, Self::Error> {
        Ok(to_json(&value)?.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct PlaybackStartResult {
    #[serde(skip)]
    pub recording_id: String,

    #[allow(dead_code)]
    #[serde(flatten)]
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Default, Serialize)]
pub struct SanitizerList {
    #[serde(rename = "Sanitizers")]
    pub sanitizers: Vec<String>,
}

impl TryFrom<SanitizerList> for RequestContent<SanitizerList> {
    type Error = azure_core::Error;
    fn try_from(value: SanitizerList) -> Result<Self, Self::Error> {
        Ok(to_json(&value)?.into())
    }
}

#[derive(Debug, Deserialize)]
pub struct RemovedSanitizers {
    #[allow(dead_code)]
    #[serde(rename = "Removed")]
    pub removed: Vec<String>,
}
