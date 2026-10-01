// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    io,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll, Waker},
};
use tokio::{
    io::{AsyncRead, AsyncWrite, ReadBuf},
    sync::Notify,
};

/// Persistent notification shared by the transport and its sessions.
#[derive(Debug, Default)]
pub(crate) struct Closed {
    closed: AtomicBool,
    notify: Notify,
}

impl Closed {
    pub fn close(&self) {
        self.closed.store(true, Ordering::Release);
        self.notify.notify_waiters();
    }

    pub fn is_closed(&self) -> bool {
        self.closed.load(Ordering::Acquire)
    }

    pub async fn wait(&self) {
        let notified = self.notify.notified();
        tokio::pin!(notified);
        notified.as_mut().enable();
        if !self.is_closed() {
            notified.await;
        }
    }
}

#[derive(Debug)]
struct State<T> {
    stream: Option<T>,
    reader: Option<Waker>,
    writer: Option<Waker>,
}

/// Retains independent ownership of the socket while the AMQP engine polls it.
#[derive(Debug)]
pub(crate) struct Transport<T> {
    state: Arc<Mutex<State<T>>>,
    pub closed: Arc<Closed>,
}

impl<T> Clone for Transport<T> {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            closed: self.closed.clone(),
        }
    }
}

impl<T> Transport<T> {
    pub fn guard(&self) -> AbortOnDrop<T> {
        AbortOnDrop(Some(self.clone()))
    }

    pub fn new(stream: T) -> (Self, Stream<T>) {
        let transport = Self {
            state: Arc::new(Mutex::new(State {
                stream: Some(stream),
                reader: None,
                writer: None,
            })),
            closed: Arc::default(),
        };
        let stream = Stream {
            transport: transport.clone(),
        };
        (transport, stream)
    }

    pub fn abort(&self) {
        let (reader, writer) = {
            let mut state = self
                .state
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            state.stream.take();
            (state.reader.take(), state.writer.take())
        };
        self.closed.close();
        for waker in [reader, writer].into_iter().flatten() {
            waker.wake();
        }
    }
}

pub(crate) struct AbortOnDrop<T>(Option<Transport<T>>);

impl<T> AbortOnDrop<T> {
    pub fn disarm(mut self) {
        self.0.take();
    }
}

impl<T> Drop for AbortOnDrop<T> {
    fn drop(&mut self) {
        if let Some(transport) = self.0.take() {
            transport.abort();
        }
    }
}

#[derive(Debug)]
pub(crate) struct Stream<T> {
    transport: Transport<T>,
}

impl<T> Drop for Stream<T> {
    fn drop(&mut self) {
        self.transport.abort();
    }
}

fn aborted() -> io::Error {
    io::Error::new(
        io::ErrorKind::ConnectionAborted,
        "AMQP transport was retired",
    )
}

impl<T: AsyncRead + Unpin> AsyncRead for Stream<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        let mut state = self
            .transport
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.reader = Some(cx.waker().clone());
        match &mut state.stream {
            Some(stream) => Pin::new(stream).poll_read(cx, buf),
            None => Poll::Ready(Err(aborted())),
        }
    }
}

impl<T: AsyncWrite + Unpin> AsyncWrite for Stream<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        let mut state = self
            .transport
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.writer = Some(cx.waker().clone());
        match &mut state.stream {
            Some(stream) => Pin::new(stream).poll_write(cx, buf),
            None => Poll::Ready(Err(aborted())),
        }
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut state = self
            .transport
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.writer = Some(cx.waker().clone());
        match &mut state.stream {
            Some(stream) => Pin::new(stream).poll_flush(cx),
            None => Poll::Ready(Err(aborted())),
        }
    }

    fn poll_shutdown(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        let mut state = self
            .transport
            .state
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        state.writer = Some(cx.waker().clone());
        match &mut state.stream {
            Some(stream) => Pin::new(stream).poll_shutdown(cx),
            None => Poll::Ready(Err(aborted())),
        }
    }
}

#[cfg(test)]
mod tests;
