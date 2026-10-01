// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::common::retry::ErrorRecoveryAction;

use crate::common::recover_azure_operation;
use azure_core::{error::ErrorKind as AzureErrorKind, http::Url};
use azure_core_amqp::{
    error::Result, AmqpError, AmqpErrorKind, AmqpMessage, AmqpSendOptions, AmqpSendOutcome,
    AmqpSender, AmqpSenderApis, AmqpSenderOptions, AmqpSession, AmqpTarget,
};
use futures::{pin_mut, select_biased, FutureExt};
use std::{
    fmt,
    future::Future,
    sync::{Arc, Weak},
};
use tracing::{instrument, warn};

/// Thin wrapper around the [`AmqpSenderApis`] trait that implements the retry functionality.
///
/// An RecoverableSender is a thin wrapper around the [`AmqpSenderApis`] trait which implements
/// the retry functionality. That allows implementations which call into the Send API to not have
/// to worry about retrying the operation themselves.
pub(crate) struct RecoverableSender {
    recoverable_connection: Weak<RecoverableConnection>,
    path: Url,
}

#[derive(Debug)]
struct SenderInvalidated;

impl fmt::Display for SenderInvalidated {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sender resources were invalidated by connection recovery.")
    }
}

impl std::error::Error for SenderInvalidated {}

impl From<SenderInvalidated> for AmqpError {
    fn from(value: SenderInvalidated) -> Self {
        AmqpErrorKind::TransportImplementationError(Box::new(value)).into()
    }
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
        if matches!(e.kind(), AmqpErrorKind::TransportImplementationError(cause) if cause.is::<SenderInvalidated>())
        {
            // A peer already invalidated the caches. Use the normal retry
            // backoff without invalidating that peer's new resources again.
            return ErrorRecoveryAction::RetryAction;
        }
        RecoverableConnection::should_retry_amqp_error(e)
    }

    fn finish_attempt<T>(result: Result<T>, generation_current: bool) -> Result<T> {
        match result {
            Err(_) if !generation_current => Err(SenderInvalidated.into()),
            result => result,
        }
    }

    async fn with_current_sender<F, Fut, T>(&self, operation: F) -> Result<T>
    where
        F: FnOnce(Arc<AmqpSender>) -> Fut,
        Fut: Future<Output = Result<T>>,
    {
        let connection = self.recoverable_connection.upgrade().ok_or_else(|| {
            AmqpError::from(azure_core::Error::with_message(
                AzureErrorKind::Other,
                "Missing connection",
            ))
        })?;
        #[cfg(test)]
        connection.get_forced_error()?;

        let (generation, invalidated) = connection.sender_invalidation().await?;
        // Preparation can run CBS recovery on this task. Let it finish before
        // racing the transport operation against the peer's notification.
        let sender = connection.ensure_sender(&self.path).await.map_err(|e| {
            AmqpError::from(azure_core::Error::with_error(
                AzureErrorKind::Other,
                e,
                "Could not ensure sender",
            ))
        });
        let generation_current =
            connection.generation_is_current(generation) && invalidated.get().is_none();
        let sender = Self::finish_attempt(sender, generation_current)?;
        if !generation_current {
            return Err(SenderInvalidated.into());
        }
        let operation = operation(sender).fuse();
        let changed = invalidated.wait().fuse();
        pin_mut!(operation, changed);
        select_biased! {
            result = operation => Self::finish_attempt(
                result,
                connection.generation_is_current(generation) && invalidated.get().is_none(),
            ),
            _ = changed => Err(SenderInvalidated.into()),
        }
    }
}

#[async_trait::async_trait]
impl AmqpSenderApis for RecoverableSender {
    // Hot per-message path: trace level, skip the message body (never log payloads),
    // and no `err` attribute to avoid per-message error spam.
    #[instrument(level = "trace", skip_all, fields(path = %self.path))]
    async fn send<M>(&self, message: M, options: Option<AmqpSendOptions>) -> Result<AmqpSendOutcome>
    where
        M: Into<AmqpMessage> + std::fmt::Debug + Send,
    {
        let message_arc = Arc::new(message.into());
        let outcome = recover_azure_operation(
            move || {
                let options = options.clone();
                let path = self.path.clone();
                let message_clone = message_arc.clone();
                async move {
                    self.with_current_sender(|sender| async move {
                        let outcome = sender.send_ref(message_clone.as_ref(), options).await?;
                        // We want to handle retries on the outcome - for instance, if we're throttled, the server rejects the send operation.
                        match outcome {
                            azure_core_amqp::AmqpSendOutcome::Rejected(error) => {
                                // If the error is described, return it as an AmqpDescribedError to let the retry logic
                                // handle it appropriately.
                                if let Some(described) = error {
                                    warn!(
                                        path = %path,
                                        condition = ?described.condition,
                                        "Send rejected by remote."
                                    );
                                    Err(AmqpError::from(AmqpErrorKind::AmqpDescribedError(
                                        described,
                                    )))
                                } else {
                                    // The server rejected the error but didn't provide a specific error.
                                    warn!(
                                        path = %path,
                                        "Send rejected by remote with no described error."
                                    );
                                    Err(AmqpError::from(AmqpErrorKind::SendRejected))
                                }
                            }
                            _ => Ok(outcome),
                        }
                    })
                    .await
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
        let max_message_size = recover_azure_operation(
            || async move {
                self.with_current_sender(|sender| async move { sender.max_message_size().await })
                    .await
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
        Ok(max_message_size)
    }
}

#[cfg(test)]
mod tests;
