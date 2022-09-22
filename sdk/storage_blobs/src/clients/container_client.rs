use crate::{clients::*, container::operations::*, prelude::PublicAccess};
use azure_core::{
    error::{Error, ErrorKind},
    headers::Headers,
    prelude::*,
    Body, Method, Request, Response, Url,
};
use azure_storage::{
    prelude::BlobSasPermissions,
    shared_access_signature::{
        service_sas::{BlobSharedAccessSignature, BlobSignedResource},
        SasToken,
    },
    StorageCredentials,
};
use time::OffsetDateTime;

#[derive(Debug, Clone)]
pub struct ContainerClient {
    service_client: BlobServiceClient,
    container_name: String,
}

impl ContainerClient {
    /// Create a new `ContainerClient` from a `BlobServiceClient` and a container name
    pub(crate) fn new(service_client: BlobServiceClient, container_name: String) -> Self {
        Self {
            service_client,
            container_name,
        }
    }

    /// Create a container
    pub fn create(&self) -> CreateBuilder {
        CreateBuilder::new(self.clone())
    }

    /// Delete a container
    pub fn delete(&self) -> DeleteBuilder {
        DeleteBuilder::new(self.clone())
    }

    /// Get a container acl
    pub fn get_acl(&self) -> GetACLBuilder {
        GetACLBuilder::new(self.clone())
    }

    /// Set a container acl
    pub fn set_acl(&self, public_access: PublicAccess) -> SetACLBuilder {
        SetACLBuilder::new(self.clone(), public_access)
    }

    /// Get a container's properties
    pub fn get_properties(&self) -> GetPropertiesBuilder {
        GetPropertiesBuilder::new(self.clone())
    }

    /// List the blobs in a container
    pub fn list_blobs(&self) -> ListBlobsBuilder {
        ListBlobsBuilder::new(self.clone())
    }

    /// Acquite a lease on a container
    pub fn acquire_lease<LD: Into<LeaseDuration>>(
        &self,
        lease_duration: LD,
    ) -> AcquireLeaseBuilder {
        AcquireLeaseBuilder::new(self.clone(), lease_duration.into())
    }

    /// Break the lease on a container
    pub fn break_lease(&self) -> BreakLeaseBuilder {
        BreakLeaseBuilder::new(self.clone())
    }

    pub fn container_lease_client(&self, lease_id: LeaseId) -> ContainerLeaseClient {
        ContainerLeaseClient::new(self.clone(), lease_id)
    }

    pub fn blob_client<BN: Into<String>>(&self, blob_name: BN) -> BlobClient {
        BlobClient::new(self.clone(), blob_name.into())
    }

    pub fn container_name(&self) -> &str {
        &self.container_name
    }

    /// Create a shared access signature.
    pub fn shared_access_signature(
        &self,
        permissions: BlobSasPermissions,
        expiry: OffsetDateTime,
    ) -> azure_core::Result<BlobSharedAccessSignature> {
        match self.service_client.credentials() {
            StorageCredentials::Key(account, ref key) => {
                let canonicalized_resource =
                    format!("/blob/{}/{}", account, self.container_name(),);
                Ok(BlobSharedAccessSignature::new(
                    key.to_string(),
                    canonicalized_resource,
                    permissions,
                    expiry,
                    BlobSignedResource::Container,
                ))
            }
            _ => Err(Error::message(
                ErrorKind::Credential,
                "Shared access signature generation - \
                SAS can be generated only from key and account clients",
            )),
        }
    }

    pub fn generate_signed_container_url<T>(&self, signature: &T) -> azure_core::Result<url::Url>
    where
        T: SasToken,
    {
        let mut url = self.url()?;
        url.set_query(Some(&signature.token()));
        Ok(url)
    }

    /// Full URL for the container.
    pub fn url(&self) -> azure_core::Result<url::Url> {
        let container_name = self
            .container_name()
            .strip_prefix('/')
            .unwrap_or_else(|| self.container_name());
        let sep = if self.service_client.url()?.path().ends_with('/') {
            ""
        } else {
            "/"
        };

        let url = format!("{}{}{}", self.service_client.url()?, sep, container_name);
        Ok(url::Url::parse(&url)?)
    }

    pub(crate) fn credentials(&self) -> &StorageCredentials {
        self.service_client.credentials()
    }

    pub(crate) async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
    ) -> azure_core::Result<Response> {
        self.service_client.send(context, request).await
    }

    pub(crate) fn finalize_request(
        &self,
        url: Url,
        method: Method,
        headers: Headers,
        request_body: Option<Body>,
    ) -> azure_core::Result<Request> {
        self.service_client
            .finalize_request(url, method, headers, request_body)
    }
}

#[cfg(test)]
#[cfg(feature = "test_integration")]
mod integration_tests {
    use super::*;
    use crate::clients::ClientBuilder;
    use futures::StreamExt;

    fn get_emulator_client(container_name: &str) -> ContainerClient {
        ClientBuilder::emulator().container_client(container_name)
    }

    #[tokio::test]
    async fn test_create_delete() {
        let container_name = uuid::Uuid::new_v4().to_string();
        let container_client = get_emulator_client(&container_name);

        container_client
            .create()
            .into_future()
            .await
            .expect("create container should succeed");
        container_client
            .delete()
            .into_future()
            .await
            .expect("delete container should succeed");
    }

    #[tokio::test]
    async fn test_list_blobs() {
        let container_name = uuid::Uuid::new_v4().to_string();
        let container_client = get_emulator_client(&container_name);

        container_client
            .create()
            .into_future()
            .await
            .expect("create container should succeed");

        let md5 = md5::compute("world");
        container_client
            .blob_client("hello.txt")
            .put_block_blob("world")
            .into_future()
            .await
            .expect("put block blob should succeed");
        let list = container_client
            .list_blobs()
            .into_stream()
            .next()
            .await
            .expect("list blobs next() should return value")
            .expect("list blobs should succeed");
        let blobs: Vec<_> = list.blobs.blobs().collect();
        assert_eq!(blobs.len(), 1);
        assert_eq!(blobs[0].name, "hello.txt");
        assert_eq!(
            blobs[0]
                .properties
                .content_md5
                .as_ref()
                .expect("has content_md5")
                .as_slice(),
            &md5.0
        );

        container_client
            .delete()
            .into_future()
            .await
            .expect("delete container should succeed");
    }
}
