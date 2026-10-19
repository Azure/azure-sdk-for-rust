// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{Closed, Transport};
use crate::{
    AmqpConnection, AmqpConnectionApis, AmqpErrorKind, AmqpMessage, AmqpReceiver, AmqpReceiverApis,
    AmqpReceiverOptions, AmqpSender, AmqpSenderApis, AmqpSession, AmqpSessionApis, AmqpSource,
    ReceiverCreditMode,
};
use azure_core::http::Url;
use fe2o3_amqp::{
    acceptor::{
        ConnectionAcceptor, LinkAcceptor, LinkEndpoint, SaslAnonymousMechanism, SessionAcceptor,
    },
    connection::ConnectionStopReason,
    link::{LinkStateError, SessionStopReason},
    types::{
        definitions::{AmqpError as ProtocolError, Error as ProtocolDescribedError},
        messaging::Body,
        performatives::{Begin, End, Open},
        primitives::Value,
        sasl::{SaslCode, SaslInit, SaslMechanisms, SaslOutcome},
    },
    Connection,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    future::{pending, poll_fn, Future},
    io,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    task::{Context, Poll},
    time::Duration,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, DuplexStream, ReadBuf},
    net::{TcpListener, TcpStream},
    sync::{mpsc, Notify},
    task::JoinSet,
    time::timeout,
};

// cspell:ignore performatives sasl

#[derive(Clone, Copy)]
enum ReceivePeerAction {
    CloseConnection,
    EndSession,
    AbortPeerSocket,
    AbortClientTransport,
    SendAfterQuietWait,
}

#[tokio::test]
async fn pending_receives_wake_after_connection_close() {
    pending_receives(ReceivePeerAction::CloseConnection).await;
}

#[tokio::test]
async fn pending_receives_wake_after_session_end() {
    pending_receives(ReceivePeerAction::EndSession).await;
}

#[tokio::test]
async fn pending_receives_wake_after_peer_socket_disappears() {
    pending_receives(ReceivePeerAction::AbortPeerSocket).await;
}

#[tokio::test]
async fn pending_receives_wake_after_client_transport_abort() {
    pending_receives(ReceivePeerAction::AbortClientTransport).await;
}

#[tokio::test]
async fn healthy_receives_wait_past_producer_budget_then_accept_delivery() {
    pending_receives(ReceivePeerAction::SendAfterQuietWait).await;
}

async fn pending_receives(action: ReceivePeerAction) {
    // The healthy case advances its isolated runtime by two minutes.
    timeout(Duration::from_secs(150), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("amqp://{}", listener.local_addr().unwrap())).unwrap();
        let release = Arc::new(Notify::new());
        let peer_release = release.clone();
        let completed = Arc::new(Notify::new());
        let peer_completed = completed.clone();
        let ready = Arc::new(Notify::new());
        let peer_ready = ready.clone();
        let mut peers = JoinSet::new();
        peers.spawn(async move {
            let (socket, _) = listener.accept().await.unwrap();
            let (transport, stream) = Transport::new(socket);
            let mut connection = ConnectionAcceptor::builder()
                .container_id("receive-peer")
                .sasl_acceptor(SaslAnonymousMechanism {})
                .build()
                .accept(stream)
                .await
                .unwrap();
            let mut session = SessionAcceptor::new()
                .accept(&mut connection)
                .await
                .unwrap();
            let mut senders = Vec::new();
            for _ in 0..2 {
                let LinkEndpoint::Sender(sender) =
                    LinkAcceptor::new().accept(&mut session).await.unwrap()
                else {
                    panic!("expected an SDK receiver")
                };
                senders.push(sender);
            }
            peer_ready.notify_one();
            peer_release.notified().await;
            match action {
                ReceivePeerAction::CloseConnection => connection.close().await.unwrap(),
                ReceivePeerAction::EndSession => session.end().await.unwrap(),
                ReceivePeerAction::AbortPeerSocket => transport.abort(),
                ReceivePeerAction::AbortClientTransport => {}
                ReceivePeerAction::SendAfterQuietWait => {
                    for sender in &mut senders {
                        sender.send("after quiet wait").await.unwrap();
                    }
                }
            }
            peer_completed.notify_one();
            // Keep the link and session handles alive after closure. The receive
            // queue must wake from protocol/transport failure, not fixture teardown.
            pending::<()>().await;
            drop((senders, session, connection));
        });

        let connection = AmqpConnection::new();
        connection
            .open("receive-client".to_string(), url, None)
            .await
            .unwrap();
        let session = AmqpSession::new();
        session.begin(&connection, None).await.unwrap();
        let (waiting, mut entered) = mpsc::unbounded_channel();
        let mut receives = JoinSet::new();
        for partition in 0..2 {
            let receiver = AmqpReceiver::new();
            receiver
                .attach(
                    &session,
                    AmqpSource::builder()
                        .with_address("hub".to_string())
                        .build(),
                    Some(AmqpReceiverOptions {
                        name: Some(format!("receiver-{partition}")),
                        // Closure cases need an empty receive, not delivery credit. Avoid
                        // racing an initial Flow frame with the peer's Close frame.
                        credit_mode: Some(
                            if matches!(action, ReceivePeerAction::SendAfterQuietWait) {
                                ReceiverCreditMode::Auto(100)
                            } else {
                                ReceiverCreditMode::Manual
                            },
                        ),
                        ..Default::default()
                    }),
                )
                .await
                .unwrap();
            let waiting = waiting.clone();
            receives.spawn(async move {
                let receive = receiver.receive_delivery();
                tokio::pin!(receive);
                poll_fn(|cx| {
                    assert!(receive.as_mut().poll(cx).is_pending());
                    Poll::Ready(())
                })
                .await;
                waiting.send(()).unwrap();
                let delivery = receive.await?;
                receiver.accept_delivery(&delivery).await
            });
        }
        for _ in 0..2 {
            entered.recv().await.unwrap();
        }
        ready.notified().await;
        assert!(receives.try_join_next().is_none());

        if matches!(action, ReceivePeerAction::SendAfterQuietWait) {
            tokio::time::pause();
            tokio::time::advance(Duration::from_secs(120)).await;
            assert!(
                receives.try_join_next().is_none(),
                "a healthy empty receive has no producer deadline"
            );
            tokio::time::resume();
        } else if matches!(action, ReceivePeerAction::AbortClientTransport) {
            connection.abort();
        }
        release.notify_one();

        for _ in 0..2 {
            let result = timeout(Duration::from_secs(2), receives.join_next())
                .await
                .expect("each receive must wake after closure or a delivery")
                .unwrap()
                .unwrap();
            if matches!(action, ReceivePeerAction::SendAfterQuietWait) {
                result.expect("the same receive must accept a delivery after its quiet wait");
            } else {
                let error = result.expect_err("a disconnected receive must return a typed error");
                let AmqpErrorKind::LinkStateError(source) = error.kind() else {
                    panic!("expected a receive link-state error, got {error:?}");
                };
                let expected = match action {
                    ReceivePeerAction::CloseConnection => {
                        SessionStopReason::ConnectionStopped(ConnectionStopReason::RemoteClosed)
                    }
                    ReceivePeerAction::EndSession => SessionStopReason::RemoteEnded,
                    ReceivePeerAction::AbortPeerSocket
                    | ReceivePeerAction::AbortClientTransport => {
                        SessionStopReason::ConnectionStopped(ConnectionStopReason::Closed)
                    }
                    ReceivePeerAction::SendAfterQuietWait => unreachable!(),
                };
                assert!(
                    matches!(
                        source.downcast_ref::<LinkStateError>(),
                        Some(LinkStateError::SessionStopped(reason)) if *reason == expected
                    ),
                    "unexpected receive stop reason: {source:?}"
                );
            }
        }
        timeout(Duration::from_secs(2), completed.notified())
            .await
            .expect("the peer must finish its action, including observing delivery acceptance");
        connection.abort();
    })
    .await
    .expect("local receive lifecycle regression must finish");
}

#[tokio::test]
async fn session_end_wakes_every_send_and_metadata_waiter() {
    session_end_wakes_waiters(false, false).await;
}

#[tokio::test]
async fn session_end_preserves_terminal_protocol_condition() {
    session_end_wakes_waiters(true, false).await;
}

#[tokio::test]
async fn cancelled_session_end_wakes_every_send_and_metadata_waiter() {
    session_end_wakes_waiters(false, true).await;
}

#[tokio::test]
async fn local_session_end_preserves_remote_error_for_waiters() {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("amqp://{}", listener.local_addr().unwrap())).unwrap();
        let release = Arc::new(Notify::new());
        let peer_release = release.clone();
        let mut peers = JoinSet::new();
        peers.spawn(async move {
            // Use a wire peer so the reply to our End carries an error. The
            // fe2o3 acceptor automatically replies to End without an error.
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut header = [0; 8];
            socket.read_exact(&mut header).await.unwrap();
            assert_eq!(&header, b"AMQP\x03\x01\x00\x00");
            socket.write_all(&header).await.unwrap();
            write_session_test_frame(&mut socket, 1, 0, SaslMechanisms::default()).await;
            let (_, init): (_, SaslInit) = read_session_test_frame(&mut socket, 1).await;
            assert_eq!(init.mechanism.as_str(), "ANONYMOUS");
            write_session_test_frame(
                &mut socket,
                1,
                0,
                SaslOutcome {
                    code: SaslCode::Ok,
                    additional_data: None,
                },
            )
            .await;
            socket.read_exact(&mut header).await.unwrap();
            assert_eq!(&header, b"AMQP\x00\x01\x00\x00");
            socket.write_all(&header).await.unwrap();
            let (_, mut open): (_, Open) = read_session_test_frame(&mut socket, 0).await;
            open.container_id = "session-error-peer".into();
            open.idle_time_out = None;
            write_session_test_frame(&mut socket, 0, 0, open).await;
            let (channel, mut begin): (_, Begin) = read_session_test_frame(&mut socket, 0).await;
            begin.remote_channel = Some(channel);
            write_session_test_frame(&mut socket, 0, 0, begin).await;
            let (end_channel, end): (_, End) = read_session_test_frame(&mut socket, 0).await;
            assert_eq!(end_channel, channel);
            assert!(end.error.is_none());
            write_session_test_frame(
                &mut socket,
                0,
                0,
                End {
                    error: Some(ProtocolDescribedError::new(
                        ProtocolError::UnauthorizedAccess,
                        Some("session permission revoked".into()),
                        Some(
                            [("reason".into(), Value::String("revoked".into()))]
                                .into_iter()
                                .collect(),
                        ),
                    )),
                },
            )
            .await;
            // Keep TCP open so connection closure cannot satisfy the waiters.
            peer_release.notified().await;
        });
        let connection = AmqpConnection::new();
        connection
            .open("session-client".into(), url, None)
            .await
            .unwrap();
        let session = AmqpSession::new();
        session.begin(&connection, None).await.unwrap();
        let closed = session.implementation.closed().unwrap();
        let mut waiters = JoinSet::new();
        for _ in 0..2 {
            let closed = closed.clone();
            waiters.spawn(async move { closed.run(pending::<crate::error::Result<()>>()).await });
        }
        tokio::task::yield_now().await;
        let error = session.end().await.unwrap_err();
        let AmqpErrorKind::AmqpDescribedError(expected) = error.kind() else {
            panic!("local end must return the remote error: {error:?}");
        };
        assert_eq!(
            expected.condition,
            crate::error::AmqpErrorCondition::UnauthorizedAccess
        );
        assert_eq!(
            expected.description.as_deref(),
            Some("session permission revoked")
        );
        assert_eq!(
            expected.info.get("reason"),
            Some(&crate::AmqpValue::String("revoked".into()))
        );
        while let Some(result) = waiters.join_next().await {
            let error = result.unwrap().unwrap_err();
            assert!(
                matches!(error.kind(), AmqpErrorKind::AmqpDescribedError(actual) if actual == expected),
                "waiter lost the remote error: {error:?}"
            );
        }
        // The saved error must also be available to operations started later.
        let error = closed
            .run(pending::<crate::error::Result<()>>())
            .await
            .unwrap_err();
        assert!(
            matches!(error.kind(), AmqpErrorKind::AmqpDescribedError(actual) if actual == expected)
        );
        release.notify_one();
        peers.join_next().await.unwrap().unwrap();
        connection.abort();
    })
    .await
    .expect("local session shutdown and its waiters must finish");
}

async fn read_session_test_frame<T: DeserializeOwned>(
    socket: &mut TcpStream,
    frame_type: u8,
) -> (u16, T) {
    let mut header = [0; 8];
    socket.read_exact(&mut header).await.unwrap();
    let size = u32::from_be_bytes(header[..4].try_into().unwrap()) as usize;
    assert!((8..=65_536).contains(&size));
    assert_eq!(header[4], 2, "fixture expects no extended frame header");
    assert_eq!(header[5], frame_type);
    let channel = u16::from_be_bytes([header[6], header[7]]);
    let mut body = vec![0; size - 8];
    socket.read_exact(&mut body).await.unwrap();
    (channel, serde_amqp::from_slice(&body).unwrap())
}

async fn write_session_test_frame(
    socket: &mut TcpStream,
    frame_type: u8,
    channel: u16,
    value: impl Serialize,
) {
    let body = serde_amqp::to_vec(&value).unwrap();
    socket
        .write_all(&u32::try_from(body.len() + 8).unwrap().to_be_bytes())
        .await
        .unwrap();
    socket.write_all(&[2, frame_type]).await.unwrap();
    socket.write_all(&channel.to_be_bytes()).await.unwrap();
    socket.write_all(&body).await.unwrap();
}

async fn session_end_wakes_waiters(with_error: bool, cancel_end: bool) {
    timeout(Duration::from_secs(5), async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = Url::parse(&format!("amqp://{}", listener.local_addr().unwrap())).unwrap();
        let end = Arc::new(Notify::new());
        let peer_end = end.clone();
        let (transfers, mut received) = mpsc::unbounded_channel();
        let mut peers = JoinSet::new();
        peers.spawn(async move {
            let (stream, _) = listener.accept().await.unwrap();
            let mut connection = ConnectionAcceptor::builder()
                .container_id("session-peer")
                .sasl_acceptor(SaslAnonymousMechanism {})
                .build()
                .accept(stream)
                .await
                .unwrap();
            let mut session = SessionAcceptor::new()
                .accept(&mut connection)
                .await
                .unwrap();
            let mut links = JoinSet::new();
            for _ in 0..2 {
                let LinkEndpoint::Receiver(mut receiver) =
                    LinkAcceptor::new().accept(&mut session).await.unwrap()
                else {
                    panic!("expected SDK sender")
                };
                receiver.set_auto_accept(false);
                let transfers = transfers.clone();
                links.spawn(async move {
                    let delivery = receiver.recv::<Body<Value>>().await.unwrap();
                    transfers.send(()).unwrap();
                    pending::<()>().await;
                    drop((receiver, delivery));
                });
            }
            peer_end.notified().await;
            if cancel_end {
                assert!(matches!(
                    session.on_end().await,
                    Err(fe2o3_amqp::session::Error::RemoteEnded)
                ));
            } else if with_error {
                session
                    .end_with_error(fe2o3_amqp::types::definitions::Error::new(
                        fe2o3_amqp::types::definitions::ErrorCondition::Custom(
                            "amqp:unauthorized-access".into(),
                        ),
                        None,
                        None,
                    ))
                    .await
                    .unwrap();
            } else {
                session.end().await.unwrap();
            }
            pending::<()>().await;
            drop((links, connection));
        });
        let connection = AmqpConnection::new();
        connection
            .open("session-client".to_string(), url, None)
            .await
            .unwrap();
        let session = AmqpSession::new();
        session.begin(&connection, None).await.unwrap();
        let mut sends = JoinSet::new();
        let mut senders = Vec::new();
        for partition in 0..2 {
            let sender = Arc::new(AmqpSender::new());
            sender
                .attach(
                    &session,
                    format!("sender-{partition}"),
                    "hub".to_string(),
                    None,
                )
                .await
                .unwrap();
            senders.push(sender.clone());
            sends.spawn(async move { sender.send(AmqpMessage::default(), None).await });
        }
        for _ in 0..2 {
            received.recv().await.unwrap();
        }
        let sender = senders[0].clone();
        let metadata = tokio::spawn(async move { sender.max_message_size().await });
        tokio::task::yield_now().await;
        let closed = session.implementation.closed().unwrap();
        let mut waiter = Box::pin(closed.run(pending::<crate::error::Result<()>>()));
        poll_fn(|cx| {
            assert!(waiter.as_mut().poll(cx).is_pending());
            Poll::Ready(())
        })
        .await;
        if cancel_end {
            // The empty control queue accepts End on the first poll, which then
            // waits on the session outcome. Cancel before yielding so the peer
            // cannot reply until after the caller's completion waker is abandoned.
            let mut ending = Box::pin(session.end());
            poll_fn(|cx| {
                assert!(ending.as_mut().poll(cx).is_pending());
                Poll::Ready(())
            })
            .await;
            drop(ending);
        }
        end.notify_one();
        timeout(Duration::from_secs(1), waiter)
            .await
            .expect("session closure must wake waiters even after end is cancelled")
            .unwrap_err();
        while let Some(result) = sends.join_next().await {
            let error = result.unwrap().err().unwrap();
            if with_error {
                assert!(
                    matches!(error.kind(), AmqpErrorKind::AmqpDescribedError(error)
                    if error.condition == crate::error::AmqpErrorCondition::UnauthorizedAccess)
                );
            } else {
                assert!(matches!(
                    error.kind(),
                    AmqpErrorKind::SessionClosedByRemote(_)
                ));
            }
        }
        // The mutex may become available before the closure notification.
        // Either metadata or its session error must return promptly.
        timeout(Duration::from_secs(1), metadata)
            .await
            .unwrap()
            .unwrap()
            .ok();
        connection.abort();
    })
    .await
    .unwrap();
}

#[derive(Debug)]
struct BlockedStream {
    inner: DuplexStream,
    blocked: Arc<AtomicBool>,
    blocked_write: Arc<Closed>,
    dropped: Arc<AtomicUsize>,
}

impl Drop for BlockedStream {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::SeqCst);
    }
}

impl AsyncRead for BlockedStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if self.blocked.load(Ordering::SeqCst) {
            return Poll::Pending;
        }
        Pin::new(&mut self.inner).poll_read(cx, buf)
    }
}

impl AsyncWrite for BlockedStream {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        if self.blocked.load(Ordering::SeqCst) {
            self.blocked_write.close();
            return Poll::Pending;
        }
        Pin::new(&mut self.inner).poll_write(cx, buf)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.inner).poll_shutdown(cx)
    }
}

#[tokio::test]
async fn abort_retires_driver_even_when_its_idle_timer_is_starved() {
    for iteration in 0..10 {
        let (client, server) = tokio::io::duplex(65_536);
        let mut peer = tokio::spawn(async move {
            let mut connection = ConnectionAcceptor::builder()
                .container_id("blocked-peer")
                .idle_time_out(20_u32)
                .build()
                .accept(server)
                .await
                .unwrap();
            let _ = connection.on_close().await;
        });
        let blocked = Arc::new(AtomicBool::new(false));
        let blocked_write = Arc::new(Closed::default());
        let dropped = Arc::new(AtomicUsize::new(0));
        let (transport, stream) = Transport::new(BlockedStream {
            inner: client,
            blocked: blocked.clone(),
            blocked_write: blocked_write.clone(),
            dropped: dropped.clone(),
        });
        let mut connection = Connection::builder()
            .container_id(format!("blocked-{iteration}"))
            .idle_time_out(100_u32)
            .open_with_stream(stream)
            .await
            .unwrap();
        blocked.store(true, Ordering::SeqCst);
        timeout(Duration::from_secs(1), blocked_write.wait())
            .await
            .unwrap();
        assert!(
            timeout(Duration::from_millis(150), connection.on_close())
                .await
                .is_err(),
            "both directions are blocked, starving the dependency's idle detector"
        );
        transport.abort();
        assert_eq!(
            dropped.load(Ordering::SeqCst),
            1,
            "abort synchronously drops the underlying I/O"
        );
        timeout(Duration::from_secs(1), connection.on_close())
            .await
            .expect("the AMQP driver must exit")
            .unwrap_err();
        timeout(Duration::from_secs(1), &mut peer)
            .await
            .unwrap()
            .unwrap();
        assert!(connection.is_closed());
        // All ten iterations retire their own driver and I/O before the next starts.
    }
}

#[tokio::test]
async fn abort_wakes_both_io_halves_and_late_waiters() {
    let (client, mut peer) = tokio::io::duplex(1);
    let (transport, stream) = Transport::new(client);
    let (mut reader, mut writer) = tokio::io::split(stream);
    writer.write_all(&[1]).await.unwrap();
    let read = tokio::spawn(async move { reader.read_u8().await });
    let write = tokio::spawn(async move { writer.write_all(&[2]).await });
    tokio::task::yield_now().await;
    transport.abort();
    assert_eq!(
        read.await.unwrap().unwrap_err().kind(),
        io::ErrorKind::ConnectionAborted
    );
    assert_eq!(
        write.await.unwrap().unwrap_err().kind(),
        io::ErrorKind::ConnectionAborted
    );
    timeout(Duration::from_secs(1), transport.closed.wait())
        .await
        .unwrap();
    assert_eq!(peer.read_u8().await.unwrap(), 1);
    assert_eq!(
        peer.read_u8().await.unwrap_err().kind(),
        io::ErrorKind::UnexpectedEof
    );
}

#[tokio::test]
async fn cancelled_preparation_aborts_only_its_captured_transport() {
    let (old, _) = tokio::io::duplex(1);
    let (replacement, _) = tokio::io::duplex(1);
    let (old, _old_stream) = Transport::new(old);
    let (replacement, _replacement_stream) = Transport::new(replacement);
    let preparation = async {
        let _guard = old.guard();
        pending::<()>().await;
    };
    let mut preparation = Box::pin(preparation);
    poll_fn(|cx| {
        assert!(preparation.as_mut().poll(cx).is_pending());
        Poll::Ready(())
    })
    .await;
    drop(preparation);
    assert!(old.closed.is_closed());
    assert!(!replacement.closed.is_closed());
}
