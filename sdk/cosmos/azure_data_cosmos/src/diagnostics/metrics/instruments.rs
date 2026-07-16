// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The OpenTelemetry instruments the metrics handler records into.
//!
//! Instruments are built once, from a single [`Meter`], when the handler is
//! constructed, and then cheaply shared (each instrument is internally
//! reference-counted). Building them eagerly keeps the per-operation hot path to
//! just `record`/`add` calls with no allocation of instrument state.

use opentelemetry::metrics::{Histogram, Meter};

use crate::diagnostics::metrics::attributes;

/// The full set of Cosmos metric instruments, created from one [`Meter`].
///
/// The stable operation-duration histogram is always recorded; the remaining
/// development instruments are recorded only when
/// [`MetricsOptions::development_metrics_enabled`](super::MetricsOptions::development_metrics_enabled)
/// is set. They are still created unconditionally because instrument creation is
/// cheap and idempotent, and doing so keeps the handler's record path branch-free
/// per instrument.
#[derive(Clone)]
pub(crate) struct Instruments {
    /// Stable: `db.client.operation.duration` (seconds).
    pub(crate) operation_duration: Histogram<f64>,

    /// Development: `azure.cosmosdb.client.operation.request_charge` (RU).
    pub(crate) request_charge: Histogram<f64>,

    /// Development: `db.client.response.returned_rows` (rows).
    pub(crate) returned_rows: Histogram<u64>,
}

impl Instruments {
    /// Builds every Cosmos instrument from `meter`.
    pub(crate) fn new(meter: &Meter) -> Self {
        let operation_duration = meter
            .f64_histogram(attributes::METRIC_OPERATION_DURATION)
            .with_unit(attributes::UNIT_SECONDS)
            .with_description("Total client-observed duration of a Cosmos DB operation.")
            .build();

        let request_charge = meter
            .f64_histogram(attributes::METRIC_OPERATION_REQUEST_CHARGE)
            .with_unit(attributes::UNIT_REQUEST_UNIT)
            .with_description("Request charge (RU) consumed by a Cosmos DB operation.")
            .build();

        let returned_rows = meter
            .u64_histogram(attributes::METRIC_RESPONSE_RETURNED_ROWS)
            .with_unit(attributes::UNIT_ROW)
            .with_description("Number of rows/items returned by a Cosmos DB operation.")
            .build();

        Self {
            operation_duration,
            request_charge,
            returned_rows,
        }
    }
}
