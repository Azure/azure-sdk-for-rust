// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Driver-level Cosmos DB constants.
//!
//! This module owns the canonical wire-name strings for the Gateway 2.0
//! HTTP/2 outer headers. The wire strings retain the historical
//! `x-ms-thinclient-*` form because the proxy is server-defined; only the
//! Rust identifier follows the `GATEWAY20_*` naming convention.
//!
//! `GATEWAY20_OPERATION_TYPE` and `GATEWAY20_RESOURCE_TYPE` are also
//! consumed cross-crate by `azure_data_cosmos` for its
//! `COSMOS_ALLOWED_HEADERS` logging allowlist — the public path
//! (`azure_data_cosmos_driver::constants::*`) is part of that contract.

use azure_core::http::headers::HeaderName;

/// Gateway 2.0 proxy operation-type header.
///
/// Contains the numeric operation type on every Gateway 2.0 request. Listed
/// in `azure_data_cosmos`'s `COSMOS_ALLOWED_HEADERS` allowlist so log
/// scrubbers do not redact it.
pub const GATEWAY20_OPERATION_TYPE: HeaderName =
    HeaderName::from_static("x-ms-thinclient-proxy-operation-type");

/// Gateway 2.0 proxy resource-type header.
///
/// Contains the numeric resource type on every Gateway 2.0 request. Listed
/// in `azure_data_cosmos`'s `COSMOS_ALLOWED_HEADERS` allowlist so log
/// scrubbers do not redact it.
pub const GATEWAY20_RESOURCE_TYPE: HeaderName =
    HeaderName::from_static("x-ms-thinclient-proxy-resource-type");

/// Lower bound of the EPK range.
///
/// Sent for feed and cross-partition operations only.
pub const GATEWAY20_RANGE_MIN: HeaderName = HeaderName::from_static("x-ms-thinclient-range-min");

/// Upper bound of the EPK range.
///
/// Sent for feed and cross-partition operations only.
pub const GATEWAY20_RANGE_MAX: HeaderName = HeaderName::from_static("x-ms-thinclient-range-max");

/// Gateway 2.0 endpoint-discovery opt-in header.
///
/// Sent on every `getDatabaseAccount` request so the server emits
/// `thinClientWritableLocations` / `thinClientReadableLocations` in the
/// metadata response. Without this header the server suppresses those
/// fields even when the federation has thin-client enabled, which
/// silently disables Gateway 2.0 routing for the client.
pub const GATEWAY20_DISCOVERY_OPT_IN: HeaderName =
    HeaderName::from_static("x-ms-cosmos-use-thinclient");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_expected_wire_strings() {
        let cases = [
            (
                GATEWAY20_OPERATION_TYPE,
                HeaderName::from_static("x-ms-thinclient-proxy-operation-type"),
            ),
            (
                GATEWAY20_RESOURCE_TYPE,
                HeaderName::from_static("x-ms-thinclient-proxy-resource-type"),
            ),
            (
                GATEWAY20_RANGE_MIN,
                HeaderName::from_static("x-ms-thinclient-range-min"),
            ),
            (
                GATEWAY20_RANGE_MAX,
                HeaderName::from_static("x-ms-thinclient-range-max"),
            ),
            (
                GATEWAY20_DISCOVERY_OPT_IN,
                HeaderName::from_static("x-ms-cosmos-use-thinclient"),
            ),
        ];

        for (actual, expected) in cases {
            assert_eq!(actual, expected);
        }
    }

    #[test]
    fn constants_have_distinct_wire_strings() {
        let constants = [
            ("GATEWAY20_OPERATION_TYPE", GATEWAY20_OPERATION_TYPE),
            ("GATEWAY20_RESOURCE_TYPE", GATEWAY20_RESOURCE_TYPE),
            ("GATEWAY20_RANGE_MIN", GATEWAY20_RANGE_MIN),
            ("GATEWAY20_RANGE_MAX", GATEWAY20_RANGE_MAX),
            ("GATEWAY20_DISCOVERY_OPT_IN", GATEWAY20_DISCOVERY_OPT_IN),
        ];

        for (index, (left_name, left_header)) in constants.iter().enumerate() {
            for (right_name, right_header) in constants.iter().skip(index + 1) {
                assert_ne!(
                    left_header, right_header,
                    "{left_name} and {right_name} must not share a wire string"
                );
            }
        }
    }
}
