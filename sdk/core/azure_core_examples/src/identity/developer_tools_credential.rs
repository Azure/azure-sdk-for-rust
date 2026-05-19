// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::credentials::{AccessToken, TokenCredential, TokenRequestOptions};
use std::{fmt, sync::Arc};

/// Options for constructing a new [`DeveloperToolsCredential`].
#[derive(Clone, Debug, Default)]
pub struct DeveloperToolsCredentialOptions;

/// Authenticates through developer tools such as the Azure CLI.
///
/// This is a stub for use in examples that do not need to authenticate.
pub struct DeveloperToolsCredential;

impl DeveloperToolsCredential {
    /// Creates a new instance of `DeveloperToolsCredential`.
    pub fn new(_options: Option<DeveloperToolsCredentialOptions>) -> azure_core::Result<Arc<Self>> {
        Ok(Arc::new(Self))
    }
}

impl fmt::Debug for DeveloperToolsCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DeveloperToolsCredential").finish()
    }
}

#[async_trait::async_trait]
impl TokenCredential for DeveloperToolsCredential {
    async fn get_token(
        &self,
        _scopes: &[&str],
        _options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        unimplemented!("DeveloperToolsCredential is not runnable in azure_core_examples")
    }
}
