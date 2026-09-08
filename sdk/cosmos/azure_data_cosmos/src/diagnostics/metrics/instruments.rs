// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The OpenTelemetry instruments the metrics handler records into.
//!
//! Instruments are built once, from a single [`Meter`], when the handler is
//! constructed, and then cheaply shared (each instrument is internally
//! reference-counted). Building them eagerly keeps the per-operation hot path to
//! just `record`/`add` calls with no allocation of instrument state.

use opentelemetry::metrics::{Counter, Histogram, Meter, UpDownCounter};

use crate::diagnostics::metrics::attributes;

/// The full set of Cosmos metric instruments, created from one [`Meter`].
///
/// The stable operation-duration histogram is always recorded; the remaining
/// per-signal instruments are recorded only when the matching
/// [`MetricsOptions`](super::MetricsOptions) toggle
/// (`request_charge_metric_enabled` / `returned_rows_metric_enabled` /
/// `active_instance_metric_enabled` / `hedged_metric_enabled`) is set. They are
/// still created unconditionally because instrument creation is cheap and
/// idempotent, and doing so keeps the handler's record path branch-free per
/// instrument.
#[derive(Clone)]
pub(crate) struct Instruments {
    /// Stable: `db.client.operation.duration` (seconds).
    pub(crate) operation_duration: Histogram<f64>,

    /// Development: `azure.cosmosdb.client.operation.request_charge` (RU).
    pub(crate) request_charge: Histogram<f64>,

    /// Development: `db.client.response.returned_rows` (rows).
    pub(crate) returned_rows: Histogram<u64>,

    /// Development: `azure.cosmosdb.client.active_instance.count` (instances).
    ///
    /// An up-down counter incremented when a
    /// [`CosmosClient`](crate::CosmosClient) is created with the handler
    /// registered, and decremented when that client is dropped, so the reported
    /// value tracks the number of live client instances per account endpoint.
    pub(crate) active_instance: UpDownCounter<i64>,

    /// Development: `azure.cosmosdb.client.operation.hedged` (operations that
    /// dispatched a cross-region hedge fan-out).
    pub(crate) hedged: Counter<u64>,
}

impl Instruments {
    /// Builds every Cosmos instrument from `meter`.
    pub(crate) fn new(meter: &Meter) -> Self {
        let operation_duration = meter
            .f64_histogram(attributes::METRIC_OPERATION_DURATION)
            .with_unit(attributes::UNIT_SECONDS)
            .with_description("Total client-observed duration of a Cosmos DB operation.")
            .with_boundaries(attributes::BUCKETS_OPERATION_DURATION_SECONDS.to_vec())
            .build();

        let request_charge = meter
            .f64_histogram(attributes::METRIC_OPERATION_REQUEST_CHARGE)
            .with_unit(attributes::UNIT_REQUEST_UNIT)
            .with_description("Request charge (RU) consumed by a Cosmos DB operation.")
            .with_boundaries(attributes::BUCKETS_REQUEST_CHARGE_RU.to_vec())
            .build();

        let returned_rows = meter
            .u64_histogram(attributes::METRIC_RESPONSE_RETURNED_ROWS)
            .with_unit(attributes::UNIT_ROW)
            .with_description("Number of rows/items returned by a Cosmos DB operation.")
            .with_boundaries(attributes::BUCKETS_RETURNED_ROWS.to_vec())
            .build();

        let active_instance = meter
            .i64_up_down_counter(attributes::METRIC_ACTIVE_INSTANCE_COUNT)
            .with_unit(attributes::UNIT_INSTANCE)
            .with_description("Number of active Cosmos DB client instances.")
            .build();

        let hedged = meter
            .u64_counter(attributes::METRIC_OPERATION_HEDGED)
            .with_unit(attributes::UNIT_OPERATION)
            .with_description("Cosmos DB operations that dispatched a cross-region hedge fan-out.")
            .build();

        Self {
            operation_duration,
            request_charge,
            returned_rows,
            active_instance,
            hedged,
        }
    }
}
