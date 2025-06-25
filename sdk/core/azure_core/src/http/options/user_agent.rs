// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Policy options to telemeter the `User-Agent` header.
#[derive(Clone, Debug, Default)]
pub struct UserAgentOptions {
    /// Set the application ID in the `User-Agent` header that can be telemetered.
    pub application_id: Option<String>,

    /// Disable to prevent sending the `User-Agent` header in requests.
    pub disabled: bool,
}
