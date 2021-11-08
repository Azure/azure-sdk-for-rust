use super::*;
use crate::authorization_policy::CosmosContext;
use crate::operations::*;
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::pipeline::Pipeline;
use azure_core::prelude::Continuation;
use azure_core::{AddAsHeader, Context, PipelineContext};
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
        let mut request = self
            .cosmos_client()
            .prepare_request_pipeline(&format!("dbs/{}", self.database_name()), http::Method::GET);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(GetDatabaseResponse::try_from(response).await?)
    }

    /// Delete the database
    pub async fn delete_database(
        &self,
        ctx: Context,
        options: DeleteDatabaseOptions,
    ) -> crate::Result<DeleteDatabaseResponse> {
        let mut request = self.cosmos_client().prepare_request_pipeline(
            &format!("dbs/{}", self.database_name()),
            http::Method::DELETE,
        );
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request)?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(DeleteDatabaseResponse::try_from(response).await?)
    }

    /// List collections in the database
    pub fn list_collections(
        &self,
        ctx: Context,
        options: ListCollectionsOptions,
    ) -> impl Stream<Item = crate::Result<ListCollectionsResponse>> + '_ {
        unfold(State::Init, move |state: State| {
            let this = self.clone();
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => {
                        let mut request = this.cosmos_client().prepare_request_pipeline(
                            &format!("dbs/{}/colls", this.database_name()),
                            http::Method::GET,
                        );
                        let mut pipeline_context =
                            PipelineContext::new(ctx.clone(), ResourceType::Collections.into());

                        r#try!(options.decorate_request(&mut request));
                        let response = r#try!(
                            this.pipeline()
                                .send(&mut pipeline_context, &mut request)
                                .await
                        );
                        ListCollectionsResponse::try_from(response).await
                    }
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        let mut request = this.cosmos_client().prepare_request_pipeline(
                            &format!("dbs/{}/colls", self.database_name()),
                            http::Method::GET,
                        );
                        let mut pipeline_context =
                            PipelineContext::new(ctx.clone(), ResourceType::Collections.into());

                        r#try!(options.decorate_request(&mut request));
                        r#try!(continuation.add_as_header2(&mut request));
                        let response = r#try!(
                            this.pipeline()
                                .send(&mut pipeline_context, &mut request)
                                .await
                        );
                        ListCollectionsResponse::try_from(response).await
                    }
                    State::Done => return None,
                };

                let response = r#try!(response);

                let next_state = response
                    .continuation_token
                    .clone()
                    .map(State::Continuation)
                    .unwrap_or(State::Done);

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
        let mut request = self.cosmos_client().prepare_request_pipeline(
            &format!("dbs/{}/colls", self.database_name()),
            http::Method::POST,
        );
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Collections.into());

        options.decorate_request(&mut request, collection_name.as_ref())?;
        let response = self
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?;

        Ok(CreateCollectionResponse::try_from(response).await?)
    }

    /// List users
    pub fn list_users(
        &self,
        ctx: Context,
        options: ListUsersOptions,
    ) -> impl Stream<Item = crate::Result<ListUsersResponse>> + '_ {
        unfold(State::Init, move |state: State| {
            let this = self.clone();
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response = match state {
                    State::Init => {
                        let mut request = this.cosmos_client().prepare_request_pipeline(
                            &format!("dbs/{}/users", this.database_name()),
                            http::Method::GET,
                        );
                        let mut pipeline_context =
                            PipelineContext::new(ctx.clone(), ResourceType::Users.into());

                        r#try!(options.decorate_request(&mut request));
                        let response = r#try!(
                            this.pipeline()
                                .send(&mut pipeline_context, &mut request)
                                .await
                        );
                        ListUsersResponse::try_from(response).await
                    }
                    State::Continuation(continuation_token) => {
                        let continuation = Continuation::new(continuation_token.as_str());
                        let mut request = this.cosmos_client().prepare_request_pipeline(
                            &format!("dbs/{}/users", self.database_name()),
                            http::Method::GET,
                        );
                        let mut pipeline_context =
                            PipelineContext::new(ctx.clone(), ResourceType::Users.into());

                        r#try!(options.decorate_request(&mut request));
                        r#try!(continuation.add_as_header2(&mut request));
                        let response = r#try!(
                            this.pipeline()
                                .send(&mut pipeline_context, &mut request)
                                .await
                        );
                        ListUsersResponse::try_from(response).await
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

    fn pipeline(&self) -> &Pipeline<CosmosContext> {
        self.cosmos_client.pipeline()
    }
}
