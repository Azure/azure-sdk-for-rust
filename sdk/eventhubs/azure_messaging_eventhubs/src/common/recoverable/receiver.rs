// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::common::retry::{
    recover_with_policy, DeliveryWait, ErrorRecoveryAction, RecoveryClock, RecoveryLogContext,
    RecoveryOperation, RecoveryPolicy, SystemClock,
};
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
    connection_id: String,
    partition_id: String,
    timeout: Option<Duration>,
}

impl RecoverableReceiver {
    pub(super) fn new(
        recoverable_connection: Weak<RecoverableConnection>,
        receiver_options: AmqpReceiverOptions,
        message_source: AmqpSource,
        source_url: Url,
        connection_id: String,
        partition_id: String,
        timeout: Option<Duration>,
    ) -> Self {
        Self {
            source_url,
            recoverable_connection,
            receiver_options,
            message_source,
            connection_id,
            partition_id,
            timeout,
        }
    }

    /// Attaches the receiver if needed, marks `wait`, then waits for one delivery.
    async fn receive_once(
        &self,
        wait: &DeliveryWait<'_, SystemClock>,
    ) -> Result<azure_core_amqp::AmqpDelivery> {
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
        wait.start();
        if let Some(delivery_timeout) = self.timeout {
            select! {
                delivery = receiver.receive_delivery().fuse() => Ok(delivery),
                _ = azure_core::sleep::sleep(delivery_timeout).fuse() => {
                     Err(Self::receive_timeout_error())
                },
            }?
        } else {
            receiver.receive_delivery().await
        }
    }

    /// Runs `attempt` under the receive recovery policy.
    ///
    /// The budget does not count the time after an attempt marks `wait`. Each
    /// call is a new recovery episode. Tests call this directly to control the
    /// attempt, the recovery action, and the clock.
    async fn receive_with_recovery<T, F, Fut, C, K>(
        &self,
        attempt: F,
        recover: RecoveryOperation<C, AmqpError>,
        context: C,
        wait: &DeliveryWait<'_, K>,
    ) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
        C: Clone,
        K: RecoveryClock,
    {
        let retry_options = self
            .recoverable_connection
            .upgrade()
            .ok_or_else(|| AmqpError::with_message("Missing connection"))?
            .retry_options
            .clone();
        recover_with_policy(
            attempt,
            &retry_options,
            Self::should_retry_receive_operation,
            Some(recover),
            Some(context),
            RecoveryPolicy {
                delivery_wait: Some(wait),
                log_context: RecoveryLogContext {
                    connection_id: Some(&self.connection_id),
                    partition_id: Some(&self.partition_id),
                },
                clock: wait.clock(),
            },
        )
        .await
    }

    fn should_retry_receive_operation(e: &AmqpError) -> ErrorRecoveryAction {
        RecoverableConnection::should_retry_receive_error(e)
    }

    /// Builds the error that a receive timeout produces. The cause goes in
    /// unboxed, because `azure_core::Error::new` boxes its argument and a
    /// pre-boxed cause defeats `downcast_ref::<std::io::Error>()`.
    fn receive_timeout_error() -> AmqpError {
        AmqpError::from(azure_core::Error::new(
            AzureErrorKind::Io,
            std::io::Error::from(std::io::ErrorKind::TimedOut),
        ))
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
        let wait = DeliveryWait::new(&SystemClock);
        self.receive_with_recovery(
            || self.receive_once(&wait),
            |connection: Weak<RecoverableConnection>, reason| {
                Box::pin(RecoverableConnection::recover_from_error(
                    connection, reason,
                ))
            },
            self.recoverable_connection.clone(),
            &wait,
        )
        .await
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
    // cspell:ignore sasl batchable

    use super::*;
    use crate::{
        common::retry::{
            test_support::{capture_logs, stop_warning, ManualClock},
            HEALTHY_WAIT,
        },
        consumer::{event_receiver::EventReceiver, StartLocation, StartPosition},
        error::{find_link_stolen, ErrorKind},
        models::ReceivedEventData,
        RetryOptions,
    };
    use azure_core::http::Url;
    use azure_core_amqp::{
        error::{AmqpErrorCondition, AmqpErrorKind},
        message::AmqpSourceFilter,
        AmqpDescribed, AmqpDescribedError, AmqpReceiverOptions, AmqpSource, AmqpTransport,
        ReceiverCreditMode,
    };
    use azure_core_test::credentials::MockCredential;
    use fe2o3_amqp::{
        acceptor::{
            ConnectionAcceptor, LinkAcceptor, LinkEndpoint, SaslAnonymousMechanism, SessionAcceptor,
        },
        types::{
            messaging::{Message, MessageAnnotations},
            primitives::Value,
        },
    };
    use futures::StreamExt;
    use std::{
        future::Future,
        pin::Pin,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc, Mutex,
        },
        time::{Duration as StdDuration, Instant},
    };
    use tokio::{
        net::TcpListener,
        sync::{mpsc, Notify},
        time::timeout,
    };
    use tracing::{Instrument, Level};

    fn stolen() -> AmqpError {
        AmqpError::from(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
            AmqpErrorCondition::LinkStolen,
            None,
            Default::default(),
        )))
    }

    // A caller that branches on a receive timeout downcasts the cause to
    // `std::io::Error`. A cause that goes in already boxed is stored as
    // `Box<std::io::Error>`, which no downcast to `std::io::Error` finds.
    #[test]
    fn receive_timeout_cause_downcasts_to_io_error() {
        let error = azure_core::Error::from(RecoverableReceiver::receive_timeout_error());
        assert_eq!(error.kind(), &AzureErrorKind::Io);
        let cause = error
            .downcast_ref::<std::io::Error>()
            .expect("the receive timeout cause must downcast to std::io::Error");
        assert_eq!(cause.kind(), std::io::ErrorKind::TimedOut);
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

    #[derive(Clone, Copy)]
    enum Outcome {
        Deliver,
        LinkClosed,
        IdleTimeout,
        SessionDropped,
        ServerBusy,
        Stolen,
        AttachFailed,
    }

    /// One scripted attempt: attach time, then delivery wait, then outcome.
    type Step = (StdDuration, StdDuration, Outcome);

    const LONG_RECEIVE_WAIT: StdDuration = StdDuration::new(335, 991_427_228);

    fn secs(secs: u64) -> StdDuration {
        StdDuration::from_secs(secs)
    }

    fn link_closed() -> AmqpError {
        AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(
            std::io::Error::other("closed"),
        )))
    }

    #[derive(Clone)]
    struct FakeRecovery {
        clock: Arc<ManualClock>,
        cost: StdDuration,
        started: Arc<Mutex<Vec<Instant>>>,
        actions: Arc<Mutex<Vec<ErrorRecoveryAction>>>,
    }

    fn fake_recover(
        recovery: FakeRecovery,
        action: ErrorRecoveryAction,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        recovery.started.lock().unwrap().push(recovery.clock.now());
        recovery.actions.lock().unwrap().push(action);
        recovery.clock.advance(recovery.cost);
        Box::pin(async { Ok(()) })
    }

    struct Harness {
        // Keeps the receiver's weak connection reference alive.
        _connection: Arc<RecoverableConnection>,
        receiver: RecoverableReceiver,
        recovery: FakeRecovery,
    }

    impl Harness {
        fn new(retry_options: RetryOptions, recovery_cost: StdDuration) -> Self {
            let connection = RecoverableConnection::new(
                Url::parse("amqps://example.servicebus.windows.net").unwrap(),
                Some(String::from("conn-1")),
                None,
                AmqpTransport::default(),
                None,
                Arc::new(MockCredential),
                retry_options,
                None,
            );
            let receiver = RecoverableReceiver::new(
                Arc::downgrade(&connection),
                AmqpReceiverOptions::default(),
                AmqpSource::default(),
                Url::parse("amqps://example.servicebus.windows.net/eh/Partitions/7").unwrap(),
                connection.get_connection_id().to_string(),
                String::from("7"),
                None,
            );
            Self {
                _connection: connection,
                receiver,
                recovery: FakeRecovery {
                    clock: ManualClock::new(),
                    cost: recovery_cost,
                    started: Arc::default(),
                    actions: Arc::default(),
                },
            }
        }

        fn recoveries(&self) -> usize {
            self.recovery.started.lock().unwrap().len()
        }

        fn recovery_gaps(&self) -> Vec<StdDuration> {
            let started = self.recovery.started.lock().unwrap();
            started.windows(2).map(|w| w[1] - w[0]).collect()
        }

        /// `script` maps each attempt index to its [`Step`]. A delivery
        /// returns its attempt index.
        async fn receive_with_logs(
            &self,
            level: Level,
            script: impl Fn(usize) -> Step,
        ) -> (Result<usize>, usize, String) {
            self.receive_with_budget(level, script, true).await
        }

        async fn receive_with_budget(
            &self,
            level: Level,
            script: impl Fn(usize) -> Step,
            exclude_delivery_wait: bool,
        ) -> (Result<usize>, usize, String) {
            let attempts = AtomicUsize::new(0);
            let clock = &self.recovery.clock;
            let wait = DeliveryWait::new(&**clock);
            let attempt = || {
                let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                let (attach, delivery_wait, outcome) = script(attempt);
                clock.advance(attach);
                if !matches!(outcome, Outcome::AttachFailed) {
                    wait.start();
                    clock.advance(delivery_wait);
                }
                async move {
                    match outcome {
                        Outcome::Deliver => Ok(attempt),
                        Outcome::LinkClosed => Err(link_closed()),
                        Outcome::IdleTimeout => Err(AmqpErrorKind::IdleTimeoutElapsed(Box::new(
                            fe2o3_amqp::transport::Error::IdleTimeoutElapsed,
                        ))
                        .into()),
                        Outcome::SessionDropped => Err(AmqpErrorKind::LinkStateError(Box::new(
                            fe2o3_amqp::link::LinkStateError::SessionStopped(
                                fe2o3_amqp::link::SessionStopReason::Ended,
                            ),
                        ))
                        .into()),
                        Outcome::ServerBusy => {
                            Err(AmqpErrorKind::AmqpDescribedError(AmqpDescribedError::new(
                                AmqpErrorCondition::ServerBusyError,
                                None,
                                Default::default(),
                            ))
                            .into())
                        }
                        Outcome::Stolen => Err(stolen()),
                        Outcome::AttachFailed => {
                            Err(RecoverableReceiver::ensure_receiver_error(link_closed()))
                        }
                    }
                }
            };
            let (result, logs) = capture_logs(level, async {
                if exclude_delivery_wait {
                    self.receiver
                        .receive_with_recovery(attempt, fake_recover, self.recovery.clone(), &wait)
                        .await
                } else {
                    // Control for the old receive policy: charge the entire attempt.
                    recover_with_policy(
                        attempt,
                        &self._connection.retry_options,
                        RecoverableReceiver::should_retry_receive_operation,
                        Some(fake_recover),
                        Some(self.recovery.clone()),
                        RecoveryPolicy {
                            delivery_wait: None,
                            log_context: RecoveryLogContext {
                                connection_id: Some(&self.receiver.connection_id),
                                partition_id: Some(&self.receiver.partition_id),
                            },
                            clock: &**clock,
                        },
                    )
                    .await
                }
            })
            .await;
            (result, attempts.load(Ordering::SeqCst), logs)
        }

        async fn receive(&self, script: impl Fn(usize) -> Step) -> (Result<usize>, usize, String) {
            self.receive_with_logs(Level::WARN, script).await
        }
    }

    fn assert_fields(warning: &str, fields: &[&str]) {
        for field in fields {
            assert!(warning.contains(field), "missing `{field}` in: {warning}");
        }
    }

    // The reported failure: an idle partition waited longer than
    // `max_total_elapsed`, then the session dropped. The wait used up the
    // budget, so the receive returned the error without a recovery.
    #[tokio::test]
    async fn long_wait_then_recoverable_error_recovers() {
        let harness = Harness::new(RetryOptions::default(), secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (secs(0), secs(120), Outcome::LinkClosed),
                _ => (secs(0), secs(0), Outcome::Deliver),
            })
            .await;
        assert_eq!(result.unwrap(), 1);
        assert_eq!((attempts, harness.recoveries()), (2, 1));
        assert!(!logs.contains("Operation recovery stopped"), "{logs}");
        assert_fields(
            &logs,
            &[
                "Error requires recovery",
                "connection_id=conn-1",
                "partition_id=7",
                "receive_wait_elapsed=120s",
            ],
        );
    }

    #[tokio::test]
    async fn long_wait_then_delivery_succeeds() {
        let harness = Harness::new(RetryOptions::default(), StdDuration::ZERO);
        let (result, attempts, logs) = harness
            .receive(|_| (secs(0), secs(600), Outcome::Deliver))
            .await;
        assert_eq!(result.unwrap(), 0);
        assert_eq!((attempts, harness.recoveries()), (1, 0));
        assert!(logs.is_empty(), "{logs}");
    }

    #[tokio::test]
    async fn observed_long_wait_recovers_after_idle_timeout_or_session_drop() {
        for (outcome, action) in [
            (
                Outcome::IdleTimeout,
                ErrorRecoveryAction::ReconnectConnection,
            ),
            (Outcome::SessionDropped, ErrorRecoveryAction::ReconnectLink),
        ] {
            let script = |attempt| match attempt {
                0 => (secs(0), LONG_RECEIVE_WAIT, outcome),
                _ => (secs(0), secs(0), Outcome::Deliver),
            };
            let baseline = Harness::new(RetryOptions::default(), secs(1));
            let (result, attempts, logs) = baseline
                .receive_with_budget(Level::WARN, script, false)
                .await;
            assert!(result.is_err());
            assert_eq!((attempts, baseline.recoveries()), (1, 0));
            assert_fields(
                stop_warning(&logs),
                &[
                    "stop_reason=elapsed_budget_exhausted",
                    "retries_attempted=0",
                    "max_retries=8",
                    "recovery_elapsed=335.991427228s",
                    "max_total_elapsed=60s",
                ],
            );

            let fixed = Harness::new(RetryOptions::default(), secs(1));
            assert_eq!(
                fixed._connection.retry_options.max_total_elapsed,
                Duration::seconds(60)
            );
            let started = fixed.recovery.clock.now();
            let (result, attempts, logs) = fixed.receive(script).await;
            assert_eq!(result.unwrap(), 1);
            assert_eq!((attempts, fixed.recoveries()), (2, 1));
            assert_eq!(*fixed.recovery.actions.lock().unwrap(), vec![action]);
            assert_eq!(
                fixed.recovery.clock.now() - started,
                LONG_RECEIVE_WAIT + secs(1)
            );
            assert!(!logs.contains("Operation recovery stopped"), "{logs}");
            assert_fields(
                &logs,
                &[
                    "Error requires recovery",
                    "retries_attempted=0",
                    "receive_wait_elapsed=335.991427228s",
                    "recovery_elapsed=0ns",
                ],
            );
        }
    }

    #[tokio::test(start_paused = true)]
    async fn healthy_pending_receive_outlives_producer_deadline() {
        let harness = Harness::new(RetryOptions::default(), secs(1));
        let clock = &*harness.recovery.clock;
        let wait = DeliveryWait::new(clock);
        let delivered = async_lock::OnceCell::new();
        let receive = harness.receiver.receive_with_recovery(
            || async {
                wait.start();
                Ok(*delivered.wait().await)
            },
            fake_recover,
            harness.recovery.clone(),
            &wait,
        );
        futures::pin_mut!(receive);
        assert!(futures::poll!(&mut receive).is_pending());
        clock.advance(LONG_RECEIVE_WAIT);
        tokio::time::advance(LONG_RECEIVE_WAIT).await;
        assert!(
            futures::poll!(&mut receive).is_pending(),
            "a healthy receive must remain pending past the producer deadline"
        );
        delivered.get_or_init(|| async { 42 }).await;
        assert_eq!(receive.await.unwrap(), 42);
        assert_eq!(harness.recoveries(), 0);
    }

    #[tokio::test]
    async fn long_wait_leaves_preparation_recovery_and_backoff_chargeable() {
        let options = RetryOptions {
            initial_delay: Duration::seconds(4),
            max_delay: Duration::seconds(4),
            ..Default::default()
        };
        let harness = Harness::new(options, secs(5));
        let started = harness.recovery.clock.now();
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (secs(3), LONG_RECEIVE_WAIT, Outcome::IdleTimeout),
                1 => (secs(7), secs(0), Outcome::ServerBusy),
                2..=5 => (secs(7), secs(0), Outcome::AttachFailed),
                _ => panic!("active recovery time must exhaust the budget before another attempt"),
            })
            .await;
        let error = result.unwrap_err();
        assert_eq!(
            RecoverableReceiver::should_retry_receive_operation(&error),
            ErrorRecoveryAction::ReconnectLink
        );
        assert!(matches!(error.kind(), AmqpErrorKind::AzureCore(_)));
        assert_eq!((attempts, harness.recoveries()), (6, 4));
        // 38 s preparation + 20 s recovery + 4 s capped backoff. The long wait is excluded.
        assert_eq!(
            harness.recovery.clock.now() - started,
            LONG_RECEIVE_WAIT + secs(62)
        );
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=elapsed_budget_exhausted",
                "retries_attempted=5",
                "max_retries=8",
                "receive_wait_elapsed=335.991427228s",
                "recovery_elapsed=62s",
                "max_total_elapsed=60s",
            ],
        );
    }

    #[tokio::test]
    async fn long_wait_preserves_retry_limits_and_terminal_errors() {
        for max_retries in [0, 2] {
            let harness = Harness::new(
                RetryOptions {
                    max_retries,
                    ..Default::default()
                },
                secs(1),
            );
            let (result, attempts, logs) = harness
                .receive(|attempt| {
                    assert!(
                        attempt <= usize::try_from(max_retries).unwrap(),
                        "receive exceeded its retry limit"
                    );
                    (
                        secs(0),
                        if attempt == 0 {
                            LONG_RECEIVE_WAIT
                        } else {
                            secs(0)
                        },
                        Outcome::SessionDropped,
                    )
                })
                .await;
            let error = result.unwrap_err();
            let AmqpErrorKind::LinkStateError(source) = error.kind() else {
                panic!("the terminal receive must retain its session-dropped error: {error:?}");
            };
            assert!(matches!(
                source.downcast_ref::<fe2o3_amqp::link::LinkStateError>(),
                Some(fe2o3_amqp::link::LinkStateError::SessionStopped(
                    fe2o3_amqp::link::SessionStopReason::Ended,
                ))
            ));
            assert_eq!(attempts, usize::try_from(max_retries + 1).unwrap());
            assert_eq!(harness.recoveries(), usize::try_from(max_retries).unwrap());
            assert_fields(
                stop_warning(&logs),
                &[
                    "stop_reason=retries_exhausted",
                    "receive_wait_elapsed=335.991427228s",
                ],
            );
        }

        let harness = Harness::new(RetryOptions::default(), secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| {
                assert_eq!(attempt, 0, "a terminal receive error must not retry");
                (secs(0), LONG_RECEIVE_WAIT, Outcome::Stolen)
            })
            .await;
        assert!(find_link_stolen(&result.unwrap_err()).is_some());
        assert_eq!((attempts, harness.recoveries()), (1, 0));
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=non_recoverable",
                "retries_attempted=0",
                "receive_wait_elapsed=335.991427228s",
                "recovery_elapsed=0ns",
            ],
        );
    }

    // The stream enters a DEBUG span with the receiver identifiers. At WARN
    // that span is off, so the warning must carry the identifiers itself.
    #[tokio::test]
    async fn repeated_failures_exhaust_elapsed_budget() {
        let harness = Harness::new(RetryOptions::default(), secs(25));
        let span = tracing::debug_span!("stream_events", connection_id = "span-conn");
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (secs(0), secs(120), Outcome::LinkClosed),
                _ => (secs(0), secs(0), Outcome::LinkClosed),
            })
            .instrument(span)
            .await;
        assert!(result.is_err());
        // Failures at 0 s, 25 s, and 50 s recover; the failure at 75 s stops.
        assert_eq!((attempts, harness.recoveries()), (4, 3));
        assert!(!logs.contains("span-conn"), "{logs}");
        assert_fields(
            stop_warning(&logs),
            &[
                "connection_id=conn-1",
                "partition_id=7",
                "stop_reason=elapsed_budget_exhausted",
                "retries_attempted=3",
                "max_retries=8",
                "receive_wait_elapsed=120s",
                "recovery_elapsed=75s",
                "max_total_elapsed=60s",
                "err=",
            ],
        );
    }

    #[tokio::test]
    async fn retry_count_limit_stops_recovery() {
        let options = RetryOptions {
            max_retries: 2,
            ..Default::default()
        };
        let harness = Harness::new(options, secs(1));
        let (result, attempts, logs) = harness
            .receive(|_| (secs(0), secs(0), Outcome::LinkClosed))
            .await;
        assert!(result.is_err());
        assert_eq!((attempts, harness.recoveries()), (3, 2));
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=retries_exhausted",
                "retries_attempted=2",
                "max_retries=2",
                "recovery_elapsed=2s",
            ],
        );
    }

    #[tokio::test]
    async fn zero_max_retries_reports_zero_attempts() {
        let options = RetryOptions {
            max_retries: 0,
            ..Default::default()
        };
        let harness = Harness::new(options, StdDuration::ZERO);
        let (result, attempts, logs) = harness
            .receive(|_| (secs(0), secs(120), Outcome::LinkClosed))
            .await;
        assert!(result.is_err());
        assert_eq!((attempts, harness.recoveries()), (1, 0));
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=retries_exhausted",
                "retries_attempted=0",
                "max_retries=0",
                "receive_wait_elapsed=120s",
                "recovery_elapsed=0ns",
            ],
        );
    }

    #[tokio::test]
    async fn non_recoverable_error_returns_without_recovery() {
        let harness = Harness::new(RetryOptions::default(), StdDuration::ZERO);
        let (result, attempts, logs) = harness
            .receive(|_| (secs(0), secs(5), Outcome::Stolen))
            .await;
        let error = result.unwrap_err();
        assert!(find_link_stolen(&error).is_some(), "{error}");
        assert_eq!((attempts, harness.recoveries()), (1, 0));
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=non_recoverable",
                "retries_attempted=0",
                "partition_id=7",
            ],
        );
    }

    // Each receive is one recovery episode. The second receive would stop at
    // once if it inherited the 50 s that the first one charged.
    #[tokio::test]
    async fn later_receive_gets_a_fresh_budget() {
        let harness = Harness::new(RetryOptions::default(), secs(50));
        let script = |attempt| match attempt {
            0 => (secs(0), secs(0), Outcome::LinkClosed),
            _ => (secs(0), secs(0), Outcome::Deliver),
        };
        assert_eq!(harness.receive(script).await.0.unwrap(), 1);
        harness.recovery.clock.advance(secs(30));
        let (result, _, logs) = harness.receive(script).await;
        assert_eq!(result.unwrap(), 1);
        assert_eq!(harness.recoveries(), 2);
        assert!(!logs.contains("Operation recovery stopped"), "{logs}");
    }

    // Before the fix, two 9 s retried waits used up a 5 s budget.
    #[tokio::test]
    async fn retried_waits_do_not_use_budget() {
        let options = RetryOptions {
            max_total_elapsed: azure_core::time::Duration::seconds(5),
            ..Default::default()
        };
        let harness = Harness::new(options, secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0..=3 => (secs(0), HEALTHY_WAIT - secs(1), Outcome::LinkClosed),
                _ => (secs(0), secs(0), Outcome::Deliver),
            })
            .await;
        assert_eq!(result.unwrap(), 4);
        assert_eq!((attempts, harness.recoveries()), (5, 4));
        assert!(!logs.contains("Operation recovery stopped"), "{logs}");
    }

    #[tokio::test]
    async fn reattach_and_recovery_time_exhaust_budget() {
        let harness = Harness::new(RetryOptions::default(), secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (secs(20), secs(0), Outcome::LinkClosed),
                _ => (secs(20), secs(0), Outcome::AttachFailed),
            })
            .await;
        assert!(result.is_err());
        // Charged time at each failure: 20 s, 41 s, and 62 s.
        assert_eq!((attempts, harness.recoveries()), (3, 2));
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=elapsed_budget_exhausted",
                "retries_attempted=2",
                "receive_wait_elapsed=0ns",
                "recovery_elapsed=62s",
            ],
        );
    }

    // Like an empty receive in .NET, a healthy wait resets the retry count.
    #[tokio::test]
    async fn healthy_wait_resets_retry_count() {
        let options = RetryOptions {
            max_retries: 2,
            ..Default::default()
        };
        let harness = Harness::new(options, secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0..=5 => (secs(1), HEALTHY_WAIT, Outcome::LinkClosed),
                _ => (secs(1), secs(0), Outcome::Deliver),
            })
            .await;
        assert_eq!(result.unwrap(), 6);
        assert_eq!((attempts, harness.recoveries()), (7, 6));
        assert!(!logs.contains("Operation recovery stopped"), "{logs}");
    }

    #[tokio::test]
    async fn failures_before_healthy_wait_exhaust_retries() {
        let options = RetryOptions {
            max_retries: 3,
            ..Default::default()
        };
        let harness = Harness::new(options, secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (secs(0), secs(120), Outcome::LinkClosed),
                1 => (secs(1), secs(0), Outcome::AttachFailed),
                20.. => (secs(0), secs(0), Outcome::Deliver),
                _ => (
                    secs(1),
                    HEALTHY_WAIT - StdDuration::from_millis(1),
                    Outcome::LinkClosed,
                ),
            })
            .await;
        assert!(result.is_err());
        assert_eq!((attempts, harness.recoveries()), (4, 3));
        assert_fields(
            stop_warning(&logs),
            &["stop_reason=retries_exhausted", "retries_attempted=3"],
        );
    }

    // Link reconnects have no backoff, so without the backoff step on a new
    // episode this link would reconnect once per `HEALTHY_WAIT`.
    #[tokio::test]
    async fn new_episode_waits_a_backoff_step_before_reconnect() {
        let options = RetryOptions {
            initial_delay: azure_core::time::Duration::seconds(20),
            ..Default::default()
        };
        let harness = Harness::new(options, StdDuration::ZERO);
        let (result, _, _) = harness
            .receive(|attempt| match attempt {
                0..=4 => (secs(0), HEALTHY_WAIT, Outcome::LinkClosed),
                _ => (secs(0), secs(0), Outcome::Deliver),
            })
            .await;
        assert_eq!(result.unwrap(), 5);
        let gaps = harness.recovery_gaps();
        assert_eq!(gaps.len(), 4);
        for gap in gaps {
            assert!(gap >= HEALTHY_WAIT + secs(20), "{gap:?}");
        }
    }

    #[tokio::test]
    async fn logs_report_charged_time_and_new_episode() {
        let options = RetryOptions {
            initial_delay: azure_core::time::Duration::seconds(1),
            max_delay: azure_core::time::Duration::seconds(1),
            max_retries: 1,
            ..Default::default()
        };
        let harness = Harness::new(options, secs(2));
        let (result, attempts, logs) = harness
            .receive_with_logs(Level::INFO, |attempt| match attempt {
                0 => (secs(3), secs(100), Outcome::LinkClosed),
                1 => (secs(3), HEALTHY_WAIT, Outcome::LinkClosed),
                _ => (secs(3), secs(4), Outcome::LinkClosed),
            })
            .await;
        assert!(result.is_err());
        assert_eq!(attempts, 3);
        let reset = logs
            .lines()
            .find(|line| line.contains("starting a new recovery episode"))
            .expect("no new episode event");
        assert_fields(
            reset,
            &[
                "INFO",
                "connection_id=conn-1",
                "partition_id=7",
                "retries_attempted=1",
                "healthy_wait=10s",
            ],
        );
        // The new episode charges 1 s backoff, 2 s recovery, and 3 s attach.
        assert_fields(
            stop_warning(&logs),
            &[
                "stop_reason=retries_exhausted",
                "retries_attempted=1",
                "max_retries=1",
                "receive_wait_elapsed=14s",
                "recovery_elapsed=6s",
                "max_total_elapsed=60s",
                "connection_id=conn-1",
                "partition_id=7",
            ],
        );
    }

    #[derive(Clone, Copy)]
    enum LocalRecoveryKind {
        Link,
        Session,
        Connection,
    }

    #[derive(Clone, Copy)]
    enum LocalAnnotations {
        OffsetAndSequence,
        SequenceOnly,
        Missing,
    }

    #[derive(Clone, Copy)]
    struct LocalPeerConfig {
        recovery: LocalRecoveryKind,
        annotations: LocalAnnotations,
        recreate_stream: bool,
        latest: bool,
        missing_after_known_offset: bool,
    }

    const LOCAL_PEER_TIMEOUT: StdDuration = StdDuration::from_secs(5);

    fn local_error(kind: LocalRecoveryKind) -> AmqpError {
        let error = std::io::Error::other("forced local receiver recovery");
        match kind {
            LocalRecoveryKind::Link => {
                AmqpError::from(AmqpErrorKind::LinkClosedByRemote(Box::new(error)))
            }
            LocalRecoveryKind::Session => {
                AmqpError::from(AmqpErrorKind::SessionClosedByRemote(Box::new(error)))
            }
            LocalRecoveryKind::Connection => {
                AmqpError::from(AmqpErrorKind::ConnectionClosedByRemote(Box::new(error)))
            }
        }
    }

    fn selector_from_sender(sender: &fe2o3_amqp::Sender) -> String {
        let source = sender
            .source()
            .as_ref()
            .expect("receiver attach must contain a source");
        let filters = source
            .filter
            .as_ref()
            .expect("receiver attach must contain a selector filter");
        let value = filters
            .values()
            .next()
            .expect("receiver attach must contain one selector filter");
        let Value::Described(described) = value else {
            panic!("selector filter must be described, got {value:?}");
        };
        let Value::String(selector) = &described.value else {
            panic!(
                "selector filter value must be a string, got {:?}",
                described.value
            );
        };
        selector.clone()
    }

    fn local_message(
        marker: u8,
        annotations: LocalAnnotations,
    ) -> Message<fe2o3_amqp::types::messaging::Data> {
        let mut builder = Message::builder();
        match annotations {
            LocalAnnotations::OffsetAndSequence => {
                builder = builder.message_annotations(
                    MessageAnnotations::builder()
                        .insert("x-opt-offset", marker.to_string())
                        .insert("x-opt-sequence-number", i64::from(marker))
                        .build(),
                );
            }
            LocalAnnotations::SequenceOnly => {
                builder = builder.message_annotations(
                    MessageAnnotations::builder()
                        .insert("x-opt-sequence-number", i64::from(marker))
                        .build(),
                );
            }
            LocalAnnotations::Missing => {}
        }
        builder.data(marker.to_string().into_bytes()).build()
    }

    async fn send_local_marker(
        sender: &mut fe2o3_amqp::Sender,
        marker: u8,
        annotations: LocalAnnotations,
    ) {
        timeout(
            LOCAL_PEER_TIMEOUT,
            sender.send(local_message(marker, annotations)),
        )
        .await
        .expect("local peer send must complete")
        .expect("local peer must send the scripted marker");
    }

    async fn send_local_marker_batchable(
        sender: &mut fe2o3_amqp::Sender,
        marker: u8,
        annotations: LocalAnnotations,
    ) {
        timeout(
            LOCAL_PEER_TIMEOUT,
            sender.send_batchable(local_message(marker, annotations)),
        )
        .await
        .expect("local peer batchable send must complete")
        .expect("local peer must send the scripted marker");
    }

    fn local_selector(start: &StartPosition) -> String {
        StartPosition::start_expression(&Some(start.clone()))
    }

    fn local_source(source_url: &Url, start: &StartPosition) -> AmqpSource {
        AmqpSource::builder()
            .with_address(source_url.to_string())
            .add_to_filter(
                AmqpSourceFilter::selector_filter().description().into(),
                Box::new(AmqpDescribed::new(
                    AmqpSourceFilter::selector_filter().code(),
                    local_selector(start),
                )),
            )
            .build()
    }

    fn local_resume_selector(sequence_only: bool) -> String {
        local_selector(&StartPosition {
            location: if sequence_only {
                StartLocation::SequenceNumber(2)
            } else {
                StartLocation::Offset("2".to_string())
            },
            inclusive: false,
        })
    }

    fn marker(event: &ReceivedEventData) -> u8 {
        event
            .event_data()
            .body()
            .expect("local event must have a marker body")
            .first()
            .copied()
            .and_then(|byte| char::from(byte).to_digit(10))
            .and_then(|digit| u8::try_from(digit).ok())
            .expect("local marker must be one decimal digit")
    }

    async fn next_local_event(
        stream: &mut (impl futures::Stream<Item = crate::error::Result<ReceivedEventData>> + Unpin),
    ) -> ReceivedEventData {
        timeout(LOCAL_PEER_TIMEOUT, stream.next())
            .await
            .expect("local receive must complete")
            .expect("local stream must yield an event")
            .expect("local stream must not yield an error")
    }

    async fn local_peer(
        listener: TcpListener,
        config: LocalPeerConfig,
        selectors: mpsc::UnboundedSender<String>,
        marker_two_consumed: Arc<Notify>,
        marker_three_consumed: Arc<Notify>,
    ) {
        let mut attach_index = 0usize;

        'connections: loop {
            let (stream, _) = timeout(LOCAL_PEER_TIMEOUT, listener.accept())
                .await
                .expect("local peer must accept a client connection")
                .expect("local peer listener must remain usable");
            let mut connection = ConnectionAcceptor::builder()
                .container_id(format!("receiver-peer-{attach_index}"))
                .sasl_acceptor(SaslAnonymousMechanism {})
                .build()
                .accept(stream)
                .await
                .expect("local peer must complete the AMQP connection");

            loop {
                let mut session = timeout(
                    LOCAL_PEER_TIMEOUT,
                    SessionAcceptor::new().accept(&mut connection),
                )
                .await
                .expect("local peer must accept a client session")
                .expect("local peer session must remain usable");
                let endpoint =
                    timeout(LOCAL_PEER_TIMEOUT, LinkAcceptor::new().accept(&mut session))
                        .await
                        .expect("local peer must accept a receiver link")
                        .expect("local peer receiver link must remain usable");
                let LinkEndpoint::Sender(mut sender) = endpoint else {
                    panic!("expected the client receiver link");
                };
                let selector = selector_from_sender(&sender);
                let current_attach = attach_index;
                attach_index += 1;
                selectors
                    .send(selector.clone())
                    .expect("receiver test must observe every attach");

                if current_attach == 0 {
                    send_local_marker_batchable(&mut sender, 1, config.annotations).await;
                    send_local_marker_batchable(&mut sender, 2, config.annotations).await;
                    timeout(LOCAL_PEER_TIMEOUT, marker_two_consumed.notified())
                        .await
                        .expect("local peer must observe marker 2");
                    if matches!(config.recovery, LocalRecoveryKind::Connection) {
                        break;
                    }
                    continue;
                }

                if config.missing_after_known_offset && current_attach == 1 {
                    send_local_marker(&mut sender, 3, LocalAnnotations::Missing).await;
                    timeout(LOCAL_PEER_TIMEOUT, marker_three_consumed.notified())
                        .await
                        .expect("local peer must observe marker 3");
                    continue;
                }

                let expected = local_resume_selector(matches!(
                    config.annotations,
                    LocalAnnotations::SequenceOnly
                ));
                let markers = if selector == expected {
                    match current_attach {
                        1 => vec![3, 4, 5],
                        2 if config.missing_after_known_offset => vec![3, 4, 5],
                        2 => vec![4, 5],
                        _ => panic!("unexpected extra local receiver attach"),
                    }
                } else if config.latest {
                    // The initial Latest selector only admits events arriving after
                    // the attach. Markers 3 through 5 already existed by recovery.
                    vec![6]
                } else {
                    vec![1, 2, 3, 4, 5]
                };
                for marker in markers {
                    send_local_marker(&mut sender, marker, config.annotations).await;
                }
                break 'connections;
            }
        }
    }

    async fn run_local_recovery(config: LocalPeerConfig, start: StartPosition) {
        timeout(StdDuration::from_secs(15), async move {
            let listener = TcpListener::bind("127.0.0.1:0")
                .await
                .expect("local peer must bind");
            let address = listener
                .local_addr()
                .expect("local peer must expose its address");
            let source_url = Url::parse(&format!("amqp://{address}/hub/Partitions/0"))
                .expect("local source URL must parse");
            let connection = RecoverableConnection::new(
                Url::parse(&format!("amqp://{address}/hub")).unwrap(),
                Some("receiver-recovery-local".to_string()),
                None,
                AmqpTransport::Tcp,
                None,
                Arc::new(MockCredential),
                RetryOptions {
                    initial_delay: azure_core::time::Duration::milliseconds(1),
                    max_delay: azure_core::time::Duration::milliseconds(10),
                    max_total_elapsed: azure_core::time::Duration::seconds(10),
                    ..Default::default()
                },
                None,
            );
            connection
                .authorizer
                .disable_authorization()
                .expect("local receiver test must disable authorization");
            connection
                .authorizer
                .set_token_refresh_bias_for_test(azure_core::time::Duration::seconds(1))
                .expect("local receiver test must configure token refresh bias");

            let receiver = EventReceiver::new(
                connection.clone(),
                AmqpReceiverOptions {
                    name: Some("receiver-recovery-local".to_string()),
                    credit_mode: Some(ReceiverCreditMode::Auto(300)),
                    auto_accept: true,
                    ..Default::default()
                },
                local_source(&source_url, &start),
                source_url,
                "0".to_string(),
                None,
            );

            let (selectors, mut observed_selectors) = mpsc::unbounded_channel();
            let marker_two_consumed = Arc::new(Notify::new());
            let marker_three_consumed = Arc::new(Notify::new());
            let peer = tokio::spawn(local_peer(
                listener,
                config,
                selectors,
                marker_two_consumed.clone(),
                marker_three_consumed.clone(),
            ));
            let mut stream = Box::pin(receiver.stream_events());

            let mut markers = vec![marker(&next_local_event(&mut stream).await)];
            markers.push(marker(&next_local_event(&mut stream).await));
            assert_eq!(markers, [1, 2]);
            let initial_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                .await
                .expect("local peer must report the initial selector")
                .expect("local peer selector channel must remain open");
            let expected_initial = local_selector(&start);
            assert_eq!(initial_selector, expected_initial);
            marker_two_consumed.notify_one();

            if config.missing_after_known_offset {
                connection
                    .force_error(local_error(config.recovery))
                    .expect("first local recovery error must be recorded");
                let third = marker(&next_local_event(&mut stream).await);
                let second_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                    .await
                    .expect("local peer must report the first recovery selector")
                    .expect("local peer selector channel must remain open");
                assert_eq!(second_selector, local_resume_selector(false));
                assert_eq!(third, 3);
                markers.push(third);
                marker_three_consumed.notify_one();
                connection
                    .force_error(local_error(config.recovery))
                    .expect("second local recovery error must be recorded");
                let fourth_resume = marker(&next_local_event(&mut stream).await);
                let third_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                    .await
                    .expect("local peer must report the second recovery selector")
                    .expect("local peer selector channel must remain open");
                assert_eq!(third_selector, local_resume_selector(false));
                markers.push(fourth_resume);
                markers.push(marker(&next_local_event(&mut stream).await));
                markers.push(marker(&next_local_event(&mut stream).await));
            } else {
                if config.recreate_stream {
                    drop(stream);
                    connection
                        .force_error(local_error(config.recovery))
                        .expect("local recovery error must be recorded");
                    let mut recreated = Box::pin(receiver.stream_events());
                    let first_resume = marker(&next_local_event(&mut recreated).await);
                    let second_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                        .await
                        .expect("local peer must report the recovery selector")
                        .expect("local peer selector channel must remain open");
                    assert_eq!(
                        second_selector,
                        local_resume_selector(matches!(
                            config.annotations,
                            LocalAnnotations::SequenceOnly
                        ))
                    );
                    markers.push(first_resume);
                    for _ in 0..3 {
                        if markers.len() == 5 {
                            break;
                        }
                        markers.push(marker(&next_local_event(&mut recreated).await));
                    }
                } else {
                    connection
                        .force_error(local_error(config.recovery))
                        .expect("local recovery error must be recorded");
                    let first_resume = marker(&next_local_event(&mut stream).await);
                    let second_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                        .await
                        .expect("local peer must report the recovery selector")
                        .expect("local peer selector channel must remain open");
                    assert_eq!(
                        second_selector,
                        local_resume_selector(matches!(
                            config.annotations,
                            LocalAnnotations::SequenceOnly
                        ))
                    );
                    markers.push(first_resume);
                    for _ in 0..3 {
                        if markers.len() == 5 {
                            break;
                        }
                        markers.push(marker(&next_local_event(&mut stream).await));
                    }
                }
            }
            if config.missing_after_known_offset {
                assert_eq!(markers, [1, 2, 3, 3, 4, 5]);
            } else {
                assert_eq!(markers, [1, 2, 3, 4, 5]);
            }
            timeout(LOCAL_PEER_TIMEOUT, peer)
                .await
                .expect("local peer must finish")
                .expect("local peer must not panic");
            connection
                .close_connection()
                .await
                .expect("local receiver connection must close cleanly");
        })
        .await
        .expect("local receiver recovery must finish within its bound");
    }

    #[tokio::test]
    async fn receiver_recovery_local_offset_link_continuous() {
        run_local_recovery(
            LocalPeerConfig {
                recovery: LocalRecoveryKind::Link,
                annotations: LocalAnnotations::OffsetAndSequence,
                recreate_stream: false,
                latest: false,
                missing_after_known_offset: false,
            },
            StartPosition {
                location: StartLocation::Offset("0".to_string()),
                inclusive: true,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn receiver_recovery_local_offset_session_recreated() {
        run_local_recovery(
            LocalPeerConfig {
                recovery: LocalRecoveryKind::Session,
                annotations: LocalAnnotations::OffsetAndSequence,
                recreate_stream: true,
                latest: false,
                missing_after_known_offset: false,
            },
            StartPosition {
                location: StartLocation::Offset("0".to_string()),
                inclusive: true,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn receiver_recovery_local_offset_connection_continuous() {
        run_local_recovery(
            LocalPeerConfig {
                recovery: LocalRecoveryKind::Connection,
                annotations: LocalAnnotations::OffsetAndSequence,
                recreate_stream: false,
                latest: false,
                missing_after_known_offset: false,
            },
            StartPosition {
                location: StartLocation::Offset("0".to_string()),
                inclusive: true,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn receiver_recovery_local_sequence_fallback() {
        run_local_recovery(
            LocalPeerConfig {
                recovery: LocalRecoveryKind::Connection,
                annotations: LocalAnnotations::SequenceOnly,
                recreate_stream: true,
                latest: false,
                missing_after_known_offset: false,
            },
            StartPosition {
                location: StartLocation::Earliest,
                inclusive: false,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn receiver_recovery_local_missing_annotations_preserve_offset() {
        run_local_recovery(
            LocalPeerConfig {
                recovery: LocalRecoveryKind::Link,
                annotations: LocalAnnotations::OffsetAndSequence,
                recreate_stream: false,
                latest: false,
                missing_after_known_offset: true,
            },
            StartPosition {
                location: StartLocation::Offset("0".to_string()),
                inclusive: true,
            },
        )
        .await;
    }

    #[tokio::test]
    async fn receiver_recovery_local_latest_moving_tail() {
        run_local_recovery(
            LocalPeerConfig {
                recovery: LocalRecoveryKind::Session,
                annotations: LocalAnnotations::OffsetAndSequence,
                recreate_stream: false,
                latest: true,
                missing_after_known_offset: false,
            },
            StartPosition::default(),
        )
        .await;
    }

    #[tokio::test]
    async fn receiver_recovery_local_before_first_selector_characterization() {
        timeout(StdDuration::from_secs(15), async {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let source_url = Url::parse(&format!("amqp://{address}/hub/Partitions/0")).unwrap();
            let connection = RecoverableConnection::new(
                Url::parse(&format!("amqp://{address}/hub")).unwrap(),
                Some("receiver-before-first".to_string()),
                None,
                AmqpTransport::Tcp,
                None,
                Arc::new(MockCredential),
                RetryOptions::default(),
                None,
            );
            connection.authorizer.disable_authorization().unwrap();
            connection
                .authorizer
                .set_token_refresh_bias_for_test(azure_core::time::Duration::seconds(1))
                .unwrap();
            let start = StartPosition {
                location: StartLocation::Offset("2".to_string()),
                inclusive: true,
            };
            let source = local_source(&source_url, &start);
            let receiver_options = AmqpReceiverOptions {
                name: Some("receiver-before-first".to_string()),
                credit_mode: Some(ReceiverCreditMode::Auto(300)),
                auto_accept: true,
                ..Default::default()
            };
            let receiver = EventReceiver::new(
                connection.clone(),
                receiver_options.clone(),
                source.clone(),
                source_url.clone(),
                "0".to_string(),
                None,
            );
            let (selectors, mut observed_selectors) = mpsc::unbounded_channel();
            let close_initial = Arc::new(Notify::new());
            let close_initial_for_peer = close_initial.clone();
            let marker_received = Arc::new(Notify::new());
            let marker_received_for_peer = marker_received.clone();
            let peer = tokio::spawn(async move {
                for attach_index in 0..2 {
                    let (stream, _) = timeout(LOCAL_PEER_TIMEOUT, listener.accept())
                        .await
                        .unwrap()
                        .unwrap();
                    let mut connection = ConnectionAcceptor::builder()
                        .container_id(format!("receiver-before-first-peer-{attach_index}"))
                        .sasl_acceptor(SaslAnonymousMechanism {})
                        .build()
                        .accept(stream)
                        .await
                        .unwrap();
                    let mut session = timeout(
                        LOCAL_PEER_TIMEOUT,
                        SessionAcceptor::new().accept(&mut connection),
                    )
                    .await
                    .unwrap()
                    .unwrap();
                    let LinkEndpoint::Sender(mut sender) =
                        timeout(LOCAL_PEER_TIMEOUT, LinkAcceptor::new().accept(&mut session))
                            .await
                            .unwrap()
                            .unwrap()
                    else {
                        panic!("expected the client receiver link");
                    };
                    selectors.send(selector_from_sender(&sender)).unwrap();
                    if attach_index == 0 {
                        timeout(LOCAL_PEER_TIMEOUT, close_initial_for_peer.notified())
                            .await
                            .unwrap();
                        let _ = timeout(LOCAL_PEER_TIMEOUT, connection.close())
                            .await
                            .unwrap();
                    } else {
                        send_local_marker_batchable(
                            &mut sender,
                            2,
                            LocalAnnotations::OffsetAndSequence,
                        )
                        .await;
                        timeout(LOCAL_PEER_TIMEOUT, marker_received_for_peer.notified())
                            .await
                            .unwrap();
                    }
                }
            });
            timeout(
                LOCAL_PEER_TIMEOUT,
                connection.ensure_receiver(&source_url, &source, &receiver_options),
            )
            .await
            .unwrap()
            .unwrap();
            let initial_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(
                initial_selector,
                local_selector(&start),
                "the initial selector must remain inclusive before the first delivery"
            );
            connection
                .force_error(local_error(LocalRecoveryKind::Connection))
                .unwrap();
            close_initial.notify_one();
            let mut stream = Box::pin(receiver.stream_events());
            let first_event = next_local_event(&mut stream).await;
            let recovery_selector = timeout(LOCAL_PEER_TIMEOUT, observed_selectors.recv())
                .await
                .unwrap()
                .unwrap();
            assert_eq!(
                recovery_selector,
                local_selector(&start),
                "recovery before the first delivery must retain the configured selector"
            );
            assert_eq!(marker(&first_event), 2);
            marker_received.notify_one();
            connection.close_connection().await.unwrap();
            timeout(LOCAL_PEER_TIMEOUT, peer).await.unwrap().unwrap();
        })
        .await
        .expect("before-first selector characterization must finish within its bound");
    }
}
