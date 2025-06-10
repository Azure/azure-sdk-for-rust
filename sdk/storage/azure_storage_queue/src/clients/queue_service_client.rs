// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::generated::clients::AzureQueueStorageServiceOperationsClient as GeneratedQueueServiceClient;
use crate::generated::models::QueueApiVersion;
use crate::generated::models::{
    AzureQueueStorageServiceOperationsClientGetPropertiesOptions, ServicePropertiesCompType,
    ServiceRestypeType, StorageServicePropertiesResponse,
};
use azure_core::{
    credentials::TokenCredential,
    http::{
        policies::{BearerTokenCredentialPolicy, Policy},
        Response, Url, XmlFormat,
    },
    Result,
};
use std::sync::Arc;

/// A client to interact with a specific Azure storage queue, although that queue may not yet exist.
pub struct QueueServiceClient {
    pub(super) endpoint: Url,
    pub(super) client: GeneratedQueueServiceClient,
    pub(super) version: QueueApiVersion,
}

impl QueueServiceClient {
    /// Returns the Url associated with this client.
    pub fn endpoint(&self) -> &Url {
        &self.endpoint
    }

    /// gets the properties of a storage account's Queue service, including properties
    /// for Storage Analytics and CORS (Cross-Origin Resource Sharing) rules.
    ///
    /// # Arguments
    ///
    /// * `restype` - restype
    /// * `comp` - comp
    /// * `version` - Specifies the version of the operation to use for this request.
    /// * `options` - Optional parameters for the request.
    pub async fn get_properties(
        &self,
        restype: ServiceRestypeType,
        comp: ServicePropertiesCompType,
        version: QueueApiVersion,
        options: Option<AzureQueueStorageServiceOperationsClientGetPropertiesOptions<'_>>,
    ) -> Result<Response<StorageServicePropertiesResponse, XmlFormat>> {
        self.client
            .get_properties(restype, comp, version, options)
            .await
    }
}
