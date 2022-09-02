use crate::service::operations::*;
use azure_core::{
    headers::Headers, Body, ClientOptions, Context, Method, Pipeline, Request, Response, Url,
};
use azure_storage::{
    clients::{
        new_pipeline_from_options, shared_access_signature, ServiceType, StorageCredentials,
    },
    prelude::{AccountSasPermissions, AccountSasResource, AccountSasResourceType},
    shared_access_signature::account_sas::AccountSharedAccessSignature,
};
use once_cell::sync::Lazy;
use time::OffsetDateTime;

use super::ContainerClient;

/// A builder for the blob service client.
#[derive(Debug, Clone)]
pub struct BlobServiceClientBuilder {
    cloud_location: CloudLocation,
    options: ClientOptions,
}

impl BlobServiceClientBuilder {
    /// Create a new instance of `BlobServiceClientBuilder`.
    #[must_use]
    pub fn new(account: impl Into<String>, credentials: StorageCredentials) -> Self {
        Self::with_location(CloudLocation::Public {
            account: account.into(),
            credentials,
        })
    }

    /// Create a new instance of `BlobServiceClientBuilder` with a cloud location.
    #[must_use]
    pub fn with_location(cloud_location: CloudLocation) -> Self {
        Self {
            options: ClientOptions::default(),
            cloud_location,
        }
    }

    /// Use the emulator with default settings
    #[must_use]
    pub fn emulator() -> Self {
        Self::with_location(CloudLocation::Emulator {
            address: "127.0.0.1".to_owned(),
            port: 10000,
        })
    }

    /// Convert the builder into a `BlobServiceClient` instance.
    #[must_use]
    pub fn build(self) -> BlobServiceClient {
        let credentials = self.cloud_location.credentials();
        BlobServiceClient {
            pipeline: new_pipeline_from_options(self.options, credentials.clone()),
            cloud_location: self.cloud_location,
        }
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
    #[must_use]
    pub fn client_options(mut self, options: impl Into<azure_core::ClientOptions>) -> Self {
        self.options = options.into();
        self
    }
}

#[derive(Debug, Clone)]
pub struct BlobServiceClient {
    pipeline: Pipeline,
    cloud_location: CloudLocation,
}

impl BlobServiceClient {
    /// Create a new `BlobServiceClient` which connects to the account's instance in the public Azure cloud.
    #[must_use]
    pub fn new(account: impl Into<String>, credentials: StorageCredentials) -> Self {
        BlobServiceClientBuilder::new(account, credentials).build()
    }

    /// Create a new `BlobServiceClientBuilder`.
    #[must_use]
    pub fn builder(
        account: impl Into<String>,
        credentials: StorageCredentials,
    ) -> BlobServiceClientBuilder {
        BlobServiceClientBuilder::new(account, credentials)
    }

    pub fn get_account_information(&self) -> GetAccountInformationBuilder {
        GetAccountInformationBuilder::new(self.clone())
    }

    pub fn find_blobs_by_tags(&self, expression: String) -> FindBlobsByTagsBuilder {
        FindBlobsByTagsBuilder::new(self.clone(), expression)
    }

    pub fn list_containers(&self) -> ListContainersBuilder {
        ListContainersBuilder::new(self.clone())
    }

    pub fn url(&self) -> azure_core::Result<url::Url> {
        self.cloud_location.url()
    }

    pub fn container_client<S: Into<String>>(&self, container_name: S) -> ContainerClient {
        ContainerClient::new(self.clone(), container_name.into())
    }

    pub fn shared_access_signature(
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
    }

    pub(crate) fn credentials(&self) -> &StorageCredentials {
        self.cloud_location.credentials()
    }

    pub(crate) fn finalize_request(
        &self,
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

/// The cloud with which you want to interact.
// TODO: Other govt clouds?
#[derive(Debug, Clone)]
pub enum CloudLocation {
    /// Azure public cloud
    Public {
        account: String,
        credentials: StorageCredentials,
    },
    /// Azure China cloud
    China {
        account: String,
        credentials: StorageCredentials,
    },
    /// Use the well-known emulator
    Emulator { address: String, port: u16 },
    /// A custom base URL
    Custom {
        uri: String,
        credentials: StorageCredentials,
    },
}

impl CloudLocation {
    /// the base URL for a given cloud location
    fn url(&self) -> azure_core::Result<url::Url> {
        let url = match self {
            CloudLocation::Public { account, .. } => {
                format!("https://{}.blob.core.windows.net", account)
            }
            CloudLocation::China { account, .. } => {
                format!("https://{}.blob.core.chinacloudapi.cn", account)
            }
            CloudLocation::Custom { uri, .. } => uri.clone(),
            CloudLocation::Emulator { address, port } => {
                format!("http://{address}:{port}/{EMULATOR_ACCOUNT}")
            }
        };
        Ok(url::Url::parse(&url)?)
    }

    fn credentials(&self) -> &StorageCredentials {
        match self {
            CloudLocation::Public { credentials, .. } => credentials,
            CloudLocation::China { credentials, .. } => credentials,
            CloudLocation::Emulator { .. } => &EMULATOR_CREDENTIALS,
            CloudLocation::Custom { credentials, .. } => credentials,
        }
    }
}

pub static EMULATOR_CREDENTIALS: Lazy<StorageCredentials> = Lazy::new(|| {
    StorageCredentials::Key(EMULATOR_ACCOUNT.to_owned(), EMULATOR_ACCOUNT_KEY.to_owned())
});

/// The well-known account used by Azurite and the legacy Azure Storage Emulator.
/// <https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key>
pub const EMULATOR_ACCOUNT: &str = "devstoreaccount1";

/// The well-known account key used by Azurite and the legacy Azure Storage Emulator.
/// <https://docs.microsoft.com/azure/storage/common/storage-use-azurite#well-known-storage-account-and-key>
pub const EMULATOR_ACCOUNT_KEY: &str =
    "Eby8vdM02xNOcqFlqUwJPLlmEtlCDXJ1OUzFT50uSRZ6IFsuFq2UVErCz4I6tq/K1SZFPTOtr/KBHBeksoGMGw==";
