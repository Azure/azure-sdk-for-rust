// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{ImdsId, ImdsManagedIdentityCredential, TokenCredentialOptions};
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    http::{headers::HeaderName, Url},
};
use std::sync::Arc;

const ENDPOINT: &str = "http://169.254.169.254/metadata/identity/oauth2/token";
const API_VERSION: &str = "2019-08-01";
const SECRET_HEADER: HeaderName = HeaderName::from_static("x-identity-header");
const SECRET_ENV: &str = "IDENTITY_HEADER";

#[derive(Debug)]
pub struct VirtualMachineManagedIdentityCredential {
    credential: ImdsManagedIdentityCredential,
}

impl VirtualMachineManagedIdentityCredential {
    pub fn new(
        id: ImdsId,
        options: impl Into<TokenCredentialOptions>,
    ) -> azure_core::Result<Arc<Self>> {
        let endpoint = Url::parse(ENDPOINT).unwrap(); // valid url constant
        Ok(Arc::new(Self {
            credential: ImdsManagedIdentityCredential::new(
                options,
                endpoint,
                API_VERSION,
                SECRET_HEADER,
                SECRET_ENV,
                id,
            ),
        }))
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl TokenCredential for VirtualMachineManagedIdentityCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions>,
    ) -> azure_core::Result<AccessToken> {
        self.credential.get_token(scopes, options).await
    }
}
