// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! RNTBD request frame serialization.

use uuid::Uuid;

use crate::models::{OperationType, ResourceType};

use super::tokens::{
    data_conversion_error, write_uuid_le, RntbdOperationType, RntbdResourceType, Token,
};

/// A Gateway 2.0 RNTBD request frame.
///
/// The body is schema-agnostic raw bytes. When [`body`](Self::body) is present,
/// serialization emits the payload length followed by the payload bytes.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct RntbdRequestFrame {
    /// Resource type encoded into the frame header.
    pub(crate) resource_type: ResourceType,
    /// Operation type encoded into the frame header.
    pub(crate) operation_type: OperationType,
    /// Activity identifier encoded as two little-endian `u64` values.
    pub(crate) activity_id: Uuid,
    /// Metadata token stream.
    pub(crate) metadata: Vec<Token>,
    /// Optional raw request payload.
    pub(crate) body: Option<Vec<u8>>,
}

impl RntbdRequestFrame {
    /// Serializes the request frame to Gateway 2.0 RNTBD bytes.
    ///
    /// The total length field is inclusive of its own four bytes. Returns
    /// [`ErrorKind::DataConversion`] when an input exceeds an RNTBD wire
    /// length limit (e.g., a metadata token value longer than the
    /// `SmallString` length prefix supports, a body larger than `u32::MAX`,
    /// or a frame whose total length exceeds `u32::MAX`).
    ///
    /// [`ErrorKind::DataConversion`]: azure_core::error::ErrorKind::DataConversion
    pub(crate) fn serialize(&self) -> azure_core::Result<Vec<u8>> {
        let metadata_len: usize = self.metadata.iter().map(Token::encoded_len).sum();
        let body_len = self.body.as_ref().map_or(0, |body| 4 + body.len());
        let total_len = 24 + metadata_len + body_len;
        let total_len_u32 = u32::try_from(total_len).map_err(|_| {
            data_conversion_error(format!(
                "RNTBD request frame length {total_len} exceeds u32::MAX"
            ))
        })?;

        let mut out = Vec::with_capacity(total_len);
        out.extend_from_slice(&total_len_u32.to_le_bytes());
        out.extend_from_slice(
            &RntbdResourceType::from(self.resource_type)
                .value()
                .to_le_bytes(),
        );
        out.extend_from_slice(
            &RntbdOperationType::from(self.operation_type)
                .value()
                .to_le_bytes(),
        );
        write_uuid_le(&mut out, self.activity_id);

        for token in &self.metadata {
            token.write_to(&mut out)?;
        }

        if let Some(body) = &self.body {
            let body_len = u32::try_from(body.len()).map_err(|_| {
                data_conversion_error(format!(
                    "RNTBD request payload length {} exceeds u32::MAX",
                    body.len()
                ))
            })?;
            out.extend_from_slice(&body_len.to_le_bytes());
            out.extend_from_slice(body);
        }

        debug_assert_eq!(out.len(), total_len);
        Ok(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::driver::transport::rntbd::tokens::{
        data_conversion_error, read_u16_le, read_u32_le, read_uuid_le, RntbdOperationType,
        RntbdResourceType, TokenValue,
    };

    #[test]
    fn request_frames_round_trip_for_slice_one_operations() {
        let operations = [
            OperationType::Create,
            OperationType::Read,
            OperationType::ReadFeed,
            OperationType::Replace,
            OperationType::Delete,
            OperationType::Upsert,
            OperationType::Query,
            OperationType::SqlQuery,
            OperationType::Head,
            OperationType::HeadFeed,
            OperationType::Batch,
        ];

        for operation_type in operations {
            for body in [None, Some(vec![0x7b, 0x7d])] {
                let frame = RntbdRequestFrame {
                    resource_type: ResourceType::Document,
                    operation_type,
                    activity_id: Uuid::from_u128(0x1234_5678_90ab_cdef_0123_4567_89ab_cdef),
                    metadata: Vec::new(),
                    body,
                };

                let bytes = frame.serialize().unwrap();
                let parsed = parse_request_for_tests(&bytes, frame.body.is_some()).unwrap();

                assert_eq!(parsed, frame);
            }
        }
    }

    #[test]
    fn query_plan_uses_sql_query_wire_id_until_metadata_rules_land() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::QueryPlan,
            activity_id: Uuid::nil(),
            metadata: Vec::new(),
            body: None,
        };

        let bytes = frame.serialize().unwrap();
        let operation_id = u16::from_le_bytes([bytes[6], bytes[7]]);

        // QueryPlan has no distinct Java RNTBD operation ID. Slice 2 will add
        // the metadata that disambiguates query-plan requests from SqlQuery.
        assert_eq!(
            operation_id,
            RntbdOperationType::from(OperationType::SqlQuery).value()
        );
    }

    #[test]
    fn metadata_tokens_are_serialized_between_header_and_body() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::nil(),
            metadata: vec![Token::new(0x00CE, TokenValue::String("account".to_owned()))],
            body: None,
        };

        let bytes = frame.serialize().unwrap();
        let parsed = parse_request_for_tests(&bytes, false).unwrap();

        assert_eq!(parsed, frame);
    }

    #[test]
    fn serialize_returns_error_when_small_string_exceeds_u8_length_prefix() {
        let oversized = "a".repeat(256);
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::nil(),
            metadata: vec![Token::new(0x0001, TokenValue::SmallString(oversized))],
            body: None,
        };

        let err = frame.serialize().unwrap_err();
        assert_eq!(*err.kind(), azure_core::error::ErrorKind::DataConversion);
    }

    fn parse_request_for_tests(
        bytes: &[u8],
        has_body: bool,
    ) -> azure_core::Result<RntbdRequestFrame> {
        let mut src = bytes;
        let total_len = read_u32_le(&mut src)? as usize;
        if total_len != bytes.len() {
            return Err(data_conversion_error(format!(
                "request frame length {total_len} did not match buffer length {}",
                bytes.len()
            )));
        }

        let resource_type =
            ResourceType::try_from(RntbdResourceType::try_from(read_u16_le(&mut src)?)?)?;
        let operation_type =
            OperationType::try_from(RntbdOperationType::try_from(read_u16_le(&mut src)?)?)?;
        let activity_id = read_uuid_le(&mut src)?;

        let mut metadata = Vec::new();
        let body = if has_body {
            let payload_len = read_u32_le(&mut src)? as usize;
            if src.len() != payload_len {
                return Err(data_conversion_error(format!(
                    "request payload length {payload_len} did not match remaining bytes {}",
                    src.len()
                )));
            }
            Some(src.to_vec())
        } else {
            while !src.is_empty() {
                metadata.push(Token::read_from(&mut src)?);
            }
            None
        };

        Ok(RntbdRequestFrame {
            resource_type,
            operation_type,
            activity_id,
            metadata,
            body,
        })
    }
}
