use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::DocumentAttributes;
use crate::ResourceQuota;

use azure_core::headers::session_token_from_headers;
use azure_core::Context;
use azure_core::Response as HttpResponse;
use azure_core::SessionToken;
use serde::Serialize;
use time::OffsetDateTime;

// operation! {
//     PatchDocument<V: Serialize + Send + 'static>,
//     client: DocumentClient,
//     operations: Vec<Operation<V>>,
//     ?condition: String,
// }

#[derive(Debug, Clone)]
pub struct PatchDocumentBuilder<V: Serialize + Send + 'static> {
    client: DocumentClient,
    operations: Vec<Operation<V>>,
    condition: Option<String>,
    context: Context,
}

impl<V: Serialize + Send + 'static> PatchDocumentBuilder<V> {
    pub(crate) fn new(client: DocumentClient, operations: Vec<Operation<V>>) -> Self {
        Self {
            client,
            operations,
            condition: None,
            context: Context::new(),
        }
    }

    setters! {
        condition: String => Some(condition),
        context: Context => context,
    }
}

impl<V: Serialize + Send + 'static> std::future::IntoFuture for PatchDocumentBuilder<V> {
    type IntoFuture = PatchDocument;
    type Output = <PatchDocument as std::future::Future>::Output;
    fn into_future(self) -> Self::IntoFuture {
        Self::into_future(self)
    }
}

/// The future returned by calling `into_future` on the builder.
pub type PatchDocument =
    futures::future::BoxFuture<'static, azure_core::Result<PatchDocumentResponse>>;

impl<V: Serialize + Send + 'static> PatchDocumentBuilder<V> {
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
struct PatchDocumentRequest<V: Serialize + Send + 'static> {
    #[serde(skip_serializing_if = "Option::is_none")]
    condition: Option<String>,
    operations: Vec<Operation<V>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
#[serde(tag = "op")]
pub enum Operation<V: Serialize + Send + 'static> {
    Add { path: String, value: V },
    Remove { path: String },
    Set { path: String, value: V },
    Incr { path: String, value: V },
    Replace { path: String, value: V },
    Move { path: String, from: String },
}

impl<V: Serialize + Send + 'static> Operation<V> {
    pub fn add<P: Into<String>>(path: P, value: V) -> Operation<V> {
        Operation::Add {
            path: path.into(),
            value,
        }
    }

    pub fn remove<P: Into<String>>(path: P) -> Operation<V> {
        Operation::Remove { path: path.into() }
    }

    pub fn set<P: Into<String>>(path: P, value: V) -> Operation<V> {
        Operation::Set {
            path: path.into(),
            value,
        }
    }

    pub fn incr<P: Into<String>>(path: P, value: V) -> Operation<V> {
        Operation::Incr {
            path: path.into(),
            value,
        }
    }

    pub fn replace<P: Into<String>>(path: P, value: V) -> Operation<V> {
        Operation::Replace {
            path: path.into(),
            value,
        }
    }

    pub fn r#move<P: Into<String>, F: Into<String>>(path: P, from: F) -> Operation<V> {
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
