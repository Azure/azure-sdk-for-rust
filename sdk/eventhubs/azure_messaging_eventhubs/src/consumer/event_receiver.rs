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
use futures::{channel::oneshot, future::Shared, pin_mut, select_biased, FutureExt, Stream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
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
    close_signal: Mutex<Option<oneshot::Sender<()>>>,
    closing: Shared<oneshot::Receiver<()>>,
    // Replaces the network receive in stream tests with a controlled delivery.
    #[cfg(test)]
    forced_receive: Mutex<Option<oneshot::Receiver<Result<ReceivedEventData>>>>,
    #[cfg(test)]
    delayed_close: Mutex<Option<(oneshot::Sender<()>, oneshot::Receiver<()>)>>,
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
        let (close_signal, closing) = oneshot::channel();
        Self {
            source_url,
            connection,
            receiver_options,
            message_source,
            partition_id,
            timeout,
            closed: AtomicBool::new(false),
            close_signal: Mutex::new(Some(close_signal)),
            closing: closing.shared(),
            #[cfg(test)]
            forced_receive: Mutex::new(None),
            #[cfg(test)]
            delayed_close: Mutex::new(None),
        }
    }

    /// Returns the partition ID of the receiver.
    pub fn partition_id(&self) -> &str {
        &self.partition_id
    }

    /// Receives events from the Event Hub partition.
    ///
    /// The stream yields events or receive errors. For a receiver managed by
    /// [`EventProcessor`](crate::EventProcessor), shutdown or partition revocation
    /// wakes pending reads and yields [`ErrorKind::ConsumerDisconnected`].
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

                let result = {
                    let closing = self.closing.clone().fuse();
                    let receive = self.receive_event().instrument(span.clone()).fuse();
                    pin_mut!(closing, receive);
                    select_biased! {
                        _ = closing => Err(EventHubsError::from(ErrorKind::ConsumerDisconnected(None))),
                        message = receive => message,
                    }
                };
                // Drop the receive future before yielding an error from try_stream!.
                let message = result?;
                // A delivery can become ready while another task requests close.
                if self.closed.load(Ordering::Acquire) {
                    Err(EventHubsError::from(ErrorKind::ConsumerDisconnected(None)))?;
                }
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

    async fn receive_event(&self) -> Result<ReceivedEventData> {
        #[cfg(test)]
        {
            let forced = self.forced_receive.lock().unwrap().take();
            if let Some(forced) = forced {
                return forced.await.expect("the test must supply a delivery");
            }
        }
        let receiver = self
            .connection
            .get_receiver(
                &self.source_url,
                self.message_source.clone(),
                self.receiver_options.clone(),
                self.timeout,
            )
            .await
            .map_err(|e| translate_attach_error(e, &self.partition_id, &self.source_url))?;
        let delivery = receiver
            .receive_delivery()
            .await
            .map_err(|e| translate_receive_error(e, &self.partition_id, &self.source_url))?;
        Ok(ReceivedEventData::from(delivery.into_message()))
    }

    /// Closes the event receiver, detaching from the remote.
    pub async fn close(self) -> Result<()> {
        self.request_close().await
    }

    /// Closes the AMQP receiver without consuming the `EventReceiver`.
    /// Used by `EventProcessor` to revoke a partition while the consumer
    /// still holds an `Arc<PartitionClient>`. Sets the close flag before
    /// the detach so the next `stream_events()` poll resolves with
    /// `ConsumerDisconnected` regardless of detach outcome.
    pub(crate) async fn request_close(&self) -> Result<()> {
        // A retained, closed receiver must not detach a replacement at the same path.
        if self.closed.swap(true, Ordering::AcqRel) {
            return Ok(());
        }
        let signal = self
            .close_signal
            .lock()
            .map_err(|_| EventHubsError::with_message("Could not lock receiver close signal."))?
            .take();
        if let Some(signal) = signal {
            let _ = signal.send(());
        }
        #[cfg(test)]
        {
            let delayed = self.delayed_close.lock().unwrap().take();
            if let Some((started, finish)) = delayed {
                let _ = started.send(());
                finish.await.expect("the test must finish the detach");
            }
        }
        self.connection.close_receiver(&self.source_url).await
    }
}

impl Drop for EventReceiver {
    fn drop(&mut self) {
        trace!("Dropping EventReceiver for partition {}", self.partition_id);
    }
}

/// Builds an `EventReceiver` over a real `RecoverableConnection` whose
/// next receiver attach fails with `attach_error`. No network activity
/// happens: the injected error stops `ensure_receiver` before it opens
/// a connection. It sits outside `mod tests` because the event processor
/// tests need it too.
#[cfg(test)]
pub(crate) fn receiver_with_failing_attach(
    partition_id: &str,
    attach_error: AmqpError,
) -> EventReceiver {
    let source_url = Url::parse(&format!(
        "amqps://example.servicebus.windows.net/eh/Partitions/{partition_id}"
    ))
    .unwrap();
    let connection = RecoverableConnection::new(
        Url::parse("amqps://example.servicebus.windows.net").unwrap(),
        None,
        None,
        Default::default(),
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
            .with_address(source_url.to_string())
            .build(),
        source_url,
        partition_id.to_string(),
        None,
    )
}

#[cfg(test)]
pub(crate) fn receiver_with_pending_receive(
    partition_id: &str,
) -> (EventReceiver, oneshot::Sender<Result<ReceivedEventData>>) {
    let receiver = receiver_with_failing_attach(partition_id, AmqpError::with_message("offline"));
    let (delivery, receive) = oneshot::channel();
    *receiver.forced_receive.lock().unwrap() = Some(receive);
    (receiver, delivery)
}

#[cfg(test)]
pub(crate) fn receiver_with_delayed_close(
    partition_id: &str,
) -> (EventReceiver, oneshot::Receiver<()>, oneshot::Sender<()>) {
    let receiver = receiver_with_failing_attach(partition_id, AmqpError::with_message("offline"));
    let (started, closing) = oneshot::channel();
    let (finish, delayed) = oneshot::channel();
    *receiver.delayed_close.lock().unwrap() = Some((started, delayed));
    (receiver, closing, finish)
}

#[cfg(test)]
mod tests {
    use super::{
        receiver_with_failing_attach, receiver_with_pending_receive, translate_attach_error,
        translate_receive_error, AmqpError, AmqpErrorKind, ErrorKind, EventHubsError,
        ReceivedEventData, Url,
    };
    use azure_core_amqp::{error::AmqpErrorCondition, AmqpDescribedError, AmqpMessage};
    use futures::{poll, StreamExt};

    #[tokio::test]
    async fn close_wakes_pending_receive() {
        let (receiver, delivery) = receiver_with_pending_receive("0");
        let mut stream = std::pin::pin!(receiver.stream_events());
        assert!(
            poll!(stream.next()).is_pending(),
            "the receive must be pending before close"
        );
        receiver.request_close().await.unwrap();
        let result = tokio::time::timeout(std::time::Duration::from_secs(1), stream.next())
            .await
            .expect("close must wake a pending receive")
            .unwrap()
            .unwrap_err();
        assert!(matches!(result.kind, ErrorKind::ConsumerDisconnected(None)));
        assert!(
            delivery.is_canceled(),
            "the pending receive must be canceled"
        );
        assert!(stream.next().await.is_none());
    }

    #[tokio::test]
    async fn close_discards_ready_delivery_from_pending_receive() {
        let (receiver, delivery) = receiver_with_pending_receive("0");
        let mut stream = std::pin::pin!(receiver.stream_events());
        assert!(poll!(stream.next()).is_pending());
        assert!(delivery
            .send(Ok(ReceivedEventData::from(AmqpMessage::default())))
            .is_ok());
        receiver.request_close().await.unwrap();
        let result = stream
            .next()
            .await
            .unwrap()
            .expect_err("close must win over the pending delivery");
        assert!(matches!(result.kind, ErrorKind::ConsumerDisconnected(None)));
    }

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

    // Drives the real stream. The function-level tests above prove what
    // `translate_attach_error` does when it is called; only this test proves
    // that `stream_events` calls it on the `get_receiver` failure path. If
    // the `map_err` at that call site is removed, the error surfaces as
    // `ErrorKind::AmqpError` and this test fails.
    #[tokio::test]
    async fn stream_events_maps_stolen_attach_to_consumer_disconnected() {
        let receiver = receiver_with_failing_attach("0", stolen());
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
        let receiver = receiver_with_failing_attach("0", AmqpError::with_message("attach failed"));
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
