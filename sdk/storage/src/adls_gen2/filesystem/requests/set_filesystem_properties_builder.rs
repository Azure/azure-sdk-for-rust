use crate::core::prelude::*;
use crate::filesystem::responses::SetFilesystemPropertiesResponse;
use crate::filesystem::{FilesystemSupport, PropertiesOption, PropertiesSupport};
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use azure_core::{ClientRequestIdOption, ClientRequestIdSupport, TimeoutOption, TimeoutSupport};
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_filesystem: PhantomData<FilesystemSet>,
    filesystem: Option<&'a str>,
    timeout: Option<u64>,
    properties: Option<&'a str>,
    if_since_condition: Option<IfModifiedSinceCondition>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> SetFilesystemPropertiesBuilder<'a, C, No>
where
    C: Client,
{
    pub(crate) fn new(client: &'a C) -> SetFilesystemPropertiesBuilder<'a, C, No> {
        SetFilesystemPropertiesBuilder {
            client,
            p_filesystem: PhantomData {},
            filesystem: None,
            timeout: None,
            properties: None,
            if_since_condition: None,
            client_request_id: None,
        }
    }
}

impl<'a, C> FilesystemSupport<'a> for SetFilesystemPropertiesBuilder<'a, C, No>
where
    C: Client,
{
    type O = SetFilesystemPropertiesBuilder<'a, C, Yes>;

    #[inline]
    fn with_filesystem(self, filesystem: &'a str) -> Self::O {
        SetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: Some(filesystem),
            timeout: self.timeout,
            properties: self.properties,
            if_since_condition: self.if_since_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> TimeoutSupport for SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        SetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: Some(timeout),
            properties: self.properties,
            if_since_condition: self.if_since_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> PropertiesSupport<'a>
    for SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_properties(self, properties: &'a str) -> Self::O {
        SetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            properties: Some(properties),
            if_since_condition: self.if_since_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdSupport<'a>
    for SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        SetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            properties: self.properties,
            if_since_condition: self.if_since_condition,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C, FilesystemSet> SetFilesystemPropertiesBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    fn with_if_since_condition(self, if_since_condition: IfModifiedSinceCondition) -> Self {
        SetFilesystemPropertiesBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            properties: self.properties,
            if_since_condition: Some(if_since_condition),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> SetFilesystemPropertiesBuilder<'a, C, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<SetFilesystemPropertiesResponse, AzureError> {
        let mut uri = format!(
            "{}/{}?resource=filesystem",
            self.client.filesystem_uri(),
            self.filesystem.unwrap()
        );

        //TODO: Reenable uri parameters
        //if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
        //    uri = format!("{}&{}", uri, nm);
        //}

        let perform_request_response = self.client.perform_request(
            &uri,
            &Method::PATCH,
            &|mut request| {
                // TODO: Fix missing headers
                //request = ClientRequestIdOption::add_optional_header(&self, request);
                //request = PropertiesOption::add_optional_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;
        SetFilesystemPropertiesResponse::from_headers(&headers)
    }
}
