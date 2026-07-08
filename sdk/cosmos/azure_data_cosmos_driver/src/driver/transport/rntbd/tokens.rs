// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! RNTBD metadata token codecs and wire ID mappings.

use std::io::Write;

use azure_core::error::ErrorKind;
use uuid::Uuid;

use crate::{
    models::{DefaultConsistencyLevel, OperationType, ResourceType},
    options::ReadConsistencyStrategy,
};

/// The token type byte used by RNTBD metadata tokens.
///
/// Variable-width types carry their own length prefix in the value payload.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum TokenType {
    /// Single unsigned byte.
    Byte,
    /// Unsigned 16-bit integer encoded little-endian.
    UShort,
    /// Unsigned 32-bit integer encoded little-endian.
    ULong,
    /// Signed 32-bit integer encoded little-endian.
    Long,
    /// Unsigned 64-bit integer encoded little-endian.
    ULongLong,
    /// Signed 64-bit integer encoded little-endian.
    LongLong,
    /// UUID encoded in Microsoft GUID byte order.
    Guid,
    /// UTF-8 string prefixed with an unsigned byte length.
    SmallString,
    /// UTF-8 string prefixed with an unsigned 16-bit length.
    String,
    /// UTF-8 string prefixed with an unsigned 32-bit length.
    ULongString,
    /// Bytes prefixed with an unsigned byte length.
    SmallBytes,
    /// Bytes prefixed with an unsigned 16-bit length.
    Bytes,
    /// Bytes prefixed with an unsigned 32-bit length.
    ULongBytes,
    /// 32-bit floating point value encoded little-endian.
    Float,
    /// 64-bit floating point value encoded little-endian.
    Double,
    /// Invalid token type sentinel.
    Invalid,
}

impl TryFrom<u8> for TokenType {
    type Error = azure_core::Error;

    fn try_from(value: u8) -> azure_core::Result<Self> {
        match value {
            0x00 => Ok(Self::Byte),
            0x01 => Ok(Self::UShort),
            0x02 => Ok(Self::ULong),
            0x03 => Ok(Self::Long),
            0x04 => Ok(Self::ULongLong),
            0x05 => Ok(Self::LongLong),
            0x06 => Ok(Self::Guid),
            0x07 => Ok(Self::SmallString),
            0x08 => Ok(Self::String),
            0x09 => Ok(Self::ULongString),
            0x0A => Ok(Self::SmallBytes),
            0x0B => Ok(Self::Bytes),
            0x0C => Ok(Self::ULongBytes),
            0x0D => Ok(Self::Float),
            0x0E => Ok(Self::Double),
            0xFF => Ok(Self::Invalid),
            other => Err(data_conversion_error(format!(
                "unknown RNTBD token type 0x{other:02X}"
            ))),
        }
    }
}

impl From<TokenType> for u8 {
    fn from(value: TokenType) -> Self {
        match value {
            TokenType::Byte => 0x00,
            TokenType::UShort => 0x01,
            TokenType::ULong => 0x02,
            TokenType::Long => 0x03,
            TokenType::ULongLong => 0x04,
            TokenType::LongLong => 0x05,
            TokenType::Guid => 0x06,
            TokenType::SmallString => 0x07,
            TokenType::String => 0x08,
            TokenType::ULongString => 0x09,
            TokenType::SmallBytes => 0x0A,
            TokenType::Bytes => 0x0B,
            TokenType::ULongBytes => 0x0C,
            TokenType::Float => 0x0D,
            TokenType::Double => 0x0E,
            TokenType::Invalid => 0xFF,
        }
    }
}

/// A decoded RNTBD metadata token value.
///
/// The enum variant determines the value codec used on the wire.
#[derive(Clone, Debug, PartialEq)]
pub(crate) enum TokenValue {
    /// Single unsigned byte.
    Byte(u8),
    /// Unsigned 16-bit integer.
    UShort(u16),
    /// Unsigned 32-bit integer.
    ULong(u32),
    /// Signed 32-bit integer.
    Long(i32),
    /// Unsigned 64-bit integer.
    ULongLong(u64),
    /// Signed 64-bit integer.
    LongLong(i64),
    /// UUID in Microsoft GUID token byte order.
    Guid(Uuid),
    /// UTF-8 string with an unsigned byte length prefix.
    SmallString(String),
    /// UTF-8 string with an unsigned 16-bit length prefix.
    String(String),
    /// UTF-8 string with an unsigned 32-bit length prefix.
    ULongString(String),
    /// Bytes with an unsigned byte length prefix.
    SmallBytes(Vec<u8>),
    /// Bytes with an unsigned 16-bit length prefix.
    Bytes(Vec<u8>),
    /// Bytes with an unsigned 32-bit length prefix.
    ULongBytes(Vec<u8>),
    /// 32-bit floating point value.
    Float(f32),
    /// 64-bit floating point value.
    Double(f64),
}

impl TokenValue {
    fn token_type(&self) -> TokenType {
        match self {
            Self::Byte(_) => TokenType::Byte,
            Self::UShort(_) => TokenType::UShort,
            Self::ULong(_) => TokenType::ULong,
            Self::Long(_) => TokenType::Long,
            Self::ULongLong(_) => TokenType::ULongLong,
            Self::LongLong(_) => TokenType::LongLong,
            Self::Guid(_) => TokenType::Guid,
            Self::SmallString(_) => TokenType::SmallString,
            Self::String(_) => TokenType::String,
            Self::ULongString(_) => TokenType::ULongString,
            Self::SmallBytes(_) => TokenType::SmallBytes,
            Self::Bytes(_) => TokenType::Bytes,
            Self::ULongBytes(_) => TokenType::ULongBytes,
            Self::Float(_) => TokenType::Float,
            Self::Double(_) => TokenType::Double,
        }
    }

    fn encoded_len(&self) -> usize {
        match self {
            Self::Byte(_) => 1,
            Self::UShort(_) => 2,
            Self::ULong(_) | Self::Long(_) | Self::Float(_) => 4,
            Self::ULongLong(_) | Self::LongLong(_) | Self::Double(_) => 8,
            Self::Guid(_) => 16,
            Self::SmallString(value) => 1 + value.len(),
            Self::String(value) => 2 + value.len(),
            Self::ULongString(value) => 4 + value.len(),
            Self::SmallBytes(value) => 1 + value.len(),
            Self::Bytes(value) => 2 + value.len(),
            Self::ULongBytes(value) => 4 + value.len(),
        }
    }

    fn write_to(&self, out: &mut impl Write) -> azure_core::Result<()> {
        match self {
            Self::Byte(value) => out.write_all(&[*value])?,
            Self::UShort(value) => out.write_all(&value.to_le_bytes())?,
            Self::ULong(value) => out.write_all(&value.to_le_bytes())?,
            Self::Long(value) => out.write_all(&value.to_le_bytes())?,
            Self::ULongLong(value) => out.write_all(&value.to_le_bytes())?,
            Self::LongLong(value) => out.write_all(&value.to_le_bytes())?,
            Self::Guid(value) => write_uuid_le(out, *value)?,
            Self::SmallString(value) => write_len_prefixed_u8(out, value.as_bytes())?,
            Self::String(value) => write_len_prefixed_u16(out, value.as_bytes())?,
            Self::ULongString(value) => write_len_prefixed_u32(out, value.as_bytes())?,
            Self::SmallBytes(value) => write_len_prefixed_u8(out, value)?,
            Self::Bytes(value) => write_len_prefixed_u16(out, value)?,
            Self::ULongBytes(value) => write_len_prefixed_u32(out, value)?,
            Self::Float(value) => out.write_all(&value.to_le_bytes())?,
            Self::Double(value) => out.write_all(&value.to_le_bytes())?,
        }
        Ok(())
    }

    fn read_from(token_type: TokenType, src: &mut &[u8]) -> azure_core::Result<Self> {
        match token_type {
            TokenType::Byte => Ok(Self::Byte(read_u8(src)?)),
            TokenType::UShort => Ok(Self::UShort(read_u16_le(src)?)),
            TokenType::ULong => Ok(Self::ULong(read_u32_le(src)?)),
            TokenType::Long => Ok(Self::Long(read_i32_le(src)?)),
            TokenType::ULongLong => Ok(Self::ULongLong(read_u64_le(src)?)),
            TokenType::LongLong => Ok(Self::LongLong(read_i64_le(src)?)),
            TokenType::Guid => Ok(Self::Guid(read_uuid_le(src)?)),
            TokenType::SmallString => {
                let len = read_u8(src)? as usize;
                Ok(Self::SmallString(read_utf8(src, len)?))
            }
            TokenType::String => {
                let len = read_u16_le(src)? as usize;
                Ok(Self::String(read_utf8(src, len)?))
            }
            TokenType::ULongString => {
                let len = read_u32_le(src)? as usize;
                Ok(Self::ULongString(read_utf8(src, len)?))
            }
            TokenType::SmallBytes => {
                let len = read_u8(src)? as usize;
                Ok(Self::SmallBytes(
                    read_exact(src, len, "small bytes")?.to_vec(),
                ))
            }
            TokenType::Bytes => {
                let len = read_u16_le(src)? as usize;
                Ok(Self::Bytes(read_exact(src, len, "bytes")?.to_vec()))
            }
            TokenType::ULongBytes => {
                let len = read_u32_le(src)? as usize;
                Ok(Self::ULongBytes(
                    read_exact(src, len, "ulong bytes")?.to_vec(),
                ))
            }
            TokenType::Float => Ok(Self::Float(f32::from_le_bytes(read_array(src)?))),
            TokenType::Double => Ok(Self::Double(f64::from_le_bytes(read_array(src)?))),
            TokenType::Invalid => Err(data_conversion_error(
                "invalid RNTBD token type sentinel encountered",
            )),
        }
    }
}

/// Strongly-typed RNTBD metadata token identifier.
///
/// Wraps the raw two-byte token ID from the RNTBD header table so token IDs
/// can't be confused with other `u16` values at call sites.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct TokenId(pub(crate) u16);

impl TokenId {
    /// Returns the raw wire value.
    pub(crate) fn value(self) -> u16 {
        self.0
    }
}

impl From<u16> for TokenId {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

/// A single RNTBD metadata token.
///
/// Tokens are encoded as a two-byte token ID, a one-byte [`TokenType`], and the
/// value bytes for that type.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Token {
    /// Token identifier from the RNTBD header table.
    pub(crate) id: TokenId,
    /// Decoded token value.
    pub(crate) value: TokenValue,
}

impl Token {
    /// Creates a metadata token from an ID and typed value.
    pub(crate) fn new(id: impl Into<TokenId>, value: TokenValue) -> Self {
        Self {
            id: id.into(),
            value,
        }
    }

    pub(crate) fn authorization_token(value: String) -> Self {
        Self::new(
            RntbdRequestToken::AuthorizationToken,
            TokenValue::String(value),
        )
    }

    pub(crate) fn payload_present(value: bool) -> Self {
        Self::new(
            RntbdRequestToken::PayloadPresent,
            TokenValue::Byte(u8::from(value)),
        )
    }

    pub(crate) fn date(value: String) -> Self {
        Self::new(RntbdRequestToken::Date, TokenValue::SmallString(value))
    }

    /// Session token (id `0x0005`, `String`) carrying the client's per-partition
    /// LSN progress so the backend can serve session-consistent reads from a
    /// replica that is caught up. Populated from the `x-ms-session-token` header.
    pub(crate) fn session_token(value: String) -> Self {
        Self::new(RntbdRequestToken::SessionToken, TokenValue::String(value))
    }

    /// Page size (id `0x0004`, `ULong`) carrying the requested max item count
    /// for query/read-feed pages. Populated from the `x-ms-max-item-count`
    /// header; a negative request value (unbounded) is encoded as `0xFFFFFFFF`.
    pub(crate) fn page_size(value: u32) -> Self {
        Self::new(RntbdRequestToken::PageSize, TokenValue::ULong(value))
    }

    /// Match condition (id `0x0008`, `String`) carrying the ETag precondition.
    /// Populated from `If-None-Match` on reads/read-feeds and from `If-Match`
    /// on all other operations.
    pub(crate) fn match_condition(value: String) -> Self {
        Self::new(RntbdRequestToken::Match, TokenValue::String(value))
    }

    pub(crate) fn consistency_level(value: DefaultConsistencyLevel) -> Self {
        Self::new(
            RntbdRequestToken::ConsistencyLevel,
            TokenValue::Byte(value.rntbd_wire_byte()),
        )
    }

    /// `ReadConsistencyStrategy` token (id `0x00FE`, `Byte`).
    ///
    /// Server requires this token in place of `ConsistencyLevel` when the caller
    /// specifies a non-`Default` read consistency strategy on a read request.
    /// `Default` MUST never be serialized (callers gate on `is_non_default`).
    pub(crate) fn read_consistency_strategy(value: ReadConsistencyStrategy) -> Self {
        let byte = value.rntbd_wire_byte().unwrap_or_else(|| {
            // Caller-side guard: `Default` is transparent on the wire.
            debug_assert!(
                false,
                "Token::read_consistency_strategy must not be called with Default"
            );
            0x00
        });
        Self::new(
            RntbdRequestToken::ReadConsistencyStrategy,
            TokenValue::Byte(byte),
        )
    }

    pub(crate) fn database_name(value: String) -> Self {
        Self::new(RntbdRequestToken::DatabaseName, TokenValue::String(value))
    }

    pub(crate) fn collection_name(value: String) -> Self {
        Self::new(RntbdRequestToken::CollectionName, TokenValue::String(value))
    }

    pub(crate) fn collection_rid(value: String) -> Self {
        Self::new(RntbdRequestToken::CollectionRid, TokenValue::String(value))
    }

    /// Binary resource id (8 bytes) emitted alongside `CollectionRid`. Required by
    /// the Gateway 2.0 proxy as the document routing key (ID 0x0000, Bytes).
    pub(crate) fn resource_id(value: Vec<u8>) -> Self {
        Self::new(RntbdRequestToken::ResourceId, TokenValue::Bytes(value))
    }

    /// String-form partition key (JSON array) emitted alongside
    /// `EffectivePartitionKey` (ID 0x002B, String).
    pub(crate) fn partition_key(value: String) -> Self {
        Self::new(RntbdRequestToken::PartitionKey, TokenValue::String(value))
    }

    pub(crate) fn document_name(value: String) -> Self {
        Self::new(RntbdRequestToken::DocumentName, TokenValue::String(value))
    }

    pub(crate) fn effective_partition_key(value: Vec<u8>) -> Self {
        Self::new(
            RntbdRequestToken::EffectivePartitionKey,
            TokenValue::Bytes(value),
        )
    }

    pub(crate) fn sdk_supported_capabilities(value: u32) -> Self {
        Self::new(
            RntbdRequestToken::SDKSupportedCapabilities,
            TokenValue::ULong(value),
        )
    }

    /// `AllowTentativeWrites` (ID 0x0066, Byte). Emitted as `1` on every request.
    pub(crate) fn allow_tentative_writes(value: bool) -> Self {
        Self::new(
            RntbdRequestToken::AllowTentativeWrites,
            TokenValue::Byte(u8::from(value)),
        )
    }

    /// `ReturnPreference` (ID 0x0082, Byte). Emitted as `1` on every request.
    pub(crate) fn return_preference(value: bool) -> Self {
        Self::new(
            RntbdRequestToken::ReturnPreference,
            TokenValue::Byte(u8::from(value)),
        )
    }

    pub(crate) fn global_database_account_name(value: String) -> Self {
        Self::new(
            RntbdRequestToken::GlobalDatabaseAccountName,
            TokenValue::String(value),
        )
    }

    /// `SupportedQueryFeatures` (ID 0x00FF, String). Forwarded from the HTTP
    /// header `x-ms-cosmos-supported-query-features` so the proxy can resolve a
    /// QueryPlan compatible with what the client supports.
    pub(crate) fn supported_query_features(value: String) -> Self {
        Self::new(
            RntbdRequestToken::SupportedQueryFeatures,
            TokenValue::String(value),
        )
    }

    /// `QueryVersion` (ID 0x0100, SmallString). Forwarded from the HTTP header
    /// `x-ms-cosmos-query-version`.
    pub(crate) fn query_version(value: String) -> Self {
        Self::new(
            RntbdRequestToken::QueryVersion,
            TokenValue::SmallString(value),
        )
    }

    /// `StartEpkHash` (ID 0x00D2, Bytes). Per-partition routing key for
    /// thin-client cross-partition queries. The partition's `minInclusive` hex
    /// string is converted to bytes and emitted alongside `EndEpkHash`.
    pub(crate) fn start_epk_hash(value: Vec<u8>) -> Self {
        Self::new(RntbdRequestToken::StartEpkHash, TokenValue::Bytes(value))
    }

    /// `EndEpkHash` (ID 0x00D3, Bytes). See [`start_epk_hash`](Self::start_epk_hash).
    pub(crate) fn end_epk_hash(value: Vec<u8>) -> Self {
        Self::new(RntbdRequestToken::EndEpkHash, TokenValue::Bytes(value))
    }

    /// `PartitionKeyRangeId` (ID 0x002C, String). Filled from the
    /// `x-ms-documentdb-partitionkeyrangeid` HTTP header. The thin-client proxy
    /// expects this token alongside StartEpkHash/EndEpkHash to identify the
    /// target physical partition.
    pub(crate) fn partition_key_range_id(value: String) -> Self {
        Self::new(
            RntbdRequestToken::PartitionKeyRangeId,
            TokenValue::String(value),
        )
    }

    /// Pagination cursor echoed back to the proxy on subsequent feed/query
    /// requests (ID 0x0006, String) — the SDK passes the value through
    /// unchanged so the backend can resume from the previous offset.
    pub(crate) fn continuation_token(value: String) -> Self {
        Self::new(
            RntbdRequestToken::ContinuationToken,
            TokenValue::String(value),
        )
    }

    /// Returns the `Byte` value, or `None` if the token holds a different type.
    pub(crate) fn as_byte(&self) -> Option<u8> {
        match self.value {
            TokenValue::Byte(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the `ULong` (`u32`) value, or `None` if the token holds a different type.
    pub(crate) fn as_u32(&self) -> Option<u32> {
        match self.value {
            TokenValue::ULong(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the `LongLong` (`i64`) value, or `None` if the token holds a different type.
    pub(crate) fn as_i64(&self) -> Option<i64> {
        match self.value {
            TokenValue::LongLong(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the `Double` (`f64`) value, or `None` if the token holds a different type.
    pub(crate) fn as_f64(&self) -> Option<f64> {
        match self.value {
            TokenValue::Double(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the owned `String` value, consuming the token, or `None` if the
    /// token holds a different type.
    pub(crate) fn into_string(self) -> Option<String> {
        match self.value {
            TokenValue::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the number of bytes this token occupies on the wire.
    pub(super) fn encoded_len(&self) -> usize {
        2 + 1 + self.value.encoded_len()
    }

    /// Writes this token to the writer.
    ///
    /// Returns an error if the token value exceeds the wire encoding's length
    /// limits (e.g., a `SmallString` longer than 255 bytes) or the writer fails.
    pub(super) fn write_to(&self, out: &mut impl Write) -> azure_core::Result<()> {
        out.write_all(&self.id.value().to_le_bytes())?;
        out.write_all(&[self.value.token_type().into()])?;
        self.value.write_to(out)
    }

    /// Reads a token from the input slice and advances the slice.
    pub(super) fn read_from(src: &mut &[u8]) -> azure_core::Result<Self> {
        let id = read_u16_le(src)?;
        let token_type = TokenType::try_from(read_u8(src)?)?;
        let value = TokenValue::read_from(token_type, src)?;
        Ok(Self {
            id: id.into(),
            value,
        })
    }
}

/// RNTBD request metadata token IDs used by Gateway 2.0 dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RntbdRequestToken {
    ResourceId,
    AuthorizationToken,
    PayloadPresent,
    Date,
    PageSize,
    SessionToken,
    ContinuationToken,
    Match,
    ConsistencyLevel,
    DatabaseName,
    CollectionName,
    CollectionRid,
    DocumentName,
    TransportRequestId,
    PartitionKey,
    PartitionKeyRangeId,
    EffectivePartitionKey,
    AllowTentativeWrites,
    ReturnPreference,
    SDKSupportedCapabilities,
    GlobalDatabaseAccountName,
    ReadConsistencyStrategy,
    SupportedQueryFeatures,
    QueryVersion,
    StartEpkHash,
    EndEpkHash,
}

impl TryFrom<u16> for RntbdRequestToken {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0000 => Ok(Self::ResourceId),
            0x0001 => Ok(Self::AuthorizationToken),
            0x0002 => Ok(Self::PayloadPresent),
            0x0003 => Ok(Self::Date),
            0x0004 => Ok(Self::PageSize),
            0x0005 => Ok(Self::SessionToken),
            0x0006 => Ok(Self::ContinuationToken),
            0x0008 => Ok(Self::Match),
            0x0010 => Ok(Self::ConsistencyLevel),
            0x0015 => Ok(Self::DatabaseName),
            0x0016 => Ok(Self::CollectionName),
            0x0017 => Ok(Self::DocumentName),
            0x002B => Ok(Self::PartitionKey),
            0x002C => Ok(Self::PartitionKeyRangeId),
            0x0035 => Ok(Self::CollectionRid),
            0x004D => Ok(Self::TransportRequestId),
            0x005A => Ok(Self::EffectivePartitionKey),
            0x0066 => Ok(Self::AllowTentativeWrites),
            0x0082 => Ok(Self::ReturnPreference),
            0x00A2 => Ok(Self::SDKSupportedCapabilities),
            0x00CE => Ok(Self::GlobalDatabaseAccountName),
            0x00FE => Ok(Self::ReadConsistencyStrategy),
            0x00FF => Ok(Self::SupportedQueryFeatures),
            0x0100 => Ok(Self::QueryVersion),
            0x00D2 => Ok(Self::StartEpkHash),
            0x00D3 => Ok(Self::EndEpkHash),
            _ => Err(()),
        }
    }
}

impl From<RntbdRequestToken> for u16 {
    fn from(value: RntbdRequestToken) -> Self {
        match value {
            RntbdRequestToken::ResourceId => 0x0000,
            RntbdRequestToken::AuthorizationToken => 0x0001,
            RntbdRequestToken::PayloadPresent => 0x0002,
            RntbdRequestToken::Date => 0x0003,
            RntbdRequestToken::PageSize => 0x0004,
            RntbdRequestToken::SessionToken => 0x0005,
            RntbdRequestToken::ContinuationToken => 0x0006,
            RntbdRequestToken::Match => 0x0008,
            RntbdRequestToken::ConsistencyLevel => 0x0010,
            RntbdRequestToken::DatabaseName => 0x0015,
            RntbdRequestToken::CollectionName => 0x0016,
            RntbdRequestToken::DocumentName => 0x0017,
            RntbdRequestToken::PartitionKey => 0x002B,
            RntbdRequestToken::PartitionKeyRangeId => 0x002C,
            RntbdRequestToken::CollectionRid => 0x0035,
            RntbdRequestToken::TransportRequestId => 0x004D,
            RntbdRequestToken::EffectivePartitionKey => 0x005A,
            RntbdRequestToken::AllowTentativeWrites => 0x0066,
            RntbdRequestToken::ReturnPreference => 0x0082,
            RntbdRequestToken::SDKSupportedCapabilities => 0x00A2,
            RntbdRequestToken::GlobalDatabaseAccountName => 0x00CE,
            RntbdRequestToken::ReadConsistencyStrategy => 0x00FE,
            RntbdRequestToken::SupportedQueryFeatures => 0x00FF,
            RntbdRequestToken::QueryVersion => 0x0100,
            RntbdRequestToken::StartEpkHash => 0x00D2,
            RntbdRequestToken::EndEpkHash => 0x00D3,
        }
    }
}

impl From<RntbdRequestToken> for TokenId {
    fn from(value: RntbdRequestToken) -> Self {
        TokenId(u16::from(value))
    }
}

/// RNTBD response metadata token IDs recognized by Slice 1.
pub(super) enum RntbdResponseToken {
    /// Payload-present flag. When true, a u32 LE body-length prefix and that
    /// many body bytes follow the metadata section.
    PayloadPresent,
    /// Continuation token.
    ContinuationToken,
    /// Entity tag.
    ETag,
    /// Retry-after delay in milliseconds.
    RetryAfterMilliseconds,
    /// Logical sequence number.
    Lsn,
    /// Request charge in request units.
    RequestCharge,
    /// Owner full name.
    OwnerFullName,
    /// Cosmos DB sub-status code.
    SubStatus,
    /// Partition key range identifier.
    PartitionKeyRangeId,
    /// Item logical sequence number.
    ItemLsn,
    /// Global committed logical sequence number.
    GlobalCommittedLsn,
    /// Transport request identifier.
    TransportRequestId,
    /// Session token.
    SessionToken,
}

impl TryFrom<u16> for RntbdResponseToken {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0x0000 => Ok(Self::PayloadPresent),
            0x0003 => Ok(Self::ContinuationToken),
            0x0004 => Ok(Self::ETag),
            0x000C => Ok(Self::RetryAfterMilliseconds),
            0x0013 => Ok(Self::Lsn),
            0x0015 => Ok(Self::RequestCharge),
            0x0017 => Ok(Self::OwnerFullName),
            0x001C => Ok(Self::SubStatus),
            0x0021 => Ok(Self::PartitionKeyRangeId),
            0x0032 => Ok(Self::ItemLsn),
            0x0029 => Ok(Self::GlobalCommittedLsn),
            0x0035 => Ok(Self::TransportRequestId),
            0x003E => Ok(Self::SessionToken),
            _ => Err(()),
        }
    }
}

/// RNTBD resource type wire ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RntbdResourceType(u16);

impl RntbdResourceType {
    /// Returns the underlying RNTBD resource type ID.
    pub(super) fn value(self) -> u16 {
        self.0
    }
}

impl From<ResourceType> for RntbdResourceType {
    fn from(value: ResourceType) -> Self {
        let id = match value {
            ResourceType::DatabaseAccount => 0x0014,
            ResourceType::Database => 0x0001,
            ResourceType::DocumentCollection => 0x0002,
            ResourceType::Document => 0x0003,
            ResourceType::StoredProcedure => 0x0007,
            ResourceType::Trigger => 0x0009,
            ResourceType::UserDefinedFunction => 0x000A,
            ResourceType::PartitionKeyRange => 0x0016,
            ResourceType::Offer => 0x000F,
            // Distributed transactions do not use the thin-client RNTBD
            // encoder; they route through the standard gateway coordinator.
            #[cfg(feature = "preview_dtx")]
            ResourceType::DistributedTransactionBatch => unreachable!(
                "ResourceType::DistributedTransactionBatch must not reach RNTBD encoding"
            ),
        };
        Self(id)
    }
}

impl TryFrom<u16> for RntbdResourceType {
    type Error = azure_core::Error;

    fn try_from(value: u16) -> azure_core::Result<Self> {
        match value {
            0x0014 | 0x0001 | 0x0002 | 0x0003 | 0x0007 | 0x0009 | 0x000A | 0x0016 | 0x000F => {
                Ok(Self(value))
            }
            other => Err(data_conversion_error(format!(
                "unknown RNTBD resource type 0x{other:04X}"
            ))),
        }
    }
}

impl TryFrom<RntbdResourceType> for ResourceType {
    type Error = azure_core::Error;

    fn try_from(value: RntbdResourceType) -> azure_core::Result<Self> {
        match value.0 {
            0x0014 => Ok(Self::DatabaseAccount),
            0x0001 => Ok(Self::Database),
            0x0002 => Ok(Self::DocumentCollection),
            0x0003 => Ok(Self::Document),
            0x0007 => Ok(Self::StoredProcedure),
            0x0009 => Ok(Self::Trigger),
            0x000A => Ok(Self::UserDefinedFunction),
            0x0016 => Ok(Self::PartitionKeyRange),
            0x000F => Ok(Self::Offer),
            _ => Err(data_conversion_error("unknown RNTBD resource type")),
        }
    }
}

/// RNTBD operation type wire ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RntbdOperationType(u16);

impl RntbdOperationType {
    /// Returns the underlying RNTBD operation type ID.
    pub(super) fn value(self) -> u16 {
        self.0
    }
}

impl From<OperationType> for RntbdOperationType {
    fn from(value: OperationType) -> Self {
        let id = match value {
            OperationType::Create => 0x0001,
            OperationType::Read => 0x0003,
            OperationType::ReadFeed => 0x0004,
            OperationType::Delete => 0x0005,
            OperationType::Replace => 0x0006,
            OperationType::Execute => 0x0008,
            OperationType::SqlQuery => 0x0009,
            // The Gateway V2 thin-client proxy dispatches QueryPlan via this
            // distinct op id 0x0042 (not SqlQuery's 0x0009).
            OperationType::QueryPlan => 0x0042,
            OperationType::Query => 0x000F,
            OperationType::Head => 0x0011,
            OperationType::HeadFeed => 0x0012,
            OperationType::Upsert => 0x0013,
            OperationType::Batch => 0x0025,
            // Patch operations are handled by the driver-side patch_handler
            // pipeline stage (Read-Modify-Write) and never reach the RNTBD
            // transport encoder. Reaching this arm is a bug in the driver
            // dispatch path.
            OperationType::Patch => {
                unreachable!(
                    "OperationType::Patch must be handled by patch_handler before RNTBD encoding"
                )
            }
            // Distributed transactions do not use the thin-client RNTBD
            // encoder; they route through the standard gateway coordinator.
            #[cfg(feature = "preview_dtx")]
            OperationType::CommitDistributedTransaction
            | OperationType::ReadDistributedTransaction => {
                unreachable!("distributed transaction operations must not reach RNTBD encoding")
            }
        };
        Self(id)
    }
}

impl TryFrom<u16> for RntbdOperationType {
    type Error = azure_core::Error;

    fn try_from(value: u16) -> azure_core::Result<Self> {
        match value {
            0x0001 | 0x0003 | 0x0004 | 0x0005 | 0x0006 | 0x0008 | 0x0009 | 0x000F | 0x0011
            | 0x0012 | 0x0013 | 0x0025 => Ok(Self(value)),
            other => Err(data_conversion_error(format!(
                "unknown RNTBD operation type 0x{other:04X}"
            ))),
        }
    }
}

impl TryFrom<RntbdOperationType> for OperationType {
    type Error = azure_core::Error;

    fn try_from(value: RntbdOperationType) -> azure_core::Result<Self> {
        match value.0 {
            0x0001 => Ok(Self::Create),
            0x0003 => Ok(Self::Read),
            0x0004 => Ok(Self::ReadFeed),
            0x0005 => Ok(Self::Delete),
            0x0006 => Ok(Self::Replace),
            0x0008 => Ok(Self::Execute),
            0x0009 => Ok(Self::SqlQuery),
            0x000F => Ok(Self::Query),
            0x0011 => Ok(Self::Head),
            0x0012 => Ok(Self::HeadFeed),
            0x0013 => Ok(Self::Upsert),
            0x0025 => Ok(Self::Batch),
            _ => Err(data_conversion_error("unknown RNTBD operation type")),
        }
    }
}

/// Creates a data-conversion error for malformed RNTBD input.
pub(super) fn data_conversion_error(message: impl Into<String>) -> azure_core::Error {
    azure_core::Error::with_message(ErrorKind::DataConversion, message.into())
}

/// Writes a UUID using the Microsoft GUID wire format.
///
/// The wire form is `Data1` (u32 LE), `Data2` (u16 LE), `Data3` (u16 LE),
/// then the final 8 bytes (`Data4`) in their natural order. This is the
/// same encoding used by the Cosmos DB RNTBD protocol for both the frame
/// header `activityId` and `Guid`-typed token values.
pub(super) fn write_uuid_le(out: &mut impl Write, id: Uuid) -> azure_core::Result<()> {
    let (data1, data2, data3, data4) = id.as_fields();
    out.write_all(&data1.to_le_bytes())?;
    out.write_all(&data2.to_le_bytes())?;
    out.write_all(&data3.to_le_bytes())?;
    out.write_all(data4)?;
    Ok(())
}

/// Reads a UUID using the Microsoft GUID wire format. See
/// [`write_uuid_le`] for the byte layout.
pub(super) fn read_uuid_le(src: &mut &[u8]) -> azure_core::Result<Uuid> {
    let data1 = u32::from_le_bytes(read_array(src)?);
    let data2 = u16::from_le_bytes(read_array(src)?);
    let data3 = u16::from_le_bytes(read_array(src)?);
    let data4 = read_array(src)?;
    Ok(Uuid::from_fields(data1, data2, data3, &data4))
}

/// Reads an unsigned byte from the input slice.
pub(super) fn read_u8(src: &mut &[u8]) -> azure_core::Result<u8> {
    Ok(read_exact(src, 1, "u8")?[0])
}

/// Reads an unsigned 16-bit little-endian integer from the input slice.
pub(super) fn read_u16_le(src: &mut &[u8]) -> azure_core::Result<u16> {
    Ok(u16::from_le_bytes(read_array(src)?))
}

/// Reads an unsigned 32-bit little-endian integer from the input slice.
pub(super) fn read_u32_le(src: &mut &[u8]) -> azure_core::Result<u32> {
    Ok(u32::from_le_bytes(read_array(src)?))
}

/// Reads an unsigned 64-bit little-endian integer from the input slice.
pub(super) fn read_u64_le(src: &mut &[u8]) -> azure_core::Result<u64> {
    Ok(u64::from_le_bytes(read_array(src)?))
}

fn read_i32_le(src: &mut &[u8]) -> azure_core::Result<i32> {
    Ok(i32::from_le_bytes(read_array(src)?))
}

fn read_i64_le(src: &mut &[u8]) -> azure_core::Result<i64> {
    Ok(i64::from_le_bytes(read_array(src)?))
}

fn read_array<const N: usize>(src: &mut &[u8]) -> azure_core::Result<[u8; N]> {
    let bytes = read_exact(src, N, "fixed-width value")?;
    let mut out = [0_u8; N];
    out.copy_from_slice(bytes);
    Ok(out)
}

fn read_exact<'a>(src: &mut &'a [u8], len: usize, context: &str) -> azure_core::Result<&'a [u8]> {
    if src.len() < len {
        return Err(data_conversion_error(format!(
            "RNTBD {context} needs {len} bytes but only {} remain",
            src.len()
        )));
    }
    let (head, tail) = src.split_at(len);
    *src = tail;
    Ok(head)
}

fn read_utf8(src: &mut &[u8], len: usize) -> azure_core::Result<String> {
    let bytes = read_exact(src, len, "UTF-8 string")?;
    String::from_utf8(bytes.to_vec())
        .map_err(|e| azure_core::Error::new(ErrorKind::DataConversion, e))
}

fn write_len_prefixed_u8(out: &mut impl Write, bytes: &[u8]) -> azure_core::Result<()> {
    let len = u8::try_from(bytes.len()).map_err(|_| {
        data_conversion_error(format!(
            "RNTBD value length {} exceeds u8 length-prefix maximum (255)",
            bytes.len()
        ))
    })?;
    out.write_all(&[len])?;
    out.write_all(bytes)?;
    Ok(())
}

fn write_len_prefixed_u16(out: &mut impl Write, bytes: &[u8]) -> azure_core::Result<()> {
    let len = u16::try_from(bytes.len()).map_err(|_| {
        data_conversion_error(format!(
            "RNTBD value length {} exceeds u16 length-prefix maximum (65535)",
            bytes.len()
        ))
    })?;
    out.write_all(&len.to_le_bytes())?;
    out.write_all(bytes)?;
    Ok(())
}

fn write_len_prefixed_u32(out: &mut impl Write, bytes: &[u8]) -> azure_core::Result<()> {
    let len = u32::try_from(bytes.len()).map_err(|_| {
        data_conversion_error(format!(
            "RNTBD value length {} exceeds u32 length-prefix maximum (4294967295)",
            bytes.len()
        ))
    })?;
    out.write_all(&len.to_le_bytes())?;
    out.write_all(bytes)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invalid_token_type_sentinel_is_rejected() {
        let mut src = [0x01, 0x00, 0xFF].as_slice();

        let err = Token::read_from(&mut src).unwrap_err();

        assert_eq!(*err.kind(), ErrorKind::DataConversion);
    }

    #[test]
    fn small_string_rejects_length_past_remaining_buffer() {
        let mut src = [0x01, 0x00, 0x07, 0x05, b'h', b'i'].as_slice();

        let err = Token::read_from(&mut src).unwrap_err();

        assert_eq!(*err.kind(), ErrorKind::DataConversion);
    }

    #[test]
    fn all_token_types_round_trip_through_wire_id() {
        // Every recognized `TokenType` value must round-trip from
        // enum -> u8 -> enum without loss.
        let all_token_types = [
            TokenType::Byte,
            TokenType::UShort,
            TokenType::ULong,
            TokenType::Long,
            TokenType::ULongLong,
            TokenType::LongLong,
            TokenType::Guid,
            TokenType::SmallString,
            TokenType::String,
            TokenType::ULongString,
            TokenType::SmallBytes,
            TokenType::Bytes,
            TokenType::ULongBytes,
            TokenType::Float,
            TokenType::Double,
            TokenType::Invalid,
        ];

        for token_type in all_token_types {
            let wire_id: u8 = token_type.into();
            let decoded = TokenType::try_from(wire_id).unwrap_or_else(|err| {
                panic!("token type {token_type:?} (wire 0x{wire_id:02X}) did not round-trip: {err}")
            });
            assert_eq!(decoded, token_type);
        }
    }

    #[test]
    fn token_value_guid_matches_reference_bytes() {
        // A `Guid` token value must round-trip to the canonical 16-byte MS
        // GUID layout. The reference UUID + bytes here are fixed so a future
        // endianness regression in `write_guid_ms` is caught.
        let id = Uuid::parse_str("8f3322cc-1786-4db4-9b97-b229c2c6f0aa").unwrap();
        let expected_bytes: [u8; 16] = [
            0xCC, 0x22, 0x33, 0x8F, 0x86, 0x17, 0xB4, 0x4D, 0x9B, 0x97, 0xB2, 0x29, 0xC2, 0xC6,
            0xF0, 0xAA,
        ];

        let token = Token::new(RntbdRequestToken::AuthorizationToken, TokenValue::Guid(id));
        let mut encoded = Vec::new();
        token.write_to(&mut encoded).unwrap();

        // Token wire layout: u16 id + u8 type + payload. The first 3 bytes
        // are framing; the remaining 16 must be the canonical MS GUID
        // encoding.
        let payload_offset = 2 + 1;
        assert_eq!(
            &encoded[payload_offset..payload_offset + 16],
            expected_bytes.as_slice(),
            "Guid token payload diverges from MS GUID wire format"
        );

        let mut src = encoded.as_slice();
        let decoded = Token::read_from(&mut src).unwrap();
        match &decoded.value {
            TokenValue::Guid(round_tripped) => assert_eq!(*round_tripped, id),
            other => panic!("expected TokenValue::Guid, got {other:?}"),
        }
        assert!(src.is_empty());
    }

    #[test]
    fn read_consistency_strategy_token_byte_mapping() {
        // The RNTBD `ReadConsistencyStrategy` token has id `0x00FE`, `Byte`
        // type, and these exact byte values. A drift here means the SDK and
        // the gateway disagree about what consistency the caller requested.
        let cases = [
            (ReadConsistencyStrategy::Eventual, 0x01u8),
            (ReadConsistencyStrategy::Session, 0x02u8),
            (ReadConsistencyStrategy::LatestCommitted, 0x03u8),
            (ReadConsistencyStrategy::GlobalStrong, 0x04u8),
        ];

        for (strategy, expected_byte) in cases {
            let token = Token::read_consistency_strategy(strategy);
            let mut encoded = Vec::new();
            token.write_to(&mut encoded).unwrap();
            // u16 id (LE) + u8 token type + u8 byte value
            assert_eq!(encoded[0], 0xFE);
            assert_eq!(encoded[1], 0x00);
            assert_eq!(
                encoded[2], 0x00,
                "ReadConsistencyStrategy must be Byte (0x00)"
            );
            assert_eq!(
                encoded[3], expected_byte,
                "{strategy:?} byte value mismatch"
            );
        }
    }
}
