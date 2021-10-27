use super::{AttachmentClient, CollectionClient, CosmosClient, DatabaseClient};
use crate::operations::{ListAttachmentsOptions, ListAttachmentsResponse};
use crate::prelude::{GetDocumentOptions, GetDocumentResponse};
use crate::resources::ResourceType;
use crate::{requests, ReadonlyString};
use azure_core::prelude::Continuation;
use azure_core::{AddAsHeader, Context, HttpClient, PipelineContext, Request};
use futures::stream::unfold;
use futures::Stream;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// A client for Cosmos document resources.
#[derive(Debug, Clone)]
pub struct DocumentClient {
    collection_client: CollectionClient,
    document_name: String,
    partition_key_serialized: String,
}

impl DocumentClient {
    /// This function creates a new instance of a DocumentClient. A document is identified by its
    /// primary key and its partition key. Partition key is eagerly evaluated: the json
    /// representation is generated as soon as you call the `new` function. This avoids doing the
    /// serialization over and over, saving time. It also releases the borrow since the serialized
    /// string is owned by the DocumentClient.
    pub(crate) fn new<S: Into<String>, PK: Serialize>(
        collection_client: CollectionClient,
        document_name: S,
        partition_key: &PK,
    ) -> Result<Self, serde_json::Error> {
        Ok(Self {
            collection_client,
            document_name: document_name.into(),
            partition_key_serialized: crate::cosmos_entity::serialize_partition_key(partition_key)?,
        })
    }

    /// Get a [`CosmosClient`]
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.collection_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`]
    pub fn database_client(&self) -> &DatabaseClient {
        self.collection_client().database_client()
    }

    /// Get a [`CollectionClient`]
    pub fn collection_client(&self) -> &CollectionClient {
        &self.collection_client
    }

    /// Get the document's name
    pub fn document_name(&self) -> &str {
        &self.document_name
    }

    /// Get the partition key
    pub fn partition_key_serialized(&self) -> &str {
        &self.partition_key_serialized
    }

    /// replace a document in a collection
    pub fn replace_document<'a>(&'a self) -> requests::ReplaceDocumentBuilder<'a, '_> {
        requests::ReplaceDocumentBuilder::new(self)
    }

    /// Get a document
    pub async fn get_document<T>(
        &self,
        ctx: Context,
        options: GetDocumentOptions<'_>,
    ) -> Result<GetDocumentResponse<T>, crate::Error>
    where
        T: DeserializeOwned,
    {
        let mut request = self.prepare_request_pipeline_with_document_name(http::Method::GET);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Documents.into());

        options.decorate_request(&mut request, self.partition_key_serialized())?;

        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        GetDocumentResponse::try_from(response).await
    }

    /// Delete a document
    pub fn delete_document(&self) -> requests::DeleteDocumentBuilder<'_> {
        requests::DeleteDocumentBuilder::new(self)
    }

    /// List all attachments for a document
    pub fn list_attachments<'a>(
        &'a self,
        ctx: Context,
        options: ListAttachmentsOptions<'a>,
    ) -> impl Stream<Item = Result<ListAttachmentsResponse, crate::Error>> + 'a {
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

        #[derive(Debug, Clone, PartialEq)]
        enum State {
            Init,
            Continuation(String),
            Done,
        }

        unfold(State::Init, move |state: State| {
            let this = self.clone();
            let ctx = ctx.clone();
            let options = options.clone();
            async move {
                let response =
                    match state {
                        State::Init => {
                            let mut request = this.cosmos_client().prepare_request_pipeline(
                                &format!(
                                    "dbs/{}/colls/{}/docs/{}/attachments",
                                    self.database_client().database_name(),
                                    self.collection_client().collection_name(),
                                    self.document_name(),
                                ),
                                http::Method::GET,
                            );
                            let mut pipeline_context =
                                PipelineContext::new(ctx.clone(), ResourceType::Attachments.into());

                            r#try!(options
                                .decorate_request(&mut request, self.partition_key_serialized()));
                            let response = r#try!(
                                this.cosmos_client()
                                    .pipeline()
                                    .send(&mut pipeline_context, &mut request)
                                    .await
                            );
                            let response = r#try!(response.validate(http::StatusCode::OK).await);
                            ListAttachmentsResponse::try_from(response).await
                        }
                        State::Continuation(continuation_token) => {
                            let continuation = Continuation::new(continuation_token.as_str());
                            let mut request = this.cosmos_client().prepare_request_pipeline(
                                &format!(
                                    "dbs/{}/colls/{}/docs/{}/attachments",
                                    self.database_client().database_name(),
                                    self.collection_client().collection_name(),
                                    self.document_name(),
                                ),
                                http::Method::GET,
                            );
                            let mut pipeline_context =
                                PipelineContext::new(ctx.clone(), ResourceType::Attachments.into());

                            r#try!(options
                                .decorate_request(&mut request, self.partition_key_serialized()));
                            r#try!(continuation.add_as_header2(&mut request));
                            let response = r#try!(
                                this.cosmos_client()
                                    .pipeline()
                                    .send(&mut pipeline_context, &mut request)
                                    .await
                            );
                            let response = r#try!(response.validate(http::StatusCode::OK).await);
                            ListAttachmentsResponse::try_from(response).await
                        }
                        State::Done => return None,
                    };

                let response = r#try!(response);

                let next_state = response
                    .continuation_token
                    .clone()
                    .map(|ct| State::Continuation(ct))
                    .unwrap_or_else(|| State::Done);

                Some((Ok(response), next_state))
            }
        })
    }

    /// Convert into an [`AttachmentClient`]
    pub fn into_attachment_client<S: Into<ReadonlyString>>(
        self,
        attachment_name: S,
    ) -> AttachmentClient {
        AttachmentClient::new(self, attachment_name)
    }

    pub(crate) fn prepare_request_with_document_name(
        &self,
        method: http::Method,
    ) -> http::request::Builder {
        self.cosmos_client().prepare_request(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_name()
            ),
            method,
            ResourceType::Documents,
        )
    }

    fn prepare_request_pipeline_with_document_name(&self, method: http::Method) -> Request {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_name()
            ),
            method,
        )
    }

    pub(crate) fn http_client(&self) -> &dyn HttpClient {
        self.cosmos_client().http_client()
    }
}
