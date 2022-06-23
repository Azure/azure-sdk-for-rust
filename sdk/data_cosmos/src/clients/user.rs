use crate::clients::*;
use crate::prelude::*;
use crate::ReadonlyString;
use azure_core::{Pipeline, Request};

/// A client for Cosmos user resources.
#[derive(Debug, Clone)]
pub struct UserClient {
    database: DatabaseClient,
    user_name: ReadonlyString,
}

impl UserClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(database: DatabaseClient, user_name: S) -> Self {
        Self {
            database,
            user_name: user_name.into(),
        }
    }

    /// Create the user.
    pub fn create_user(&self) -> CreateUserBuilder {
        CreateUserBuilder::new(self.clone())
    }

    /// Get the user.
    pub fn get_user(&self) -> GetUserBuilder {
        GetUserBuilder::new(self.clone())
    }

    /// Replace the user.
    pub fn replace_user<S: Into<String>>(&self, user_name: S) -> ReplaceUserBuilder {
        ReplaceUserBuilder::new(self.clone(), user_name.into())
    }

    /// Delete the user.
    pub fn delete_user(&self) -> DeleteUserBuilder {
        DeleteUserBuilder::new(self.clone())
    }

    /// List the user's permissions.
    pub fn list_permissions(&self) -> ListPermissionsBuilder {
        ListPermissionsBuilder::new(self.clone())
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        &self.database
    }

    /// Convert into a [`PermissionClient`].
    pub fn permission_client<S: Into<ReadonlyString>>(
        &self,
        permission_name: S,
    ) -> PermissionClient {
        PermissionClient::new(self.clone(), permission_name)
    }

    /// Get the user name.
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub(crate) fn user_request(&self, method: http::Method) -> Request {
        self.cosmos_client().request(
            &format!(
                "dbs/{}/users/{}",
                self.database_client().database_name(),
                self.user_name()
            ),
            method,
        )
    }

    pub(crate) fn pipeline(&self) -> &Pipeline {
        self.cosmos_client().pipeline()
    }
}
