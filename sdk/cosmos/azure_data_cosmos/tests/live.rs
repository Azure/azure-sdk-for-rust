// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Integration tests compose large Cosmos operation futures on tokio test threads.
#![allow(clippy::large_futures)]

#[path = "live_tests/cosmos_change_feed.rs"]
mod cosmos_change_feed;
#[path = "live_tests/cosmos_query.rs"]
mod cosmos_query;
mod framework;
