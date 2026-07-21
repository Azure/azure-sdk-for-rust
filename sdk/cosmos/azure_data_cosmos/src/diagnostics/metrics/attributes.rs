// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Centralized metric- and attribute-name string literals for Cosmos DB OTel
//! metrics.
//!
//! The OpenTelemetry semantic conventions for database clients define the exact
//! metric and attribute names emitted here. Keeping them in one module gives the
//! Cosmos SDK a single source of truth: every name a handler emits lives here and
//! nowhere else, so a rename is a one-line change and reviewers can diff the wire
//! contract in one place.
//!
//! Two tiers, matching the semantic conventions:
//! - **Stable** names — emitted unconditionally (operation-scope, low cardinality).
//! - **Development** names — still-evolving; emitted only when explicitly opted
//!   in via [`MetricsOptions`](super::MetricsOptions).

// =========================================================================
// Metric names
// =========================================================================

/// Stable histogram (seconds): total client-observed duration of an operation.
///
/// This is the primary Cosmos metric — the one to graph for latency SLOs.
pub const METRIC_OPERATION_DURATION: &str = "db.client.operation.duration";

/// Development histogram (request units): request charge (RU) for an operation.
pub const METRIC_OPERATION_REQUEST_CHARGE: &str = "azure.cosmosdb.client.operation.request_charge";

/// Development histogram (rows): number of rows/items returned by an operation.
pub const METRIC_RESPONSE_RETURNED_ROWS: &str = "db.client.response.returned_rows";

// =========================================================================
// Instrument units
// =========================================================================
//
// Unit strings follow the Unified Code for Units of Measure (UCUM):
// <https://ucum.org/ucum>. OpenTelemetry adopts UCUM for instrument units; see
// <https://opentelemetry.io/docs/specs/semconv/general/metrics/#instrument-units>.

/// Unit for [`METRIC_OPERATION_DURATION`] — seconds.
pub const UNIT_SECONDS: &str = "s";

/// Unit for [`METRIC_OPERATION_REQUEST_CHARGE`] — Cosmos request units.
pub const UNIT_REQUEST_UNIT: &str = "{request_unit}";

/// Unit for [`METRIC_RESPONSE_RETURNED_ROWS`] — rows.
pub const UNIT_ROW: &str = "{row}";

// =========================================================================
// Stable attributes (always emitted; operation scope, low cardinality)
// =========================================================================

/// `db.system.name` — identifies the database system.
pub const ATTR_DB_SYSTEM_NAME: &str = "db.system.name";

/// Value for [`ATTR_DB_SYSTEM_NAME`] on every Cosmos metric.
pub const DB_SYSTEM_NAME_VALUE: &str = "azure.cosmosdb";

/// `db.operation.name` — canonical operation name (e.g. `read_item`).
pub const ATTR_DB_OPERATION_NAME: &str = "db.operation.name";

/// `db.collection.name` — the container name.
pub const ATTR_DB_COLLECTION_NAME: &str = "db.collection.name";

/// `db.namespace` — the database name.
pub const ATTR_DB_NAMESPACE: &str = "db.namespace";

/// `db.response.status_code` — the HTTP status code of the response.
pub const ATTR_DB_RESPONSE_STATUS_CODE: &str = "db.response.status_code";

/// `error.type` — present only when the operation failed.
pub const ATTR_ERROR_TYPE: &str = "error.type";

/// `server.address` — host of the contacted endpoint.
pub const ATTR_SERVER_ADDRESS: &str = "server.address";

/// Fallback value for [`ATTR_ERROR_TYPE`] when the error is otherwise unknown
/// (per semantic conventions).
pub const ERROR_TYPE_OTHER: &str = "_OTHER";

// =========================================================================
// Development attributes (opt-in; may be higher cardinality)
// =========================================================================

/// `azure.cosmosdb.consistency.level` — effective consistency level.
pub const ATTR_CONSISTENCY_LEVEL: &str = "azure.cosmosdb.consistency.level";

/// `azure.cosmosdb.operation.contacted_regions` — regions contacted.
pub const ATTR_CONTACTED_REGIONS: &str = "azure.cosmosdb.operation.contacted_regions";

/// `azure.cosmosdb.response.sub_status_code` — Cosmos sub-status code.
pub const ATTR_SUB_STATUS_CODE: &str = "azure.cosmosdb.response.sub_status_code";

/// `azure.cosmosdb.connection.mode` — gateway vs. direct connection mode.
pub const ATTR_CONNECTION_MODE: &str = "azure.cosmosdb.connection.mode";
