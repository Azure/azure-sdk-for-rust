use crate::core::prelude::*;
use crate::filesystem::responses::GetFilesystemPropertiesResponse;
use crate::filesystem::{FilesystemRequired, FilesystemSupport};
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::{ClientRequestIdOption, ClientRequestIdSupport, TimeoutOption, TimeoutSupport};
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_filesystem: PhantomData<FilesystemSet>,
    filesystem: Option<&'a str>,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> GetFilesystemPropertiesBuilder<'a, C, No>
where
    C: Client,
{
    pub(crate) fn new(client: &'a C) -> GetFilesystemPropertiesBuilder<'a, C, No> {
        GetFilesystemPropertiesBuilder {
            client,
            p_filesystem: PhantomData {},
            filesystem: None,
            timeout: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequired<'a, C>
    for GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

impl<'a, C> FilesystemRequired<'a> for GetFilesystemPropertiesBuilder<'a, C, Yes>
where
    C: Client,
{
    #[inline]
    fn filesystem(&self) -> &'a str {
        self.filesystem.unwrap()
    }
}

impl<'a, C, FilesystemSet> TimeoutOption for GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdOption<'a>
    for GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> FilesystemSupport<'a> for GetFilesystemPropertiesBuilder<'a, C, No>
where
    C: Client,
{
    type O = GetFilesystemPropertiesBuilder<'a, C, Yes>;

    #[inline]
    fn with_filesystem(self, filesystem: &'a str) -> Self::O {
        GetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: Some(filesystem),
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> TimeoutSupport for GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdSupport<'a>
    for GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C, FilesystemSet> GetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
}

impl<'a, C> GetFilesystemPropertiesBuilder<'a, C, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<GetFilesystemPropertiesResponse, AzureError> {
        let mut uri = format!(
            "{}/{}?resource=filesystem",
            self.client().filesystem_uri(),
            self.filesystem()
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::HEAD,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) = check_status_extract_headers_and_body(
            perform_request_response.response_future,
            StatusCode::OK,
        )
        .await?;
        GetFilesystemPropertiesResponse::from_headers(&headers)
    }
}
