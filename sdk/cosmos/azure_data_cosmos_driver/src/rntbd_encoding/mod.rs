// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! In-memory RNTBD framing, serialization, and parsing.
//!
//! HTTP envelope adaptation and dispatch belong to the transport pipeline.

pub(crate) mod request;
pub(crate) mod response;
pub(crate) mod status;
pub(crate) mod tokens;

pub(crate) use request::RntbdRequestFrame;
pub(crate) use response::RntbdResponse;
pub(crate) use tokens::Token;
