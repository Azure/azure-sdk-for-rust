// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{json::to_json, RequestContent, StatusCode};
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

#[derive(Debug, Serialize)]
pub struct StartPayload {
    #[serde(rename = "x-recording-file", skip_serializing_if = "Option::is_none")]
    pub recording_file: Option<String>,
    #[serde(
        rename = "x-recording-assets-file",
        skip_serializing_if = "Option::is_none"
    )]
    pub recording_assets_file: Option<String>,
}

impl TryFrom<StartPayload> for RequestContent<StartPayload> {
    type Error = azure_core::Error;
    fn try_from(value: StartPayload) -> Result<Self, Self::Error> {
        RequestContent::try_from(to_json(&value)?)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordStartResult {
    #[serde(skip)]
    pub recording_id: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct VariablePayload {
    #[serde(rename = "Variables")]
    pub variables: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct PlaybackStartResult {
    #[serde(skip)]
    pub recording_id: Option<String>,
    #[allow(dead_code)]
    #[serde(flatten)]
    pub variables: HashMap<String, String>,
}
