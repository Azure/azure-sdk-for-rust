// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::env::Env;
use crate::{ImdsId, ImdsManagedIdentityCredential};
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    http::{
        headers::HeaderName, ClientOptions, ExponentialRetryOptions, PipelineOptions, RetryOptions,
        StatusCode, Url,
    },
    time::Duration,
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
        mut client_options: ClientOptions,
        env: Env,
    ) -> azure_core::Result<Arc<Self>> {
        let endpoint = Url::parse(ENDPOINT).unwrap(); // valid url constant
        let pipeline_options = Some(PipelineOptions {
            // https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/how-to-use-vm-token#error-handling
            retry_status_codes: Vec::from([
                StatusCode::NotFound,
                StatusCode::Gone,
                StatusCode::TooManyRequests,
                StatusCode::InternalServerError,
                StatusCode::NotImplemented,
                StatusCode::BadGateway,
                StatusCode::ServiceUnavailable,
                StatusCode::GatewayTimeout,
                StatusCode::HttpVersionNotSupported,
                StatusCode::VariantAlsoNegotiates,
                StatusCode::InsufficientStorage,
                StatusCode::LoopDetected,
                StatusCode::NotExtended,
                StatusCode::NetworkAuthenticationRequired,
            ]),
            ..Default::default()
        });
        // these settings approximate the recommendations at
        // https://learn.microsoft.com/entra/identity/managed-identities-azure-resources/how-to-use-vm-token#retry-guidance
        client_options.retry = RetryOptions::exponential(ExponentialRetryOptions {
            initial_delay: Duration::milliseconds(1340),
            max_retries: 6,
            max_total_elapsed: Duration::seconds(72),
            ..Default::default()
        });
        Ok(Arc::new(Self {
            credential: ImdsManagedIdentityCredential::new(
                endpoint,
                API_VERSION,
                SECRET_HEADER,
                SECRET_ENV,
                id,
                client_options,
                pipeline_options,
                env,
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
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.credential.get_token(scopes, options).await
    }
}
