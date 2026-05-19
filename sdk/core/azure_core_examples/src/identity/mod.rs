// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Example identity credentials backed by azure_core primitives.

mod developer_tools_credential;
mod mock_credential;

pub use developer_tools_credential::{DeveloperToolsCredential, DeveloperToolsCredentialOptions};
pub use mock_credential::MockCredential;
