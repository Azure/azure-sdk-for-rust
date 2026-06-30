// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hand-written Shared Access Signature (SAS) support.
//!
//! Available when the `sas_builder` feature is enabled. Holds the URL-terminal
//! builders returned by the client `user_delegation_sas` methods and the
//! internal helpers they rely on for assembling signed URLs.

mod builders;
pub(crate) mod helpers;

pub use builders::{BlobContainerSasBuilder, BlobSasBuilder};
