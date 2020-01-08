use crate::clients::{CosmosUriBuilder, PermissionClient, ResourceType};
use crate::prelude::*;
use crate::responses::CreateUserResponse;
use crate::{PermissionMode, PermissionTrait, Resource, UserTrait};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::{No, ToAssign, Yes};
use core::marker::PhantomData;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct CreatePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: Resource,
{
    permission_client: &'a PermissionClient<'a, CUB>,
    p_permission_mode: PhantomData<PermissionSet>,
    permission_mode: Option<&'a PermissionMode<R>>,
}

impl<'a, CUB, R> CreatePermissionBuilder<'a, CUB, R, No>
where
    CUB: CosmosUriBuilder,
    R: Resource,
{
    #[inline]
    pub(crate) fn new(
        permission_client: &'a PermissionClient<'a, CUB>,
    ) -> CreatePermissionBuilder<'a, CUB, R, No> {
        CreatePermissionBuilder {
            permission_client,
            p_permission_mode: PhantomData {},
            permission_mode: None,
        }
    }
}

impl<'a, CUB, R, PermissionSet> PermissionClientRequired<'a, CUB>
    for CreatePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: Resource,
{
    #[inline]
    fn permission_client(&self) -> &'a PermissionClient<'a, CUB> {
        self.permission_client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, CUB, R> PermissionModeRequired<'a, R> for CreatePermissionBuilder<'a, CUB, R, Yes>
where
    CUB: CosmosUriBuilder,
    R: Resource,
{
    #[inline]
    fn permission_mode(&self) -> &'a PermissionMode<R> {
        self.permission_mode.unwrap()
    }
}

impl<'a, CUB, R> PermissionModeSupport<'a, R> for CreatePermissionBuilder<'a, CUB, R, No>
where
    CUB: CosmosUriBuilder,
    R: Resource,
{
    type O = CreatePermissionBuilder<'a, CUB, R, Yes>;

    #[inline]
    fn with_permission_mode(self, permission_mode: &'a PermissionMode<R>) -> Self::O {
        CreatePermissionBuilder {
            permission_client: self.permission_client,
            p_permission_mode: PhantomData {},
            permission_mode: Some(permission_mode),
        }
    }
}

// methods callable regardless
impl<'a, CUB, R, PermissionSet> CreatePermissionBuilder<'a, CUB, R, PermissionSet>
where
    PermissionSet: ToAssign,
    CUB: CosmosUriBuilder,
    R: Resource,
{
}

// methods callable only when every mandatory field has been filled
impl<'a, CUB, R> CreatePermissionBuilder<'a, CUB, R, Yes>
where
    CUB: CosmosUriBuilder,
    R: Resource,
{
    pub async fn execute(&self) -> Result<CreateUserResponse, AzureError> {
        trace!("CreatePermissionBuilder::execute called");

        let mut req = self.permission_client.main_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions",
                self.permission_client.database_name().name(),
                self.permission_client.user_name().id(),
            ),
            hyper::Method::POST,
            ResourceType::Permissions,
        );

        //req.header(http::header::CONTENT_TYPE, "application/json");

        //#[derive(Serialize, Deserialize)]
        //struct RequestBody<'a> {
        //    id: &'a str,
        //}
        //let request_body = RequestBody {
        //    id: self.user_client().user_name().id(),
        //};
        //let request_body = serde_json::to_string(&request_body)?;

        //let req = req.body(hyper::Body::from(request_body))?;
        //debug!("\nreq == {:?}", req);

        //let (headers, body) = check_status_extract_headers_and_body(
        //    self.user_client.hyper_client().request(req),
        //    StatusCode::CREATED,
        //)
        //.await?;

        //Ok((&headers, &body as &[u8]).try_into()?)
    }
}
