// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Shared OpenTelemetry semantic-convention attribute-name literals.
//!
//! Both the metrics and the distributed-tracing handlers emit the same
//! database-client semantic-convention attributes. Keeping the string literals in
//! one place gives the SDK a single source of truth — a rename is a one-line
//! change, and the two handlers can never drift.
//!
//! Some names are used by only one handler, so under a single feature a few of
//! these are unused; the module-level `allow(dead_code)` keeps that from being a
//! warning rather than forcing per-const gating.

#![allow(dead_code)]

/// `db.system.name` — identifies the database system. Always
/// [`DB_SYSTEM_NAME_VALUE`] for Cosmos DB.
pub(crate) const DB_SYSTEM_NAME: &str = "db.system.name";

/// The stable `db.system.name` value for Azure Cosmos DB.
pub(crate) const DB_SYSTEM_NAME_VALUE: &str = "azure.cosmosdb";

/// `db.operation.name` — the canonical operation name (e.g. `read_item`).
pub(crate) const DB_OPERATION_NAME: &str = "db.operation.name";

/// `db.namespace` — the database name.
pub(crate) const DB_NAMESPACE: &str = "db.namespace";

/// `db.collection.name` — the container name.
pub(crate) const DB_COLLECTION_NAME: &str = "db.collection.name";

/// `db.response.status_code` — the HTTP status code of the response, as a string.
pub(crate) const DB_RESPONSE_STATUS_CODE: &str = "db.response.status_code";

/// `server.address` — the host contacted for the request.
pub(crate) const SERVER_ADDRESS: &str = "server.address";

/// `error.type` — a low-cardinality identifier of the error (the status code).
pub(crate) const ERROR_TYPE: &str = "error.type";

/// Fallback value for [`ERROR_TYPE`] when a failure carries no status anywhere
/// (per semantic conventions).
pub(crate) const ERROR_TYPE_OTHER: &str = "_OTHER";

/// `azure.cosmosdb.consistency.level` — effective consistency level.
pub(crate) const CONSISTENCY_LEVEL: &str = "azure.cosmosdb.consistency.level";

/// `azure.cosmosdb.connection.mode` — gateway vs. direct connection mode.
pub(crate) const CONNECTION_MODE: &str = "azure.cosmosdb.connection.mode";

/// `azure.cosmosdb.operation.contacted_regions` — regions contacted (ordered `string[]`).
pub(crate) const CONTACTED_REGIONS: &str = "azure.cosmosdb.operation.contacted_regions";

/// `azure.cosmosdb.response.sub_status_code` — the Cosmos sub-status code.
pub(crate) const SUB_STATUS_CODE: &str = "azure.cosmosdb.response.sub_status_code";

/// `azure.cosmosdb.operation.request_charge` — Request Units consumed (span attribute).
pub(crate) const REQUEST_CHARGE: &str = "azure.cosmosdb.operation.request_charge";

/// `azure.cosmosdb.request.activity_id` — the per-attempt activity id.
pub(crate) const ACTIVITY_ID: &str = "azure.cosmosdb.request.activity_id";

/// `azure.cosmosdb.machine_id` — the client/machine instance identifier.
pub(crate) const MACHINE_ID: &str = "azure.cosmosdb.machine_id";

/// `azure.cosmosdb.sampling.reason` — why the operation was tail-sampled for
/// emission (a failure, or which threshold it crossed).
pub(crate) const SAMPLING_REASON: &str = "azure.cosmosdb.sampling.reason";
