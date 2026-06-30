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
        let mut lsn = None;
        let mut request_charge = None;
        let mut owner_full_name = None;
        let mut sub_status = None;
        let mut partition_key_range_id = None;
        let mut item_lsn = None;
        let mut global_committed_lsn = None;
        let mut transport_request_id = None;
        let mut session_token = None;

        while !frame.is_empty() {
            let token = Token::read_from(&mut frame)?;
            match RntbdResponseToken::try_from(token.id.value()) {
                Ok(RntbdResponseToken::PayloadPresent) => {
                    payload_present = expect_byte(token, "PayloadPresent")? != 0;
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
    token
        .into_string()
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
