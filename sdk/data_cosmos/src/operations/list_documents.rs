use crate::headers::from_headers::*;
use crate::prelude::*;
use crate::resources::document::{Document, DocumentAttributes};
use crate::resources::ResourceType;
use crate::ResourceQuota;
use azure_core::headers::{
    continuation_token_from_headers_optional, item_count_from_headers, session_token_from_headers,
};
use azure_core::{prelude::*, Pageable};
use azure_core::{Response, SessionToken};
use serde::de::DeserializeOwned;
use time::OffsetDateTime;

operation! {
    #[stream]
    ListDocuments,
    client: CollectionClient,
    ?max_item_count: MaxItemCount,
    ?a_im: ChangeFeed,
    ?if_match_condition: IfMatchCondition,
    ?consistency_level: ConsistencyLevel,
    ?partition_range_id: PartitionRangeId
}

impl ListDocumentsBuilder {
    pub fn into_stream<T>(self) -> ListDocuments<T>
    where
        T: DeserializeOwned + Send + Sync,
    {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let ctx = self.context.clone();
            async move {
                let mut req = this.client.cosmos_client().request(
                    &format!(
                        "dbs/{}/colls/{}/docs",
                        this.client.database_client().database_name(),
                        this.client.collection_name()
                    ),
                    azure_core::Method::Get,
                );

                req.insert_headers(&this.if_match_condition);
                if let Some(cl) = &this.consistency_level {
                    req.insert_headers(cl);
                }
                req.insert_headers(&this.max_item_count.unwrap_or_default());
                req.insert_headers(&this.a_im.unwrap_or_default());
                req.insert_headers(&this.partition_range_id);
                req.insert_headers(&continuation);

                let response = this
                    .client
                    .pipeline()
                    .send(ctx.clone().insert(ResourceType::Documents), &mut req)
                    .await?;

                ListDocumentsResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

pub type ListDocuments<T> = Pageable<ListDocumentsResponse<T>, azure_core::error::Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDocumentsResponseAttributes {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub documents: Vec<DocumentAttributes>,
}

#[derive(Debug, Clone)]
pub struct ListDocumentsResponse<T> {
    pub rid: String,
    pub documents: Vec<Document<T>>,
    pub content_location: Option<String>,
    pub last_state_change: OffsetDateTime,
    pub resource_quota: Vec<ResourceQuota>,
    pub resource_usage: Vec<ResourceQuota>,
    pub lsn: u64,
    pub item_count: u32,
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
    pub continuation_token: Option<Continuation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ListDocumentsResponseEntities<T> {
    #[serde(rename = "_rid")]
    pub rid: String,
    #[serde(rename = "Documents")]
    pub entities: Vec<T>,
}

impl<T> ListDocumentsResponse<T>
where
    T: DeserializeOwned,
{
    pub(crate) async fn try_from(response: Response) -> azure_core::Result<Self> {
        let (_status_code, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        let headers = &headers;

        // we will proceed in three steps:
        // 1- Deserialize the result as DocumentAttributes. The extra field will be ignored.
        // 2- Deserialize the result a type T. The extra fields will be ignored.
        // 3- Zip 1 and 2 in the resulting structure.
        // There is a lot of data movement here, let's hope the compiler is smarter than me :)
        let document_attributes: ListDocumentsResponseAttributes = serde_json::from_slice(&body)?;
        let entries: ListDocumentsResponseEntities<T> = serde_json::from_slice(&body)?;

        let documents = document_attributes
            .documents
            .into_iter()
            .zip(entries.entities.into_iter())
            .map(|(da, e)| Document {
                document_attributes: da,
                document: e,
            })
            .collect();

        Ok(ListDocumentsResponse {
            rid: document_attributes.rid,
            documents,
            content_location: content_location_from_headers(headers)?,
            last_state_change: last_state_change_from_headers(headers)?,
            resource_quota: resource_quota_from_headers(headers)?,
            resource_usage: resource_usage_from_headers(headers)?,
            lsn: lsn_from_headers(headers)?,
            item_count: item_count_from_headers(headers)?,
            schema_version: schema_version_from_headers(headers)?,
            alt_content_path: alt_content_path_from_headers(headers)?,
            content_path: content_path_from_headers(headers)?,
            quorum_acked_lsn: quorum_acked_lsn_from_headers_optional(headers)?,
            current_write_quorum: current_write_quorum_from_headers_optional(headers)?,
            current_replica_set_size: current_replica_set_size_from_headers_optional(headers)?,
            role: role_from_headers(headers)?,
            global_committed_lsn: global_committed_lsn_from_headers(headers)?,
            number_of_read_regions: number_of_read_regions_from_headers(headers)?,
            transport_request_id: transport_request_id_from_headers(headers)?,
            cosmos_llsn: cosmos_llsn_from_headers(headers)?,
            cosmos_quorum_acked_llsn: cosmos_quorum_acked_llsn_from_headers_optional(headers)?,
            session_token: session_token_from_headers(headers)?,
            charge: request_charge_from_headers(headers)?,
            service_version: service_version_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            gateway_version: gateway_version_from_headers(headers)?,
            continuation_token: continuation_token_from_headers_optional(headers)?,
            date: date_from_headers(headers)?,
        })
    }
}

impl<T> Continuable for ListDocumentsResponse<T> {
    type Continuation = Continuation;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.continuation_token.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BODY: &str = "
{
    \"_rid\": \"3iNTAJKxVCk=\",
    \"Documents\": [
        {
            \"color\": \"red\",
            \"myvalue\": \"#f00\",
            \"id\": \"c5d11a65-2e5a-3d9f-4de8-2447259dff38\",
            \"_rid\": \"3iNTAJKxVCkBAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkBAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100eb0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        },
        {
            \"color\": \"yellow\",
            \"myvalue\": \"#ff0\",
            \"id\": \"894dd5ff-573e-f38a-b8c4-5eae5071c900\",
            \"_rid\": \"3iNTAJKxVCkCAAAAAAAAAA==\",
            \"_self\": \"dbs/3iNTAA==/colls/3iNTAJKxVCk=/docs/3iNTAJKxVCkCAAAAAAAAAA==/\",
            \"_etag\": \"\\\"0100ec0a-0000-0c00-0000-5ded4fe30000\\\"\",
            \"_attachments\": \"attachments/\",
            \"_ts\": 1575833571
        }
    ],
    \"_count\": 7
}";

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    struct MyStruct {
        id: String,
        color: String,
        myvalue: String,
    }

    #[test]
    fn test_list_document() {
        let _document_attributes =
            serde_json::from_slice::<ListDocumentsResponseAttributes>(BODY.as_bytes()).unwrap();
        let _entries =
            serde_json::from_slice::<ListDocumentsResponseEntities<MyStruct>>(BODY.as_bytes())
                .unwrap();
    }
}
