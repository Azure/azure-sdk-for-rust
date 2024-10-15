// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Serialize;

use super::{IndexingPolicy, PartitionKeyDefinition};

/// Describes the properties used to create a new container.
#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ContainerDefinition {
    /// The ID of the new container.
    pub id: String,

    /// The partition key used for the new container.
    pub partition_key: PartitionKeyDefinition,

    /// The indexing policy for the new container.
    pub indexing_policy: Option<IndexingPolicy>,
}
