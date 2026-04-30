// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 RNTBD wire-format support.
//!
//! This module owns in-memory request serialization and response deserialization
//! for RNTBD frames carried by the Gateway 2.0 transport path.

// Slice 1 intentionally lands the wire-format module before later slices wire it
// into the transport pipeline.
#![allow(dead_code, unused_imports)]

pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod status;
pub(crate) mod tokens;

pub(crate) use request::RntbdRequestFrame;
pub(crate) use response::RntbdResponse;
pub(crate) use status::map_rntbd_status_to_cosmos_status;
pub(crate) use tokens::{Token, TokenType, TokenValue};
