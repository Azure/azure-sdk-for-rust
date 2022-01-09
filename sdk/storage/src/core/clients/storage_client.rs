use crate::core::clients::{ServiceType, StorageAccountClient};
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

pub trait AsStorageClient {
    fn as_storage_client(&self) -> Arc<StorageClient>;
}

// TODO temporary fix until all clients are migrated to "into" pattern
impl AsStorageClient for StorageAccountClient {
    fn as_storage_client(&self) -> Arc<StorageClient> {
        Arc::new(StorageClient::new(self.clone()))
    }
}

#[derive(Debug, Clone)]
pub struct StorageClient {
    client: StorageAccountClient,
}

impl StorageClient {
    pub(crate) fn new(client: StorageAccountClient) -> Self {
        Self { client }
    }

    #[allow(dead_code)]
    pub fn storage_account_client(&self) -> &StorageAccountClient {
        &self.client
    }

    #[allow(dead_code)]
    pub fn http_client(&self) -> &dyn azure_core::HttpClient {
        self.client.http_client()
    }

    fn url_with_segments<'a, I>(mut url: url::Url, segments: I) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        {
            let mut segs = url
                .path_segments_mut()
                .map_err(|_| url::ParseError::SetHostOnCannotBeABaseUrl)?;
            for segment in segments.into_iter() {
                segs.push(segment);
            }
        }
        Ok(url)
    }

    pub fn blob_url_with_segments<'a, I>(&'a self, segments: I) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self::url_with_segments(self.client.blob_storage_url().to_owned(), segments)
    }

    pub fn queue_url_with_segments<'a, I>(
        &'a self,
        segments: I,
    ) -> Result<url::Url, url::ParseError>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Self::url_with_segments(self.client.queue_storage_url().to_owned(), segments)
    }

    #[cfg(feature = "account")]
    pub fn get_account_information(
        &self,
    ) -> crate::account::requests::GetAccountInformationBuilder {
        crate::account::requests::GetAccountInformationBuilder::new(self)
    }

    pub fn find_blobs_by_tags(&self) -> crate::account::requests::FindBlobsByTagsBuilder {
        crate::account::requests::FindBlobsByTagsBuilder::new(self)
    }

    #[allow(dead_code)]
    pub fn prepare_request(
        &self,
        url: &str,
        method: &Method,
        http_header_adder: &dyn Fn(Builder) -> Builder,
        request_body: Option<Bytes>,
    ) -> crate::Result<(Request<Bytes>, url::Url)> {
        self.client.prepare_request(
            url,
            method,
            http_header_adder,
            ServiceType::Blob,
            request_body,
        )
    }
}
