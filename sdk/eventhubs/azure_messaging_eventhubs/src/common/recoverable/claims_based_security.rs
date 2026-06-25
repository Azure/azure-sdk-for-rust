// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    common::{
        recover_azure_operation, recoverable::RecoverableConnection, retry::ErrorRecoveryAction,
    },
    RetryOptions,
};
use azure_core::{credentials::Secret, error::ErrorKind as AzureErrorKind, time::OffsetDateTime};
use azure_core_amqp::{
    error::Result, AmqpClaimsBasedSecurity, AmqpClaimsBasedSecurityApis, AmqpConnection, AmqpError,
    AmqpSession, AmqpSessionApis,
};
use std::sync::{Arc, Weak};
use tracing::warn;

/// Thin wrapper around the [`AmqpClaimsBasedSecurityApis`] trait that implements the retry functionality.
///
/// A RecoverableClaimsBasedSecurity is a thin wrapper around the [`AmqpClaimsBasedSecurityApis`] trait which implements
/// the retry functionality. That allows implementations which call into the authorize_path API to not have
/// to worry about retrying the operation themselves.
pub(crate) struct RecoverableClaimsBasedSecurity {
    recoverable_connection: Weak<RecoverableConnection>,
}

impl RecoverableClaimsBasedSecurity {
    /// Creates a new RecoverableClaimsBasedSecurity.
    ///
    /// # Arguments
    ///
    /// * `recoverable_connection` - The recoverable connection to use for authorization.
    pub(super) fn new(recoverable_connection: Weak<RecoverableConnection>) -> Self {
        Self {
            recoverable_connection,
        }
    }

    pub(super) async fn create_claims_based_security(
        connection: Arc<AmqpConnection>,
        retry_options: &RetryOptions,
    ) -> Result<Arc<AmqpClaimsBasedSecurity>> {
        recover_azure_operation(
            || async {
                let session = AmqpSession::new();
                session.begin(connection.as_ref(), None).await?;

                let claims_based_security = Arc::new(AmqpClaimsBasedSecurity::new(session)?);

                // Attach the claims_based_security client to the session.
                claims_based_security.attach().await?;
                Ok(claims_based_security)
            },
            retry_options,
            Self::should_retry_claims_based_security_response,
            None,
            None::<()>,
        )
        .await
    }

    fn should_retry_claims_based_security_response(e: &AmqpError) -> ErrorRecoveryAction {
        let action = RecoverableConnection::should_retry_amqp_error(e);
        // This classifier is shared by the create and authorize_path CBS paths.
        // `recover_azure_operation` only hands us the error (a plain `fn`
        // pointer, not a closure), so we cannot capture the link path here; the
        // path-scoped context is logged at the call site in `authorize_path`.
        // Surface the error condition and the resulting retry decision so a
        // failing CBS round-trip is diagnosable.
        warn!(
            operation = "claims_based_security",
            action = ?action,
            err = ?e,
            "Claims-based-security AMQP operation failed."
        );
        action
    }
}

#[async_trait::async_trait]
impl AmqpClaimsBasedSecurityApis for RecoverableClaimsBasedSecurity {
    // skip_all keeps the `secret` (the SAS/bearer token) out of the span; only
    // the safe identifiers are promoted into fields. `expires_on` is a timestamp,
    // not a credential, so it is safe to record.
    #[tracing::instrument(
        level = "debug",
        skip_all,
        fields(
            path = %path,
            operation = "authorize_path",
            expires_on = %expires_on,
        ),
        err(level = "warn"),
    )]
    async fn authorize_path(
        &self,
        path: String,
        token_type: Option<String>,
        secret: &Secret,
        expires_on: OffsetDateTime,
    ) -> Result<()> {
        let result = recover_azure_operation(
            || {
                let path = path.clone();
                let token_type = token_type.clone();
                let secret = secret.clone();

                async move {
                    let claims_based_security_client = self
                        .recoverable_connection
                        .upgrade()
                        .ok_or_else(|| AmqpError::with_message("Missing Connection"))?
                        .ensure_amqp_cbs()
                        .await
                        .map_err(|e| {
                            AmqpError::from(azure_core::Error::with_error(
                                AzureErrorKind::Other,
                                e,
                                "Failed to ensure AMQP CBS",
                            ))
                        })?;
                    claims_based_security_client
                        .authorize_path(path, token_type, &secret, expires_on)
                        .await
                }
            },
            &self
                .recoverable_connection
                .upgrade()
                .ok_or_else(|| AmqpError::with_message("Missing connection"))?
                .retry_options,
            Self::should_retry_claims_based_security_response,
            Some(move |connection: Weak<RecoverableConnection>, reason| {
                let connection = connection.clone();
                Box::pin(async move {
                    // Use the static method from RecoverableConnection to recover from the error.
                    RecoverableConnection::recover_from_error(connection, reason).await
                })
            }),
            Some(self.recoverable_connection.clone()),
        )
        .await?;
        Ok(result)
    }

    async fn attach(&self) -> Result<()> {
        unimplemented!("AmqpClaimsBasedSecurityClient does not support attach operation");
    }

    async fn detach(self) -> Result<()> {
        unimplemented!("AmqpClaimsBasedSecurityClient does not support detach operation");
    }
}
