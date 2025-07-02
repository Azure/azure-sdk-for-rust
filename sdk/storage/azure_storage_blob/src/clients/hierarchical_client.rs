// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    generated::clients::{
        HierarchicalClient as GeneratedHierarchicalClient, HierarchicalClientOptions,
    },
    models::HierarchicalClientCreateOptions,
    pipeline::StorageHeadersPolicy,
    BlobClientOptions,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        NoFormat, RawResponse, RequestContent, Response, Url,
    },
    Bytes, Result,
};
use std::marker::PhantomData;
use std::sync::Arc;

/// Marker types for type state, TODO: Export elsewhere
pub struct File;
pub struct Directory;

// Struct, use struct initializer to get top-level, No State client
pub struct HierarchicalClient<T> {
    pub(crate) endpoint: Url,
    pub(crate) client: GeneratedHierarchicalClient,
    pub(crate) _marker: PhantomData<T>,
}

// Generic type, shared functionality
impl<T> HierarchicalClient<T> {}

// Conversion methods from No State -> State
impl HierarchicalClient<()> {
    pub fn file(self) -> HierarchicalClient<File> {
        HierarchicalClient {
            endpoint: self.endpoint,
            client: self.client,
            _marker: PhantomData::<File>,
        }
    }

    pub fn directory(self) -> HierarchicalClient<Directory> {
        HierarchicalClient {
            endpoint: self.endpoint,
            client: self.client,
            _marker: PhantomData::<Directory>,
        }
    }
}

// File state specific functions
impl HierarchicalClient<File> {
    pub async fn create(
        &self,
        options: Option<HierarchicalClientCreateOptions<'_>>,
    ) -> Result<RawResponse> {
        self.client.create("file".to_string(), options).await
    }
}

// Directory state specific functions
impl HierarchicalClient<Directory> {
    pub async fn create(
        &self,
        options: Option<HierarchicalClientCreateOptions<'_>>,
    ) -> Result<RawResponse> {
        self.client.create("directory".to_string(), options).await
    }
}
