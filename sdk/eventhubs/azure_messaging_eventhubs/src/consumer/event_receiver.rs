// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::models::ReceivedEventData;
use async_stream::try_stream;
use azure_core::error::{ErrorKind as AzureErrorKind, Result};
use azure_core_amqp::{AmqpDeliveryApis as _, AmqpReceiver, AmqpReceiverApis as _};
use futures::Stream;
use futures_lite::future::FutureExt;
use tracing::trace;

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
///        .open("my_namespace", "my_eventhub", my_credential).await?;
///     let partition_id = "0";
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
    timeout: Option<std::time::Duration>,
}

impl EventReceiver {
    pub(crate) fn new(
        receiver: AmqpReceiver,
        partition_id: &str,
        timeout: Option<std::time::Duration>,
    ) -> Self {
        Self {
            receiver,
            partition_id: partition_id.to_string(),
            timeout,
        }
    }

    /// Returns the partition ID of the receiver.
    pub fn partition_id(&self) -> &String {
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
    pub fn stream_events(&self) -> impl Stream<Item = azure_core::Result<ReceivedEventData>> + '_ {
        // Use async_stream to create a stream that yields messages from the receiver.
        let stream = try_stream! {
            loop {
                 let delivery = if let Some(delivery_timeout) = self.timeout {
                     self.receiver.receive_delivery().or(async {
                        azure_core::sleep::sleep(delivery_timeout).await;
                        Err(azure_core::Error::new(AzureErrorKind::Io, Box::new(std::io::Error::from(std::io::ErrorKind::TimedOut))))
                     }).await?
                 } else {
                     self.receiver.receive_delivery().await?
                 };
                 // Now that we have a delivery, we can process it.
                 let message = delivery.into_message();
                 let message = ReceivedEventData::from(message);
                 trace!("Received message: {:?}", message);
                 yield message;
            }
        };

        Box::pin(stream)
    }

    /// Closes the event receiver, detaching from the remote.
    pub async fn close(self) -> Result<()> {
        self.receiver.detach().await?;
        Ok(())
    }
}
