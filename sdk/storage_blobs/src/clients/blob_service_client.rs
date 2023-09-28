use crate::service::operations::*;
use azure_core::{
    headers::Headers, request_options::LeaseId, Body, ClientOptions, Context, Method, Pipeline,
    Request, Response, Url,
};
use azure_storage::{
    clients::{new_pipeline_from_options, shared_access_signature, ServiceType},
    prelude::{AccountSasPermissions, AccountSasResource, AccountSasResourceType},
    shared_access_signature::account_sas::AccountSharedAccessSignature,
    CloudLocation, StorageCredentials,
};
use time::OffsetDateTime;

use super::{BlobClient, BlobLeaseClient, ContainerClient, ContainerLeaseClient};

/// A builder for the blob service client.
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    cloud_location: CloudLocation,
    options: ClientOptions,
    credentials: StorageCredentials,
}

impl ClientBuilder {
    /// Create a new instance of `ClientBuilder`.
    #[must_use]
    pub fn new<A, C>(account: A, credentials: C) -> Self
    where
        A: Into<String>,
        C: Into<StorageCredentials>,
    {
        Self::with_location(
            CloudLocation::Public {
                account: account.into(),
            },
            credentials,
        )
    }

    /// Create a new instance of `ClientBuilder` with a cloud location.
    #[must_use]
    pub fn with_location<C>(cloud_location: CloudLocation, credentials: C) -> Self
    where
        C: Into<StorageCredentials>,
    {
        Self {
            options: ClientOptions::default(),
            cloud_location,
            credentials: credentials.into(),
        }
    }

    /// Use the emulator with default settings
    #[must_use]
    pub fn emulator() -> Self {
        Self::with_location(
            CloudLocation::Emulator {
                address: "127.0.0.1".to_owned(),
                port: 10000,
            },
            StorageCredentials::emulator(),
        )
    }

    /// Convert the builder into a `BlobServiceClient` instance.
    #[must_use]
    pub fn blob_service_client(self) -> BlobServiceClient {
        let Self {
            cloud_location,
            options,
            credentials,
        } = self;

        BlobServiceClient {
            pipeline: new_pipeline_from_options(options, credentials.clone()),
            cloud_location,
            credentials,
        }
    }

    /// Convert the builder into a `ContainerClient` instance.
    #[must_use]
    pub fn container_client(self, container_name: impl Into<String>) -> ContainerClient {
        self.blob_service_client().container_client(container_name)
    }

    /// Convert the builder into a `BlobClient` instance.
    #[must_use]
    pub fn blob_client(
        self,
        container_name: impl Into<String>,
        blob_name: impl Into<String>,
    ) -> BlobClient {
        self.blob_service_client()
            .container_client(container_name)
            .blob_client(blob_name)
    }

    /// Convert the builder into a `ContainerLeaseClient` instance.
    #[must_use]
    pub fn container_lease_client(
        self,
        container_name: impl Into<String>,
        lease_id: LeaseId,
    ) -> ContainerLeaseClient {
        self.blob_service_client()
            .container_client(container_name)
            .container_lease_client(lease_id)
    }

    /// Convert the builder into a `BlobLeaseClient` instance.
    #[must_use]
    pub fn blob_lease_client(
        self,
        container_name: impl Into<String>,
        blob_name: impl Into<String>,
        lease_id: LeaseId,
    ) -> BlobLeaseClient {
        self.blob_service_client()
            .container_client(container_name)
            .blob_client(blob_name)
            .blob_lease_client(lease_id)
    }

    /// Set the cloud location.
    #[must_use]
    pub fn cloud_location(mut self, cloud_location: CloudLocation) -> Self {
        self.cloud_location = cloud_location;
        self
    }

    /// Set the retry options.
    #[must_use]
    pub fn retry(mut self, retry: impl Into<azure_core::RetryOptions>) -> Self {
        self.options = self.options.retry(retry);
        self
    }

    /// Set the transport options.
    #[must_use]
    pub fn transport(mut self, transport: impl Into<azure_core::TransportOptions>) -> Self {
        self.options = self.options.transport(transport);
        self
    }

    /// Override all of the client options.
    ///
    /// *Warning!*: This overrides all client options that have been previously set on this builder.
    #[must_use]
    pub fn client_options(mut self, options: impl Into<azure_core::ClientOptions>) -> Self {
        self.options = options.into();
        self
    }
}

/// A client for interacting with the blob storage service.
#[derive(Debug, Clone)]
pub struct BlobServiceClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
    credentials: StorageCredentials,
}

impl BlobServiceClient {
    /// Create a new `BlobServiceClient` which connects to the account's instance in the public Azure cloud.
    #[must_use]
    pub fn new(account: impl Into<String>, credentials: impl Into<StorageCredentials>) -> Self {
        ClientBuilder::new(account, credentials).blob_service_client()
    }

    /// Create a new `ClientBuilder`.
    #[must_use]
    pub fn builder(
        account: impl Into<String>,
        credentials: impl Into<StorageCredentials>,
    ) -> ClientBuilder {
        ClientBuilder::new(account, credentials)
    }

    /// Get information about the blob storage account
    pub fn get_account_information(&self) -> GetAccountInformationBuilder {
        GetAccountInformationBuilder::new(self.clone())
    }

    /// Get all the blobs with the given tags in the where expression
    pub fn find_blobs_by_tags(&self, expression: String) -> FindBlobsByTagsBuilder {
        FindBlobsByTagsBuilder::new(self.clone(), expression)
    }

    /// List all the containers in the blob account
    pub fn list_containers(&self) -> ListContainersBuilder {
        ListContainersBuilder::new(self.clone())
    }

    pub fn url(&self) -> azure_core::Result<url::Url> {
        self.cloud_location.url(ServiceType::Blob)
    }

    pub fn container_client<S: Into<String>>(&self, container_name: S) -> ContainerClient {
        ContainerClient::new(self.clone(), container_name.into())
    }

    pub fn get_user_deligation_key(
        &self,
        start: OffsetDateTime,
        expiry: OffsetDateTime,
    ) -> GetUserDelegationKeyBuilder {
        GetUserDelegationKeyBuilder::new(self.clone(), start, expiry)
    }

    pub async fn shared_access_signature(
        &self,
        resource_type: AccountSasResourceType,
        expiry: OffsetDateTime,
        permissions: AccountSasPermissions,
    ) -> azure_core::Result<AccountSharedAccessSignature> {
        shared_access_signature(
            self.credentials(),
            AccountSasResource::Blob,
            resource_type,
            expiry,
            permissions,
        )
        .await
    }

    pub(crate) fn credentials(&self) -> &StorageCredentials {
        &self.credentials
    }

    pub(crate) fn finalize_request(
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        azure_storage::clients::finalize_request(url, method, headers, request_body)
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.pipeline
            .send(context.insert(ServiceType::Blob), request)
            .await
    }
}
