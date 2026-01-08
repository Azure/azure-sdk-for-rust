// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod shared_key_policy;
mod storage_headers_policy;

#[cfg(feature = "azurite")]
pub(crate) use shared_key_policy::SharedKeyPolicy;
pub use storage_headers_policy::StorageHeadersPolicy;
