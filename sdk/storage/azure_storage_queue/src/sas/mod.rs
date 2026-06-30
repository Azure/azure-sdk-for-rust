// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Hand-written Shared Access Signature (SAS) support.
//!
//! Available when the `sas_builder` feature is enabled. Holds the URL-terminal
//! builder returned by [`QueueClient::user_delegation_sas`](crate::QueueClient::user_delegation_sas).

mod builders;

pub use builders::QueueSasBuilder;
