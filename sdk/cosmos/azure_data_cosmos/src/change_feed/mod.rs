// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types for working with the Cosmos DB change feed.
//!
//! The change feed is a persistent log of changes to items in a container, ordered by modification time.
//! Use [`ContainerClient::query_items_change_feed()`](crate::clients::ContainerClient::query_items_change_feed)
//! to read the change feed.

mod feed_range;
mod mode;
mod start_from;

pub use feed_range::FeedRange;
pub use mode::ChangeFeedMode;
pub use start_from::ChangeFeedStartFrom;

// Internal modules for advanced change feed state management.
// These are used for continuation token handling and split/merge scenarios.
#[allow(dead_code)]
pub(crate) mod composite_continuation_token;
#[allow(dead_code)]
pub(crate) mod feed_range_composite_continuation;
#[allow(dead_code)]
pub(crate) mod feed_range_internal;
#[allow(dead_code)]
pub(crate) mod state;
