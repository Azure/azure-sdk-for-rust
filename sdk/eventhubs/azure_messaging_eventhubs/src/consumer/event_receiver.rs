// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

//cspell: words amqp mgmt amqps

use crate::models::ReceivedEventData;
use async_stream::try_stream;
use azure_core::error::Result;
use azure_core_amqp::{
    messaging::AmqpDeliveryApis,
    receiver::{AmqpReceiver, AmqpReceiverApis},
};
use futures::stream::Stream;
use tracing::trace;

/// A message receiver that can be used to receive messages from an Event Hub.
///
/// This is the main type for receiving messages from an Event Hub. It can be used to receive messages from an Event Hub partition.
///
/// # Example
///
/// ```no_run
/// use azure_messaging_eventhubs::consumer::ConsumerClient;
/// use azure_identity::{DefaultAzureCredential, TokenCredentialOptions};
/// use async_std::stream::StreamExt;
/// use futures::pin_mut;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let my_credential = DefaultAzureCredential::new()?;
///     let consumer = ConsumerClient::new("my_namespace".to_string(), "my_eventhub".to_string(), None, my_credential, None);
///     let partition_id = "0";
///
///     consumer.open().await?;
///
///     let receiver  = consumer.open_receiver_on_partition(partition_id.to_string(), None).await?;
///
///     let event_stream = receiver.stream_events();
///
///     pin_mut!(event_stream);
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
///     Ok(())
/// }
/// ```
/// ```
pub struct EventReceiver {
    receiver: AmqpReceiver,
    timeout: Option<std::time::Duration>,
}

impl EventReceiver {
    pub(crate) fn new(receiver: AmqpReceiver, timeout: Option<std::time::Duration>) -> Self {
        Self { receiver, timeout }
    }

    /// Receives messages from the Event Hub.
    /// This method returns a stream of `ReceivedEventData` that can be used to receive messages from the Event Hub.
    /// The stream will continue to yield messages as long as the receiver is not closed.
    /// The stream will yield an error if there is an issue receiving messages from the Event Hub.
    ///
    pub fn stream_events(&self) -> impl Stream<Item = Result<ReceivedEventData>> + '_ {
        try_stream! {
            loop {
                if let Some(delivery_timeout) = self.timeout {
                    let delivery_or_timeout =  async_std::future::timeout(delivery_timeout,
                        self.receiver.receive_delivery()).await;
                    match delivery_or_timeout {
                        Ok(delivery_or_error) => {
                            let delivery = delivery_or_error?;
                            self.receiver.accept_delivery(&delivery).await?;
                            let message = delivery.into_message();
                            let message = ReceivedEventData::from(message);
                            trace!("Received message: {:?}", message);
                            yield message;
                        }
                        Err(e) => {
                            trace!("Timeout receiving delivery: {e:?}");
                            break;
                        }
                    }
                } else {
                        let delivery = self.receiver.receive_delivery().await?;
                        self.receiver.accept_delivery(&delivery).await?;
                        let message = delivery.into_message();
                        let message = ReceivedEventData::from(message);
                        trace!("Received message: {:?}", message);
                        yield message;
                }
            }
        }
    }
}
