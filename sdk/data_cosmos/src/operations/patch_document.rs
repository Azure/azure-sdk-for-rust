use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::DocumentAttributes;
use crate::ResourceQuota;

use azure_core::headers::session_token_from_headers;
use azure_core::Response as HttpResponse;
use azure_core::SessionToken;
use serde::Serialize;
use serde_json::Value;
use time::OffsetDateTime;

operation! {
    PatchDocument,
    client: DocumentClient,
    operations: Vec<Operation>,
    ?condition: String
}

impl PatchDocumentBuilder {
    pub fn into_future(self) -> PatchDocument {
        Box::pin(async move {
            let mut request = self.client.document_request(azure_core::Method::Patch);

            crate::cosmos_entity::add_as_partition_key_header_serialized(
                self.client.partition_key_serialized(),
                &mut request,
            );

            let patch_request = PatchDocumentRequest {
                condition: self.condition,
                operations: self.operations,
            };

            let serialized = azure_core::to_json(&patch_request)?;
            request.set_body(serialized);

            let response = self
                .client
                .cosmos_client()
                .pipeline()
                .send(
                    self.context.clone().insert(ResourceType::Documents),
                    &mut request,
                )
                .await?;

            PatchDocumentResponse::try_from(response).await
        })
    }
}

#[derive(Serialize, Debug)]
struct PatchDocumentRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    operations: Vec<Operation>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "op")]
pub enum Operation {
    Add { path: String, value: Value },
    Remove { path: String },
    Set { path: String, value: Value },
    Incr { path: String, value: Value },
    Replace { path: String, value: Value },
    Move { path: String, from: String },
}

impl Operation {
    pub fn add<P: Into<String>, V: Serialize>(
        path: P,
        value: V,
    ) -> Result<Operation, serde_json::Error> {
        Ok(Operation::Add {
            path: path.into(),
            value: serde_json::to_value(value)?,
        })
    }

    pub fn remove<P: Into<String>>(path: P) -> Operation {
        Operation::Remove { path: path.into() }
    }

    pub fn set<P: Into<String>, V: Serialize>(
        path: P,
        value: V,
    ) -> Result<Operation, serde_json::Error> {
        Ok(Operation::Set {
            path: path.into(),
            value: serde_json::to_value(value)?,
        })
    }

    pub fn incr<P: Into<String>, V: Serialize>(
        path: P,
        value: V,
    ) -> Result<Operation, serde_json::Error> {
        Ok(Operation::Incr {
            path: path.into(),
            value: serde_json::to_value(value)?,
        })
    }

    pub fn replace<P: Into<String>, V: Serialize>(
        path: P,
        value: V,
    ) -> Result<Operation, serde_json::Error> {
        Ok(Operation::Replace {
            path: path.into(),
            value: serde_json::to_value(value)?,
        })
    }

    pub fn r#move<P: Into<String>, F: Into<String>>(path: P, from: F) -> Operation {
        Operation::Move {
            path: path.into(),
            from: from.into(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct PatchDocumentResponse {
    pub document_attributes: DocumentAttributes,
    pub content_location: Option<String>,
    pub last_state_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub schema_version: String,
    pub alt_content_path: String,
    pub content_path: String,
    pub quorum_acked_lsn: Option<u64>,
    pub current_write_quorum: Option<u64>,
    pub current_replica_set_size: Option<u64>,
    pub role: u32,
    pub global_committed_lsn: u64,
    pub number_of_read_regions: u32,
    pub transport_request_id: u64,
    pub cosmos_llsn: u64,
    pub cosmos_quorum_acked_llsn: Option<u64>,
    pub session_token: SessionToken,
    pub charge: f64,
    pub service_version: String,
    pub activity_id: uuid::Uuid,
    pub gateway_version: String,
    pub date: OffsetDateTime,
}

impl PatchDocumentResponse {
    pub async fn try_from(response: HttpResponse) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        let document_attributes = serde_json::from_slice(&body)?;

        Ok(Self {
            content_location: content_location_from_headers(&headers)?,
            last_state_change: last_state_change_from_headers(&headers)?,
            resource_quota: resource_quota_from_headers(&headers)?,
            resource_usage: resource_usage_from_headers(&headers)?,
            lsn: lsn_from_headers(&headers)?,
            schema_version: schema_version_from_headers(&headers)?,
            alt_content_path: alt_content_path_from_headers(&headers)?,
            content_path: content_path_from_headers(&headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers_optional(&headers)?,
            current_write_quorum: current_write_quorum_from_headers_optional(&headers)?,
            current_replica_set_size: current_replica_set_size_from_headers_optional(&headers)?,
            role: role_from_headers(&headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(&headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(&headers)?,
            transport_request_id: transport_request_id_from_headers(&headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(&headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(&headers)?,
            session_token: session_token_from_headers(&headers)?,
            charge: request_charge_from_headers(&headers)?,
            service_version: service_version_from_headers(&headers)?,
            activity_id: activity_id_from_headers(&headers)?,
            gateway_version: gateway_version_from_headers(&headers)?,
            date: date_from_headers(&headers)?,
            document_attributes,
        })
    }
}
