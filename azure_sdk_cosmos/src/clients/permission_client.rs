use crate::clients::{Client, CosmosUriBuilder, UserClient};
use crate::database::DatabaseName;
use crate::{requests, PermissionName, PermissionResource, PermissionTrait, UserName, UserTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct PermissionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    user_client: &'a UserClient<'a, CUB>,
    permission_name: &'a dyn PermissionName,
}

impl<'a, CUB> PermissionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        user_client: &'a UserClient<'a, CUB>,
        permission_name: &'a dyn PermissionName,
    ) -> Self {
        Self {
            user_client,
            permission_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.user_client.main_client()
    }

    pub(crate) fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.main_client().hyper_client()
    }
}

impl<'a, CUB> PermissionTrait<'a, CUB> for PermissionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    fn database_name(&self) -> &'a dyn DatabaseName {
        self.user_client.database_name()
    }

    fn user_name(&self) -> &'a dyn UserName {
        self.user_client.user_name()
    }

    fn permission_name(&self) -> &'a dyn PermissionName {
        self.permission_name
    }

    fn create_permission<R>(&self) -> requests::CreatePermissionBuilder<'_, CUB, R, No>
    where
        R: PermissionResource,
    {
        requests::CreatePermissionBuilder::new(self)
    }
}
