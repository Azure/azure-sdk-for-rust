// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Constants defining HTTP headers and other values used internally by the SDK.

use azure_core::http::headers::HeaderName;

pub const OFFER_THROUGHPUT: HeaderName = HeaderName::from_static("x-ms-offer-throughput");
pub const OFFER_AUTOPILOT_SETTINGS: HeaderName =
    HeaderName::from_static("x-ms-cosmos-offer-autopilot-settings");

#[cfg(test)]
pub const OFFER_REPLACE_PENDING: HeaderName = HeaderName::from_static("x-ms-offer-replace-pending");

// cSpell:enable

// -----------------------------------------------------------------------
// Environment-variable names
// -----------------------------------------------------------------------

/// Controls whether the per-partition circuit breaker is enabled.
///
/// Expected values: `"true"` or `"false"`. Defaults to `true` when unset.
pub const AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED: &str =
    "AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED";
