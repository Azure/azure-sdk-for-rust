use crate::prelude::*;
use crate::resources::ResourceType;
use crate::ReadonlyString;
use azure_core::prelude::ContentType;
use azure_core::{Context, PipelineContext, Request as HttpRequest};
use bytes::Bytes;

use super::*;

/// A client for Cosmos attachment resources.
#[derive(Debug, Clone)]
pub struct AttachmentClient {
    document_client: DocumentClient,
    attachment_name: ReadonlyString,
}

impl AttachmentClient {
    /// Create a new client
    pub(crate) fn new<S: Into<ReadonlyString>>(
        document_client: DocumentClient,
        attachment_name: S,
    ) -> Self {
        Self {
            document_client,
            attachment_name: attachment_name.into(),
        }
    }

    /// Get a [`CosmosClient`].
    pub fn cosmos_client(&self) -> &CosmosClient {
        self.document_client().cosmos_client()
    }

    /// Get a [`DatabaseClient`].
    pub fn database_client(&self) -> &DatabaseClient {
        self.document_client().database_client()
    }

    /// Get a [`CollectionClient`].
    pub fn collection_client(&self) -> &CollectionClient {
        self.document_client().collection_client()
    }

    /// Get a [`DocumentClient`].
    pub fn document_client(&self) -> &DocumentClient {
        &self.document_client
    }

    /// Get the attachment name.
    pub fn attachment_name(&self) -> &str {
        &self.attachment_name
    }

    /// Get an attachment.
    pub async fn get(
        &self,
        ctx: Context,
        options: GetAttachmentOptions<'_>,
    ) -> Result<GetAttachmentResponse, crate::Error> {
        let mut request = self.prepare_request_with_attachment_name(http::Method::GET);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());
      
        options.decorate_request(
            &mut request,
            self.document_client().partition_key_serialized(),
        )?;
        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        GetAttachmentResponse::try_from(response).await
    }

    /// Initiate a request to delete an attachment.
    pub async fn delete(
        &self,
        ctx: Context,
        options: DeleteAttachmentOptions<'_, '_>,
    ) -> Result<DeleteAttachmentResponse, crate::Error> {
        let mut request = self.prepare_request_with_attachment_name(http::Method::DELETE);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request)?;
        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::NO_CONTENT)
            .await?;

        DeleteAttachmentResponse::try_from(response).await
    }

    /// Initiate a request to create an attachment with a slug.
    pub async fn create_slug<B: Into<Bytes>>(
        &self,
        ctx: Context,
        body: B,
        options: CreateSlugAttachmentOptions<'_, '_>,
    ) -> Result<CreateSlugAttachmentResponse, crate::Error> {
        let mut request = self.prepare_request(http::Method::POST);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request, body)?;
        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::CREATED)
            .await?;

        CreateSlugAttachmentResponse::try_from(response).await
    }

    /// Initiate a request to replace an attachment.
    pub async fn replace_slug<B: Into<Bytes>>(
        &self,
        ctx: Context,
        body: B,
        options: ReplaceSlugAttachmentOptions<'_, '_>,
    ) -> Result<ReplaceSlugAttachmentResponse, crate::Error> {
        let mut request = self.prepare_request_with_attachment_name(http::Method::PUT);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request, body)?;
        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        ReplaceSlugAttachmentResponse::try_from(response).await
    }

    /// Initiate a request to create an attachment.
    pub async fn create_reference<'c, M, C>(
        &self,
        ctx: Context,
        media: M,
        content_type: C,
        options: CreateReferenceAttachmentOptions<'_, '_>,
    ) -> Result<CreateReferenceAttachmentResponse, crate::Error>
    where
        M: AsRef<str>,
        C: Into<ContentType<'c>>,
    {
        let mut request = self.prepare_request(http::Method::POST);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request, media, content_type)?;
        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::CREATED)
            .await?;

        CreateReferenceAttachmentResponse::try_from(response).await
    }

    /// Initiate a request to replace an attachment.
    pub async fn replace_reference<'c, M, C>(
        &self,
        ctx: Context,
        media: M,
        content_type: C,
        options: ReplaceReferenceAttachmentOptions<'_, '_>,
    ) -> Result<ReplaceReferenceAttachmentResponse, crate::Error>
    where
        M: AsRef<str>,
        C: Into<ContentType<'c>>,
    {
        let mut request = self.prepare_request_with_attachment_name(http::Method::PUT);
        let mut pipeline_context = PipelineContext::new(ctx, ResourceType::Databases.into());

        options.decorate_request(&mut request, media, content_type)?;
        let response = self
            .cosmos_client()
            .pipeline()
            .send(&mut pipeline_context, &mut request)
            .await?
            .validate(http::StatusCode::OK)
            .await?;

        ReplaceReferenceAttachmentResponse::try_from(response).await
    }

    pub(crate) fn prepare_request(&self, method: http::Method) -> HttpRequest {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
            ),
            method,
        )
    }

    pub(crate) fn prepare_request_with_attachment_name(&self, method: http::Method) -> HttpRequest {
        self.cosmos_client().prepare_request_pipeline(
            &format!(
                "dbs/{}/colls/{}/docs/{}/attachments/{}",
                self.database_client().database_name(),
                self.collection_client().collection_name(),
                self.document_client().document_name(),
                self.attachment_name()
            ),
            method,
        )
    }
}
