// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Semantic-convention attribute names used on Cosmos DB spans.
//!
//! These centralize the OpenTelemetry attribute-name string literals in one
//! place (the analogous constants in `azure_core` are module-private). Names
//! follow the OpenTelemetry database semantic conventions, with Cosmos-specific
//! attributes under the `azure.cosmosdb.*` namespace.

/// `db.system.name` — identifies the database system. Always
/// [`DB_SYSTEM_NAME_VALUE`] for Cosmos DB.
pub(crate) const DB_SYSTEM_NAME: &str = "db.system.name";

/// The stable `db.system.name` value for Azure Cosmos DB.
pub(crate) const DB_SYSTEM_NAME_VALUE: &str = "azure.cosmosdb";

/// `db.operation.name` — the canonical operation name (e.g. `read_item`).
pub(crate) const DB_OPERATION_NAME: &str = "db.operation.name";

/// `db.response.status_code` — the HTTP status code of the response, as a string.
pub(crate) const DB_RESPONSE_STATUS_CODE: &str = "db.response.status_code";

/// `server.address` — the host contacted for the request.
pub(crate) const SERVER_ADDRESS: &str = "server.address";

/// `error.type` — a low-cardinality identifier of the error (the status code).
pub(crate) const ERROR_TYPE: &str = "error.type";

/// `azure.cosmosdb.operation.request_charge` — Request Units consumed.
pub(crate) const AZURE_COSMOSDB_REQUEST_CHARGE: &str = "azure.cosmosdb.operation.request_charge";

/// `azure.cosmosdb.operation.contacted_regions` — regions contacted, joined.
pub(crate) const AZURE_COSMOSDB_CONTACTED_REGIONS: &str =
    "azure.cosmosdb.operation.contacted_regions";

/// `azure.cosmosdb.response.sub_status_code` — the Cosmos sub-status code.
pub(crate) const AZURE_COSMOSDB_SUB_STATUS_CODE: &str = "azure.cosmosdb.response.sub_status_code";

/// `azure.cosmosdb.request.activity_id` — the per-attempt activity id.
pub(crate) const AZURE_COSMOSDB_ACTIVITY_ID: &str = "azure.cosmosdb.request.activity_id";

/// `azure.cosmosdb.machine_id` — the client/machine instance identifier.
pub(crate) const AZURE_COSMOSDB_MACHINE_ID: &str = "azure.cosmosdb.machine_id";
