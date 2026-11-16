// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Integration tests compose large Cosmos operation futures on tokio test threads.
#![allow(clippy::large_futures)]

// Ordinary live selection is not approval for the separately opted-in 40,000 RU/s suite.
#[cfg(any(test_category = "live_order_by", not(test_category = "live")))]
#[path = "live_tests/binary_order_by.rs"]
mod binary_order_by;
#[path = "live_tests/cosmos_change_feed.rs"]
mod cosmos_change_feed;
#[path = "live_tests/cosmos_query.rs"]
mod cosmos_query;
mod framework;
