// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{collections::HashMap, sync::Arc};
use url::Url;

/// Type alias matching C# `ReadOnlyCollection<Uri>` semantics in a lightweight form.
/// We use `Arc<[Url]>` for cheap cloning and slice immutability.
pub type ReadOnlyUrlCollection = Arc<[Url]>;

/// Type alias representing `ReadOnlyDictionary<string, Uri>`.
pub type ReadOnlyLocationMap = Arc<HashMap<String, Url>>;

mod location_cache;
