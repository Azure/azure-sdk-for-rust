use crate::core::clients::{ServiceType, StorageAccountClient};
use crate::operations::*;
use azure_core::{
    error::{Error, ErrorKind},
    Context, Request, Response,
};
use bytes::Bytes;
use http::method::Method;
use std::sync::Arc;

pub trait AsStorageClient {
    fn storage_client(&self) -> Arc<StorageClient>;
}

impl AsStorageClient for Arc<StorageAccountClient> {
    fn storage_client(&self) -> Arc<StorageClient> {
        StorageClient::new(self.clone())
    }
}

#[derive(Debug, Clone)]
pub struct StorageClient {
    storage_account_client: Arc<StorageAccountClient>,
}

impl StorageClient {
    pub(crate) fn new(storage_account_client: Arc<StorageAccountClient>) -> Arc<Self> {
        Arc::new(Self {
            storage_account_client,
        })
    }

    #[allow(dead_code)]
    pub fn storage_account_client(&self) -> &StorageAccountClient {
        self.storage_account_client.as_ref()
    }

    #[allow(dead_code)]
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.storage_account_client.http_client()
    }

    fn url_with_segments<'a, I>(mut url: url::Url, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        {
            let original_url = url.clone();
            let mut segs = url.path_segments_mut().map_err(|_| {
                Error::with_message(ErrorKind::DataConversion, || {
                    format!("failed to parse url path segments. url: {original_url}")
                })
            })?;
            for segment in segments.into_iter() {
                segs.push(segment);
            }
        }
        Ok(url)
    }

    pub fn blob_url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self::url_with_segments(
            self.storage_account_client.blob_storage_url().to_owned(),
            segments,
        )
    }

    pub fn queue_url_with_segments<'a, I>(&'a self, segments: I) -> azure_core::Result<url::Url>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self::url_with_segments(
            self.storage_account_client.queue_storage_url().to_owned(),
            segments,
        )
    }

    #[cfg(feature = "account")]
    pub fn get_account_information(&self) -> GetAccountInformationBuilder {
        GetAccountInformationBuilder::new(self.clone())
    }

    pub fn find_blobs_by_tags(&self) -> FindBlobsByTagsBuilder {
        FindBlobsByTagsBuilder::new(self.clone())
    }

    pub fn prepare_request(
        &self,
        url: &str,
        method: Method,
        request_body: Option<Bytes>,
    ) -> azure_core::Result<Request> {
        self.storage_account_client
            .prepare_request(url, method, ServiceType::Blob, request_body)
    }

    pub async fn send(
        &self,
        context: &mut Context,
        request: &mut Request,
        service_type: ServiceType,
    ) -> azure_core::Result<Response> {
        self.storage_account_client
            .send(context, request, service_type)
            .await
    }
}
