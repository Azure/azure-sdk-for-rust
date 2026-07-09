// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::env::Env;
use crate::{ImdsId, ImdsManagedIdentityCredential};
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    http::{
        ClientOptions, ExponentialRetryOptions, PipelineOptions, RetryOptions, StatusCode, Url,
    },
    time::Duration,
};
use std::{any::type_name, fmt, sync::Arc};

const DEFAULT_ENDPOINT: &str = "http://localhost:40342/metadata/identity/oauth2/token";
const API_VERSION: &str = "2021-02-01";

pub struct AzureArcCredential {
    credential: ImdsManagedIdentityCredential,
}

impl fmt::Debug for AzureArcCredential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>()).finish_non_exhaustive()
    }
}

impl AzureArcCredential {
    pub fn new(
        id: ImdsId,
        client_options: ClientOptions,
        env: Env,
    ) -> azure_core::Result<Arc<Self>> {
        let identity_endpoint = match (
            env.var("IDENTITY_ENDPOINT").ok(),
            env.var("IMDS_ENDPOINT").ok(),
        ) {
            (Some(identity_endpoint), Some(_)) => identity_endpoint,
            _ => DEFAULT_ENDPOINT.to_owned(),
        };

        let token_url = Url::parse(&identity_endpoint)?;
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
        let client_options = ClientOptions {
            retry: RetryOptions::exponential(ExponentialRetryOptions {
                initial_delay: Duration::milliseconds(1340),
                max_retries: 6,
                max_total_elapsed: Duration::seconds(72),
                ..Default::default()
            }),
            ..client_options
        };
        Ok(Arc::new(Self {
            credential: ImdsManagedIdentityCredential::new(
                token_url,
                API_VERSION,
                None,
                None,
                id,
                client_options,
                pipeline_options,
                env,
            ),
        }))
    }
}

#[async_trait::async_trait]
impl TokenCredential for AzureArcCredential {
    async fn get_token(
        &self,
        scopes: &[&str],
        options: Option<TokenRequestOptions<'_>>,
    ) -> azure_core::Result<AccessToken> {
        self.credential.get_token(scopes, options).await
    }
}
