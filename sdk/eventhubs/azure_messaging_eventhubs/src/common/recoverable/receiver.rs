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
use tracing::debug;

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
        RecoverableConnection::should_retry_amqp_error(e)
    }
}

impl Drop for RecoverableReceiver {
    fn drop(&mut self) {
        debug!("Dropping RecoverableReceiver for {}", self.source_url);
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
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
                debug!("Starting receive_delivery operation");
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
                        .map_err(|e| {
                            AmqpError::with_message(format!("Failed to ensure receiver: {e}"))
                        })?
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
