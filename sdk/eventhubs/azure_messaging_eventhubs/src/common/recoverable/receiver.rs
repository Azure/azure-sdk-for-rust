// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::common::recover_azure_operation;
use crate::common::retry::ErrorRecoveryAction;
use azure_core::{error::ErrorKind as AzureErrorKind, http::Url, time::Duration};
use azure_core_amqp::{
    error::Result, AmqpError, AmqpReceiverApis, AmqpReceiverOptions, AmqpSession, AmqpSource,
};
use futures::{select, FutureExt};
use std::sync::Weak;
use tracing::{debug, instrument, trace};

pub(crate) struct RecoverableReceiver {
    recoverable_connection: Weak<RecoverableConnection>,
    source_url: Url,
    message_source: AmqpSource,
    receiver_options: AmqpReceiverOptions,
    timeout: Option<Duration>,
}

impl RecoverableReceiver {
    pub(super) fn new(
        recoverable_connection: Weak<RecoverableConnection>,
        receiver_options: AmqpReceiverOptions,
        message_source: AmqpSource,
        source_url: Url,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            source_url,
            recoverable_connection,
            receiver_options,
            message_source,
            timeout,
        }
    }

    fn should_retry_receive_operation(e: &AmqpError) -> ErrorRecoveryAction {
        RecoverableConnection::should_retry_receive_error(e)
    }

    /// Wraps an `ensure_receiver` failure so the original error stays reachable
    /// through the source chain.
    ///
    /// A re-attach that the broker rejects with `amqp:link:stolen` arrives here
    /// as an `AmqpDescribedError`. Flattening it into a message string would
    /// destroy the condition, so the retry decider and the stream translation
    /// could no longer tell a stolen partition from any other attach failure.
    /// This mirrors the wrapper the sender and CBS paths use.
    pub(crate) fn ensure_receiver_error(e: AmqpError) -> AmqpError {
        AmqpError::from(azure_core::Error::with_error(
            AzureErrorKind::Other,
            e,
            "Failed to ensure receiver",
        ))
    }
}

impl Drop for RecoverableReceiver {
    fn drop(&mut self) {
        debug!("Dropping RecoverableReceiver for {}", self.source_url);
    }
}

#[async_trait::async_trait]
impl AmqpReceiverApis for RecoverableReceiver {
    async fn attach(
        &self,
        _session: &AmqpSession,
        _source: impl Into<AmqpSource> + Send,
        _options: Option<AmqpReceiverOptions>,
    ) -> Result<()> {
        unimplemented!("AmqpReceiverClient does not support attach operation");
    }

    async fn detach(self) -> Result<()> {
        unimplemented!("AmqpReceiverClient does not support detach operation");
    }

    async fn set_credit_mode(&self, _mode: azure_core_amqp::ReceiverCreditMode) -> Result<()> {
        unimplemented!("AmqpReceiverClient does not support set_credit_mode operation");
    }

    async fn credit_mode(&self) -> Result<azure_core_amqp::ReceiverCreditMode> {
        unimplemented!("AmqpReceiverClient does not support credit_mode operation");
    }

    // Hot per-event path: trace level and no `err` attribute to avoid per-delivery
    // error spam; carry only the partition source URL for correlation.
    #[instrument(level = "trace", skip_all, fields(source_url = %self.source_url))]
    async fn receive_delivery(&self) -> Result<azure_core_amqp::AmqpDelivery> {
        let retry_options = {
            self.recoverable_connection
                .upgrade()
                .ok_or_else(|| AmqpError::with_message("Missing connection"))?
                .retry_options
                .clone()
        };
        let delivery = recover_azure_operation(
            || async move {
                trace!(source_url = %self.source_url, "Starting receive_delivery operation.");
                let receiver = {
                    let connection = self
                        .recoverable_connection
                        .upgrade()
                        .ok_or_else(|| AmqpError::with_message("Missing connection"))?;

                    // Check for forced error.
                    #[cfg(test)]
                    connection.get_forced_error()?;

                    connection
                        .ensure_receiver(
                            &self.source_url,
                            &self.message_source,
                            &self.receiver_options,
                        )
                        .await
                        .map_err(Self::ensure_receiver_error)?
                };
                if let Some(delivery_timeout) = self.timeout {
                    select! {
                        delivery = receiver.receive_delivery().fuse() => Ok(delivery),
                        _ = azure_core::sleep::sleep(delivery_timeout).fuse() => {
                             Err(AmqpError::from(azure_core::Error::new(
                                AzureErrorKind::Io,
                                Box::new(std::io::Error::from(std::io::ErrorKind::TimedOut)))))
                        },
                    }?
                } else {
                    receiver.receive_delivery().await
                }
            },
            &retry_options,
            Self::should_retry_receive_operation,
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
        Ok(delivery)
    }

    async fn accept_delivery(&self, _delivery: &azure_core_amqp::AmqpDelivery) -> Result<()> {
        unimplemented!("AmqpReceiverClient does not support accept_delivery operation");
    }

    async fn reject_delivery(&self, _delivery: &azure_core_amqp::AmqpDelivery) -> Result<()> {
        unimplemented!("AmqpReceiverClient does not support reject_delivery operation");
    }

    async fn release_delivery(&self, _delivery: &azure_core_amqp::AmqpDelivery) -> Result<()> {
        unimplemented!("AmqpReceiverClient does not support release_delivery operation");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::{find_link_stolen, ErrorKind};
    use azure_core_amqp::{
        error::{AmqpErrorCondition, AmqpErrorKind},
        AmqpDescribedError,
    };

    fn stolen() -> AmqpError {
        AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkStolen,
            None,
            Default::default(),
        )))
    }

    // The broker rejects a re-attach at the old epoch with `amqp:link:stolen`.
    // The wrapper must keep that condition reachable. Before the fix the
    // wrapper was `AmqpError::with_message`, which has no source, so the
    // condition was gone and the stream reported a plain AMQP error.
    #[test]
    fn ensure_receiver_error_keeps_link_stolen_reachable() {
        let wrapped = RecoverableReceiver::ensure_receiver_error(stolen());
        assert!(
            find_link_stolen(&wrapped).is_some(),
            "wrapper lost the link-stolen condition: {wrapped}"
        );
    }

    // The retry decider must see through the wrapper and refuse to reattach a
    // stolen link.
    #[test]
    fn ensure_receiver_error_is_not_retried_when_link_stolen() {
        let wrapped = RecoverableReceiver::ensure_receiver_error(stolen());
        assert_eq!(
            RecoverableReceiver::should_retry_receive_operation(&wrapped),
            ErrorRecoveryAction::ReturnError
        );
    }

    // The wrapped error must still convert into the typed variant once it
    // reaches the caller.
    #[test]
    fn ensure_receiver_error_surfaces_as_consumer_disconnected() {
        let wrapped = RecoverableReceiver::ensure_receiver_error(stolen());
        let described = find_link_stolen(&wrapped).cloned();
        let error = crate::error::EventHubsError::from(ErrorKind::ConsumerDisconnected(described));
        assert!(matches!(
            error.kind,
            ErrorKind::ConsumerDisconnected(Some(_))
        ));
    }

    // A non-stolen attach failure keeps its own classification, so the retry
    // layer can still recover a transport-level attach failure.
    #[test]
    fn ensure_receiver_error_preserves_other_kinds() {
        let inner = AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
            std::io::Error::other("closed"),
        )));
        let wrapped = RecoverableReceiver::ensure_receiver_error(inner);
        assert!(find_link_stolen(&wrapped).is_none());
        assert_eq!(
            RecoverableReceiver::should_retry_receive_operation(&wrapped),
            ErrorRecoveryAction::ReconnectLink
        );
    }
}
