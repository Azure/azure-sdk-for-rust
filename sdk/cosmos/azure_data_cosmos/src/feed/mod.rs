// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types related to Cosmos DB feed operations, including query and change feed
//! iteration, pagination and related models.

// =========================================================================
// Public API
// =========================================================================

#[doc(inline)]
pub use azure_data_cosmos_driver::models::{ContinuationToken, FeedRange};
pub use change_feed_iterator::ChangeFeedPageIterator;
pub use iterator::{QueryItemIterator, QueryPageIterator};
pub use page::FeedPage;
pub use query::{FeedScope, Query};
pub use query_page::QueryFeedPage;

// =========================================================================
// Crate-internal re-exports
// =========================================================================

pub(crate) use page::FeedBody;

// =========================================================================
// Internal modules
// =========================================================================

mod change_feed_iterator;
mod iterator;
mod page;
mod query;
mod query_page;
