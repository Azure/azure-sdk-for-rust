use crate::headers::from_headers::*;
use crate::prelude::*;

use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::Response as HttpResponse;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct DeleteDocumentBuilder {
    client: DocumentClient,
    if_match_condition: Option<IfMatchCondition>,
    if_modified_since: Option<IfModifiedSince>,
    consistency_level: Option<ConsistencyLevel>,
    allow_tentative_writes: TentativeWritesAllowance,
    context: Context,
}

impl DeleteDocumentBuilder {
    pub(crate) fn new(client: DocumentClient) -> DeleteDocumentBuilder {
        Self {
            client,
            if_match_condition: None,
            if_modified_since: None,
            consistency_level: None,
            allow_tentative_writes: TentativeWritesAllowance::Deny,
            context: Context::new(),
        }
    }

    setters! {
        consistency_level: ConsistencyLevel => Some(consistency_level),
        if_match_condition: IfMatchCondition => Some(if_match_condition),
        allow_tentative_writes: TentativeWritesAllowance,
        if_modified_since: DateTime<Utc> => Some(IfModifiedSince::new(if_modified_since)),
        context: Context => context,
    }

    pub fn into_future(self) -> DeleteDocument {
        Box::pin(async move {
            let mut request = self
                .client
                .prepare_request_pipeline_with_document_name(http::Method::DELETE);

            azure_core::headers::add_optional_header2(&self.if_match_condition, &mut request)?;
            azure_core::headers::add_optional_header2(&self.if_modified_since, &mut request)?;
            azure_core::headers::add_optional_header2(&self.consistency_level, &mut request)?;
            azure_core::headers::add_mandatory_header2(&self.allow_tentative_writes, &mut request)?;

            crate::cosmos_entity::add_as_partition_key_header_serialized2(
                self.client.partition_key_serialized(),
                &mut request,
            );

            let response = self
                .client
                .cosmos_client()
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Documents),
                    &mut request,
                )
                .await?;

            DeleteDocumentResponse::try_from(response).await
        })
    }
}

/// The future returned by calling `into_future` on the builder.
pub type DeleteDocument =
    futures::future::BoxFuture<'static, crate::Result<DeleteDocumentResponse>>;

#[cfg(feature = "into_future")]
impl std::future::IntoFuture for DeleteDocumentBuilder {
    type IntoFuture = DeleteDocument;
    type Output = <DeleteDocument as std::future::Future>::Output;
    fn into_future(self) -> Self::Future {
        Self::into_future(self)
    }
}

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl DeleteDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> crate::Result<Self> {
        let (_status_code, headers, _pinned_stream) = response.deconstruct();

        let charge = request_charge_from_headers(&headers)?;
        let activity_id = activity_id_from_headers(&headers)?;
        let session_token = session_token_from_headers(&headers)?;

        Ok(Self {
            charge,
            activity_id,
            session_token,
        })
    }
}
