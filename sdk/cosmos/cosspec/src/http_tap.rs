// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! `HttpTap`: wire-level evidence of what the SDK actually sent.
//!
//! Diagnostics report what the SDK *believes* happened. A tap records every HTTP
//! exchange as it crosses the transport boundary, with real start and completion
//! instants, so specs can prove properties such as "the hedge request was in flight
//! at the same time as the primary".
//!
//! This module depends only on `std` and the driver's `transport_observer` hook so
//! it can move into its own crate later.

use azure_data_cosmos_driver::transport_observer::{
    HttpExchange, HttpOutcome, HttpOutcomeKind, TransportObserver,
};
use bytes::Bytes;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// One recorded HTTP exchange.
#[derive(Clone, Debug)]
pub struct HttpRecord {
    /// Order in which the exchange started, unique per client.
    pub sequence: u64,
    /// HTTP method, for example `GET`.
    pub method: String,
    /// Full request URL.
    pub url: String,
    /// Logical operation, for example `ReadItem`, when the SDK tagged one.
    pub operation: Option<String>,
    /// Target region name, for example `East US`, when the SDK tagged one.
    pub region: Option<String>,
    /// The request's activity ID, when present.
    pub activity_id: Option<String>,
    /// The label of the observer that recorded the exchange, from
    /// [`HttpTap::labeled_observer`].
    pub label: Option<String>,
    /// When the request was handed to the transport.
    pub started_at: Instant,
    /// When the exchange finished. `None` while still in flight.
    pub completed_at: Option<Instant>,
    /// How the exchange finished. `None` while still in flight.
    pub outcome: Option<HttpOutcomeKind>,
    /// The request body, when the request had one.
    pub request_body: Option<Bytes>,
    /// The response body, when a response arrived.
    pub response_body: Option<Bytes>,
}

impl HttpRecord {
    /// Whether this exchange and `other` were in flight at the same time.
    ///
    /// An exchange still in flight is treated as open-ended.
    pub fn overlaps(&self, other: &HttpRecord) -> bool {
        let end = |r: &HttpRecord| r.completed_at.unwrap_or_else(Instant::now);
        self.started_at < end(other) && other.started_at < end(self)
    }

    /// The exchange's duration, if it finished.
    pub fn duration(&self) -> Option<Duration> {
        self.completed_at.map(|end| end - self.started_at)
    }

    /// The HTTP status code, if a response arrived.
    pub fn status(&self) -> Option<u16> {
        match self.outcome {
            Some(HttpOutcomeKind::Response { status, .. }) => Some(status),
            _ => None,
        }
    }

    /// Whether the caller dropped the request before it finished.
    pub fn was_cancelled(&self) -> bool {
        matches!(self.outcome, Some(HttpOutcomeKind::Cancelled))
    }

    /// Whether the record is for `operation`, for example `ReadItem`.
    pub fn is(&self, operation: &str) -> bool {
        self.operation.as_deref() == Some(operation)
    }
}

/// Records every HTTP exchange of the clients it is attached to.
///
/// Cloning a tap is cheap; clones share the same recording.
#[derive(Clone, Debug, Default)]
pub struct HttpTap {
    records: Arc<Mutex<Vec<HttpRecord>>>,
}

impl HttpTap {
    /// Creates an empty tap.
    pub fn new() -> Self {
        Self::default()
    }

    /// Every exchange recorded so far, in start order.
    pub fn records(&self) -> Vec<HttpRecord> {
        let mut records = self.records.lock().unwrap().clone();
        records.sort_by_key(|r| r.started_at);
        records
    }

    /// The exchanges for one operation, for example `ReadItem`, in start order.
    pub fn operation(&self, operation: &str) -> Vec<HttpRecord> {
        self.records()
            .into_iter()
            .filter(|r| r.is(operation))
            .collect()
    }

    /// Forgets every recorded exchange, for example after setup traffic.
    pub fn clear(&self) {
        self.records.lock().unwrap().clear();
    }

    /// Wraps the tap as a transport observer for a client builder.
    pub fn observer(&self) -> Arc<dyn TransportObserver> {
        Arc::new(self.clone())
    }

    /// Wraps the tap as a transport observer that stamps every record with
    /// `label`, so one tap can tell several clients apart.
    pub fn labeled_observer(&self, label: impl Into<String>) -> Arc<dyn TransportObserver> {
        Arc::new(Labeled {
            tap: self.clone(),
            label: label.into(),
        })
    }

    fn start(&self, exchange: &HttpExchange, label: Option<&str>) {
        self.records.lock().unwrap().push(HttpRecord {
            sequence: exchange.sequence(),
            method: exchange.method().to_string(),
            url: exchange.url().to_string(),
            operation: exchange.operation().map(str::to_owned),
            region: exchange.region().map(str::to_owned),
            activity_id: exchange.activity_id().map(str::to_owned),
            label: label.map(str::to_owned),
            started_at: exchange.started_at(),
            completed_at: None,
            outcome: None,
            request_body: exchange.body().cloned(),
            response_body: None,
        });
    }

    fn complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome) {
        let mut records = self.records.lock().unwrap();
        if let Some(record) = records
            .iter_mut()
            .rev()
            .find(|r| r.sequence == exchange.sequence() && r.started_at == exchange.started_at())
        {
            record.completed_at = Some(outcome.completed_at());
            record.outcome = Some(outcome.kind().clone());
            record.response_body = outcome.body().cloned();
        }
    }
}

impl TransportObserver for HttpTap {
    fn on_start(&self, exchange: &HttpExchange) {
        self.start(exchange, None);
    }

    fn on_complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome) {
        self.complete(exchange, outcome);
    }
}

#[derive(Debug)]
struct Labeled {
    tap: HttpTap,
    label: String,
}

impl TransportObserver for Labeled {
    fn on_start(&self, exchange: &HttpExchange) {
        self.tap.start(exchange, Some(&self.label));
    }

    fn on_complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome) {
        self.tap.complete(exchange, outcome);
    }
}

/// Combines observers so one client can report to several taps.
pub fn fan_out(observers: Vec<Arc<dyn TransportObserver>>) -> Arc<dyn TransportObserver> {
    Arc::new(FanOut(observers))
}

#[derive(Debug)]
struct FanOut(Vec<Arc<dyn TransportObserver>>);

impl TransportObserver for FanOut {
    fn on_start(&self, exchange: &HttpExchange) {
        for observer in &self.0 {
            observer.on_start(exchange);
        }
    }

    fn on_complete(&self, exchange: &HttpExchange, outcome: &HttpOutcome) {
        for observer in &self.0 {
            observer.on_complete(exchange, outcome);
        }
    }
}
