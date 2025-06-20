// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::{common::retry_azure_operation, RetryOptions};
use azure_core::{
    credentials::Secret, error::ErrorKind as AzureErrorKind, error::Result, time::OffsetDateTime,
};
use azure_core_amqp::{
    AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis, AmqpConnection, AmqpError, AmqpSession,
    AmqpSessionApis,
};
use std::error::Error;
use std::sync::Arc;
use tracing::{debug, warn};

/// Thin wrapper around the [`AmqpClaimsBasedSecurityApis`] trait that implements the retry functionality.
///
/// A RecoverableClaimsBasedSecurity is a thin wrapper around the [`AmqpClaimsBasedSecurityApis`] trait which implements
/// the retry functionality. That allows implementations which call into the authorize_path API to not have
/// to worry about retrying the operation themselves.
pub(crate) struct RecoverableClaimsBasedSecurity {
    recoverable_connection: Arc<RecoverableConnection>,
}

impl RecoverableClaimsBasedSecurity {
    /// Creates a new RecoverableClaimsBasedSecurity.
    ///
    /// # Arguments
    ///
    /// * `recoverable_connection` - The recoverable connection to use for authorization.
    pub(super) fn new(recoverable_connection: Arc<RecoverableConnection>) -> Self {
        Self {
            recoverable_connection,
        }
    }

    pub(super) async fn create_claims_based_security(
        connection: Arc<AmqpConnection>,
        retry_options: &RetryOptions,
    ) -> Result<Arc<AmqpClaimsBasedSecurity>> {
        retry_azure_operation(
            || async {
                let session = AmqpSession::new();
                session.begin(connection.as_ref(), None).await?;

                let claims_based_security = Arc::new(AmqpClaimsBasedSecurity::new(session)?);

                // Attach the claims_based_security client to the session.
                claims_based_security.attach().await?;
                Ok(claims_based_security)
            },
            retry_options,
            Some(Self::should_retry_claims_based_security_response),
        )
        .await
    }

    fn should_retry_claims_based_security_response(e: &azure_core::Error) -> bool {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!(err=?e, "Amqp operation failed: {:?}", e.source());
                if let Some(e) = e.source() {
                    debug!(err=?e, "Error: {e}");

                    if let Some(amqp_error) = e.downcast_ref::<Box<AmqpError>>() {
                        RecoverableConnection::should_retry_amqp_error(amqp_error)
                    } else if let Some(amqp_error) = e.downcast_ref::<AmqpError>() {
                        RecoverableConnection::should_retry_amqp_error(amqp_error)
                    } else {
                        debug!(err=?e, "Non AMQP error: {e}");
                        false
                    }
                } else {
                    debug!("No source error found");
                    false
                }
            }
            _ => {
                debug!(err=?e, "Non AMQP error: {e}");
                false
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpClaimsBasedSecurityApis for RecoverableClaimsBasedSecurity {
    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: &Secret,
        expires_on: OffsetDateTime,
    ) -> Result<()> {
        let result = retry_azure_operation(
            || {
                let path = path.clone();
                let token_type = token_type.clone();
                let secret = secret.clone();

                async move {
                    let claims_based_security_client =
                        self.recoverable_connection.ensure_amqp_cbs().await?;
                    claims_based_security_client
                        .authorize_path(path, token_type, &secret, expires_on)
                        .await
                }
            },
            &self.recoverable_connection.retry_options,
            Some(Self::should_retry_claims_based_security_response),
        )
        .await?;
        Ok(result)
    }

    async fn attach(&self) -> azure_core::Result<()> {
        unimplemented!("AmqpClaimsBasedSecurityClient does not support attach operation");
    }

    async fn detach(self) -> azure_core::Result<()> {
        unimplemented!("AmqpClaimsBasedSecurityClient does not support detach operation");
    }
}
