// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::common::retry::ErrorRecoveryAction;
use crate::{common::recover_azure_operation, ErrorKind, EventHubsError};
use azure_core::{error::ErrorKind as AzureErrorKind, error::Result, http::Url};
use azure_core_amqp::{
    error::AmqpErrorKind, AmqpError, AmqpMessage, AmqpSendOptions, AmqpSendOutcome, AmqpSenderApis,
    AmqpSenderOptions, AmqpSession, AmqpTarget,
};
use std::error::Error;
use std::sync::{Arc, Weak};
use tracing::{debug, warn};

/// Thin wrapper around the [`AmqpSenderApis`] trait that implements the retry functionality.
///
/// An RecoverableSender is a thin wrapper around the [`AmqpSenderApis`] trait which implements
/// the retry functionality. That allows implementations which call into the Send API to not have
/// to worry about retrying the operation themselves.
pub(crate) struct RecoverableSender {
    recoverable_connection: Weak<RecoverableConnection>,
    path: Url,
}

impl RecoverableSender {
    /// Creates a new RecoverableSender.
    ///
    /// # Arguments
    ///
    /// * `recoverable_connection` - The recoverable connection to use for sending messages.
    /// * `path` - The URL path of the sender.
    pub fn new(recoverable_connection: Weak<RecoverableConnection>, path: Url) -> Self {
        Self {
            recoverable_connection,
            path,
        }
    }

    fn should_retry_send_operation(e: &azure_core::Error) -> ErrorRecoveryAction {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!(err=?e, "Amqp operation failed: {e}");
                if let Some(e) = e.source() {
                    debug!(err=?e, "Error: {e}");

                    if let Some(amqp_error) = e.downcast_ref::<Box<AmqpError>>() {
                        RecoverableConnection::should_retry_amqp_error(amqp_error)
                    } else if let Some(amqp_error) = e.downcast_ref::<AmqpError>() {
                        RecoverableConnection::should_retry_amqp_error(amqp_error)
                    } else {
                        debug!(err=?e, "Non AMQP error: {e}");
                        ErrorRecoveryAction::ReturnError
                    }
                } else {
                    debug!("No source error found");
                    ErrorRecoveryAction::ReturnError
                }
            }
            _ => {
                debug!(err=?e, "Non AMQP error: {e}");
                ErrorRecoveryAction::ReturnError
            }
        }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpSenderApis for RecoverableSender {
    async fn send<M>(
        &self,
        message: M,
        options: Option<AmqpSendOptions>,
    ) -> azure_core::Result<AmqpSendOutcome>
    where
        M: Into<AmqpMessage> + std::fmt::Debug + Send,
    {
        let message_arc = Arc::new(message.into());
        let outcome = recover_azure_operation(
            move || {
                //                let sender = self.sender.clone();
                let options = options.clone();
                let path = self.path.clone();
                let message_clone = message_arc.clone();
                async move {
                    let connection = self
                        .recoverable_connection
                        .upgrade()
                        .ok_or_else(|| EventHubsError::from(ErrorKind::MissingConnection))?;

                    // Check for forced error.
                    #[cfg(test)]
                    connection.get_forced_error()?;

                    let sender = connection.ensure_sender(&path).await?;
                    let outcome = sender.send_ref(message_clone.as_ref(), options).await?;
                    // We treat all outcomes other than "rejected" as successful.
                    match outcome {
                        azure_core_amqp::AmqpSendOutcome::Rejected(error) => {
                            // If the error is described, return it as an AmqpDescribedError to let the retry logic
                            // handle it appropriately.
                            if let Some(described) = error {
                                warn!("Send rejected: {:?}", described);
                                return Err(azure_core::Error::new(
                                    azure_core::error::ErrorKind::Amqp,
                                    AmqpError::from(AmqpErrorKind::AmqpDescribedError(described)),
                                ));
                            }
                            Err(azure_core::Error::new(
                                azure_core::error::ErrorKind::Amqp,
                                EventHubsError {
                                    kind: ErrorKind::SendRejected(error),
                                },
                            ))
                        }
                        _ => Ok(outcome),
                    }
                }
            },
            &self
                .recoverable_connection
                .upgrade()
                .ok_or_else(|| EventHubsError::from(ErrorKind::MissingConnection))?
                .retry_options,
            Self::should_retry_send_operation,
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
        Ok(outcome)
    }

    #[doc(hidden)]
    /// Sends a message reference to the Event Hubs service.
    ///
    /// Note: We do not implement this method because none of the callers of AmqpSenderClient call send_ref.
    async fn send_ref<M>(
        &self,
        _message: M,
        _options: Option<AmqpSendOptions>,
    ) -> Result<AmqpSendOutcome>
    where
        M: AsRef<AmqpMessage> + std::fmt::Debug + Send,
    {
        unimplemented!("AmqpSenderClient does not support send_ref operation");
    }

    async fn attach(
        &self,
        _session: &AmqpSession,
        _name: String,
        _target: impl Into<AmqpTarget> + Send,
        _options: Option<AmqpSenderOptions>,
    ) -> azure_core::Result<()> {
        unimplemented!("AmqpSenderClient does not support attach operation");
    }

    async fn detach(self) -> azure_core::Result<()> {
        unimplemented!("AmqpSenderClient does not support detach operation");
    }

    async fn max_message_size(&self) -> azure_core::Result<Option<u64>> {
        self.recoverable_connection
            .upgrade()
            .ok_or_else(|| EventHubsError::from(ErrorKind::MissingConnection))?
            .ensure_sender(&self.path)
            .await?
            .max_message_size()
            .await
    }
}
