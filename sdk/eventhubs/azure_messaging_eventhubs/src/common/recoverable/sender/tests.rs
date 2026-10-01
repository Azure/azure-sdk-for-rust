// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore sasl

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
    future::pending,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::{
    net::TcpListener,
    sync::{mpsc, Notify},
    task::JoinSet,
    time::timeout,
};

#[tokio::test]
async fn pending_sends_recover_after_peer_invalidates_connection() {
    timeout(Duration::from_secs(15), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("amqp://{}/hub", listener.local_addr().unwrap())).unwrap();
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
                                connection.close().await.unwrap();
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
            url.clone(), None, None, AmqpTransport::Tcp, Arc::new(MockCredential),
            Default::default(), None,
        );
        connection.authorizer.disable_authorization().unwrap();
        connection.authorizer.set_token_refresh_bias_for_test(
            azure_core::time::Duration::seconds(1),
        ).unwrap();
        let mut sends = JoinSet::new();
        for partition in 0..2 {
            let path = Url::parse(&format!("{url}/Partitions/{partition}")).unwrap();
            let sender = connection.get_sender(path).await.unwrap();
            sends.spawn(async move { sender.send(AmqpMessage::default(), None).await });
        }
        for _ in 0..2 {
            assert_eq!(transfers_rx.recv().await.unwrap().0, 0);
        }
        assert!(sends.try_join_next().is_none(), "the peer withholds both settlements");
        let metadata_sender = connection.get_sender(
            Url::parse(&format!("{url}/Partitions/0")).unwrap(),
        ).await.unwrap();
        let metadata = metadata_sender.max_message_size();
        futures::pin_mut!(metadata);
        assert!(futures::poll!(&mut metadata).is_pending());

        close_old.notify_one();
        RecoverableConnection::recover_from_error(
            Arc::downgrade(&connection), ErrorRecoveryAction::ReconnectConnection,
        ).await.unwrap();

        // A new caller must remain independent of sends on the old connection.
        let healthy = connection.get_sender(Url::parse(&format!("{url}/Partitions/2")).unwrap()).await.unwrap();
        assert!(matches!(healthy.send(AmqpMessage::default(), None).await.unwrap(), AmqpSendOutcome::Accepted));
        for _ in 0..2 {
            let result = timeout(Duration::from_secs(3), sends.join_next()).await
                .expect("pending sends must wake and retry after peer recovery")
                .unwrap().unwrap().unwrap();
            assert!(matches!(result, AmqpSendOutcome::Accepted));
        }
        timeout(Duration::from_secs(3), metadata).await
            .expect("sender metadata must stop waiting on the retired sender lock")
            .unwrap();
        assert_eq!(connection.generation(), 2, "cancelled sends must not invalidate the new connection");
        let mut retried = Vec::new();
        for _ in 0..3 {
            let (generation, path) = transfers_rx.recv().await.unwrap();
            assert_eq!(generation, 1);
            retried.push(path.rsplit('/').next().unwrap().to_string());
        }
        retried.sort();
        assert_eq!(retried, ["0", "1", "2"]);
        assert!(transfers_rx.try_recv().is_err(), "each pending send retries only once");
    }).await.expect("the local peer regression must finish");
}

#[tokio::test]
async fn repeated_sender_invalidations_respect_retry_limit() {
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
