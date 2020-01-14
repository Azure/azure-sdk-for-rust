use crate::clients::{CosmosUriBuilder, PermissionClient, ResourceType};
use crate::prelude::*;
use crate::responses::ReplacePermissionResponse;
use crate::{PermissionMode, PermissionResource};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::{No, ToAssign, Yes};
use core::marker::PhantomData;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ReplacePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    permission_client: &'a PermissionClient<'a, CUB>,
    p_permission_mode: PhantomData<PermissionSet>,
    permission_mode: Option<&'a PermissionMode<R>>,
    expiry_seconds: u64,
}

impl<'a, CUB, R> ReplacePermissionBuilder<'a, CUB, R, No>
where
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    #[inline]
    pub(crate) fn new(
        permission_client: &'a PermissionClient<'a, CUB>,
    ) -> ReplacePermissionBuilder<'a, CUB, R, No> {
        ReplacePermissionBuilder {
            permission_client,
            p_permission_mode: PhantomData {},
            permission_mode: None,
            expiry_seconds: 3600,
        }
    }
}

impl<'a, CUB, R, PermissionSet> PermissionClientRequired<'a, CUB>
    for ReplacePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    #[inline]
    fn permission_client(&self) -> &'a PermissionClient<'a, CUB> {
        self.permission_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB, R> PermissionModeRequired<'a, R> for ReplacePermissionBuilder<'a, CUB, R, Yes>
where
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    #[inline]
    fn permission_mode(&self) -> &'a PermissionMode<R> {
        self.permission_mode.unwrap()
    }
}

impl<'a, CUB, R, PermissionSet> ExpirySecondsOption
    for ReplacePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    #[inline]
    fn expiry_seconds(&self) -> u64 {
        self.expiry_seconds
    }
}

impl<'a, CUB, R> PermissionModeSupport<'a, R> for ReplacePermissionBuilder<'a, CUB, R, No>
where
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    type O = ReplacePermissionBuilder<'a, CUB, R, Yes>;

    #[inline]
    fn with_permission_mode(self, permission_mode: &'a PermissionMode<R>) -> Self::O {
        ReplacePermissionBuilder {
            permission_client: self.permission_client,
            p_permission_mode: PhantomData {},
            permission_mode: Some(permission_mode),
            expiry_seconds: self.expiry_seconds,
        }
    }
}

impl<'a, CUB, R, PermissionSet> ExpirySecondsSupport
    for ReplacePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    type O = ReplacePermissionBuilder<'a, CUB, R, PermissionSet>;

    #[inline]
    fn with_expiry_seconds(self, expiry_seconds: u64) -> Self::O {
        ReplacePermissionBuilder {
            permission_client: self.permission_client,
            p_permission_mode: PhantomData {},
            permission_mode: self.permission_mode,
            expiry_seconds,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB, R> ReplacePermissionBuilder<'a, CUB, R, Yes>
where
    CUB: CosmosUriBuilder,
    R: PermissionResource,
{
    pub async fn execute(&self) -> Result<ReplacePermissionResponse<'a>, AzureError> {
        trace!("ReplacePermissionBuilder::execute called");

        let mut req = self.permission_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions/{}",
                self.permission_client.database_name().name(),
                self.permission_client.user_name().id(),
                self.permission_client.permission_name().name()
            ),
            hyper::Method::PUT,
            ResourceType::Permissions,
        );

        req = req.header(http::header::CONTENT_TYPE, "application/json");

        #[derive(Serialize, Deserialize)]
        struct RequestBody<'a> {
            id: &'a str,
            #[serde(rename = "permissionMode")]
            permission_mode: &'a str,
            resource: &'a str,
        }

        let (permission_mode, resource) = self.permission_mode().to_elements();

        let request_body = RequestBody {
            id: self.permission_client.permission_name().name(),
            permission_mode,
            resource: resource.uri(),
        };
        let request_body = serde_json::to_string(&request_body)?;

        let req = req.body(hyper::Body::from(request_body))?;
        debug!("\nreq == {:#?}", req);

        let (headers, body) = check_status_extract_headers_and_body(
            self.permission_client.hyper_client().request(req),
            StatusCode::OK,
        )
        .await?;

        Ok((&headers, &body as &[u8]).try_into()?)
    }
}
