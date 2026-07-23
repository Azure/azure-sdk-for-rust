// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Centralized metric- and attribute-name string literals for Cosmos DB OTel
//! metrics.
//!
//! The OpenTelemetry semantic conventions for database clients define the exact
//! metric and attribute names emitted here. Metric names and instrument units are
//! metrics-specific and defined here; the shared **attribute** names are re-exported
//! from the crate-internal `diagnostics::attributes` module so metrics and tracing
//! can't drift.
//!
//! Two tiers, matching the semantic conventions:
//! - **Stable** names — emitted unconditionally (operation-scope, low cardinality).
//! - **Optional** names — emitted only when the matching
//!   [`MetricsOptions`](super::MetricsOptions) toggle is opted into.

use crate::diagnostics::attributes;

// =========================================================================
// Metric names
// =========================================================================

/// Stable histogram (seconds): total client-observed duration of an operation.
///
/// This is the primary Cosmos metric — the one to graph for latency SLOs.
pub const METRIC_OPERATION_DURATION: &str = "db.client.operation.duration";

/// Optional histogram (request units): request charge (RU) for an operation.
pub const METRIC_OPERATION_REQUEST_CHARGE: &str = "azure.cosmosdb.client.operation.request_charge";

/// Optional histogram (rows): number of rows/items returned by an operation.
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
//
// These alias the shared semconv literals in `crate::diagnostics::attributes`
// so the string lives in exactly one place.
// =========================================================================

/// `db.system.name` — identifies the database system.
pub const ATTR_DB_SYSTEM_NAME: &str = attributes::DB_SYSTEM_NAME;

/// Value for [`ATTR_DB_SYSTEM_NAME`] on every Cosmos metric.
pub const DB_SYSTEM_NAME_VALUE: &str = attributes::DB_SYSTEM_NAME_VALUE;

/// `db.operation.name` — canonical operation name (e.g. `read_item`).
pub const ATTR_DB_OPERATION_NAME: &str = attributes::DB_OPERATION_NAME;

/// `db.collection.name` — the container name.
pub const ATTR_DB_COLLECTION_NAME: &str = attributes::DB_COLLECTION_NAME;

/// `db.namespace` — the database name.
pub const ATTR_DB_NAMESPACE: &str = attributes::DB_NAMESPACE;

/// `db.response.status_code` — the HTTP status code of the response.
pub const ATTR_DB_RESPONSE_STATUS_CODE: &str = attributes::DB_RESPONSE_STATUS_CODE;

/// `error.type` — present only when the operation failed.
pub const ATTR_ERROR_TYPE: &str = attributes::ERROR_TYPE;

/// `server.address` — host of the contacted endpoint.
pub const ATTR_SERVER_ADDRESS: &str = attributes::SERVER_ADDRESS;

/// Fallback value for [`ATTR_ERROR_TYPE`] when the error is otherwise unknown
/// (per semantic conventions).
pub const ERROR_TYPE_OTHER: &str = attributes::ERROR_TYPE_OTHER;

// =========================================================================
// Extended attributes (opt-in; may be higher cardinality)
// =========================================================================

/// `azure.cosmosdb.consistency.level` — effective consistency level.
pub const ATTR_CONSISTENCY_LEVEL: &str = attributes::CONSISTENCY_LEVEL;

/// `azure.cosmosdb.operation.contacted_regions` — regions contacted.
pub const ATTR_CONTACTED_REGIONS: &str = attributes::CONTACTED_REGIONS;

/// `azure.cosmosdb.response.sub_status_code` — Cosmos sub-status code.
pub const ATTR_SUB_STATUS_CODE: &str = attributes::SUB_STATUS_CODE;

/// `azure.cosmosdb.connection.mode` — gateway vs. direct connection mode.
pub const ATTR_CONNECTION_MODE: &str = attributes::CONNECTION_MODE;
