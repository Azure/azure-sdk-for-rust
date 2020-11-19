use super::*;
use crate::{requests, ReadonlyString, ResourceType};

#[derive(Debug, Clone)]
pub struct PermissionClient {
    user_client: UserClient,
    permission_name: ReadonlyString,
}

impl PermissionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        user_client: UserClient,
        permission_name: S,
    ) -> Self {
        Self {
            user_client,
            permission_name: permission_name.into(),
        }
    }

    pub fn hyper_client(
        &self,
    ) -> &hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>> {
        self.user_client.hyper_client()
    }

    pub fn cosmos_client(&self) -> &CosmosClient {
        self.user_client.cosmos_client()
    }

    pub fn database_client(&self) -> &DatabaseClient {
        self.user_client.database_client()
    }

    pub fn user_client(&self) -> &UserClient {
        &self.user_client
    }

    pub fn permission_name(&self) -> &str {
        &self.permission_name
    }

    pub fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_> {
        requests::CreatePermissionBuilder::new(self)
    }

    pub fn replace_permission(&self) -> requests::ReplacePermissionBuilder<'_, '_> {
        requests::ReplacePermissionBuilder::new(self)
    }

    pub fn get_permission(&self) -> requests::GetPermissionBuilder<'_, '_> {
        requests::GetPermissionBuilder::new(self)
    }

    pub fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, '_> {
        requests::DeletePermissionsBuilder::new(self)
    }

    pub fn prepare_request(&self, method: hyper::Method) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions",
                self.database_client().database_name(),
                self.user_client().user_name()
            ),
            method,
            ResourceType::Permissions,
        )
    }

    pub fn prepare_request_with_permission_name(
        &self,
        method: hyper::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/users/{}/permissions/{}",
                self.database_client().database_name(),
                self.user_client().user_name(),
                self.permission_name()
            ),
            method,
            ResourceType::Permissions,
        )
    }
}
