// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end integration tests that compose multiple `dataflow` layers
//! (planner + drain + request + snapshot + continuation-token) against the
//! mock executor and topology provider. Unit tests live next to the layer
//! they cover; cross-layer scenarios live here.

mod change_feed_resume;
mod query_resume;
