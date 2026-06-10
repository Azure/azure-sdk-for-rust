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
        let header_len = 24 + metadata_len;
        let body_section_len = self.body.as_ref().map_or(0, |body| 4 + body.len());
        let total_len = header_len + body_section_len;
        // The `LengthInBytes` field on the wire covers only the request header
        // section (the length field itself, resource/operation type, activity
        // ID, and metadata tokens). The body length prefix and body bytes
        // follow the header section and are NOT counted here. This matches
        // Java's `RntbdRequest.computeLength()`.
        let header_len_u32 = u32::try_from(header_len).map_err(|_| {
            data_conversion_error(format!(
                "RNTBD request header length {header_len} exceeds u32::MAX"
            ))
        })?;

        let mut out = Vec::with_capacity(total_len);
        out.extend_from_slice(&header_len_u32.to_le_bytes());
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

    /// Regression: the Gateway 2.0 proxy's RNTBD reader treats `LengthInBytes`
    /// as the header-section length. We previously wrote `total_frame_length`
    /// into that field, which the proxy then interpreted as a giant token
    /// stream and rejected with HTTP 400 sub-status 13007 ("error routing the
    /// request"). Pin the header-only convention so a future refactor cannot
    /// silently re-break wire compatibility with the proxy (and Java).
    #[test]
    fn length_in_bytes_covers_header_section_only_excluding_body() {
        let body = b"{\"id\":\"doc1\"}".to_vec();
        let body_len = body.len();
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id: Uuid::nil(),
            metadata: vec![Token::new(0x00CE, TokenValue::String("account".to_owned()))],
            body: Some(body),
        };

        let bytes = frame.serialize().unwrap();
        let length_in_bytes = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        // Header section = LengthInBytes(4) + ResourceType(2) + OperationType(2)
        // + ActivityId(16) + metadata token(3 header + 2 string-len prefix + 7 "account") = 36.
        assert_eq!(length_in_bytes, 36, "LengthInBytes must cover header only");
        // Total frame = header + body-length prefix (4 bytes) + body bytes.
        assert_eq!(bytes.len(), length_in_bytes + 4 + body_len);
        // The body-length prefix immediately follows the header section.
        let prefix_offset = length_in_bytes;
        let body_len_on_wire = u32::from_le_bytes([
            bytes[prefix_offset],
            bytes[prefix_offset + 1],
            bytes[prefix_offset + 2],
            bytes[prefix_offset + 3],
        ]) as usize;
        assert_eq!(body_len_on_wire, body_len);
    }

    /// Pin that `LengthInBytes` is inclusive of its own 4 bytes. With no
    /// metadata and no body the header section is exactly 24 bytes:
    /// `LengthInBytes(4) + ResourceType(2) + OperationType(2) + ActivityId(16)`.
    #[test]
    fn length_in_bytes_is_inclusive_of_itself_and_equals_24_for_empty_metadata() {
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::nil(),
            metadata: Vec::new(),
            body: None,
        };

        let bytes = frame.serialize().unwrap();
        let length_in_bytes = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        assert_eq!(length_in_bytes, 24);
        assert_eq!(bytes.len(), 24);
    }

    /// Pin that `LengthInBytes` does NOT grow when the body grows. The proxy
    /// reads exactly `LengthInBytes` bytes as header tokens; if our code
    /// regressed back to writing total frame length, a bigger body would
    /// silently make `LengthInBytes` larger and the proxy would read garbage
    /// past the header section.
    #[test]
    fn length_in_bytes_is_invariant_to_body_size() {
        let metadata = vec![Token::new(0x00CE, TokenValue::String("acct".to_owned()))];
        let make = |body: Option<Vec<u8>>| RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id: Uuid::nil(),
            metadata: metadata.clone(),
            body,
        };

        let length_in_bytes = |bytes: &[u8]| -> usize {
            u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize
        };

        let no_body = make(None).serialize().unwrap();
        let tiny_body = make(Some(b"{}".to_vec())).serialize().unwrap();
        let medium_body = make(Some(vec![0u8; 1024])).serialize().unwrap();
        let large_body = make(Some(vec![0u8; 65_536])).serialize().unwrap();

        let header_len = length_in_bytes(&no_body);
        assert_eq!(length_in_bytes(&tiny_body), header_len);
        assert_eq!(length_in_bytes(&medium_body), header_len);
        assert_eq!(length_in_bytes(&large_body), header_len);

        // And total frame size grows with body, while LengthInBytes does not.
        assert!(tiny_body.len() > no_body.len());
        assert!(medium_body.len() > tiny_body.len());
        assert!(large_body.len() > medium_body.len());
    }

    /// Pin that a present-but-empty body still emits the 4-byte body-length
    /// prefix (with value 0) after the header section, distinct from a `None`
    /// body which emits nothing after the header. The proxy's frame reader
    /// keys off `PayloadPresent` and the trailing length prefix; if these
    /// drift apart the proxy will hang waiting for body bytes that never
    /// arrive.
    #[test]
    fn length_in_bytes_excludes_body_prefix_even_for_empty_body() {
        let with_empty_body = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::nil(),
            metadata: Vec::new(),
            body: Some(Vec::new()),
        };
        let without_body = RntbdRequestFrame {
            body: None,
            ..with_empty_body.clone()
        };

        let with_bytes = with_empty_body.serialize().unwrap();
        let without_bytes = without_body.serialize().unwrap();

        let with_len =
            u32::from_le_bytes([with_bytes[0], with_bytes[1], with_bytes[2], with_bytes[3]])
                as usize;
        let without_len = u32::from_le_bytes([
            without_bytes[0],
            without_bytes[1],
            without_bytes[2],
            without_bytes[3],
        ]) as usize;

        assert_eq!(with_len, without_len, "header length is body-independent");
        assert_eq!(
            with_bytes.len(),
            with_len + 4,
            "empty body still writes the 4-byte length prefix"
        );
        assert_eq!(
            without_bytes.len(),
            without_len,
            "no body writes nothing past the header"
        );
        // The body length prefix for an empty body is 0.
        let prefix = u32::from_le_bytes([
            with_bytes[with_len],
            with_bytes[with_len + 1],
            with_bytes[with_len + 2],
            with_bytes[with_len + 3],
        ]);
        assert_eq!(prefix, 0);
    }

    /// Pin that `LengthInBytes` is encoded as little-endian u32 in the first
    /// four bytes of the frame. Java's `RntbdRequest` writes
    /// `LITTLE_ENDIAN` u32 here; a big-endian encoding would parse as a
    /// nonsense large value on the proxy.
    #[test]
    fn length_in_bytes_is_little_endian_u32_at_offset_zero() {
        // Pick metadata that gives a header length whose LE and BE encodings
        // are visibly different (295 = 0x0127 -> LE [0x27, 0x01, 0, 0],
        // BE [0, 0, 0x01, 0x27]).
        let big_metadata = Token::new(0x00CE, TokenValue::String("x".repeat(266)));
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Read,
            activity_id: Uuid::nil(),
            metadata: vec![big_metadata],
            body: None,
        };

        let bytes = frame.serialize().unwrap();
        assert_eq!(bytes.len(), 295);
        assert_eq!(&bytes[0..4], &[0x27, 0x01, 0x00, 0x00]);
    }

    /// Pin the precise header section content the proxy parses: exactly
    /// `LengthInBytes` bytes consumed from offset 0 yields all metadata
    /// tokens (and nothing else). Any drift between `serialize()`'s
    /// computed header length and the actual bytes emitted produces a
    /// trailing-byte assertion failure here.
    #[test]
    fn header_section_bytes_parse_to_exactly_the_emitted_metadata_tokens() {
        let metadata = vec![
            Token::new(0x0001, TokenValue::String("auth-token".to_owned())),
            Token::new(
                0x0003,
                TokenValue::SmallString("Wed, 21 Oct 2015 07:28:00 GMT".to_owned()),
            ),
            Token::new(0x00CE, TokenValue::String("account".to_owned())),
            Token::new(0x0035, TokenValue::String("rid-base64".to_owned())),
            Token::new(0x00A2, TokenValue::ULong(11)),
        ];

        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id: Uuid::from_u128(0x1234_5678_90ab_cdef_0123_4567_89ab_cdef),
            metadata: metadata.clone(),
            body: Some(b"{}".to_vec()),
        };

        let bytes = frame.serialize().unwrap();
        let length_in_bytes = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        // Skip the 24-byte fixed header and consume exactly the metadata
        // section: tokens must parse cleanly and stop exactly at `length_in_bytes`.
        let metadata_bytes = &bytes[24..length_in_bytes];
        let mut cursor = metadata_bytes;
        let mut parsed = Vec::new();
        while !cursor.is_empty() {
            parsed.push(Token::read_from(&mut cursor).unwrap());
        }
        assert!(cursor.is_empty(), "metadata section ended mid-token");
        assert_eq!(parsed, metadata);
    }

    /// Pin that the proxy can locate the body by jumping to offset
    /// `LengthInBytes` from the start of the frame: the next 4 bytes are the
    /// body-length prefix, and the body bytes that follow round-trip exactly.
    /// This is the proxy's actual extraction algorithm (Java's
    /// `RntbdRequestDecoder.decode`).
    #[test]
    fn body_is_locatable_at_offset_length_in_bytes() {
        let body = b"{\"id\":\"doc1\",\"pk\":\"abc\"}".to_vec();
        let frame = RntbdRequestFrame {
            resource_type: ResourceType::Document,
            operation_type: OperationType::Create,
            activity_id: Uuid::nil(),
            metadata: vec![
                Token::new(0x00CE, TokenValue::String("account".to_owned())),
                Token::new(0x0015, TokenValue::String("db1".to_owned())),
                Token::new(0x0016, TokenValue::String("coll1".to_owned())),
            ],
            body: Some(body.clone()),
        };

        let bytes = frame.serialize().unwrap();
        let header_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

        // Proxy reads body-length prefix at offset = header_len.
        let body_len = u32::from_le_bytes([
            bytes[header_len],
            bytes[header_len + 1],
            bytes[header_len + 2],
            bytes[header_len + 3],
        ]) as usize;
        assert_eq!(body_len, body.len());

        let body_start = header_len + 4;
        assert_eq!(&bytes[body_start..body_start + body_len], body.as_slice());
        assert_eq!(
            bytes.len(),
            body_start + body_len,
            "no trailing bytes after body"
        );
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
        // The leading length field is the request HEADER length (length field
        // itself + resource/operation type + activity id + metadata tokens) and
        // does NOT include the body length prefix or body bytes.
        let header_len = read_u32_le(&mut src)? as usize;
        let resource_type =
            ResourceType::try_from(RntbdResourceType::try_from(read_u16_le(&mut src)?)?)?;
        let operation_type =
            OperationType::try_from(RntbdOperationType::try_from(read_u16_le(&mut src)?)?)?;
        let activity_id = read_uuid_le(&mut src)?;

        let mut metadata = Vec::new();
        // Bytes consumed so far: 4 (length) + 2 (resource) + 2 (operation) + 16 (activity) = 24.
        let metadata_end = header_len.saturating_sub(24);
        let metadata_bytes = src.get(..metadata_end).ok_or_else(|| {
            data_conversion_error(format!("request header length {header_len} exceeds buffer"))
        })?;
        let mut metadata_src = metadata_bytes;
        while !metadata_src.is_empty() {
            metadata.push(Token::read_from(&mut metadata_src)?);
        }
        src = &src[metadata_end..];

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
            if !src.is_empty() {
                return Err(data_conversion_error(format!(
                    "unexpected {} trailing bytes after header section",
                    src.len()
                )));
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
