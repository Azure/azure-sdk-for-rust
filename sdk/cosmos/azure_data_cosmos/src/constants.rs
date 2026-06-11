// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Constants defining HTTP headers and other values used internally by the SDK.

#[cfg(test)]
use azure_core::http::headers::HeaderName;

#[cfg(test)]
pub const OFFER_REPLACE_PENDING: HeaderName = HeaderName::from_static("x-ms-offer-replace-pending");
