use std::sync::Arc;

use fe2o3_amqp::{session::{SessionHandle, BeginError}, Session};

use crate::util::sharable::Sharable;

use super::CONNECTION_IDENTIFIER;

/// A wrapper around the AMQP connection handle that also tracks the identifier for the connection.
#[derive(Debug)]
pub(crate) struct AmqpConnection {
    pub identifier: u32,
    pub handle: fe2o3_amqp::connection::ConnectionHandle<()>,
}

impl AmqpConnection {
    pub(crate) fn new(handle: fe2o3_amqp::connection::ConnectionHandle<()>) -> Self {
        Self {
            identifier: CONNECTION_IDENTIFIER.fetch_add(1, std::sync::atomic::Ordering::Relaxed),
            handle,
        }
    }
}

impl Sharable<AmqpConnection> {
    pub(crate) async fn is_closed(&self) -> bool {
        match self {
            Self::Owned(connection) => connection.handle.is_closed(),
            Self::Shared(connection) => connection.read().await.handle.is_closed(),
            Self::None => unreachable!(),
        }
    }

    // Close regardless of ownership
    pub(crate) async fn close(&mut self) -> Result<(), fe2o3_amqp::connection::Error> {
        match self {
            Self::Owned(connection) => connection.handle.close().await,
            Self::Shared(connection) => {
                let mut connection = connection.write().await;
                connection.handle.close().await
            },
            Self::None => Ok(())
        }
    }

    pub(crate) async fn close_if_owned(&mut self) -> Result<(), fe2o3_amqp::connection::Error> {
        match self {
            Self::Owned(connection) => connection.handle.close().await,
            Self::Shared(connection) => match Arc::strong_count(connection) {
                1 => connection.write().await.handle.close().await,
                _ => Ok(()),
            },
            Self::None => Ok(())
        }
    }

    pub(crate) async fn begin_session(&mut self) -> Result<SessionHandle<()>, BeginError> {
        match self {
            Sharable::Owned(c) => Session::begin(&mut c.handle).await,
            Sharable::Shared(lock) => {
                let mut guard = lock.write().await;
                Session::begin(&mut guard.handle).await
            }
            Sharable::None => unreachable!(),
        }
    }
}

