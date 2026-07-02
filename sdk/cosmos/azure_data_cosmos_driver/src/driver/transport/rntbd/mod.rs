// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 RNTBD wire-format support.
//!
//! This module owns in-memory request serialization and response deserialization
//! for RNTBD frames carried by the Gateway 2.0 transport path.

pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod status;
pub(crate) mod tokens;

pub(crate) use request::RntbdRequestFrame;
pub(crate) use response::RntbdResponse;
pub(crate) use tokens::Token;
