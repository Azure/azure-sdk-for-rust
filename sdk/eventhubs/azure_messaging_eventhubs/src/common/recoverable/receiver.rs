// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use super::RecoverableConnection;
use crate::common::retry::{
    recover_with_policy, BudgetStart, ErrorRecoveryAction, RecoveryClock, RecoveryLogContext,
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

    /// Attaches the receiver if needed, then waits for one delivery.
    async fn receive_once(&self) -> Result<azure_core_amqp::AmqpDelivery> {
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
    /// The elapsed-time budget starts at the first failure, so the wait for an
    /// event before that failure does not use it. Each call is a new recovery
    /// episode with a new budget. Tests call this directly to control the
    /// attempt, the recovery action, and the clock.
    async fn receive_with_recovery<T, F, Fut, C, K>(
        &self,
        attempt: F,
        recover: RecoveryOperation<C, AmqpError>,
        context: C,
        clock: &K,
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
                budget_start: BudgetStart::FirstFailure,
                log_context: RecoveryLogContext {
                    connection_id: Some(&self.connection_id),
                    partition_id: Some(&self.partition_id),
                },
                clock,
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
        self.receive_with_recovery(
            || self.receive_once(),
            |connection: Weak<RecoverableConnection>, reason| {
                Box::pin(RecoverableConnection::recover_from_error(
                    connection, reason,
                ))
            },
            self.recoverable_connection.clone(),
            &SystemClock,
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
    use super::*;
    use crate::{
        common::retry::test_support::{capture_warnings, stop_warning, ManualClock},
        error::{find_link_stolen, ErrorKind},
        RetryOptions,
    };
    use azure_core_amqp::{
        error::{AmqpErrorCondition, AmqpErrorKind},
        AmqpDescribedError, AmqpTransport,
    };
    use azure_core_test::credentials::MockCredential;
    use std::{
        future::Future,
        pin::Pin,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::Duration as StdDuration,
    };
    use tracing::Instrument;

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
        Stolen,
    }

    #[derive(Clone)]
    struct FakeRecovery {
        clock: Arc<ManualClock>,
        cost: StdDuration,
        count: Arc<AtomicUsize>,
    }

    fn fake_recover(
        recovery: FakeRecovery,
        _: ErrorRecoveryAction,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        recovery.count.fetch_add(1, Ordering::SeqCst);
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
                    count: Arc::default(),
                },
            }
        }

        fn recoveries(&self) -> usize {
            self.recovery.count.load(Ordering::SeqCst)
        }

        /// Runs one `receive_with_recovery` call. `script` maps the attempt
        /// index to the seconds the attempt waits and its outcome. A delivery
        /// returns its attempt index.
        async fn receive(
            &self,
            script: impl Fn(usize) -> (u64, Outcome),
        ) -> (Result<usize>, usize, String) {
            let attempts = AtomicUsize::new(0);
            let clock = &self.recovery.clock;
            let (result, logs) = capture_warnings(self.receiver.receive_with_recovery(
                || {
                    let attempt = attempts.fetch_add(1, Ordering::SeqCst);
                    let (wait, outcome) = script(attempt);
                    clock.advance(StdDuration::from_secs(wait));
                    async move {
                        match outcome {
                            Outcome::Deliver => Ok(attempt),
                            Outcome::LinkClosed => {
                                Err(AmqpError::from(AmqpErrorKind::LinkClosedByRemote(
                                    Box::new(std::io::Error::other("closed")),
                                )))
                            }
                            Outcome::Stolen => Err(stolen()),
                        }
                    }
                },
                fake_recover,
                self.recovery.clone(),
                &**clock,
            ))
            .await;
            (result, attempts.load(Ordering::SeqCst), logs)
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
        let harness = Harness::new(RetryOptions::default(), StdDuration::from_secs(1));
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (120, Outcome::LinkClosed),
                _ => (0, Outcome::Deliver),
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
        let (result, attempts, logs) = harness.receive(|_| (600, Outcome::Deliver)).await;
        assert_eq!(result.unwrap(), 0);
        assert_eq!((attempts, harness.recoveries()), (1, 0));
        assert!(logs.is_empty(), "{logs}");
    }

    // The stream enters a DEBUG span with the receiver identifiers. At WARN
    // that span is off, so the warning must carry the identifiers itself.
    #[tokio::test]
    async fn repeated_failures_exhaust_elapsed_budget() {
        let harness = Harness::new(RetryOptions::default(), StdDuration::from_secs(25));
        let span = tracing::debug_span!("stream_events", connection_id = "span-conn");
        let (result, attempts, logs) = harness
            .receive(|attempt| match attempt {
                0 => (120, Outcome::LinkClosed),
                _ => (0, Outcome::LinkClosed),
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
        let harness = Harness::new(options, StdDuration::from_secs(1));
        let (result, attempts, logs) = harness.receive(|_| (0, Outcome::LinkClosed)).await;
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
        let (result, attempts, logs) = harness.receive(|_| (120, Outcome::LinkClosed)).await;
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
        let (result, attempts, logs) = harness.receive(|_| (5, Outcome::Stolen)).await;
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
        let harness = Harness::new(RetryOptions::default(), StdDuration::from_secs(50));
        let script = |attempt| match attempt {
            0 => (0, Outcome::LinkClosed),
            _ => (0, Outcome::Deliver),
        };
        assert_eq!(harness.receive(script).await.0.unwrap(), 1);
        harness.recovery.clock.advance(StdDuration::from_secs(30));
        let (result, _, logs) = harness.receive(script).await;
        assert_eq!(result.unwrap(), 1);
        assert_eq!(harness.recoveries(), 2);
        assert!(!logs.contains("Operation recovery stopped"), "{logs}");
    }
}
