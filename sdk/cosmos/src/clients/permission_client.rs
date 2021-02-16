use super::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::HttpClient;

/// A client for Cosmos permission resources.
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

    /// Get a [`CosmosClient`
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.user_client.cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        self.user_client.database_client()
    }

    /// Get the [`UserClient`]
    pub fn user_client(&self) -> &UserClient {
        &self.user_client
    }

    /// Get the permission's name
    pub fn permission_name(&self) -> &str {
        &self.permission_name
    }

    /// Create the permission
    pub fn create_permission(&self) -> requests::CreatePermissionBuilder<'_, '_> {
        requests::CreatePermissionBuilder::new(self)
    }

    /// Replace the permission
    pub fn replace_permission(&self) -> requests::ReplacePermissionBuilder<'_, '_> {
        requests::ReplacePermissionBuilder::new(self)
    }

    /// Get the permission
    pub fn get_permission(&self) -> requests::GetPermissionBuilder<'_, '_> {
        requests::GetPermissionBuilder::new(self)
    }

    /// Delete the permission
    pub fn delete_permission(&self) -> requests::DeletePermissionsBuilder<'_, '_> {
        requests::DeletePermissionsBuilder::new(self)
    }

    pub(crate) fn prepare_request_with_permission_name(
        &self,
        method: http::Method,
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

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }
}
