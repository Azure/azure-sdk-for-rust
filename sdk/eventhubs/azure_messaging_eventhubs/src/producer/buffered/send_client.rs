// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

// cspell: ignore retryable

use crate::{error::Result, producer::ProducerClient};
use azure_core::http::Url;
use azure_core_amqp::{AmqpMessage, AmqpSendOutcome};
use std::sync::Arc;

/// The operations that a partition worker needs from the AMQP layer.
///
/// The buffered producer talks to the service only through this trait. The
/// production implementation forwards to [`ProducerClient`], which applies the
/// retry policy and the connection recovery. A test implementation supplies
/// scripted outcomes, so the worker tests do not need a network.
#[async_trait::async_trait]
pub(crate) trait BufferedSendClient: Send + Sync + 'static {
    /// Returns the partition IDs of the Event Hub.
    async fn partition_ids(&self) -> Result<Vec<String>>;

    /// Returns the largest message that the link to the partition accepts.
    async fn max_message_size(&self, partition_id: &str) -> Result<u64>;

    /// Sends one batch envelope to a partition and returns the AMQP outcome.
    ///
    /// An `Err` means the retry policy is exhausted, or the error is not
    /// retryable. The caller must treat an `Err` as a terminal failure.
    async fn send_envelope(
        &self,
        partition_id: &str,
        envelope: AmqpMessage,
    ) -> Result<AmqpSendOutcome>;
}

/// The production [`BufferedSendClient`], backed by a [`ProducerClient`].
pub(crate) struct ProducerSendClient {
    producer: Arc<ProducerClient>,
}

impl ProducerSendClient {
    pub(crate) fn new(producer: Arc<ProducerClient>) -> Self {
        Self { producer }
    }

    fn partition_path(&self, partition_id: &str) -> Result<Url> {
        let path = format!("{}/Partitions/{}", self.producer.base_url(), partition_id);
        Url::parse(&path).map_err(|e| azure_core::Error::from(e).into())
    }
}

#[async_trait::async_trait]
impl BufferedSendClient for ProducerSendClient {
    async fn partition_ids(&self) -> Result<Vec<String>> {
        Ok(self.producer.get_eventhub_properties().await?.partition_ids)
    }

    async fn max_message_size(&self, partition_id: &str) -> Result<u64> {
        let path = self.partition_path(partition_id)?;
        self.producer.max_message_size(path).await?.ok_or_else(|| {
            crate::EventHubsError::with_message(
                "No maximum message size available from the sender link.",
            )
        })
    }

    async fn send_envelope(
        &self,
        partition_id: &str,
        envelope: AmqpMessage,
    ) -> Result<AmqpSendOutcome> {
        let path = self.partition_path(partition_id)?;
        self.producer.send_batch_envelope(path, envelope).await
    }
}

#[cfg(test)]
pub(crate) mod mock {
    use super::*;
    use crate::EventHubsError;
    use azure_core_amqp::message::AmqpMessageBody;
    use futures::channel::{mpsc, oneshot};
    use std::{
        collections::{HashMap, VecDeque},
        sync::Mutex,
    };

    /// The outcome that the mock returns for one send.
    #[derive(Clone, Debug)]
    pub(crate) enum SendScript {
        Accepted,
        Modified,
        Released,
        /// The recoverable sender turns a rejected outcome into an error, so a
        /// rejected send reaches the worker as an error.
        Rejected,
        /// The retry policy is exhausted, or the error is not retryable.
        Error(&'static str),
    }

    /// One send that the mock observed.
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub(crate) struct RecordedSend {
        pub(crate) partition_id: String,
        pub(crate) event_count: usize,
    }

    #[derive(Default)]
    struct MockState {
        script: HashMap<String, VecDeque<SendScript>>,
        gates: HashMap<String, VecDeque<oneshot::Receiver<()>>>,
        sends: Vec<RecordedSend>,
    }

    /// A [`BufferedSendClient`] for tests.
    ///
    /// The mock records every send, returns scripted outcomes, and can hold a
    /// send open until the test releases it. Tests use the gates instead of
    /// sleeps, so no test depends on timing to prove a race.
    pub(crate) struct MockSendClient {
        partitions: Vec<String>,
        max_message_size: u64,
        state: Mutex<MockState>,
        started_tx: mpsc::UnboundedSender<String>,
    }

    impl MockSendClient {
        /// Creates a mock over the given partition IDs.
        ///
        /// The returned receiver reports the partition ID of every send as it
        /// starts, before the mock waits on any gate for that send.
        pub(crate) fn new(partitions: &[&str]) -> (Arc<Self>, mpsc::UnboundedReceiver<String>) {
            let (started_tx, started_rx) = mpsc::unbounded();
            let client = Arc::new(Self {
                partitions: partitions.iter().map(|p| p.to_string()).collect(),
                max_message_size: 1024 * 1024,
                state: Mutex::new(MockState::default()),
                started_tx,
            });
            (client, started_rx)
        }

        /// Creates a mock whose link reports the given maximum message size.
        pub(crate) fn with_max_message_size(
            partitions: &[&str],
            max_message_size: u64,
        ) -> (Arc<Self>, mpsc::UnboundedReceiver<String>) {
            let (started_tx, started_rx) = mpsc::unbounded();
            let client = Arc::new(Self {
                partitions: partitions.iter().map(|p| p.to_string()).collect(),
                max_message_size,
                state: Mutex::new(MockState::default()),
                started_tx,
            });
            (client, started_rx)
        }

        /// Queues one scripted outcome for a partition.
        ///
        /// The mock returns `Accepted` when a partition has no queued outcome.
        pub(crate) fn push_outcome(&self, partition_id: &str, outcome: SendScript) {
            self.state
                .lock()
                .unwrap()
                .script
                .entry(partition_id.to_string())
                .or_default()
                .push_back(outcome);
        }

        /// Holds the next un-gated send on a partition until the test drops or
        /// completes the returned sender.
        pub(crate) fn gate(&self, partition_id: &str) -> oneshot::Sender<()> {
            let (tx, rx) = oneshot::channel();
            self.state
                .lock()
                .unwrap()
                .gates
                .entry(partition_id.to_string())
                .or_default()
                .push_back(rx);
            tx
        }

        /// Returns every send that the mock observed, in order.
        pub(crate) fn sends(&self) -> Vec<RecordedSend> {
            self.state.lock().unwrap().sends.clone()
        }

        /// Returns the total number of events across every observed send.
        pub(crate) fn total_events(&self) -> usize {
            self.sends().iter().map(|s| s.event_count).sum()
        }

        fn count_events(envelope: &AmqpMessage) -> usize {
            match &envelope.body {
                AmqpMessageBody::Binary(items) => items.len(),
                _ => 0,
            }
        }
    }

    #[async_trait::async_trait]
    impl BufferedSendClient for MockSendClient {
        async fn partition_ids(&self) -> Result<Vec<String>> {
            Ok(self.partitions.clone())
        }

        async fn max_message_size(&self, _partition_id: &str) -> Result<u64> {
            Ok(self.max_message_size)
        }

        async fn send_envelope(
            &self,
            partition_id: &str,
            envelope: AmqpMessage,
        ) -> Result<AmqpSendOutcome> {
            let event_count = Self::count_events(&envelope);

            // Report the start before waiting on a gate, so a test can observe
            // that a send is in flight while it is still held.
            let _ = self.started_tx.unbounded_send(partition_id.to_string());

            let gate = self
                .state
                .lock()
                .unwrap()
                .gates
                .get_mut(partition_id)
                .and_then(|g| g.pop_front());
            if let Some(gate) = gate {
                // A dropped sender resolves the receiver with an error. Either
                // way the send proceeds once the test releases the gate.
                let _ = gate.await;
            }

            let outcome = {
                let mut state = self.state.lock().unwrap();
                state.sends.push(RecordedSend {
                    partition_id: partition_id.to_string(),
                    event_count,
                });
                state
                    .script
                    .get_mut(partition_id)
                    .and_then(|s| s.pop_front())
                    .unwrap_or(SendScript::Accepted)
            };

            match outcome {
                SendScript::Accepted => Ok(AmqpSendOutcome::Accepted),
                // `SendModification` is not re-exported from `azure_core_amqp`,
                // so build it through `Default` instead of naming the type.
                SendScript::Modified => Ok(AmqpSendOutcome::Modified(Default::default())),
                SendScript::Released => Ok(AmqpSendOutcome::Released),
                SendScript::Rejected => Err(EventHubsError::with_message(
                    "Batch was rejected by the Event Hub.",
                )),
                SendScript::Error(message) => Err(EventHubsError::with_message(message)),
            }
        }
    }
}
