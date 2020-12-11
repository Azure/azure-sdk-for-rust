use crate::core::prelude::*;
use crate::filesystem::responses::DeleteFilesystemResponse;
use crate::filesystem::{FilesystemRequired, FilesystemSupport};
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use azure_core::{
    ClientRequestIdOption, ClientRequestIdSupport, IfSinceConditionOption, IfSinceConditionSupport,
    TimeoutOption, TimeoutSupport,
};
use azure_core::{No, ToAssign, Yes};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_filesystem: PhantomData<FilesystemSet>,
    filesystem: Option<&'a str>,
    timeout: Option<u64>,
    if_since_condition: Option<IfSinceCondition>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> DeleteFilesystemBuilder<'a, C, No>
where
    C: Client,
{
    pub(crate) fn new(client: &'a C) -> DeleteFilesystemBuilder<'a, C, No> {
        DeleteFilesystemBuilder {
            client,
            p_filesystem: PhantomData {},
            filesystem: None,
            timeout: None,
            if_since_condition: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequired<'a, C> for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

impl<'a, C> FilesystemRequired<'a> for DeleteFilesystemBuilder<'a, C, Yes>
where
    C: Client,
{
    #[inline]
    fn filesystem(&self) -> &'a str {
        self.filesystem.unwrap()
    }
}

impl<'a, C, FilesystemSet> TimeoutOption for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, FilesystemSet> IfSinceConditionOption for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn if_since_condition(&self) -> Option<IfSinceCondition> {
        self.if_since_condition
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdOption<'a>
    for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> FilesystemSupport<'a> for DeleteFilesystemBuilder<'a, C, No>
where
    C: Client,
{
    type O = DeleteFilesystemBuilder<'a, C, Yes>;

    #[inline]
    fn with_filesystem(self, filesystem: &'a str) -> Self::O {
        DeleteFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: Some(filesystem),
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> TimeoutSupport for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = DeleteFilesystemBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: Some(timeout),
            if_since_condition: self.if_since_condition,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> IfSinceConditionSupport for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = DeleteFilesystemBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_if_since_condition(self, if_since_condition: IfSinceCondition) -> Self::O {
        DeleteFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            if_since_condition: Some(if_since_condition),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, FilesystemSet> ClientRequestIdSupport<'a>
    for DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
    type O = DeleteFilesystemBuilder<'a, C, FilesystemSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteFilesystemBuilder {
            client: self.client,
            p_filesystem: PhantomData {},
            filesystem: self.filesystem,
            timeout: self.timeout,
            if_since_condition: self.if_since_condition,
            client_request_id: Some(client_request_id),
        }
    }
}

impl<'a, C, FilesystemSet> DeleteFilesystemBuilder<'a, C, FilesystemSet>
where
    FilesystemSet: ToAssign,
    C: Client,
{
}

impl<'a, C> DeleteFilesystemBuilder<'a, C, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<DeleteFilesystemResponse, AzureError> {
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
            &Method::DELETE,
            &|mut request| {
                request = IfSinceConditionOption::add_optional_header(&self, request);
                request = ClientRequestIdOption::add_optional_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::ACCEPTED)
            .await?;
        DeleteFilesystemResponse::from_headers(&headers)
    }
}
