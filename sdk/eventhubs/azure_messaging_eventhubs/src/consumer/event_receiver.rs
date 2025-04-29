// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{common::retry_azure_operation, models::ReceivedEventData, RetryOptions};
use async_stream::try_stream;
use azure_core::error::{ErrorKind as AzureErrorKind, Result};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpDeliveryApis as _, AmqpError, AmqpReceiver, AmqpReceiverApis as _,
};
use futures::{select, FutureExt, Stream, StreamExt};
use std::{error::Error, time::Duration};
use tracing::{debug, trace, warn};

/// A message receiver that can be used to receive messages from an Event Hub.
///
/// This is the main type for receiving messages from an Event Hub. It can be used to receive messages from an Event Hubs partition.
///
/// # Examples
///
/// ```no_run
/// use azure_messaging_eventhubs::ConsumerClient;
/// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let my_credential = DefaultAzureCredential::new()?;
///     let consumer = ConsumerClient::builder()
///        .open("my_namespace", "my_eventhub".to_string(), my_credential).await?;
///     let partition_id = "0".to_string();
///
///     let receiver  = consumer.open_receiver_on_partition(partition_id, None).await?;
///
///     let mut event_stream = receiver.stream_events();
///
///     while let Some(event_result) = event_stream.next().await {
///         match event_result {
///             Ok(event) => {
///                 // Process the received event
///                 println!("Received event: {:?}", event);
///             }
///             Err(err) => {
///                 // Handle the error
///                 eprintln!("Error receiving event: {:?}", err);
///             }
///         }
///     }
///
///     consumer.close().await?;
///     Ok(())
/// }
/// ```
pub struct EventReceiver {
    receiver: AmqpReceiver,
    partition_id: String,
    retry_options: RetryOptions,
    timeout: Option<Duration>,
}

impl EventReceiver {
    pub(crate) fn new(
        receiver: AmqpReceiver,
        partition_id: String,
        retry_options: RetryOptions,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            receiver,
            partition_id,
            timeout,
            retry_options,
        }
    }

    /// Returns the partition ID of the receiver.
    pub fn partition_id(&self) -> &str {
        &self.partition_id
    }

    fn should_retry_receive_operation(e: &azure_core::Error) -> bool {
        match e.kind() {
            AzureErrorKind::Amqp => {
                warn!("Amqp operation failed: {}", e.source().unwrap());
                if let Some(e) = e.source() {
                    debug!("Error: {}", e);

                    if let Some(amqp_error) = e.downcast_ref::<Box<AmqpError>>() {
                        Self::should_retry_amqp_error(amqp_error)
                    } else {
                        debug!("Non AMQP error: {}", e);
                        false
                    }
                } else {
                    debug!("No source error found");
                    false
                }
            }
            _ => {
                debug!("Non AMQP error: {}", e);
                false
            }
        }
    }

    fn should_retry_amqp_error(amqp_error: &AmqpError) -> bool {
        match amqp_error.kind() {
            AmqpErrorKind::ManagementStatusCode(code, _) => {
                debug!("Management operation error: {}", code);
                match code {
                    // Retry on 408 (Request Timeout) and 429 (Too Many Requests)
                    azure_core::http::StatusCode::RequestTimeout
                    | azure_core::http::StatusCode::TooManyRequests
                    | azure_core::http::StatusCode::InternalServerError
                    | azure_core::http::StatusCode::BadGateway
                    | azure_core::http::StatusCode::ServiceUnavailable
                    | azure_core::http::StatusCode::GatewayTimeout => true,
                    _ => false,
                }
            }
            AmqpErrorKind::AmqpDescribedError(described_error) => {
                debug!("AMQP described error: {:?}", described_error);
                matches!(
                    described_error.condition(),
                    AmqpErrorCondition::ResourceLimitExceeded
                        | AmqpErrorCondition::ConnectionFramingError
                        | AmqpErrorCondition::LinkStolen
                )
            }
            _ => {
                debug!("Other AMQP error: {}", amqp_error);
                false
            }
        }
    }

    /// Receives messages from the Event Hub partition.
    /// This method returns a stream of [`ReceivedEventData`] that can be used to receive messages from the Event Hub.
    /// The stream will continue to yield messages as long as the receiver is not closed.
    /// The stream will yield an error if there is an issue receiving messages from the Event Hub.
    ///
    /// # Returns
    ///
    /// A stream of [`ReceivedEventData`] that can be used to receive messages from the Event Hub.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use azure_messaging_eventhubs::EventReceiver;
    /// use futures::stream::StreamExt;
    ///
    /// async fn receive_events(receiver: &EventReceiver) {
    ///     let mut event_stream = receiver.stream_events();
    ///
    ///     while let Some(event_result) = event_stream.next().await {
    ///         match event_result {
    ///             Ok(event) => {
    ///                 // Process the received event
    ///                 println!("Received event: {:?}", event);
    ///             }
    ///             Err(err) => {
    ///                 // Handle the error
    ///                 eprintln!("Error receiving event: {:?}", err);
    ///             }
    ///         }
    ///     }
    /// }
    ///
    /// ```
    ///
    pub fn stream_events(&self) -> impl Stream<Item = azure_core::Result<ReceivedEventData>> + '_ {
        // Use async_stream to create a stream that yields messages from the receiver.
        try_stream! {
            loop {
                 let delivery = retry_azure_operation(async move || {
                     if let Some(delivery_timeout) = self.timeout {
                    select! {
                        delivery = self.receiver.receive_delivery().fuse() => Ok(delivery),
                        _ = azure_core::sleep::sleep(delivery_timeout).fuse() => {
                             Err(azure_core::Error::new(
                                AzureErrorKind::Io,
                                Box::new(std::io::Error::from(std::io::ErrorKind::TimedOut))))
                        },
                    }?
                 } else {
                     self.receiver.receive_delivery().await
                 }}, &self.retry_options, Some(Self::should_retry_receive_operation)).await?;

                 // Now that we have a delivery, we can process it.
                 let message = delivery.into_message();
                 let message = ReceivedEventData::from(message);
                 trace!("Received message: {:?}", message);
                 yield message;
            }
        }
        .boxed()
    }

    /// Closes the event receiver, detaching from the remote.
    pub async fn close(self) -> Result<()> {
        self.receiver.detach().await?;
        Ok(())
    }
}
