// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Stream implementations for Azure Blob Storage.

#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
pub mod tokio;
