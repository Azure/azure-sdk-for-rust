use crate::clients::{Client, CosmosUriBuilder, DatabaseClient};
use crate::database::DatabaseName;
use crate::{requests, DatabaseTrait, PermissionName, PermissionTrait};
use azure_sdk_core::No;

#[derive(Debug, Clone)]
pub struct PermissionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    database_client: &'a DatabaseClient<'a, CUB>,
    permission_name: &'a dyn PermissionName,
}

impl<'a, CUB> PermissionClient<'a, CUB>
where
    CUB: CosmosUriBuilder,
{
    pub(crate) fn new(
        database_client: &'a DatabaseClient<'a, CUB>,
        permission_name: &'a dyn PermissionName,
    ) -> Self {
        Self {
            database_client,
            permission_name,
        }
    }

    pub(crate) fn main_client(&self) -> &Client<CUB> {
        self.database_client.main_client()
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
        self.database_client.database_name()
    }

    fn permission_name(&self) -> &'a dyn PermissionName {
        self.permission_name
    }
}
