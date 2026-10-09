// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore sasl blackhole

use super::{
    recover_azure_operation, AmqpError, AmqpMessage, AmqpSendOutcome, AmqpSenderApis,
    ErrorRecoveryAction, RecoverableConnection, RecoverableSender, SenderInvalidated,
};
use crate::RetryOptions;
use azure_core::http::Url;
use azure_core_amqp::{AmqpErrorKind, AmqpTransport};
use azure_core_test::credentials::MockCredential;
use fe2o3_amqp::{
    acceptor::{
        ConnectionAcceptor, LinkAcceptor, LinkEndpoint, SaslAnonymousMechanism, SessionAcceptor,
    },
    types::{messaging::Body, primitives::Value},
};
use std::{
    error::Error,
    future::pending,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};
use tokio::{
    io::AsyncReadExt,
    net::TcpListener,
    sync::{mpsc, Notify},
    task::JoinSet,
    time::timeout,
};

#[tokio::test]
async fn pending_sends_recover_after_peer_invalidates_connection() {
    pending_sends_recover(true, 2, false).await;
}

#[tokio::test]
async fn pending_sends_recover_after_transport_closes() {
    pending_sends_recover(false, 2, false).await;
}

#[tokio::test]
async fn thirty_pending_sends_recover_after_transport_closes() {
    pending_sends_recover(false, 30, false).await;
}

#[tokio::test]
async fn short_blackhole_recovers_within_explicit_finite_budget() {
    pending_sends_recover(false, 2, true).await;
}

async fn pending_sends_recover(invalidate_from_peer: bool, count: usize, blackhole: bool) {
    timeout(Duration::from_secs(15), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let freeze = Arc::new(Notify::new());
        let frozen = Arc::new(Notify::new());
        let resume = Arc::new(Notify::new());
        let mut proxies = JoinSet::new();
        let client_address = if blackhole {
            let proxy = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let proxy_address = proxy.local_addr().unwrap();
            let freeze = freeze.clone();
            let frozen = frozen.clone();
            let resume = resume.clone();
            proxies.spawn(async move {
                let mut sockets = JoinSet::new();
                for generation in 0..2 {
                    let (mut client, _) = proxy.accept().await.unwrap();
                    let mut server = tokio::net::TcpStream::connect(address).await.unwrap();
                    let freeze = freeze.clone();
                    let frozen = frozen.clone();
                    let resume = resume.clone();
                    sockets.spawn(async move {
                        let forwarding = tokio::io::copy_bidirectional(&mut client, &mut server);
                        tokio::pin!(forwarding);
                        if generation == 0 {
                            tokio::select! {
                                result = &mut forwarding => { result.unwrap(); return; }
                                _ = freeze.notified() => {
                                    frozen.notify_one();
                                    resume.notified().await;
                                }
                            }
                        }
                        let _ = forwarding.await;
                    });
                }
                while let Some(result) = sockets.join_next().await { result.unwrap(); }
            });
            proxy_address
        } else { address };
        let url = Url::parse(&format!("amqp://{client_address}/hub")).unwrap();
        let close_old = Arc::new(Notify::new());
        let (transfers_tx, mut transfers_rx) = mpsc::unbounded_channel();
        let mut peer_tasks = JoinSet::new();
        let peer_close = close_old.clone();
        peer_tasks.spawn(async move {
            let mut connections = JoinSet::new();
            for generation in 0..2 {
                let (stream, _) = listener.accept().await.unwrap();
                let transfers = transfers_tx.clone();
                let close = peer_close.clone();
                connections.spawn(async move {
                    let mut connection = ConnectionAcceptor::builder()
                        .container_id(format!("peer-{generation}"))
                        .sasl_acceptor(SaslAnonymousMechanism {})
                        .build()
                        .accept(stream)
                        .await
                        .unwrap();
                    let mut links = JoinSet::new();
                    let session_acceptor = SessionAcceptor::new();
                    loop {
                        tokio::select! {
                            _ = close.notified(), if generation == 0 => {
                                // Recovery can retire the socket before the peer's
                                // close handshake finishes.
                                let _ = connection.close().await;
                                break;
                            }
                            session = session_acceptor.accept(&mut connection) => {
                                let mut session = session.unwrap();
                                let transfers = transfers.clone();
                                links.spawn(async move {
                                    let LinkEndpoint::Receiver(mut receiver) =
                                        LinkAcceptor::new().accept(&mut session).await.unwrap()
                                    else {
                                        panic!("expected a sender from the SDK");
                                    };
                                    receiver.set_auto_accept(false);
                                    let path = receiver.target().as_ref().unwrap().address.as_ref().unwrap().to_string();
                                    let delivery = receiver.recv::<Body<Value>>().await.unwrap();
                                    transfers.send((generation, path)).unwrap();
                                    if generation != 0 {
                                        receiver.accept(&delivery).await.unwrap();
                                    }
                                    pending::<()>().await;
                                    drop((receiver, session, delivery));
                                });
                            }
                        }
                    }
                });
            }
            pending::<()>().await;
            drop(connections);
        });

        let connection = RecoverableConnection::new(
            url.clone(), None, None, AmqpTransport::Tcp, None, Arc::new(MockCredential),
            RetryOptions { max_total_elapsed: azure_core::time::Duration::seconds(3), ..Default::default() }, None,
        );
        connection.authorizer.disable_authorization().unwrap();
        connection.authorizer.set_token_refresh_bias_for_test(
            azure_core::time::Duration::seconds(1),
        ).unwrap();
        let mut sends = JoinSet::new();
        for partition in 0..count {
            let path = Url::parse(&format!("{url}/Partitions/{partition}")).unwrap();
            let sender = connection.get_sender(path).await.unwrap();
            sends.spawn(async move { sender.send(AmqpMessage::default(), None).await });
        }
        for _ in 0..count {
            assert_eq!(transfers_rx.recv().await.unwrap().0, 0);
        }
        assert!(sends.try_join_next().is_none(), "the peer withholds both settlements");
        let metadata_sender = connection.get_sender(
            Url::parse(&format!("{url}/Partitions/0")).unwrap(),
        ).await.unwrap();
        let metadata = metadata_sender.max_message_size();
        futures::pin_mut!(metadata);
        assert!(futures::poll!(&mut metadata).is_pending());

        if blackhole {
            freeze.notify_one();
            frozen.notified().await;
        }
        close_old.notify_one();
        if blackhole {
            tokio::time::sleep(Duration::from_millis(300)).await;
            assert!(sends.try_join_next().is_none(), "the TCP proxy blocks both directions");
            resume.notify_one();
        }
        if invalidate_from_peer {
            RecoverableConnection::recover_from_error(
                Arc::downgrade(&connection), ErrorRecoveryAction::ReconnectConnection,
            ).await.unwrap();
        }

        // A new caller must remain independent of sends on the old connection.
        for _ in 0..count {
            let result = timeout(Duration::from_secs(3), sends.join_next()).await
                .expect("pending sends must wake and retry after peer recovery")
                .unwrap().unwrap().unwrap();
            assert!(matches!(result, AmqpSendOutcome::Accepted));
        }
        timeout(Duration::from_secs(3), metadata).await
            .expect("sender metadata must stop waiting on the retired sender lock")
            .unwrap();
        let healthy = connection.get_sender(Url::parse(&format!("{url}/Partitions/{count}")).unwrap()).await.unwrap();
        assert!(matches!(healthy.send(AmqpMessage::default(), None).await.unwrap(), AmqpSendOutcome::Accepted));
        assert_eq!(connection.generation(), 2, "cancelled sends must not invalidate the new connection");
        let mut retried = Vec::new();
        for _ in 0..=count {
            let (generation, path) = transfers_rx.recv().await.unwrap();
            assert_eq!(generation, 1);
            retried.push(path.rsplit('/').next().unwrap().parse::<usize>().unwrap());
        }
        retried.sort();
        assert_eq!(retried, (0..=count).collect::<Vec<_>>());
        assert!(transfers_rx.try_recv().is_err(), "each pending send retries only once");
    }).await.expect("the local peer regression must finish");
}

#[tokio::test]
async fn repeated_generation_invalidations_respect_retry_limit() {
    let attempts = AtomicUsize::new(0);
    let options = RetryOptions {
        initial_delay: azure_core::time::Duration::ZERO,
        max_delay: azure_core::time::Duration::ZERO,
        max_retries: 2,
        ..Default::default()
    };
    let error = recover_azure_operation(
        || async {
            attempts.fetch_add(1, Ordering::SeqCst);
            Err::<(), AmqpError>(SenderInvalidated.into())
        },
        &options,
        RecoverableSender::should_retry_send_operation,
        None,
        None::<()>,
    )
    .await
    .unwrap_err();
    assert_eq!(attempts.load(Ordering::SeqCst), 3);
    assert_eq!(
        RecoverableSender::should_retry_send_operation(&error),
        ErrorRecoveryAction::RetryAction,
    );
}

#[test]
fn stale_errors_do_not_recover_the_new_generation() {
    let error = || {
        AmqpError::from(AmqpErrorKind::ConnectionDropped(Box::new(
            std::io::Error::from(std::io::ErrorKind::ConnectionAborted),
        )))
    };
    let stale = RecoverableSender::finish_attempt::<()>(Err(error()), false).unwrap_err();
    assert_eq!(
        RecoverableSender::should_retry_send_operation(&stale),
        ErrorRecoveryAction::RetryAction,
    );
    let current = RecoverableSender::finish_attempt::<()>(Err(error()), true).unwrap_err();
    assert_eq!(
        RecoverableSender::should_retry_send_operation(&current),
        ErrorRecoveryAction::ReconnectConnection,
    );
}

#[test]
fn acknowledged_outcome_wins_over_concurrent_invalidation() {
    assert_eq!(
        RecoverableSender::finish_attempt(Ok(42), false).unwrap(),
        42
    );
}

fn connection_with_options(options: RetryOptions) -> Arc<RecoverableConnection> {
    RecoverableConnection::new(
        Url::parse("amqp://localhost/hub").unwrap(),
        None,
        None,
        AmqpTransport::Tcp,
        None,
        Arc::new(MockCredential),
        options,
        None,
    )
}

fn assert_timed_out(error: &AmqpError) {
    let AmqpErrorKind::AzureCore(error) = error.kind() else {
        panic!("expected an Azure core timeout, got {error:?}");
    };
    assert_eq!(error.kind(), &azure_core::error::ErrorKind::Io);
    assert_eq!(
        error
            .source()
            .unwrap()
            .downcast_ref::<std::io::Error>()
            .unwrap()
            .kind(),
        std::io::ErrorKind::TimedOut
    );
}

#[tokio::test(start_paused = true)]
async fn default_deadline_interrupts_pending_operation_with_zero_retries() {
    let connection = connection_with_options(RetryOptions {
        max_retries: 0,
        ..Default::default()
    });
    let sender = RecoverableSender::new(Arc::downgrade(&connection), connection.url.clone());
    let operation = sender.recover(|_| pending::<azure_core_amqp::Result<()>>());
    futures::pin_mut!(operation);
    assert!(futures::poll!(&mut operation).is_pending());
    tokio::time::advance(Duration::from_secs(60)).await;
    assert_timed_out(&operation.await.unwrap_err());
    assert_eq!(connection.generation(), 2);
    assert_eq!(sender.recover(|_| async { Ok(42) }).await.unwrap(), 42);
}

#[tokio::test(start_paused = true)]
async fn acknowledged_result_wins_when_deadline_is_ready() {
    let connection = connection_with_options(RetryOptions::default());
    let sender = RecoverableSender::new(Arc::downgrade(&connection), connection.url.clone());
    let acknowledged = tokio::sync::oneshot::channel();
    let outcome = async { Ok(acknowledged.1.await.unwrap()) };
    let outcome = std::sync::Mutex::new(Some(outcome));
    let operation = sender.recover(|_| outcome.lock().unwrap().take().unwrap());
    futures::pin_mut!(operation);
    assert!(futures::poll!(&mut operation).is_pending());
    tokio::time::advance(Duration::from_secs(60)).await;
    acknowledged.0.send(42).unwrap();
    assert_eq!(operation.await.unwrap(), 42);
    assert_eq!(connection.generation(), 0);
}

#[tokio::test]
async fn deadline_retires_pending_sends_and_subsequent_call_reconnects() {
    timeout(Duration::from_secs(10), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("amqp://{}/hub", listener.local_addr().unwrap())).unwrap();
        let allow_ack = Arc::new(AtomicBool::new(false));
        let active = Arc::new(AtomicUsize::new(0));
        let accepted = Arc::new(AtomicUsize::new(0));
        let (transfers, mut received) = mpsc::unbounded_channel();
        let mut peer = JoinSet::new();
        let peer_allow = allow_ack.clone();
        let peer_active = active.clone();
        let peer_accepted = accepted.clone();
        peer.spawn(async move {
            let mut connections = JoinSet::new();
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                let allow = peer_allow.clone();
                let active = peer_active.clone();
                let transfers = transfers.clone();
                peer_accepted.fetch_add(1, Ordering::SeqCst);
                connections.spawn(async move {
                    active.fetch_add(1, Ordering::SeqCst);
                    let mut connection = ConnectionAcceptor::builder()
                        .container_id("deadline-peer")
                        .sasl_acceptor(SaslAnonymousMechanism {})
                        .build()
                        .accept(stream)
                        .await
                        .unwrap();
                    let mut links = JoinSet::new();
                    loop {
                        let session = SessionAcceptor::new().accept(&mut connection).await;
                        let Ok(mut session) = session else { break };
                        let allow = allow.clone();
                        let transfers = transfers.clone();
                        links.spawn(async move {
                            let Ok(LinkEndpoint::Receiver(mut receiver)) =
                                LinkAcceptor::new().accept(&mut session).await
                            else {
                                return;
                            };
                            receiver.set_auto_accept(false);
                            let Ok(delivery) = receiver.recv::<Body<Value>>().await else {
                                return;
                            };
                            transfers.send(()).unwrap();
                            if allow.load(Ordering::SeqCst) {
                                receiver.accept(&delivery).await.unwrap();
                            }
                            pending::<()>().await;
                            drop((receiver, session, delivery));
                        });
                    }
                    active.fetch_sub(1, Ordering::SeqCst);
                });
            }
        });
        let connection = RecoverableConnection::new(
            url.clone(),
            None,
            None,
            AmqpTransport::Tcp,
            None,
            Arc::new(MockCredential),
            RetryOptions {
                max_total_elapsed: azure_core::time::Duration::milliseconds(500),
                initial_delay: azure_core::time::Duration::milliseconds(500),
                max_delay: azure_core::time::Duration::milliseconds(500),
                ..Default::default()
            },
            None,
        );
        connection.authorizer.disable_authorization().unwrap();
        connection
            .authorizer
            .set_token_refresh_bias_for_test(azure_core::time::Duration::seconds(1))
            .unwrap();
        let mut sends = JoinSet::new();
        let started = Instant::now();
        for partition in 0..30 {
            let sender = connection
                .get_sender(Url::parse(&format!("{url}/Partitions/{partition}")).unwrap())
                .await
                .unwrap();
            sends.spawn(async move { sender.send(AmqpMessage::default(), None).await });
        }
        for _ in 0..30 {
            received.recv().await.unwrap();
        }
        while let Some(result) = sends.join_next().await {
            assert_timed_out(
                &result
                    .unwrap()
                    .err()
                    .expect("unacknowledged sends must time out"),
            );
        }
        assert!(started.elapsed() < Duration::from_secs(2));
        timeout(Duration::from_secs(1), async {
            while active.load(Ordering::SeqCst) != 0 {
                tokio::task::yield_now().await;
            }
        })
        .await
        .expect("retired sockets and connection tasks must finish");
        assert_eq!(connection.generation(), 2, "concurrent expiries coalesce");
        allow_ack.store(true, Ordering::SeqCst);
        let sender = connection
            .get_sender(Url::parse(&format!("{url}/Partitions/0")).unwrap())
            .await
            .unwrap();
        assert!(matches!(
            sender.send(AmqpMessage::default(), None).await.unwrap(),
            AmqpSendOutcome::Accepted
        ));
        assert_eq!(accepted.load(Ordering::SeqCst), 2);
        connection.close_connection().await.unwrap();
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn deadline_cancels_open_session_attach_and_cbs_preparation() {
    timeout(Duration::from_secs(15), async {
        for phase in ["open", "session", "attach", "cbs"] {
            let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
            let url =
                Url::parse(&format!("amqp://{}/hub", listener.local_addr().unwrap())).unwrap();
            let entered = Arc::new(Notify::new());
            let peer_entered = entered.clone();
            let mut peers = JoinSet::new();
            peers.spawn(async move {
                let (mut stream, _) = listener.accept().await.unwrap();
                if phase == "open" {
                    let mut header = [0; 8];
                    stream.read_exact(&mut header).await.unwrap();
                    peer_entered.notify_one();
                    let mut remaining = Vec::new();
                    let _ = stream.read_to_end(&mut remaining).await;
                    return;
                }
                let mut connection = ConnectionAcceptor::builder()
                    .container_id(format!("stalled-{phase}"))
                    .sasl_acceptor(SaslAnonymousMechanism {})
                    .build()
                    .accept(stream)
                    .await
                    .unwrap();
                if phase == "session" {
                    peer_entered.notify_one();
                    let _ = connection.on_close().await;
                    return;
                }
                let mut session = SessionAcceptor::new()
                    .accept(&mut connection)
                    .await
                    .unwrap();
                let mut links = Vec::new();
                if phase == "cbs" {
                    for _ in 0..2 {
                        links.push(LinkAcceptor::new().accept(&mut session).await.unwrap());
                    }
                    for link in &mut links {
                        if let LinkEndpoint::Receiver(receiver) = link {
                            receiver.recv::<Body<Value>>().await.unwrap();
                        }
                    }
                }
                peer_entered.notify_one();
                let _ = connection.on_close().await;
            });
            let connection = RecoverableConnection::new(
                url.clone(),
                None,
                None,
                AmqpTransport::Tcp,
                None,
                Arc::new(MockCredential),
                RetryOptions {
                    max_total_elapsed: azure_core::time::Duration::milliseconds(500),
                    max_retries: 0,
                    ..Default::default()
                },
                None,
            );
            if phase != "cbs" {
                connection.authorizer.disable_authorization().unwrap();
            }
            connection
                .authorizer
                .set_token_refresh_bias_for_test(azure_core::time::Duration::seconds(1))
                .unwrap();
            let sender = connection.get_sender(url).await.unwrap();
            let mut sends = JoinSet::new();
            sends.spawn(async move { sender.send(AmqpMessage::default(), None).await });
            entered.notified().await;
            let error = sends
                .join_next()
                .await
                .unwrap()
                .unwrap()
                .err()
                .expect("preparation must time out");
            assert_timed_out(&error);
            assert_eq!(
                connection.generation(),
                2,
                "{phase} cancellation leaves a complete recovery"
            );
            timeout(Duration::from_secs(1), peers.join_next())
                .await
                .expect("cancelled preparation must release its socket")
                .unwrap()
                .unwrap();
            assert!(connection
                .generation_invalidation()
                .await
                .unwrap()
                .1
                .get()
                .is_none());
        }
    })
    .await
    .unwrap();
}

#[tokio::test(start_paused = true)]
async fn deadline_after_self_recovery_preserves_replacement_generation() {
    let connection = connection_with_options(RetryOptions::default());
    let sender = RecoverableSender::new(Arc::downgrade(&connection), connection.url.clone());
    let operation = sender.recover(|generation| {
        let connection = connection.clone();
        async move {
            connection
                .recover_generation(
                    generation.load(Ordering::Acquire),
                    ErrorRecoveryAction::ReconnectConnection,
                )
                .await;
            pending::<azure_core_amqp::Result<()>>().await
        }
    });
    futures::pin_mut!(operation);
    assert!(futures::poll!(&mut operation).is_pending());
    assert_eq!(connection.generation(), 2);
    let (_, replacement) = connection.generation_invalidation().await.unwrap();
    tokio::time::advance(Duration::from_secs(60)).await;
    assert_timed_out(&operation.await.unwrap_err());
    assert_eq!(connection.generation(), 2);
    assert!(replacement.get().is_none());
}
