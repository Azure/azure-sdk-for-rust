use crate::headers::from_headers::*;
use crate::prelude::*;

use azure_core::headers::session_token_from_headers;
use azure_core::prelude::*;
use azure_core::Response as HttpResponse;

operation! {
    DeleteDocument,
    client: DocumentClient,
    ?if_match_condition: IfMatchCondition,
    ?if_modified_since: IfModifiedSince,
    ?allow_tentative_writes: TentativeWritesAllowance,
    ?consistency_level: ConsistencyLevel
}

impl DeleteDocumentBuilder {
    pub fn into_future(self) -> DeleteDocument {
        Box::pin(async move {
            let mut request = self.client.document_request(azure_core::Method::Delete);

            request.insert_headers(&self.if_match_condition);
            request.insert_headers(&self.if_modified_since);
            if let Some(cl) = &self.consistency_level {
                request.insert_headers(cl);
            }
            request.insert_headers(
                &self
                    .allow_tentative_writes
                    .unwrap_or(TentativeWritesAllowance::Deny),
            );

            crate::cosmos_entity::add_as_partition_key_header_serialized(
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

#[derive(Debug, Clone)]
pub struct DeleteDocumentResponse {
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
}

impl DeleteDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
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
