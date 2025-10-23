// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::common::retry::ErrorRecoveryAction;

use crate::common::recover_azure_operation;
use azure_core::{error::ErrorKind as AzureErrorKind, http::Url};
use azure_core_amqp::{
    error::Result, AmqpError, AmqpErrorKind, AmqpMessage, AmqpSendOptions, AmqpSendOutcome,
    AmqpSenderApis, AmqpSenderOptions, AmqpSession, AmqpTarget,
};
use std::sync::{Arc, Weak};
use tracing::warn;

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

    fn should_retry_send_operation(e: &AmqpError) -> ErrorRecoveryAction {
        RecoverableConnection::should_retry_amqp_error(e)
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl AmqpSenderApis for RecoverableSender {
    async fn send<M>(&self, message: M, options: Option<AmqpSendOptions>) -> Result<AmqpSendOutcome>
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
                    let connection = self.recoverable_connection.upgrade().ok_or_else(|| {
                        AmqpError::from(azure_core::Error::with_message(
                            AzureErrorKind::Other,
                            "Missing connection",
                        ))
                    })?;

                    // Check for forced error.
                    #[cfg(test)]
                    connection.get_forced_error()?;

                    let sender = connection.ensure_sender(&path).await.map_err(|e| {
                        AmqpError::from(azure_core::Error::with_error(
                            AzureErrorKind::Other,
                            e,
                            "Could not ensure sender",
                        ))
                    })?;
                    let outcome = sender.send_ref(message_clone.as_ref(), options).await?;
                    // We want to handle retries on the outcome - for instance, if we're throttled, the server rejects the send operation.
                    match outcome {
                        azure_core_amqp::AmqpSendOutcome::Rejected(error) => {
                            // If the error is described, return it as an AmqpDescribedError to let the retry logic
                            // handle it appropriately.
                            if let Some(described) = error {
                                warn!("Send rejected: {:?}", described);
                                Err(AmqpError::from(AmqpErrorKind::AmqpDescribedError(
                                    described,
                                )))
                            } else {
                                // The server rejected the error but didn't provide a specific error.
                                Err(AmqpError::from(AmqpErrorKind::SendRejected))
                            }
                        }
                        _ => Ok(outcome),
                    }
                }
            },
            &self
                .recoverable_connection
                .upgrade()
                .ok_or_else(|| {
                    AmqpError::from(azure_core::Error::with_message(
                        AzureErrorKind::Other,
                        "Missing connection",
                    ))
                })?
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
    ) -> Result<()> {
        unimplemented!("AmqpSenderClient does not support attach operation");
    }

    async fn detach(self) -> Result<()> {
        unimplemented!("AmqpSenderClient does not support detach operation");
    }

    async fn max_message_size(&self) -> Result<Option<u64>> {
        self.recoverable_connection
            .upgrade()
            .ok_or_else(|| {
                AmqpError::from(azure_core::Error::with_message(
                    AzureErrorKind::Other,
                    "Missing connection",
                ))
            })?
            .ensure_sender(&self.path)
            .await
            .map_err(|e| {
                AmqpError::from(azure_core::Error::with_error(
                    AzureErrorKind::Other,
                    e,
                    "Could not ensure sender",
                ))
            })?
            .max_message_size()
            .await
    }
}
