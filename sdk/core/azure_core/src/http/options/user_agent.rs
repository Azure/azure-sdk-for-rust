// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

/// Policy options to telemeter the `User-Agent` header.
#[derive(Clone, Debug, Default)]
pub struct UserAgentOptions {
    /// Set the application ID in the `User-Agent` header that can be telemetered.
    ///
    /// # Panics
    ///
    /// Panics if [`UserAgentOptions::application_id`] is greater than 24 characters.
    /// See [guidelines](https://azure.github.io/azure-sdk/general_azurecore.html#azurecore-http-telemetry-appid-length) for details.
    pub application_id: Option<String>,
}
