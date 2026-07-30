// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Constants defining HTTP headers and other values used internally by the SDK.

#[cfg(all(test, feature = "control_plane"))]
use azure_core::http::headers::HeaderName;

#[cfg(all(test, feature = "control_plane"))]
pub const OFFER_REPLACE_PENDING: HeaderName = HeaderName::from_static("x-ms-offer-replace-pending");
