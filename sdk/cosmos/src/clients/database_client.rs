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
        let response = self
            .cosmos_client()
            .run_pipeline(
                ctx,
                &format!("dbs/{}", self.database_name()),
                http::Method::GET,
                ResourceType::Databases,
                |request: &mut azure_core::Request| options.decorate_request(request),
            )
            .await?;

        Ok(GetDatabaseResponse::try_from(response).await?)
    }

    /// Delete the database
    pub async fn delete_database(
        &self,
        ctx: Context,
        options: DeleteDatabaseOptions,
    ) -> crate::Result<DeleteDatabaseResponse> {
        let response = self
            .cosmos_client()
            .run_pipeline(
                ctx,
                &format!("dbs/{}", self.database_name()),
                http::Method::GET,
                ResourceType::Databases,
                |request| options.decorate_request(request),
            )
            .await?;

        Ok(DeleteDatabaseResponse::try_from(response).await?)
    }

    /// List collections in the database
    pub fn list_collections(
        &self,
        ctx: Context,
        options: ListCollectionsOptions,
    ) -> impl Stream<Item = crate::Result<ListCollectionsResponse>> + '_ {
        async fn do_request<'a, F>(
            this: &'a DatabaseClient,
            ctx: Context,
            options: F,
        ) -> crate::Result<ListCollectionsResponse>
        where
            F: FnOnce(&mut azure_core::Request) -> crate::Result<()> + 'a,
        {
            let response = this
                .cosmos_client()
                .run_pipeline(
                    ctx,
                    &format!("dbs/{}/colls", this.database_name()),
                    http::Method::GET,
                    ResourceType::Users,
                    options,
                )
                .await?;
            ListCollectionsResponse::try_from(response).await
        }
        unfold(State::Init, move |state: State| {
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => do_request(self, ctx, |req| options.decorate_request(req)).await,
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        do_request(self, ctx, |request| {
                            options.decorate_request(request)?;
                            continuation.add_as_header2(request)?;
                            Ok(())
                        })
                        .await
                    }
                    State::Done => return None,
                };

                let response = r#try!(response);

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
        let response = self
            .cosmos_client()
            .run_pipeline(
                ctx,
                &format!("dbs/{}/colls", self.database_name()),
                http::Method::POST,
                ResourceType::Collections,
                |request| options.decorate_request(request, collection_name.as_ref()),
            )
            .await?;

        Ok(CreateCollectionResponse::try_from(response).await?)
    }

    /// List users
    pub fn list_users(
        &self,
        ctx: Context,
        options: ListUsersOptions,
    ) -> impl Stream<Item = crate::Result<ListUsersResponse>> + '_ {
        async fn do_request<'a, F>(
            this: &'a DatabaseClient,
            ctx: Context,
            options: F,
        ) -> crate::Result<ListUsersResponse>
        where
            F: FnOnce(&mut azure_core::Request) -> crate::Result<()> + 'a,
        {
            let response = this
                .cosmos_client()
                .run_pipeline(
                    ctx,
                    &format!("dbs/{}/users", this.database_name()),
                    http::Method::GET,
                    ResourceType::Users,
                    options,
                )
                .await?;
            ListUsersResponse::try_from(response).await
        }
        unfold(State::Init, move |state: State| {
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => do_request(self, ctx, |req| options.decorate_request(req)).await,
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        do_request(self, ctx, |request| {
                            options.decorate_request(request)?;
                            continuation.add_as_header2(request)?;
                            Ok(())
                        })
                        .await
                    }
                    State::Done => return None,
                };

                let response = r#try!(response);

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
}
