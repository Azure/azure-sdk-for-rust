// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Types related to Cosmos DB feed operations, including query and change feed
//! iteration, pagination and related models.

use azure_core::http::Context;

use crate::diagnostics::{CosmosOperationContext, DiagnosticsContext, DiagnosticsHandlerChain};

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

/// Dispatches a completed feed page's diagnostics to the registered handler
/// chain, carrying the paged operation's identity ([`query_items`] /
/// [`query_change_feed`] etc. plus database/container).
///
/// This is the per-page completion seam for query and change-feed pagination,
/// mirroring the singleton completion seam on `ClientContext`: it lets metrics,
/// tracing, and sampled-logging handlers observe each page fetch, on both
/// success and failure. It is skipped when no handler is registered, so the
/// default paginated path does no diagnostics work per page.
///
/// `returned_item_count`, when supplied, feeds the `returned_rows` development
/// metric with the number of items the page yielded.
///
/// [`query_items`]: crate::clients::ContainerClient::query_items
/// [`query_change_feed`]: crate::clients::ContainerClient::query_change_feed
pub(crate) fn dispatch_page_diagnostics(
    diagnostics: &DiagnosticsHandlerChain,
    op_context: &CosmosOperationContext,
    page_diagnostics: &DiagnosticsContext,
    returned_item_count: Option<u64>,
) {
    if diagnostics.is_empty() {
        return;
    }
    let mut op = op_context.clone();
    if let Some(count) = returned_item_count {
        op = op.with_returned_item_count(count);
    }
    let cx = Context::new().with_value(op);
    diagnostics.dispatch(page_diagnostics, &cx);
}
