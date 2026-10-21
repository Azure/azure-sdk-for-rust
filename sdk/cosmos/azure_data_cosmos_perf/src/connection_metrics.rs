// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, MutexGuard,
    },
    time::{Duration, Instant},
};

use serde::Serialize;
use tracing::{field::Visit, span::Attributes, Event, Metadata, Subscriber};
use tracing_subscriber::{
    layer::{Context, Filter},
    registry::LookupSpan,
    Layer,
};

const H2_CONNECTION: &str = "h2::proto::connection";

#[derive(Debug, Default)]
struct Counters {
    opened: AtomicU64,
    goaway_received: AtomicU64,
    closed: AtomicU64,
    closed_after_goaway: AtomicU64,
    lifetimes: Mutex<LifetimeSnapshot>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize)]
pub(crate) struct LifetimeSnapshot {
    pub completed: u64,
    pub total_ms: f64,
    pub goaway_completed: u64,
    pub goaway_total_ms: f64,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct ConnectionMetrics(Arc<Counters>);

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Snapshot {
    pub opened: u64,
    pub goaway_received: u64,
    pub closed: u64,
    pub closed_after_goaway: u64,
    pub lifetimes: LifetimeSnapshot,
}

impl ConnectionMetrics {
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            opened: self.0.opened.load(Ordering::Relaxed),
            goaway_received: self.0.goaway_received.load(Ordering::Relaxed),
            closed: self.0.closed.load(Ordering::Relaxed),
            closed_after_goaway: self.0.closed_after_goaway.load(Ordering::Relaxed),
            lifetimes: self.lifetimes().clone(),
        }
    }

    fn lifetimes(&self) -> MutexGuard<'_, LifetimeSnapshot> {
        match self.0.lifetimes.lock() {
            Ok(guard) => guard,
            Err(_) => panic!("connection lifetime metrics lock poisoned"),
        }
    }

    fn record_lifetime(&self, duration: Duration, after_goaway: bool) {
        let milliseconds = duration.as_secs_f64() * 1000.0;
        let mut values = self.lifetimes();
        values.completed += 1;
        values.total_ms += milliseconds;
        if after_goaway {
            values.goaway_completed += 1;
            values.goaway_total_ms += milliseconds;
        }
    }
}

struct Connection {
    started: Instant,
    received_goaway: bool,
    closed: bool,
}

impl Default for Connection {
    fn default() -> Self {
        Self {
            started: Instant::now(),
            received_goaway: false,
            closed: false,
        }
    }
}

#[derive(Default)]
struct Fields {
    peer: String,
    message: String,
    state: String,
}

impl Visit for Fields {
    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        match field.name() {
            "peer" => self.peer = value.to_owned(),
            "message" => self.message = value.to_owned(),
            "connection.state" => self.state = value.to_owned(),
            _ => {}
        }
    }

    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn fmt::Debug) {
        // Frame/header fields are deliberately never formatted or retained.
        if matches!(field.name(), "peer" | "message" | "connection.state") {
            self.record_str(field, &format!("{value:?}"));
        }
    }
}

pub(crate) struct ConnectionFilter;

impl<S> Filter<S> for ConnectionFilter {
    fn enabled(&self, metadata: &Metadata<'_>, _: &Context<'_, S>) -> bool {
        metadata.target() == H2_CONNECTION
            && (metadata.is_span() && metadata.name() == "Connection"
                || metadata.is_event()
                    && (metadata.fields().field("connection.state").is_some()
                        || metadata.fields().field("frame").is_some()))
    }
}

impl<S> Layer<S> for ConnectionMetrics
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attributes: &Attributes<'_>, id: &tracing::Id, context: Context<'_, S>) {
        let mut fields = Fields::default();
        attributes.record(&mut fields);
        if fields.peer != "Client" {
            return;
        }
        if let Some(span) = context.span(id) {
            span.extensions_mut().insert(Connection::default());
            self.0.opened.fetch_add(1, Ordering::Relaxed);
        }
    }

    fn on_event(&self, event: &Event<'_>, context: Context<'_, S>) {
        let Some(scope) = context.event_scope(event) else {
            return;
        };
        let mut fields = Fields::default();
        event.record(&mut fields);
        for span in scope {
            let mut extensions = span.extensions_mut();
            let Some(connection) = extensions.get_mut::<Connection>() else {
                continue;
            };
            if fields.message == "recv GOAWAY" {
                connection.received_goaway = true;
                self.0.goaway_received.fetch_add(1, Ordering::Relaxed);
            }
            // h2 emits this state only after its connection shutdown completes.
            if fields.state.starts_with("Closed(") && !connection.closed {
                connection.closed = true;
                self.0.closed.fetch_add(1, Ordering::Relaxed);
                if connection.received_goaway {
                    self.0.closed_after_goaway.fetch_add(1, Ordering::Relaxed);
                }
            }
            break;
        }
    }

    fn on_close(&self, id: tracing::Id, context: Context<'_, S>) {
        if let Some(span) = context.span(&id) {
            if let Some(connection) = span.extensions().get::<Connection>() {
                self.record_lifetime(
                    connection.started.elapsed(),
                    connection.received_goaway && connection.closed,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ConnectionFilter, ConnectionMetrics, LifetimeSnapshot};
    use std::{fmt, time::Duration};
    use tracing::instrument::WithSubscriber;
    use tracing_subscriber::{layer::SubscriberExt, Layer};

    #[test]
    fn lifetime_totals_support_weighted_averages() {
        let metrics = ConnectionMetrics::default();
        metrics.record_lifetime(Duration::from_secs(1), false);
        metrics.record_lifetime(Duration::from_secs(3), true);
        assert_eq!(
            metrics.snapshot().lifetimes,
            LifetimeSnapshot {
                completed: 2,
                total_ms: 4000.0,
                goaway_completed: 1,
                goaway_total_ms: 3000.0,
            }
        );
    }

    #[test]
    fn open_connections_are_not_reported_as_completed() {
        let metrics = ConnectionMetrics::default();
        let subscriber =
            tracing_subscriber::registry().with(metrics.clone().with_filter(ConnectionFilter));
        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Client");
            assert_eq!(metrics.snapshot().lifetimes.completed, 0);
            let clone = span.clone();
            drop(span);
            assert_eq!(metrics.snapshot().lifetimes.completed, 0);
            drop(clone);
            assert_eq!(metrics.snapshot().lifetimes.completed, 1);
        });
    }

    #[test]
    fn receiving_goaway_alone_is_not_a_closed_connection() {
        let metrics = ConnectionMetrics::default();
        let subscriber =
            tracing_subscriber::registry().with(metrics.clone().with_filter(ConnectionFilter));
        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Client");
            let _entered = span.enter();
            tracing::trace!(target: "h2::proto::connection", frame = "ignored", "recv GOAWAY");
        });
        assert_eq!(
            (
                metrics.snapshot().goaway_received,
                metrics.snapshot().closed_after_goaway
            ),
            (1, 0)
        );
    }

    #[test]
    fn counts_one_close_for_multiple_goaway_frames() {
        let metrics = ConnectionMetrics::default();
        let subscriber =
            tracing_subscriber::registry().with(metrics.clone().with_filter(ConnectionFilter));
        tracing::subscriber::with_default(subscriber, || {
            let span = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Client");
            let _entered = span.enter();
            tracing::trace!(target: "h2::proto::connection", frame = "ignored", "recv GOAWAY");
            tracing::trace!(target: "h2::proto::connection", frame = "ignored", "recv GOAWAY");
            tracing::event!(target: "h2::proto::connection", tracing::Level::TRACE, { "connection.state" = "Closed(NO_ERROR, Library)" });
            tracing::event!(target: "h2::proto::connection", tracing::Level::TRACE, { "connection.state" = "Closed(NO_ERROR, Library)" });
        });
        let snapshot = metrics.snapshot();
        assert_eq!(
            (
                snapshot.opened,
                snapshot.goaway_received,
                snapshot.closed_after_goaway
            ),
            (1, 2, 1)
        );
    }

    #[test]
    fn ordinary_closes_count_without_goaway_and_server_connections_are_ignored() {
        let metrics = ConnectionMetrics::default();
        let subscriber =
            tracing_subscriber::registry().with(metrics.clone().with_filter(ConnectionFilter));
        tracing::subscriber::with_default(subscriber, || {
            {
                let span = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Client");
                let _entered = span.enter();
                tracing::event!(target: "h2::proto::connection", tracing::Level::TRACE, { "connection.state" = "Closed(NO_ERROR, Library)" });
            }
            let span = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Server");
            let _entered = span.enter();
            tracing::trace!(target: "h2::proto::connection", frame = "ignored", "recv GOAWAY");
            tracing::event!(target: "h2::proto::connection", tracing::Level::TRACE, { "connection.state" = "Closed(NO_ERROR, Library)" });
        });
        let snapshot = metrics.snapshot();
        assert_eq!(
            (
                snapshot.opened,
                snapshot.closed,
                snapshot.closed_after_goaway
            ),
            (1, 1, 0)
        );
    }

    #[test]
    fn goaway_is_scoped_to_its_connection_without_formatting_frames() {
        struct PrivateFrame;

        impl fmt::Debug for PrivateFrame {
            fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
                panic!("frame contents must not be formatted");
            }
        }

        let metrics = ConnectionMetrics::default();
        let subscriber =
            tracing_subscriber::registry().with(metrics.clone().with_filter(ConnectionFilter));
        tracing::subscriber::with_default(subscriber, || {
            let first = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Client");
            let second = tracing::debug_span!(target: "h2::proto::connection", "Connection", peer = "Client");
            first.in_scope(|| {
                tracing::trace!(target: "h2::proto::connection", frame = ?PrivateFrame, "recv GOAWAY");
            });
            second.in_scope(|| {
                tracing::event!(target: "h2::proto::connection", tracing::Level::TRACE, { "connection.state" = "Closed(NO_ERROR, Library)" });
            });
        });
        let snapshot = metrics.snapshot();
        assert_eq!(
            (
                snapshot.opened,
                snapshot.goaway_received,
                snapshot.closed,
                snapshot.closed_after_goaway,
                snapshot.lifetimes.completed,
                snapshot.lifetimes.goaway_completed,
            ),
            (2, 1, 1, 0, 2, 0)
        );
    }

    #[tokio::test]
    async fn observes_real_h2_peer_goaway_and_completed_shutdown() {
        let metrics = ConnectionMetrics::default();
        let observer = metrics.clone();
        let subscriber =
            tracing_subscriber::registry().with(metrics.clone().with_filter(ConnectionFilter));
        let test = async move {
            let (client_io, server_io) = tokio::io::duplex(65536);
            let server = tokio::spawn(
                async move {
                    let mut connection = h2::server::handshake(server_io).await.unwrap();
                    let (_, mut response) = connection.accept().await.unwrap().unwrap();
                    response
                        .send_response(http::Response::new(()), true)
                        .unwrap();
                    connection.graceful_shutdown();
                    if let Some(request) = connection.accept().await {
                        match request {
                            Err(error)
                                if error.get_io().is_some_and(|error| {
                                    error.kind() == std::io::ErrorKind::BrokenPipe
                                }) => {}
                            other => panic!("unexpected request after shutdown: {other:?}"),
                        }
                    }
                }
                .with_current_subscriber(),
            );
            let (mut client, connection) = h2::client::handshake(client_io).await.unwrap();
            let connection = tokio::spawn(connection.with_current_subscriber());
            let (response, _) = client.send_request(http::Request::new(()), true).unwrap();
            response.await.unwrap();
            while observer.snapshot().goaway_received == 0 {
                tokio::task::yield_now().await;
            }
            drop(client);
            connection.await.unwrap().unwrap();
            server.await.unwrap();
        }
        .with_subscriber(subscriber);
        tokio::time::timeout(Duration::from_secs(5), test)
            .await
            .unwrap();
        let snapshot = metrics.snapshot();
        assert_eq!(
            (
                snapshot.closed_after_goaway,
                snapshot.lifetimes.goaway_completed
            ),
            (1, 1)
        );
    }
}
