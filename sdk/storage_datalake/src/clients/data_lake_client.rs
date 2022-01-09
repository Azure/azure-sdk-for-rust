use crate::authorization_policy::AuthorizationPolicy;
use crate::clients::FileSystemClient;
use crate::operations::ListFileSystems;
use crate::requests::*;
use azure_core::{HttpClient, Pipeline};
use azure_storage::core::prelude::*;
use bytes::Bytes;
use http::method::Method;
use http::request::{Builder, Request};
use std::sync::Arc;

const DEFAULT_DNS_SUFFIX: &str = "dfs.core.windows.net";

#[derive(Debug, Clone)]
pub struct DataLakeClient {
    client: StorageAccountClient,
}

impl DataLakeClient {
    pub fn new(client: StorageAccountClient) -> Self {
        Self { client }
    }

    pub fn new_with_credential<A>(account: A, storage_credentials: StorageCredentials) -> Self
    where
        A: Into<String>,
    {
        Self::new_with_options(
            account,
            storage_credentials,
            StorageAccountOptions::default(),
        )
    }

    pub(crate) fn new_with_options<A>(
        account: A,
        storage_credentials: StorageCredentials,
        options: StorageAccountOptions,
    ) -> Self
    where
        A: Into<String>,
    {
        let client = StorageAccountClient::new(account, storage_credentials, options);
        Self { client }
    }

    #[cfg(feature = "mock_transport_framework")]
    pub fn new_with_transaction<A, T>(
        account: A,
        storage_credentials: StorageCredentials,
        transaction_name: T,
    ) -> Self
    where
        A: Into<String>,
        T: Into<String>,
    {
        Self::new_with_options(
            account,
            storage_credentials,
            StorageAccountOptions::new_with_transaction_name(transaction_name.into()),
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.client.http_client()
    }

    pub(crate) fn url(&self) -> &str {
        &self.client.filesystem_url().as_str()
    }

    pub fn list_file_systems(&self) -> ListFileSystems {
        ListFileSystems::new(self.client.clone())
    }

    pub fn into_file_system_client(self, file_system_name: String) -> FileSystemClient {
        FileSystemClient::new(self, file_system_name)
    }

    pub(crate) fn prepare_request(
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

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.client.pipeline()
    }
}
