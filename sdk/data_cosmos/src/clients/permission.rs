use super::*;
use crate::prelude::*;
use crate::resources::permission::PermissionMode;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};

/// A client for Cosmos permission resources.
#[derive(Debug, Clone)]
pub struct PermissionClient {
    user: UserClient,
    permission_name: ReadonlyString,
}

impl PermissionClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(user: UserClient, permission_name: S) -> Self {
        Self {
            user,
            permission_name: permission_name.into(),
        }
    }

    /// Get a [`CosmosClient`
    #[must_use]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.user.cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    #[must_use]
    pub fn database_client(&self) -> &DatabaseClient {
        self.user.database_client()
    }

    /// Get the [`UserClient`]
    #[must_use]
    pub fn user_client(&self) -> &UserClient {
        &self.user
    }

    /// Get the permission's name
    #[must_use]
    pub fn permission_name(&self) -> &str {
        &self.permission_name
    }

    /// Create the permission
    #[must_use]
    pub fn create_permission(&self, permission_mode: PermissionMode) -> CreatePermissionBuilder {
        CreatePermissionBuilder::new(self.clone(), permission_mode)
    }

    /// Replace the permission
    #[must_use]
    pub fn replace_permission(&self, permission_mode: PermissionMode) -> ReplacePermissionBuilder {
        ReplacePermissionBuilder::new(self.clone(), permission_mode)
    }

    /// Get the permission
    #[must_use]
    pub fn get_permission(&self) -> GetPermissionBuilder {
        GetPermissionBuilder::new(self.clone())
    }

    /// Delete the permission
    #[must_use]
    pub fn delete_permission(&self) -> DeletePermissionBuilder {
        DeletePermissionBuilder::new(self.clone())
    }

    pub(crate) fn prepare_request_with_permission_name(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/users/{}/permissions/{}",
                self.database_client().database_name(),
                self.user_client().user_name(),
                self.permission_name()
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
