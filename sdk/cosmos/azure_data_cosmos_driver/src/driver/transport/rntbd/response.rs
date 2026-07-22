// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! RNTBD response frame deserialization.

use uuid::Uuid;

use crate::models::CosmosStatus;

use super::{
    status::map_rntbd_status_to_cosmos_status,
    tokens::{data_conversion_error, read_u32_le, read_uuid_le, RntbdResponseToken, Token},
};

/// A decoded Gateway 2.0 RNTBD response frame.
///
/// The body is schema-agnostic raw bytes. Recognized metadata tokens are surfaced
/// as typed optional fields; unknown token IDs are silently consumed.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RntbdResponse {
    /// Status composed from the frame HTTP status and optional SubStatus token.
    pub(crate) status: CosmosStatus,
    /// Activity identifier echoed by the service.
    pub(crate) activity_id: Uuid,
    /// Raw response payload bytes.
    pub(crate) body: Vec<u8>,
    /// Continuation token for feed-style operations.
    pub(crate) continuation_token: Option<String>,
    /// Entity tag returned by the service.
    pub(crate) etag: Option<String>,
    /// Retry-after delay in milliseconds.
    pub(crate) retry_after_ms: Option<u32>,
    /// Timestamp of the last replica state change.
    pub(crate) last_state_change_date_time: Option<String>,
    /// Maximum resource quota values.
    pub(crate) storage_max_resource_quota: Option<String>,
    /// Current resource quota usage.
    pub(crate) storage_resource_quota_usage: Option<String>,
    /// Resource schema version.
    pub(crate) schema_version: Option<String>,
    /// Logical sequence number.
    pub(crate) lsn: Option<i64>,
    /// Number of items returned.
    pub(crate) item_count: Option<u32>,
    /// Request charge in request units.
    pub(crate) request_charge: Option<f64>,
    /// Backend request processing duration in milliseconds.
    pub(crate) backend_request_duration_ms: Option<f64>,
    /// Owner full name metadata.
    pub(crate) owner_full_name: Option<String>,
    /// Owner resource identifier.
    pub(crate) owner_id: Option<String>,
    /// Quorum-acknowledged logical sequence number.
    pub(crate) quorum_acked_lsn: Option<i64>,
    /// Current write quorum.
    pub(crate) current_write_quorum: Option<u32>,
    /// Current replica set size.
    pub(crate) current_replica_set_size: Option<u32>,
    /// Partition key range identifier.
    pub(crate) partition_key_range_id: Option<String>,
    /// Cross-partition role.
    pub(crate) xp_role: Option<u32>,
    /// Number of read regions.
    pub(crate) number_of_read_regions: Option<u32>,
    /// Item logical sequence number.
    pub(crate) item_lsn: Option<i64>,
    /// Global committed logical sequence number.
    pub(crate) global_committed_lsn: Option<i64>,
    /// Local logical sequence number.
    pub(crate) local_lsn: Option<i64>,
    /// Quorum-acknowledged local logical sequence number.
    pub(crate) quorum_acked_local_lsn: Option<i64>,
    /// Item local logical sequence number.
    pub(crate) item_local_lsn: Option<i64>,
    /// Transport request identifier.
    pub(crate) transport_request_id: Option<u32>,
    /// Session token for session consistency.
    pub(crate) session_token: Option<String>,
    /// Query execution metadata.
    pub(crate) query_execution_info: Option<String>,
    /// Whether partition-key deletion is pending.
    pub(crate) pending_pk_delete: Option<bool>,
    /// Physical partition identifier.
    pub(crate) physical_partition_id: Option<String>,
    /// Conflict resolution timestamp.
    pub(crate) conflict_resolved_timestamp: Option<u64>,
}

impl RntbdResponse {
    /// Reads a Gateway 2.0 RNTBD response frame.
    ///
    /// Wire layout:
    ///   * `u32` LE length — total bytes of `[u32 status + 16-byte activity id + metadata tokens]` (24 + metadata).
    ///     The length does NOT include the trailing body section.
    ///   * `u32` LE status (frame HTTP status).
    ///   * 16-byte activity id (Microsoft GUID byte order).
    ///   * Metadata token stream filling `length - 24` bytes.
    ///   * When the `PayloadPresent` token (id `0x0000`) is true, a `u32` LE body length and that many body bytes
    ///     follow immediately after the metadata section.
    ///
    /// Unknown metadata token IDs are silently consumed when their token type is known.
    /// Malformed token values and unknown token type bytes return errors.
    pub(crate) fn read(bytes: &[u8]) -> azure_core::Result<Self> {
        let mut src = bytes;
        let total_len = read_u32_le(&mut src)? as usize;
        if total_len > bytes.len() {
            return Err(data_conversion_error(format!(
                "RNTBD response length {total_len} exceeds buffer length {}",
                bytes.len()
            )));
        }
        if total_len < 24 {
            return Err(data_conversion_error(format!(
                "RNTBD response length {total_len} is smaller than the 24-byte header"
            )));
        }

        let mut frame = &bytes[4..total_len];
        let http_status = read_u32_le(&mut frame)?;
        let activity_id = read_uuid_le(&mut frame)?;

        let mut payload_present = false;
        let mut continuation_token = None;
        let mut etag = None;
        let mut retry_after_ms = None;
        let mut last_state_change_date_time = None;
        let mut storage_max_resource_quota = None;
        let mut storage_resource_quota_usage = None;
        let mut schema_version = None;
        let mut lsn = None;
        let mut item_count = None;
        let mut request_charge = None;
        let mut backend_request_duration_ms = None;
        let mut owner_full_name = None;
        let mut owner_id = None;
        let mut quorum_acked_lsn = None;
        let mut sub_status = None;
        let mut current_write_quorum = None;
        let mut current_replica_set_size = None;
        let mut partition_key_range_id = None;
        let mut xp_role = None;
        let mut number_of_read_regions = None;
        let mut item_lsn = None;
        let mut global_committed_lsn = None;
        let mut local_lsn = None;
        let mut quorum_acked_local_lsn = None;
        let mut item_local_lsn = None;
        let mut transport_request_id = None;
        let mut session_token = None;
        let mut query_execution_info = None;
        let mut pending_pk_delete = None;
        let mut physical_partition_id = None;
        let mut conflict_resolved_timestamp = None;

        while !frame.is_empty() {
            let token = Token::read_from(&mut frame)?;
            match RntbdResponseToken::try_from(token.id.value()) {
                Ok(RntbdResponseToken::PayloadPresent) => {
                    payload_present = expect_byte(token, "PayloadPresent")? != 0;
                }
                Ok(RntbdResponseToken::LastStateChangeDateTime) => {
                    last_state_change_date_time =
                        Some(expect_small_string(token, "LastStateChangeDateTime")?);
                }
                Ok(RntbdResponseToken::ContinuationToken) => {
                    continuation_token = Some(expect_string(token, "ContinuationToken")?);
                }
                Ok(RntbdResponseToken::ETag) => {
                    etag = Some(expect_string(token, "ETag")?);
                }
                Ok(RntbdResponseToken::RetryAfterMilliseconds) => {
                    retry_after_ms = Some(expect_u32(token, "RetryAfterMilliseconds")?);
                }
                Ok(RntbdResponseToken::StorageMaxResourceQuota) => {
                    storage_max_resource_quota =
                        Some(expect_string(token, "StorageMaxResourceQuota")?);
                }
                Ok(RntbdResponseToken::StorageResourceQuotaUsage) => {
                    storage_resource_quota_usage =
                        Some(expect_string(token, "StorageResourceQuotaUsage")?);
                }
                Ok(RntbdResponseToken::SchemaVersion) => {
                    schema_version = Some(expect_small_string(token, "SchemaVersion")?);
                }
                Ok(RntbdResponseToken::Lsn) => {
                    lsn = Some(expect_i64(token, "LSN")?);
                }
                Ok(RntbdResponseToken::ItemCount) => {
                    item_count = Some(expect_u32(token, "ItemCount")?);
                }
                Ok(RntbdResponseToken::RequestCharge) => {
                    request_charge = Some(expect_f64(token, "RequestCharge")?);
                }
                Ok(RntbdResponseToken::BackendRequestDurationMilliseconds) => {
                    backend_request_duration_ms =
                        Some(expect_f64(token, "BackendRequestDurationMilliseconds")?);
                }
                Ok(RntbdResponseToken::OwnerFullName) => {
                    owner_full_name = Some(expect_string(token, "OwnerFullName")?);
                }
                Ok(RntbdResponseToken::OwnerId) => {
                    owner_id = Some(expect_string(token, "OwnerId")?);
                }
                Ok(RntbdResponseToken::QuorumAckedLsn) => {
                    quorum_acked_lsn = Some(expect_i64(token, "QuorumAckedLSN")?);
                }
                Ok(RntbdResponseToken::SubStatus) => {
                    sub_status = Some(expect_u32(token, "SubStatus")?);
                }
                Ok(RntbdResponseToken::CurrentWriteQuorum) => {
                    current_write_quorum = Some(expect_u32(token, "CurrentWriteQuorum")?);
                }
                Ok(RntbdResponseToken::CurrentReplicaSetSize) => {
                    current_replica_set_size = Some(expect_u32(token, "CurrentReplicaSetSize")?);
                }
                Ok(RntbdResponseToken::PartitionKeyRangeId) => {
                    partition_key_range_id = Some(expect_string(token, "PartitionKeyRangeId")?);
                }
                Ok(RntbdResponseToken::XpRole) => {
                    xp_role = Some(expect_u32(token, "XPRole")?);
                }
                Ok(RntbdResponseToken::NumberOfReadRegions) => {
                    number_of_read_regions = Some(expect_u32(token, "NumberOfReadRegions")?);
                }
                Ok(RntbdResponseToken::ItemLsn) => {
                    item_lsn = Some(expect_i64(token, "ItemLSN")?);
                }
                Ok(RntbdResponseToken::GlobalCommittedLsn) => {
                    global_committed_lsn = Some(expect_i64(token, "GlobalCommittedLSN")?);
                }
                Ok(RntbdResponseToken::LocalLsn) => {
                    local_lsn = Some(expect_i64(token, "LocalLSN")?);
                }
                Ok(RntbdResponseToken::QuorumAckedLocalLsn) => {
                    quorum_acked_local_lsn = Some(expect_i64(token, "QuorumAckedLocalLSN")?);
                }
                Ok(RntbdResponseToken::ItemLocalLsn) => {
                    item_local_lsn = Some(expect_i64(token, "ItemLocalLSN")?);
                }
                Ok(RntbdResponseToken::TransportRequestId) => {
                    transport_request_id = Some(expect_u32(token, "TransportRequestID")?);
                }
                Ok(RntbdResponseToken::SessionToken) => {
                    session_token = Some(expect_string(token, "SessionToken")?);
                }
                Ok(RntbdResponseToken::QueryExecutionInfo) => {
                    query_execution_info = Some(expect_string(token, "QueryExecutionInfo")?);
                }
                Ok(RntbdResponseToken::PendingPkDelete) => {
                    pending_pk_delete = Some(expect_byte(token, "PendingPKDelete")? != 0);
                }
                Ok(RntbdResponseToken::PhysicalPartitionId) => {
                    physical_partition_id = Some(expect_string(token, "PhysicalPartitionId")?);
                }
                Ok(RntbdResponseToken::ConflictResolvedTimestamp) => {
                    conflict_resolved_timestamp =
                        Some(expect_u64(token, "ConflictResolvedTimestamp")?);
                }
                Err(()) => {}
            }
        }

        let body = if payload_present {
            let mut tail = &bytes[total_len..];
            let body_len = read_u32_le(&mut tail)? as usize;
            if body_len > tail.len() {
                return Err(data_conversion_error(format!(
                    "RNTBD response body length {body_len} exceeds remaining buffer length {}",
                    tail.len()
                )));
            }
            tail[..body_len].to_vec()
        } else {
            Vec::new()
        };

        Ok(Self {
            status: map_rntbd_status_to_cosmos_status(http_status, sub_status),
            activity_id,
            body,
            continuation_token,
            etag,
            retry_after_ms,
            last_state_change_date_time,
            storage_max_resource_quota,
            storage_resource_quota_usage,
            schema_version,
            lsn,
            item_count,
            request_charge,
            backend_request_duration_ms,
            owner_full_name,
            owner_id,
            quorum_acked_lsn,
            current_write_quorum,
            current_replica_set_size,
            partition_key_range_id,
            xp_role,
            number_of_read_regions,
            item_lsn,
            global_committed_lsn,
            local_lsn,
            quorum_acked_local_lsn,
            item_local_lsn,
            transport_request_id,
            session_token,
            query_execution_info,
            pending_pk_delete,
            physical_partition_id,
            conflict_resolved_timestamp,
        })
    }
}

fn expect_string(token: Token, name: &str) -> azure_core::Result<String> {
    token
        .into_string()
        .ok_or_else(|| unexpected_token_type(name))
}

fn expect_small_string(token: Token, name: &str) -> azure_core::Result<String> {
    token
        .into_small_string()
        .ok_or_else(|| unexpected_token_type(name))
}

fn expect_byte(token: Token, name: &str) -> azure_core::Result<u8> {
    token.as_byte().ok_or_else(|| unexpected_token_type(name))
}

fn expect_u32(token: Token, name: &str) -> azure_core::Result<u32> {
    token.as_u32().ok_or_else(|| unexpected_token_type(name))
}

fn expect_i64(token: Token, name: &str) -> azure_core::Result<i64> {
    token.as_i64().ok_or_else(|| unexpected_token_type(name))
}

fn expect_u64(token: Token, name: &str) -> azure_core::Result<u64> {
    token.as_u64().ok_or_else(|| unexpected_token_type(name))
}

fn expect_f64(token: Token, name: &str) -> azure_core::Result<f64> {
    token.as_f64().ok_or_else(|| unexpected_token_type(name))
}

fn unexpected_token_type(name: &str) -> azure_core::Error {
    data_conversion_error(format!("RNTBD token {name} had an unexpected value type"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;

    use crate::driver::transport::rntbd::tokens::{write_uuid_le, TokenValue};

    #[test]
    fn unknown_token_id_is_silently_skipped() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0015, TokenValue::Double(1.5))
            .write_to(&mut frame)
            .unwrap();
        Token::new(0xFFFE, TokenValue::SmallString("hello".to_owned()))
            .write_to(&mut frame)
            .unwrap();
        Token::new(0x001C, TokenValue::ULong(1002))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);

        let response = RntbdResponse::read(&frame).unwrap();

        assert_eq!(response.status.status_code(), StatusCode::Ok);
        assert_eq!(response.status.sub_status().unwrap().value(), 1002);
        assert_eq!(response.request_charge, Some(1.5));
        assert!(response.body.is_empty());
    }

    #[test]
    fn backend_request_duration_is_decoded() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0051, TokenValue::Double(12.75))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);

        let response = RntbdResponse::read(&frame).unwrap();

        assert_eq!(response.backend_request_duration_ms, Some(12.75));
    }

    #[test]
    fn backend_request_duration_rejects_wrong_token_type() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0051, TokenValue::ULong(12))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);

        let err = RntbdResponse::read(&frame).unwrap_err();

        assert_eq!(*err.kind(), azure_core::error::ErrorKind::DataConversion);
    }

    #[test]
    fn known_token_with_wrong_type_is_rejected() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0010, TokenValue::String("1.0".into()))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);

        let err = RntbdResponse::read(&frame).unwrap_err();

        assert_eq!(*err.kind(), azure_core::error::ErrorKind::DataConversion);
    }

    #[test]
    fn live_observed_gateway_v2_metadata_is_decoded() {
        let mut frame = response_header(StatusCode::Ok);
        let tokens = [
            Token::new(
                0x0002,
                TokenValue::SmallString("2026-07-21T00:00:00Z".into()),
            ),
            Token::new(0x000E, TokenValue::String("documentSize=10240;".into())),
            Token::new(0x000F, TokenValue::String("documentSize=1;".into())),
            Token::new(0x0010, TokenValue::SmallString("1.0".into())),
            Token::new(0x0014, TokenValue::ULong(1)),
            Token::new(0x0018, TokenValue::String("owner-rid".into())),
            Token::new(0x001A, TokenValue::LongLong(40)),
            Token::new(0x001E, TokenValue::ULong(3)),
            Token::new(0x001F, TokenValue::ULong(4)),
            Token::new(0x0026, TokenValue::ULong(1)),
            Token::new(0x0030, TokenValue::ULong(2)),
            Token::new(0x003A, TokenValue::LongLong(41)),
            Token::new(0x003B, TokenValue::LongLong(40)),
            Token::new(0x003C, TokenValue::LongLong(39)),
            Token::new(
                0x0045,
                TokenValue::String("{\"reverseRidEnabled\":false}".into()),
            ),
            Token::new(0x0055, TokenValue::Byte(0)),
            Token::new(0x0063, TokenValue::String("physical-0".into())),
            Token::new(0x0087, TokenValue::ULongLong(1_234_567)),
        ];
        for token in tokens {
            token.write_to(&mut frame).unwrap();
        }
        patch_total_len(&mut frame);

        let response = RntbdResponse::read(&frame).unwrap();

        assert_eq!(
            response.last_state_change_date_time.as_deref(),
            Some("2026-07-21T00:00:00Z")
        );
        assert_eq!(
            response.storage_max_resource_quota.as_deref(),
            Some("documentSize=10240;")
        );
        assert_eq!(
            response.storage_resource_quota_usage.as_deref(),
            Some("documentSize=1;")
        );
        assert_eq!(response.schema_version.as_deref(), Some("1.0"));
        assert_eq!(response.item_count, Some(1));
        assert_eq!(response.owner_id.as_deref(), Some("owner-rid"));
        assert_eq!(response.quorum_acked_lsn, Some(40));
        assert_eq!(response.current_write_quorum, Some(3));
        assert_eq!(response.current_replica_set_size, Some(4));
        assert_eq!(response.xp_role, Some(1));
        assert_eq!(response.number_of_read_regions, Some(2));
        assert_eq!(response.local_lsn, Some(41));
        assert_eq!(response.quorum_acked_local_lsn, Some(40));
        assert_eq!(response.item_local_lsn, Some(39));
        assert_eq!(
            response.query_execution_info.as_deref(),
            Some("{\"reverseRidEnabled\":false}")
        );
        assert_eq!(response.pending_pk_delete, Some(false));
        assert_eq!(
            response.physical_partition_id.as_deref(),
            Some("physical-0")
        );
        assert_eq!(response.conflict_resolved_timestamp, Some(1_234_567));
    }

    #[test]
    fn total_length_past_buffer_is_rejected() {
        let mut frame = response_header(StatusCode::Ok);
        let total_len = (frame.len() as u32) + 1;
        frame[0..4].copy_from_slice(&total_len.to_le_bytes());

        let err = RntbdResponse::read(&frame).unwrap_err();

        assert_eq!(*err.kind(), azure_core::error::ErrorKind::DataConversion);
    }

    #[test]
    fn payload_present_with_body_is_decoded() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0000, TokenValue::Byte(1))
            .write_to(&mut frame)
            .unwrap();
        Token::new(0x0015, TokenValue::Double(2.5))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);
        let body = b"{\"id\":\"abc\"}";
        frame.extend_from_slice(&(body.len() as u32).to_le_bytes());
        frame.extend_from_slice(body);

        let response = RntbdResponse::read(&frame).unwrap();

        assert_eq!(response.request_charge, Some(2.5));
        assert_eq!(response.body, body.to_vec());
    }

    #[test]
    fn payload_present_false_keeps_body_empty() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0000, TokenValue::Byte(0))
            .write_to(&mut frame)
            .unwrap();
        Token::new(0x0015, TokenValue::Double(1.0))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);
        // Trailing bytes beyond total_len must not be read when payload_present is false.
        frame.extend_from_slice(&[0xDE, 0xAD]);

        let response = RntbdResponse::read(&frame).unwrap();

        assert!(response.body.is_empty());
    }

    #[test]
    fn payload_present_with_truncated_body_is_rejected() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0000, TokenValue::Byte(1))
            .write_to(&mut frame)
            .unwrap();
        patch_total_len(&mut frame);
        // Claim 16 body bytes but only provide 4.
        frame.extend_from_slice(&16_u32.to_le_bytes());
        frame.extend_from_slice(&[0, 1, 2, 3]);

        let err = RntbdResponse::read(&frame).unwrap_err();

        assert_eq!(*err.kind(), azure_core::error::ErrorKind::DataConversion);
    }

    fn response_header(status_code: StatusCode) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(&0_u32.to_le_bytes());
        frame.extend_from_slice(&u16::from(status_code).to_le_bytes());
        frame.extend_from_slice(&0_u16.to_le_bytes());
        write_uuid_le(&mut frame, Uuid::nil()).unwrap();
        frame
    }

    fn patch_total_len(frame: &mut [u8]) {
        let total_len = u32::try_from(frame.len()).unwrap();
        frame[0..4].copy_from_slice(&total_len.to_le_bytes());
    }
}
