// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! RNTBD response frame deserialization.

use uuid::Uuid;

use crate::models::CosmosStatus;

use super::{
    status::map_rntbd_status_to_cosmos_status,
    tokens::{
        data_conversion_error, read_u32_le, read_uuid_le, RntbdResponseToken, Token, TokenValue,
    },
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
    /// Logical sequence number.
    pub(crate) lsn: Option<i64>,
    /// Request charge in request units.
    pub(crate) request_charge: Option<f64>,
    /// Owner full name metadata.
    pub(crate) owner_full_name: Option<String>,
    /// Partition key range identifier.
    pub(crate) partition_key_range_id: Option<String>,
    /// Item logical sequence number.
    pub(crate) item_lsn: Option<i64>,
    /// Global committed logical sequence number.
    pub(crate) global_committed_lsn: Option<i64>,
    /// Transport request identifier.
    pub(crate) transport_request_id: Option<u32>,
    /// Session token for session consistency.
    pub(crate) session_token: Option<String>,
}

impl RntbdResponse {
    /// Deserializes a Gateway 2.0 RNTBD response frame.
    ///
    /// Unknown metadata token IDs are silently consumed when their token type is
    /// known. Malformed token values and unknown token type bytes return errors.
    /// The Slice 1 frame shape has no separate metadata length, so the parser
    /// advances the tracking offset by decoding complete metadata tokens and
    /// preserves any trailing bytes shorter than a token header as body bytes.
    pub(crate) fn deserialize(bytes: &[u8]) -> azure_core::Result<Self> {
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

        let mut continuation_token = None;
        let mut etag = None;
        let mut retry_after_ms = None;
        let mut lsn = None;
        let mut request_charge = None;
        let mut owner_full_name = None;
        let mut sub_status = None;
        let mut partition_key_range_id = None;
        let mut item_lsn = None;
        let mut global_committed_lsn = None;
        let mut transport_request_id = None;
        let mut session_token = None;

        while frame.len() >= 3 {
            let token = Token::read_from(&mut frame)?;
            match RntbdResponseToken::try_from(token.id) {
                Ok(RntbdResponseToken::ContinuationToken) => {
                    continuation_token = Some(expect_string(token, "ContinuationToken")?);
                }
                Ok(RntbdResponseToken::ETag) => {
                    etag = Some(expect_string(token, "ETag")?);
                }
                Ok(RntbdResponseToken::RetryAfterMilliseconds) => {
                    retry_after_ms = Some(expect_u32(token, "RetryAfterMilliseconds")?);
                }
                Ok(RntbdResponseToken::Lsn) => {
                    lsn = Some(expect_i64(token, "LSN")?);
                }
                Ok(RntbdResponseToken::RequestCharge) => {
                    request_charge = Some(expect_f64(token, "RequestCharge")?);
                }
                Ok(RntbdResponseToken::OwnerFullName) => {
                    owner_full_name = Some(expect_string(token, "OwnerFullName")?);
                }
                Ok(RntbdResponseToken::SubStatus) => {
                    sub_status = Some(expect_u32(token, "SubStatus")?);
                }
                Ok(RntbdResponseToken::PartitionKeyRangeId) => {
                    partition_key_range_id = Some(expect_string(token, "PartitionKeyRangeId")?);
                }
                Ok(RntbdResponseToken::ItemLsn) => {
                    item_lsn = Some(expect_i64(token, "ItemLSN")?);
                }
                Ok(RntbdResponseToken::GlobalCommittedLsn) => {
                    global_committed_lsn = Some(expect_i64(token, "GlobalCommittedLSN")?);
                }
                Ok(RntbdResponseToken::TransportRequestId) => {
                    transport_request_id = Some(expect_u32(token, "TransportRequestID")?);
                }
                Ok(RntbdResponseToken::SessionToken) => {
                    session_token = Some(expect_string(token, "SessionToken")?);
                }
                Err(()) => {}
            }
        }

        Ok(Self {
            status: map_rntbd_status_to_cosmos_status(http_status, sub_status),
            activity_id,
            body: frame.to_vec(),
            continuation_token,
            etag,
            retry_after_ms,
            lsn,
            request_charge,
            owner_full_name,
            partition_key_range_id,
            item_lsn,
            global_committed_lsn,
            transport_request_id,
            session_token,
        })
    }
}

fn expect_string(token: Token, name: &str) -> azure_core::Result<String> {
    match token.value {
        TokenValue::String(value) => Ok(value),
        _ => Err(unexpected_token_type(name)),
    }
}

fn expect_u32(token: Token, name: &str) -> azure_core::Result<u32> {
    match token.value {
        TokenValue::ULong(value) => Ok(value),
        _ => Err(unexpected_token_type(name)),
    }
}

fn expect_i64(token: Token, name: &str) -> azure_core::Result<i64> {
    match token.value {
        TokenValue::LongLong(value) => Ok(value),
        _ => Err(unexpected_token_type(name)),
    }
}

fn expect_f64(token: Token, name: &str) -> azure_core::Result<f64> {
    match token.value {
        TokenValue::Double(value) => Ok(value),
        _ => Err(unexpected_token_type(name)),
    }
}

fn unexpected_token_type(name: &str) -> azure_core::Error {
    data_conversion_error(format!("RNTBD token {name} had an unexpected value type"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::http::StatusCode;

    use crate::driver::transport::rntbd::tokens::write_uuid_le;

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

        let response = RntbdResponse::deserialize(&frame).unwrap();

        assert_eq!(response.status.status_code(), StatusCode::Ok);
        assert_eq!(response.status.sub_status().unwrap().value(), 1002);
        assert_eq!(response.request_charge, Some(1.5));
        assert!(response.body.is_empty());
    }

    #[test]
    fn total_length_past_buffer_is_rejected() {
        let mut frame = response_header(StatusCode::Ok);
        let total_len = (frame.len() as u32) + 1;
        frame[0..4].copy_from_slice(&total_len.to_le_bytes());

        let err = RntbdResponse::deserialize(&frame).unwrap_err();

        assert_eq!(*err.kind(), azure_core::error::ErrorKind::DataConversion);
    }

    #[test]
    fn trailing_bytes_shorter_than_token_header_are_body() {
        let mut frame = response_header(StatusCode::Ok);
        frame.extend_from_slice(&[0xAA, 0xBB]);
        patch_total_len(&mut frame);

        let response = RntbdResponse::deserialize(&frame).unwrap();

        assert_eq!(response.body, vec![0xAA, 0xBB]);
    }

    #[test]
    fn metadata_before_short_body_is_preserved() {
        let mut frame = response_header(StatusCode::Ok);
        Token::new(0x0015, TokenValue::Double(2.5))
            .write_to(&mut frame)
            .unwrap();
        frame.extend_from_slice(&[0xAA, 0xBB]);
        patch_total_len(&mut frame);

        let response = RntbdResponse::deserialize(&frame).unwrap();

        assert_eq!(response.request_charge, Some(2.5));
        assert_eq!(response.body, vec![0xAA, 0xBB]);
    }

    fn response_header(status_code: StatusCode) -> Vec<u8> {
        let mut frame = Vec::new();
        frame.extend_from_slice(&0_u32.to_le_bytes());
        frame.extend_from_slice(&u16::from(status_code).to_le_bytes());
        frame.extend_from_slice(&0_u16.to_le_bytes());
        write_uuid_le(&mut frame, Uuid::nil());
        frame
    }

    fn patch_total_len(frame: &mut [u8]) {
        let total_len = u32::try_from(frame.len()).unwrap();
        frame[0..4].copy_from_slice(&total_len.to_le_bytes());
    }
}
