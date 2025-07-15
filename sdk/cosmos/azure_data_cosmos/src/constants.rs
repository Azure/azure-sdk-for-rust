// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:ignoreRegExp /x-[^\s]+/

//! Constants defining HTTP headers and other values relevant to Azure Cosmos DB APIs.

use azure_core::http::{
    headers::{HeaderName, HeaderValue},
    request::options::ContentType,
};

pub const QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-query");
pub const PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");
pub const PARTITION_KEY_RANGE_ID: HeaderName =
    HeaderName::from_static("x-ms-documentdb-partitionkeyrangeid");
pub const QUERY_ENABLE_CROSS_PARTITION: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-enablecrosspartition");
pub const IS_QUERY_PLAN_REQUEST: HeaderName =
    HeaderName::from_static("x-ms-cosmos-is-query-plan-request");
pub const SUPPORTED_QUERY_FEATURES: HeaderName =
    HeaderName::from_static("x-ms-cosmos-supported-query-features");
pub const CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
pub const INDEX_METRICS: HeaderName = HeaderName::from_static("x-ms-cosmos-index-utilization");
pub const QUERY_METRICS: HeaderName = HeaderName::from_static("x-ms-documentdb-query-metrics");
pub const IS_UPSERT: HeaderName = HeaderName::from_static("x-ms-documentdb-is-upsert");
pub const OFFER_THROUGHPUT: HeaderName = HeaderName::from_static("x-ms-offer-throughput");
pub const OFFER_AUTOPILOT_SETTINGS: HeaderName =
    HeaderName::from_static("x-ms-cosmos-offer-autopilot-settings");
pub const CONSISTENCY_LEVEL: HeaderName = HeaderName::from_static("x-ms-consistency-level");
pub const PRE_TRIGGER_INCLUDE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-pre-trigger-include");
pub const POST_TRIGGER_INCLUDE: HeaderName =
    HeaderName::from_static("x-ms-documentdb-post-trigger-include");
pub const SESSION_TOKEN: HeaderName = HeaderName::from_static("x-ms-session-token");
pub const INDEXING_DIRECTIVE: HeaderName = HeaderName::from_static("x-ms-indexing-directive");

pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");

pub(crate) const PREFER_MINIMAL: HeaderValue = HeaderValue::from_static("return=minimal");
