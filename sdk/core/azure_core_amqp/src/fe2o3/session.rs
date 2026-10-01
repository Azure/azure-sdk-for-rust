// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use crate::{
    connection::AmqpConnection,
    error::{AmqpDescribedError, AmqpErrorKind, Result},
    fe2o3::transport::{AbortOnDrop, Closed, Transport},
    session::{AmqpSessionApis, AmqpSessionOptions},
    AmqpError,
};
use std::{
    borrow::BorrowMut,
    future::{poll_fn, Future},
    io,
    pin::pin,
    sync::{Arc, OnceLock},
};
use tokio::{net::TcpStream, sync::Mutex};
use tracing::{debug, trace};

#[derive(Debug, Clone, Default)]
pub(crate) struct Fe2o3AmqpSession {
    session: OnceLock<Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>>,
    monitor: OnceLock<Arc<SessionMonitor>>,
}

#[derive(Debug, Clone)]
pub(crate) struct SessionClosed {
    connection: Arc<Closed>,
    session: Arc<Closed>,
    error: Arc<OnceLock<AmqpDescribedError>>,
    transport: Transport<TcpStream>,
}

impl SessionClosed {
    pub fn guard(&self) -> AbortOnDrop<TcpStream> {
        self.transport.guard()
    }

    pub async fn run<T>(&self, operation: impl Future<Output = Result<T>>) -> Result<T> {
        tokio::select! {
            biased;
            result = operation => result,
            _ = self.connection.wait() => Err(AmqpErrorKind::ConnectionDropped(Box::new(
                io::Error::new(io::ErrorKind::ConnectionAborted, "AMQP connection closed"),
            )).into()),
            _ = self.session.wait() => Err(match self.error.get() {
                Some(error) => AmqpErrorKind::AmqpDescribedError(error.clone()).into(),
                None => AmqpErrorKind::SessionClosedByRemote(Box::new(
                    io::Error::new(io::ErrorKind::ConnectionAborted, "AMQP session ended"),
                )).into(),
            }),
        }
    }
}

#[derive(Debug)]
struct SessionMonitor {
    closed: SessionClosed,
    task: tokio::task::JoinHandle<()>,
}

impl Drop for SessionMonitor {
    fn drop(&mut self) {
        self.task.abort();
    }
}

impl Drop for Fe2o3AmqpSession {
    fn drop(&mut self) {
        debug!("Dropping Fe2o3AmqpSession: {:?}.", self.session);
    }
}

impl Fe2o3AmqpSession {
    pub fn new() -> Self {
        Self {
            session: OnceLock::new(),
            monitor: OnceLock::new(),
        }
    }

    /// Returns a reference to the session handle
    pub fn get(&self) -> Result<Arc<Mutex<fe2o3_amqp::session::SessionHandle<()>>>> {
        Ok(self
            .session
            .get()
            .ok_or_else(Self::session_not_set)?
            .clone())
    }

    pub fn closed(&self) -> Result<SessionClosed> {
        self.monitor
            .get()
            .map(|monitor| monitor.closed.clone())
            .ok_or_else(Self::session_not_set)
    }

    fn session_already_attached() -> AmqpError {
        AmqpError::with_message("AMQP Session is already attached")
    }
    fn session_not_set() -> AmqpError {
        AmqpError::with_message("AMQP Session is not set")
    }
    fn could_not_set_session() -> AmqpError {
        AmqpError::with_message("Could not set AMQP Session")
    }
}

#[async_trait::async_trait]
impl AmqpSessionApis for Fe2o3AmqpSession {
    async fn begin(
        &self,
        connection: &AmqpConnection,
        options: Option<AmqpSessionOptions>,
    ) -> Result<()> {
        let transport = connection.implementation.transport()?;
        let connection_closed = connection.implementation.closed()?;
        let mut connection = connection
            .implementation
            .get()
            .get()
            .ok_or_else(Self::session_already_attached)?
            .lock()
            .await;

        let mut session_builder = fe2o3_amqp::session::Session::builder();

        if let Some(options) = options {
            if let Some(incoming_window) = options.incoming_window {
                session_builder = session_builder.incoming_window(incoming_window);
            }
            if let Some(outgoing_window) = options.outgoing_window {
                session_builder = session_builder.outgoing_window(outgoing_window);
            }
            if let Some(handle_max) = options.handle_max {
                session_builder = session_builder.handle_max(handle_max);
            }
            if let Some(offered_capabilities) = options.offered_capabilities {
                session_builder = session_builder.set_offered_capabilities(
                    offered_capabilities.iter().map(Into::into).collect(),
                );
            }
            if let Some(desired_capabilities) = options.desired_capabilities {
                session_builder = session_builder.set_desired_capabilities(
                    desired_capabilities.iter().map(Into::into).collect(),
                );
            }
            if let Some(properties) = options.properties {
                session_builder = session_builder.properties(
                    properties
                        .iter()
                        .map(|(k, v)| (k.into(), v.into()))
                        .collect(),
                );
            }
            if let Some(buffer_size) = options.buffer_size {
                session_builder = session_builder.buffer_size(buffer_size);
            }
        }
        let guard = transport.guard();
        let session = session_builder
            .begin(connection.borrow_mut())
            .await
            .map_err(AmqpError::from);
        guard.disarm();
        let session = session?;
        let session = Arc::new(Mutex::new(session));
        self.session
            .set(session.clone())
            .map_err(|_| Self::could_not_set_session())?;
        let closed = SessionClosed {
            connection: connection_closed,
            session: Arc::default(),
            error: Arc::default(),
            transport,
        };
        let notification = closed.session.clone();
        let error = closed.error.clone();
        let task = tokio::spawn(async move {
            let mut lock = Box::pin(session.clone().lock_owned());
            // Register the outcome waker, then release the handle between polls.
            // Keeping the mutex guard while waiting would prevent link attachment.
            let result = poll_fn(|cx| {
                let mut handle = std::task::ready!(lock.as_mut().poll(cx));
                let result = pin!(handle.on_end()).poll(cx);
                drop(handle);
                lock = Box::pin(session.clone().lock_owned());
                result
            })
            .await;
            if let Err(fe2o3_amqp::session::Error::RemoteEndedWithError(reason)) = result {
                // Preserve terminal protocol conditions for every waiter.
                let _ = error.set(reason.into());
            }
            notification.close();
        });
        self.monitor
            .set(Arc::new(SessionMonitor { closed, task }))
            .map_err(|_| Self::could_not_set_session())?;
        Ok(())
    }

    async fn end(&self) -> Result<()> {
        let mut session = self
            .session
            .get()
            .ok_or_else(Self::session_not_set)?
            .lock()
            .await;
        if session.is_ended() {
            trace!("Session already ended, returning.");
            return Ok(());
        }
        let result = session.end().await.map_err(AmqpError::from);
        self.closed()?.session.close();
        result
    }
}

impl From<fe2o3_amqp::session::BeginError> for AmqpError {
    fn from(e: fe2o3_amqp::session::BeginError) -> Self {
        match e {
            fe2o3_amqp::session::BeginError::IllegalState
            | fe2o3_amqp::session::BeginError::IllegalConnectionState => {
                AmqpErrorKind::ConnectionDropped(Box::new(e)).into()
            }
            fe2o3_amqp::session::BeginError::RemoteEnded => {
                AmqpErrorKind::SessionClosedByRemote(Box::new(e)).into()
            }
            fe2o3_amqp::session::BeginError::RemoteEndedWithError(error) => {
                AmqpErrorKind::AmqpDescribedError(error.into()).into()
            }
            fe2o3_amqp::session::BeginError::LocalChannelMaxReached => {
                AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
            }
        }
    }
}

impl From<fe2o3_amqp::session::Error> for AmqpError {
    fn from(e: fe2o3_amqp::session::Error) -> Self {
        match e {
            fe2o3_amqp::session::Error::UnattachedHandle
            | fe2o3_amqp::session::Error::RemoteAttachingLinkNameNotFound
            | fe2o3_amqp::session::Error::HandleInUse
            | fe2o3_amqp::session::Error::IllegalState
            | fe2o3_amqp::session::Error::IllegalConnectionState
            | fe2o3_amqp::session::Error::TransferFrameToSender => {
                AmqpErrorKind::TransportImplementationError(Box::new(e)).into()
            }
            fe2o3_amqp::session::Error::RemoteEnded => {
                AmqpErrorKind::SessionClosedByRemote(Box::new(e)).into()
            }
            fe2o3_amqp::session::Error::RemoteEndedWithError(error) => {
                AmqpErrorKind::AmqpDescribedError(error.into()).into()
            }
        }
    }
}
