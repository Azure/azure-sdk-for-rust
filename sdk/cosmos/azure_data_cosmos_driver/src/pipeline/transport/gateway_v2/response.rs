// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Maps decoded RNTBD responses to HTTP-shaped transport data.

use azure_core::http::headers::Headers;

use crate::{models::cosmos_headers::response_header_names, pipeline::transport::gateway_v2};

use super::super::cosmos_transport_client::HttpResponse;
use crate::rntbd_encoding::RntbdResponse;

use super::{X_MS_GLOBAL_COMMITTED_LSN, X_MS_LSN};

/// Decodes a Gateway 2.0 RNTBD response body into a synthetic HTTP response.
pub(crate) fn unwrap_response_for_gateway_v2(
    response: HttpResponse,
) -> azure_core::Result<HttpResponse> {
    let response = RntbdResponse::read(&response.body)?;
    let status = u16::from(response.status.status_code());
    if !(100..=599).contains(&status) {
        return Err(gateway_v2::data_conversion_error(format!(
            "Gateway 2.0 RNTBD response contained invalid HTTP status {status}"
        )));
    }

    let mut headers = Headers::new();
    headers.insert(
        response_header_names::ACTIVITY_ID,
        response.activity_id.to_string(),
    );
    if let Some(charge) = response.request_charge {
        headers.insert(response_header_names::REQUEST_CHARGE, charge.to_string());
    }
    if let Some(duration_ms) = response.backend_request_duration_ms {
        headers.insert(
            response_header_names::SERVER_DURATION_MS,
            duration_ms.to_string(),
        );
    }
    if let Some(value) = response.last_state_change_date_time {
        headers.insert(response_header_names::LAST_STATE_CHANGE_UTC, value);
    }
    if let Some(value) = response.storage_max_resource_quota {
        headers.insert(response_header_names::RESOURCE_QUOTA, value);
    }
    if let Some(value) = response.storage_resource_quota_usage {
        headers.insert(response_header_names::RESOURCE_USAGE, value);
    }
    if let Some(value) = response.schema_version {
        headers.insert(response_header_names::SCHEMA_VERSION, value);
    }
    if let Some(value) = response.item_count {
        headers.insert(response_header_names::ITEM_COUNT, value.to_string());
    }
    if let Some(value) = response.owner_id {
        headers.insert(response_header_names::OWNER_ID, value);
    }
    if let Some(value) = response.quorum_acked_lsn {
        headers.insert(response_header_names::QUORUM_ACKED_LSN, value.to_string());
    }
    if let Some(value) = response.current_write_quorum {
        headers.insert(
            response_header_names::CURRENT_WRITE_QUORUM,
            value.to_string(),
        );
    }
    if let Some(value) = response.current_replica_set_size {
        headers.insert(
            response_header_names::CURRENT_REPLICA_SET_SIZE,
            value.to_string(),
        );
    }
    if let Some(value) = response.xp_role {
        headers.insert(response_header_names::XP_ROLE, value.to_string());
    }
    if let Some(value) = response.number_of_read_regions {
        headers.insert(
            response_header_names::NUMBER_OF_READ_REGIONS,
            value.to_string(),
        );
    }
    if let Some(value) = response.local_lsn.filter(|value| *value >= 0) {
        headers.insert(response_header_names::LOCAL_LSN, value.to_string());
    }
    if let Some(value) = response.quorum_acked_local_lsn {
        headers.insert(
            response_header_names::QUORUM_ACKED_LOCAL_LSN,
            value.to_string(),
        );
    }
    if let Some(value) = response.item_local_lsn.filter(|value| *value >= 0) {
        headers.insert(response_header_names::ITEM_LOCAL_LSN, value.to_string());
    }
    if let Some(value) = response.query_execution_info {
        headers.insert(response_header_names::QUERY_EXECUTION_INFO, value);
    }
    if let Some(value) = response.pending_pk_delete {
        headers.insert(
            response_header_names::PENDING_PK_DELETE,
            if value { "True" } else { "False" },
        );
    }
    if let Some(value) = response.physical_partition_id {
        headers.insert(response_header_names::PHYSICAL_PARTITION_ID, value);
    }
    if let Some(value) = response.conflict_resolved_timestamp {
        headers.insert(
            response_header_names::CONFLICT_RESOLVED_TIMESTAMP,
            value.to_string(),
        );
    }
    if let Some(value) = response.transport_request_id {
        headers.insert(
            response_header_names::TRANSPORT_REQUEST_ID,
            value.to_string(),
        );
    }
    if let Some(value) = response.partition_key_range_id.as_ref() {
        headers.insert(response_header_names::PARTITION_KEY_RANGE_ID, value.clone());
    }
    if let Some(token) = response.session_token {
        // GW2 surfaces only the vector portion of the session token, with the
        // partition key range id carried separately. Classic gateway emits
        // `<pkRangeId>:<vector>`, and the backend rejects a bare vector with a
        // terminal 400 on re-supply, so re-compose the composite form here.
        let composite = match &response.partition_key_range_id {
            Some(pk_range_id) if !token.contains(':') => format!("{pk_range_id}:{token}"),
            _ => token,
        };
        headers.insert(response_header_names::SESSION_TOKEN, composite);
    }
    if let Some(etag) = response.etag {
        headers.insert(response_header_names::ETAG, etag);
    }
    if let Some(continuation) = response.continuation_token {
        headers.insert(response_header_names::CONTINUATION, continuation);
    }
    if let Some(substatus) = response.status.sub_status() {
        headers.insert(
            response_header_names::SUBSTATUS,
            substatus.value().to_string(),
        );
    }
    if let Some(retry_after_ms) = response.retry_after_ms {
        headers.insert("x-ms-retry-after-ms", retry_after_ms.to_string());
    }
    if let Some(lsn) = response.lsn {
        let value = lsn.to_string();
        headers.insert(response_header_names::LSN, value.clone());
        headers.insert(X_MS_LSN, value);
    }
    if let Some(item_lsn) = response.item_lsn {
        headers.insert(response_header_names::ITEM_LSN, item_lsn.to_string());
    }
    if let Some(global_committed_lsn) = response.global_committed_lsn {
        headers.insert(X_MS_GLOBAL_COMMITTED_LSN, global_committed_lsn.to_string());
    }
    if let Some(owner_full_name) = response.owner_full_name {
        headers.insert(response_header_names::OWNER_FULL_NAME, owner_full_name);
    }
    if let Some(query_metrics) = response.query_metrics {
        headers.insert(response_header_names::QUERY_METRICS, query_metrics);
    }
    if let Some(index_utilization) = response.index_utilization {
        headers.insert(response_header_names::INDEX_METRICS, index_utilization);
    }

    Ok(HttpResponse {
        status,
        headers,
        body: response.body,
    })
}
