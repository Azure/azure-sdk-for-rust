// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:ignoreRegExp /x-[^\s]+/

use azure_core::{headers::HeaderName, request_options::ContentType};

pub const QUERY: HeaderName = HeaderName::from_static("x-ms-documentdb-query");
pub const PARTITION_KEY: HeaderName = HeaderName::from_static("x-ms-documentdb-partitionkey");
pub const CONTINUATION: HeaderName = HeaderName::from_static("x-ms-continuation");
pub const INDEX_METRICS: HeaderName = HeaderName::from_static("x-ms-cosmos-index-utilization");
pub const QUERY_METRICS: HeaderName = HeaderName::from_static("x-ms-documentdb-query-metrics");

pub const QUERY_CONTENT_TYPE: ContentType = ContentType::from_static("application/query+json");
