// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Region selection strategies for Azure Cosmos DB client routing.

use crate::regions::Region;

/// Determines how the SDK selects Azure regions for routing requests.
///
/// This is a required parameter when building a [`CosmosClient`](crate::CosmosClient),
/// ensuring that every client is configured with an explicit region selection strategy.
///
/// # Examples
///
/// ```rust
/// use azure_data_cosmos::{Region, RoutingStrategy};
///
/// let strategy = RoutingStrategy::ProximityTo(Region::EAST_US);
/// ```
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RoutingStrategy {
    /// Select regions by geographic proximity to the given region.
    ///
    /// The SDK generates a list of preferred regions sorted by estimated
    /// proximity to the specified region. Proximity estimates are built
    /// into the SDK and may differ from actual round-trip times observed
    /// at runtime.
    ///
    /// If the application is not running in an Azure region, specify the
    /// closest Azure region to the application's actual location.
    ///
    /// Specifying an unknown region name results in undefined region
    /// selection behavior that may change in future versions of the SDK.
    ProximityTo(Region),

    /// Select target regions using a fixed preference order.
    ///
    /// This list **does not** restrict which regions the SDK may select for routing,
    /// only the order in which it considers them. After failover exhausts the list of preferred regions,
    /// the SDK will arbitrarily select from other available regions.
    ///
    /// If you need to prohibit the SDK from selecting certain regions,
    /// use [`OperationOptions::excluded_regions`](crate::options::OperationOptions::excluded_regions).
    PreferredRegions(Vec<Region>),
}
