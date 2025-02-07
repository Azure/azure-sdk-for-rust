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
/// use azure_messaging_eventhubs::EventHubClient;
/// use azure_identity::DefaultAzureCredential;
/// use futures::stream::StreamExt;
/// use std::error::Error;
/// ```
pub struct MessageReceiver {
    receiver: AmqpReceiver,
    timeout: Option<std::time::Duration>,
}

impl MessageReceiver {
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
                    let delivery_or_timeout =  tokio::time::timeout(delivery_timeout,
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
