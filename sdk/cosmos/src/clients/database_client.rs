use super::*;
use crate::operations::*;
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::prelude::Continuation;
use azure_core::{AddAsHeader, Context};
use futures::stream::unfold;
use futures::Stream;

/// Macro for short cutting a stream on error
macro_rules! r#try {
    ($expr:expr $(,)?) => {
        match $expr {
            Result::Ok(val) => val,
            Result::Err(err) => {
                return Some((Err(err.into()), State::Done));
            }
        }
    };
}

/// Stream state
#[derive(Debug, Clone, PartialEq)]
enum State {
    Init,
    Continuation(String),
    Done,
}

/// A client for Cosmos database resources.
#[derive(Debug, Clone)]
pub struct DatabaseClient {
    cosmos_client: CosmosClient,
    database_name: ReadonlyString,
}

impl DatabaseClient {
    pub(crate) fn new<S: Into<ReadonlyString>>(
        cosmos_client: CosmosClient,
        database_name: S,
    ) -> Self {
        Self {
            cosmos_client,
            database_name: database_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        &self.cosmos_client
    }

    /// Get the database's name
    pub fn database_name(&self) -> &str {
        &self.database_name
    }

    /// Get the database
    pub async fn get_database(
        &self,
        ctx: Context,
        options: GetDatabaseOptions,
    ) -> crate::Result<GetDatabaseResponse> {
        self.run_dbs_pipeline(ctx, http::Method::GET, |request| {
            options.decorate_request(request)
        })
        .await
    }

    /// Delete the database
    pub async fn delete_database(
        &self,
        ctx: Context,
        options: DeleteDatabaseOptions,
    ) -> crate::Result<DeleteDatabaseResponse> {
        self.run_dbs_pipeline(ctx, http::Method::DELETE, |request| {
            options.decorate_request(request)
        })
        .await
    }

    /// List collections in the database
    pub fn list_collections(
        &self,
        ctx: Context,
        options: ListCollectionsOptions,
    ) -> impl Stream<Item = crate::Result<ListCollectionsResponse>> + '_ {
        unfold(State::Init, move |state: State| {
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => {
                        self.run_colls_pipeline(ctx, http::Method::GET, |req| {
                            options.decorate_request(req)
                        })
                        .await
                    }
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        self.run_colls_pipeline(ctx, http::Method::GET, |req| {
                            options.decorate_request(req)?;
                            continuation.add_as_header2(req)?;
                            Ok(())
                        })
                        .await
                    }
                    State::Done => return None,
                };

                let response: ListCollectionsResponse = r#try!(response);

                let next_state = response
                    .continuation_token
                    .clone()
                    .map(State::Continuation)
                    .unwrap_or_else(|| State::Done);

                Some((Ok(response), next_state))
            }
        })
    }

    /// Create a collection
    pub async fn create_collection<S: AsRef<str>>(
        &self,
        ctx: Context,
        collection_name: S,
        options: CreateCollectionOptions,
    ) -> crate::Result<CreateCollectionResponse> {
        self.run_colls_pipeline(ctx, http::Method::POST, |request| {
            options.decorate_request(request, collection_name.as_ref())
        })
        .await
    }

    /// List users
    pub fn list_users(
        &self,
        ctx: Context,
        options: ListUsersOptions,
    ) -> impl Stream<Item = crate::Result<ListUsersResponse>> + '_ {
        unfold(State::Init, move |state: State| {
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => {
                        self.run_users_pipeline(ctx, http::Method::GET, |req| {
                            options.decorate_request(req)
                        })
                        .await
                    }
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        self.run_users_pipeline(ctx, http::Method::GET, |req| {
                            options.decorate_request(req)?;
                            continuation.add_as_header2(req)?;
                            Ok(())
                        })
                        .await
                    }
                    State::Done => return None,
                };

                let response: ListUsersResponse = r#try!(response);

                let next_state = response
                    .continuation_token
                    .clone()
                    .map(State::Continuation)
                    .unwrap_or_else(|| State::Done);

                Some((Ok(response), next_state))
            }
        })
    }

    /// Convert into a [`CollectionClient`]
    pub fn into_collection_client<S: Into<ReadonlyString>>(
        self,
        collection_name: S,
    ) -> CollectionClient {
        CollectionClient::new(self, collection_name)
    }

    /// Convert into a [`UserClient`]
    pub fn into_user_client<S: Into<ReadonlyString>>(self, user_name: S) -> UserClient {
        UserClient::new(self, user_name)
    }

    async fn run_dbs_pipeline<'a, F, T>(
        &'a self,
        ctx: Context,
        method: http::Method,
        options: F,
    ) -> Result<T, crate::Error>
    where
        F: FnOnce(&mut azure_core::Request) -> crate::Result<()> + 'a,
        T: azure_core::util::AsyncTryFrom<azure_core::Response, Error = crate::Error>,
    {
        self.cosmos_client()
            .run_pipeline(
                ctx,
                &format!("dbs/{}", self.database_name()),
                method,
                ResourceType::Databases,
                options,
            )
            .await
    }

    async fn run_colls_pipeline<'a, F, T>(
        &'a self,
        ctx: Context,
        method: http::Method,
        options: F,
    ) -> Result<T, crate::Error>
    where
        F: FnOnce(&mut azure_core::Request) -> crate::Result<()> + 'a,
        T: azure_core::util::AsyncTryFrom<azure_core::Response, Error = crate::Error>,
    {
        self.cosmos_client()
            .run_pipeline(
                ctx,
                &format!("dbs/{}/colls", self.database_name()),
                method,
                ResourceType::Collections,
                options,
            )
            .await
    }

    async fn run_users_pipeline<'a, F, T>(
        &'a self,
        ctx: Context,
        method: http::Method,
        options: F,
    ) -> Result<T, crate::Error>
    where
        F: FnOnce(&mut azure_core::Request) -> crate::Result<()> + 'a,
        T: azure_core::util::AsyncTryFrom<azure_core::Response, Error = crate::Error>,
    {
        self.cosmos_client()
            .run_pipeline(
                ctx,
                &format!("dbs/{}/users", self.database_name()),
                method,
                ResourceType::Users,
                options,
            )
            .await
    }
}
