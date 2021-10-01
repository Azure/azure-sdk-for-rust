use super::*;
use crate::authorization_policy::CosmosContext;
use crate::prelude::*;
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::PipelineContext;
use azure_core::{pipeline::Pipeline, Context, HttpClient};

/// A client for Cosmos user resources.
#[derive(Debug, Clone)]
pub struct UserClient {
    database_client: DatabaseClient,
    user_name: ReadonlyString,
}

impl UserClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        database_client: DatabaseClient,
        user_name: S,
    ) -> Self {
        Self {
            database_client,
            user_name: user_name.into(),
        }
    }

    /// Get a [`CosmosClient`]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.database_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        &self.database_client
    }

    /// Get the user name
    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    /// Create the user
    pub async fn create_user(
        &self,
        ctx: Context,
        options: CreateUserOptions,
    ) -> Result<CreateUserResponse, crate::Error> {
        let mut request = self.cosmos_client().prepare_request_pipeline(
            &format!("dbs/{}/users", self.database_client.database_name()),
            http::Method::POST,
        );
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Users.into());

        options.decorate_request(&mut request, self.user_name())?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::CREATED)
            .await?;

        Ok(CreateUserResponse::try_from(response).await?)
    }

    /// Get the user
    pub async fn get_user(
        &self,
        ctx: Context,
        options: GetUserOptions,
    ) -> Result<GetUserResponse, crate::Error> {
        let mut request = self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/users/{}",
                self.database_client.database_name(),
                self.user_name
            ),
            http::Method::GET,
        );
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Users.into());

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        Ok(GetUserResponse::try_from(response).await?)
    }

    /// Replace the user
    pub async fn replace_user<S: AsRef<str>>(
        &self,
        ctx: Context,
        user_name: S,
        options: ReplaceUserOptions,
    ) -> Result<ReplaceUserResponse, crate::Error> {
        let mut request = self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/users/{}",
                self.database_client.database_name(),
                self.user_name
            ),
            http::Method::PUT,
        );
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Users.into());

        options.decorate_request(&mut request, user_name.as_ref())?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        Ok(ReplaceUserResponse::try_from(response).await?)
    }

    /// Delete the user
    pub fn delete_user(&self) -> requests::DeleteUserBuilder<'_, '_> {
        requests::DeleteUserBuilder::new(self)
    }

    /// List the user's permissions
    pub fn list_permissions(&self) -> requests::ListPermissionsBuilder<'_, '_> {
        requests::ListPermissionsBuilder::new(self)
    }

    /// Convert into a [`PermissionClient`]
    pub fn into_permission_client<S: Into<ReadonlyString>>(
        self,
        permission_name: S,
    ) -> PermissionClient {
        PermissionClient::new(self, permission_name)
    }

    pub(crate) fn prepare_request_with_user_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/users/{}",
                self.database_client().database_name(),
                self.user_name()
            ),
            method,
            ResourceType::Users,
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }

    fn pipeline(&self) -> &Pipeline<CosmosContext> {
        self.cosmos_client().pipeline()
    }
}
