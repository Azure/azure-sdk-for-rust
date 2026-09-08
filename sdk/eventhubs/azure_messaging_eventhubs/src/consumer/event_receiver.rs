// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    common::recoverable::RecoverableConnection,
    error::{find_link_stolen, ErrorKind, EventHubsError, Result},
    models::ReceivedEventData,
};
use async_stream::try_stream;
use azure_core::{http::Url, time::Duration};
use azure_core_amqp::{
    error::AmqpErrorKind, AmqpDeliveryApis as _, AmqpError, AmqpReceiverApis as _,
    AmqpReceiverOptions, AmqpSource,
};
use futures::Stream;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tracing::{debug, trace, warn, Instrument};

/// Maps `amqp:link:stolen` (broker-initiated epoch displacement) on the
/// receive path to the typed `ConsumerDisconnected` variant. Other errors
/// pass through unchanged.
///
/// `partition_id` and `source_url` are accepted purely for diagnostics so the
/// silent failure path (link-stolen displacement and other receive errors) is
/// logged with the partition/link context before the error propagates.
fn translate_receive_error(
    error: AmqpError,
    partition_id: &str,
    source_url: &Url,
) -> EventHubsError {
    // The condition can be wrapped: a re-attach rejected with `amqp:link:stolen`
    // reaches this point inside the `ensure_receiver` wrapper, so look at the
    // whole source chain and not only the top-level kind.
    if let Some(described) = find_link_stolen(&error) {
        // Broker displaced this consumer (a higher epoch/owner attached).
        // Recoverable on the processor path, so warn rather than error.
        warn!(
            partition_id = %partition_id,
            source_url = %source_url,
            condition = ?described.condition,
            "Receiver link stolen by the broker (epoch displacement); mapping to ConsumerDisconnected."
        );
        return EventHubsError::from(ErrorKind::ConsumerDisconnected(Some(described.clone())));
    }
    if let AmqpErrorKind::AmqpDescribedError(described) = error.kind() {
        warn!(
            partition_id = %partition_id,
            source_url = %source_url,
            condition = ?described.condition,
            "Receive delivery failed with an AMQP error condition."
        );
    } else {
        warn!(
            partition_id = %partition_id,
            source_url = %source_url,
            err = ?error,
            "Receive delivery failed."
        );
    }
    EventHubsError::from(error)
}

/// Maps `amqp:link:stolen` on the attach path to `ConsumerDisconnected`.
///
/// The stream re-attaches on every loop iteration, so the broker can reject the
/// attach itself with `amqp:link:stolen` rather than failing an in-flight
/// receive. Without this, the same displacement would surface as a plain
/// `ErrorKind::AmqpError` only because it took the attach path.
fn translate_attach_error(
    error: EventHubsError,
    partition_id: &str,
    source_url: &Url,
) -> EventHubsError {
    let ErrorKind::AmqpError(amqp_error) = &error.kind else {
        return error;
    };
    match find_link_stolen(amqp_error) {
        Some(described) => {
            warn!(
                partition_id = %partition_id,
                source_url = %source_url,
                condition = ?described.condition,
                "Receiver attach rejected by the broker (epoch displacement); mapping to ConsumerDisconnected."
            );
            EventHubsError::from(ErrorKind::ConsumerDisconnected(Some(described.clone())))
        }
        None => error,
    }
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
    // Set by `request_close()` to terminate `stream_events()` even if
    // `close_receiver` could not detach by-value because an in-flight
    // receive holds a strong Arc on the AMQP receiver.
    closed: AtomicBool,
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
            closed: AtomicBool::new(false),
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
        // Attach a span to the returned stream rather than using
        // `#[tracing::instrument]` on this sync fn: the attribute would only span
        // the stream's *construction* (which returns immediately), leaving the
        // receive loop's awaits and events with no parent span. Instrumenting each
        // awaited future with this span keeps per-partition correlation for the loop.
        let span = tracing::debug_span!(
            "stream_events",
            connection_id = %self.connection.get_connection_id(),
            partition_id = %self.partition_id,
            source_url = %self.source_url,
        );
        Box::pin(try_stream! {
            loop {
                // Stop here if `request_close` has been called; otherwise
                // `get_receiver` below would reattach a new link.
                if self.closed.load(Ordering::Acquire) {
                    span.in_scope(|| debug!(
                        partition_id = %self.partition_id,
                        source_url = %self.source_url,
                        "Event stream terminating: receiver was closed by request_close()."
                    ));
                    Err(EventHubsError::from(ErrorKind::ConsumerDisconnected(None)))?;
                }

                // Instrument each awaited operation with the stream's span so the
                // receive loop is parented under it on every poll (see the span
                // construction above for why this is not a fn-level attribute).
                let receiver = self.connection.get_receiver(&self.source_url,
                    self.message_source.clone(),
                    self.receiver_options.clone(),
                    self.timeout
                ).instrument(span.clone()).await
                    .map_err(|e| translate_attach_error(e, &self.partition_id, &self.source_url))?;

                let delivery = receiver
                    .receive_delivery()
                    .instrument(span.clone())
                    .await
                    .map_err(|e| translate_receive_error(e, &self.partition_id, &self.source_url))?;

                // Now that we have a delivery, we can process it.
                let message = delivery.into_message();
                let message = ReceivedEventData::from(message);
                // SENSITIVE-DATA: `{:?}` on a ReceivedEventData dumps the
                // raw AMQP message, including the customer payload body and any PII in
                // application properties. This is redacted by the SafeDebug derive ONLY
                // when the azure_core / typespec `debug` cargo feature is OFF (the
                // default). If a downstream build enables that feature, this trace will
                // emit full message bodies. Keep this at trace! and prefer logging only
                // sequence_number / offset / partition_id at higher levels. See the
                // matching note on EventData/ReceivedEventData in models/event_data.rs.
                span.in_scope(|| trace!("Received message: {:?}", message));
                yield message;
            }
        })
    }

    /// Closes the event receiver, detaching from the remote.
    pub async fn close(self) -> Result<()> {
        self.connection.close_receiver(&self.source_url).await
    }

    /// Closes the AMQP receiver without consuming the `EventReceiver`.
    /// Used by `EventProcessor` to revoke a partition while the consumer
    /// still holds an `Arc<PartitionClient>`. Sets the close flag before
    /// the detach so the next `stream_events()` poll resolves with
    /// `ConsumerDisconnected` regardless of detach outcome.
    pub(crate) async fn request_close(&self) -> Result<()> {
        self.closed.store(true, Ordering::Release);
        self.connection.close_receiver(&self.source_url).await
    }
}

impl Drop for EventReceiver {
    fn drop(&mut self) {
        trace!("Dropping EventReceiver for partition {}", self.partition_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core_amqp::{error::AmqpErrorCondition, AmqpDescribedError};

    fn source_url() -> Url {
        Url::parse("amqps://example.servicebus.windows.net/eh/Partitions/0").unwrap()
    }

    fn stolen() -> AmqpError {
        AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkStolen,
            Some("New receiver with higher epoch of '1' is created".to_string()),
            Default::default(),
        )))
    }

    /// Wraps an error exactly as the receive retry loop does for an attach
    /// failure. This calls the production wrapper rather than copying its
    /// shape, so a change to the wrapper cannot leave these tests passing
    /// against a shape that no longer exists.
    fn wrapped_in_ensure_receiver(inner: AmqpError) -> AmqpError {
        crate::common::recoverable::receiver::RecoverableReceiver::ensure_receiver_error(inner)
    }

    // A stolen link reported directly on the receive path is the case the
    // 0.15.0 CHANGELOG documents.
    #[test]
    fn translate_receive_error_maps_top_level_link_stolen() {
        let translated = translate_receive_error(stolen(), "0", &source_url());
        assert!(matches!(
            translated.kind,
            ErrorKind::ConsumerDisconnected(Some(_))
        ));
    }

    // The broker can reject the re-attach instead of the in-flight receive. The
    // condition then reaches the stream wrapped by `ensure_receiver`. Before the
    // fix this wrapper was a message string, so the condition was lost and the
    // caller saw `ErrorKind::AmqpError`.
    #[test]
    fn translate_receive_error_maps_link_stolen_wrapped_by_ensure_receiver() {
        let translated =
            translate_receive_error(wrapped_in_ensure_receiver(stolen()), "0", &source_url());
        assert!(
            matches!(translated.kind, ErrorKind::ConsumerDisconnected(Some(_))),
            "expected ConsumerDisconnected, got {:?}",
            translated.kind
        );
    }

    // Other conditions must keep their existing shape.
    #[test]
    fn translate_receive_error_passes_other_conditions_through() {
        let other = AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::ServerBusyError,
            None,
            Default::default(),
        )));
        let translated = translate_receive_error(other, "0", &source_url());
        assert!(matches!(translated.kind, ErrorKind::AmqpError(_)));
    }

    #[test]
    fn translate_receive_error_passes_non_described_errors_through() {
        let translated =
            translate_receive_error(AmqpError::with_message("boom"), "0", &source_url());
        assert!(matches!(translated.kind, ErrorKind::AmqpError(_)));
    }

    // The stream re-attaches on every loop iteration, so a displacement can be
    // reported by `get_receiver` and never touch the receive path at all.
    #[test]
    fn translate_attach_error_maps_link_stolen() {
        let attach_error = EventHubsError::from(stolen());
        let translated = translate_attach_error(attach_error, "0", &source_url());
        assert!(
            matches!(translated.kind, ErrorKind::ConsumerDisconnected(Some(_))),
            "expected ConsumerDisconnected, got {:?}",
            translated.kind
        );
    }

    #[test]
    fn translate_attach_error_maps_link_stolen_wrapped_in_azure_core() {
        let attach_error = EventHubsError::from(wrapped_in_ensure_receiver(stolen()));
        let translated = translate_attach_error(attach_error, "0", &source_url());
        assert!(
            matches!(translated.kind, ErrorKind::ConsumerDisconnected(Some(_))),
            "expected ConsumerDisconnected, got {:?}",
            translated.kind
        );
    }

    /// Builds an `EventReceiver` over a real `RecoverableConnection` whose
    /// next receiver attach fails with `attach_error`. No network activity
    /// happens: the injected error stops `ensure_receiver` before it opens
    /// a connection.
    fn receiver_with_failing_attach(attach_error: AmqpError) -> EventReceiver {
        let connection = RecoverableConnection::new(
            Url::parse("amqps://example.servicebus.windows.net").unwrap(),
            None,
            None,
            Arc::new(azure_core_test::credentials::MockCredential),
            Default::default(),
            None,
        );
        connection.force_attach_error(attach_error).unwrap();
        EventReceiver::new(
            connection,
            AmqpReceiverOptions::default(),
            AmqpSource::builder()
                .with_address(source_url().to_string())
                .build(),
            source_url(),
            "0".to_string(),
            None,
        )
    }

    // Drives the real stream. The function-level tests above prove what
    // `translate_attach_error` does when it is called; only this test proves
    // that `stream_events` calls it on the `get_receiver` failure path. If
    // the `map_err` at that call site is removed, the error surfaces as
    // `ErrorKind::AmqpError` and this test fails.
    #[tokio::test]
    async fn stream_events_maps_stolen_attach_to_consumer_disconnected() {
        use futures::StreamExt;

        let receiver = receiver_with_failing_attach(stolen());
        let mut stream = std::pin::pin!(receiver.stream_events());
        let error = stream
            .next()
            .await
            .expect("the stream yields the attach failure")
            .expect_err("the injected attach error must surface");
        assert!(
            matches!(error.kind, ErrorKind::ConsumerDisconnected(Some(_))),
            "expected ConsumerDisconnected, got {:?}",
            error.kind
        );
    }

    // A non-stolen attach failure must keep its kind through the same path,
    // so callers cannot mistake a transport failure for a stolen partition.
    #[tokio::test]
    async fn stream_events_passes_other_attach_errors_through() {
        use futures::StreamExt;

        let receiver = receiver_with_failing_attach(AmqpError::with_message("attach failed"));
        let mut stream = std::pin::pin!(receiver.stream_events());
        let error = stream
            .next()
            .await
            .expect("the stream yields the attach failure")
            .expect_err("the injected attach error must surface");
        assert!(
            matches!(error.kind, ErrorKind::AmqpError(_)),
            "expected AmqpError, got {:?}",
            error.kind
        );
    }

    // An attach that fails for any other reason keeps its kind, so callers that
    // match on `ConsumerDisconnected` do not treat a transport failure as a
    // stolen partition.
    #[test]
    fn translate_attach_error_passes_other_errors_through() {
        let attach_error = EventHubsError::from(AmqpError::with_message("attach failed"));
        let translated = translate_attach_error(attach_error, "0", &source_url());
        assert!(matches!(translated.kind, ErrorKind::AmqpError(_)));

        let attach_error = EventHubsError::with_message("not an AMQP error");
        let translated = translate_attach_error(attach_error, "0", &source_url());
        assert!(matches!(translated.kind, ErrorKind::SimpleMessage(_)));
    }
}
