// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    common::recoverable::RecoverableConnection,
    error::{ErrorKind, EventHubsError, Result},
    models::ReceivedEventData,
};
use async_stream::try_stream;
use azure_core::{http::Url, time::Duration};
use azure_core_amqp::{
    error::{AmqpErrorCondition, AmqpErrorKind},
    AmqpDeliveryApis as _, AmqpError, AmqpReceiverApis as _, AmqpReceiverOptions, AmqpSource,
};
use futures::Stream;
use std::sync::Arc;
use tracing::trace;

/// Translates AMQP errors raised on the receive path into `EventHubsError`,
/// recognizing the `amqp:link:stolen` condition (broker-initiated displacement
/// when another receiver attaches with a higher-or-equal epoch) and surfacing
/// it as the typed `ConsumerDisconnected` variant. The retry layer (see
/// `should_retry_amqp_error` in `common/recoverable/connection.rs`) deliberately
/// returns this error rather than silently re-attaching, so this mapping is the
/// signal a stolen-partition consumer observes.
fn translate_receive_error(error: AmqpError) -> EventHubsError {
    if let AmqpErrorKind::AmqpDescribedError(described) = error.kind() {
        if matches!(described.condition, AmqpErrorCondition::LinkStolen) {
            return EventHubsError::from(ErrorKind::ConsumerDisconnected(Some(described.clone())));
        }
    }
    EventHubsError::from(error)
}

/// A message receiver that can be used to receive messages from an Event Hub.
///
/// This is the main type for receiving messages from an Event Hub. It can be used to receive messages from an Event Hubs partition.
///
/// # Examples
///
/// ```no_run
/// use azure_messaging_eventhubs::ConsumerClient;
/// use azure_identity::DeveloperToolsCredential;
/// use futures::stream::StreamExt;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let my_credential = DeveloperToolsCredential::new(None)?;
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
    connection: Arc<RecoverableConnection>,
    receiver_options: AmqpReceiverOptions,
    message_source: AmqpSource,
    source_url: Url,
    partition_id: String,
    timeout: Option<Duration>,
}

impl EventReceiver {
    pub(crate) fn new(
        connection: Arc<RecoverableConnection>,
        receiver_options: AmqpReceiverOptions,
        message_source: AmqpSource,
        source_url: Url,
        partition_id: String,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            source_url,
            connection,
            receiver_options,
            message_source,
            partition_id,
            timeout,
        }
    }

    /// Returns the partition ID of the receiver.
    pub fn partition_id(&self) -> &str {
        &self.partition_id
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
    pub fn stream_events(&self) -> impl Stream<Item = Result<ReceivedEventData>> + '_ {
        // Use async_stream to create a stream that yields messages from the receiver.
        // AMQP errors are translated through `translate_receive_error` so the
        // broker-initiated link-stolen condition surfaces as the typed
        // `EventHubsError::ConsumerDisconnected` rather than an opaque
        // `AmqpError`. Consumers pattern-matching on
        // `ErrorKind::ConsumerDisconnected` can break out of their receive
        // loop and re-acquire a partition client.
        Box::pin(try_stream! {
            loop {
                let receiver = self.connection.get_receiver(&self.source_url,
                    self.message_source.clone(),
                    self.receiver_options.clone(),
                    self.timeout
                ).await?;

                let delivery = receiver.receive_delivery().await.map_err(translate_receive_error)?;


                 // Now that we have a delivery, we can process it.
                 let message = delivery.into_message();
                 let message = ReceivedEventData::from(message);
                 trace!("Received message: {:?}", message);
                 yield message;
            }
        })
    }

    /// Closes the event receiver, detaching from the remote.
    pub async fn close(self) -> Result<()> {
        self.connection.close_receiver(&self.source_url).await
    }

    /// Closes the underlying AMQP receiver link without consuming the
    /// `EventReceiver` value.
    ///
    /// This is the cooperative-shutdown counterpart to `close(self)` for
    /// callers that hold the receiver by shared reference (for example, the
    /// `EventProcessor`'s partition-revocation path, which must close a
    /// receiver while user code still holds an `Arc<PartitionClient>` over
    /// it). The next `stream_events()` poll on this receiver will resolve
    /// with an error so the consumer's loop can terminate.
    pub(crate) async fn request_close(&self) -> Result<()> {
        self.connection.close_receiver(&self.source_url).await
    }
}

impl Drop for EventReceiver {
    fn drop(&mut self) {
        trace!("Dropping EventReceiver for partition {}", self.partition_id);
    }
}
