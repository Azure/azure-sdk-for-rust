// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resource identification types for Cosmos DB resources.
//!
//! This module provides newtypes for resource names and RIDs (resource identifiers),
//! as well as internal ID enums that enforce either all-names or all-RIDs addressing.

use std::borrow::Cow;

/// A resource name (user-provided identifier).
///
/// Used for human-readable identifiers like database names, container names, etc.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) struct ResourceName(Cow<'static, str>);

impl ResourceName {
    /// Creates a new resource name.
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }

    /// Returns the name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for ResourceName {
    fn from(s: &'static str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ResourceName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for ResourceName {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ResourceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A resource identifier (RID) - internal Cosmos DB identifier.
///
/// RIDs are base64-encoded internal identifiers assigned by Cosmos DB.
/// They encode the resource hierarchy (account → database → container → document).
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) struct ResourceRid(Cow<'static, str>);

impl ResourceRid {
    /// Creates a new resource RID.
    pub fn new(rid: impl Into<Cow<'static, str>>) -> Self {
        Self(rid.into())
    }

    /// Returns the RID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&'static str> for ResourceRid {
    fn from(s: &'static str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ResourceRid {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl AsRef<str> for ResourceRid {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ResourceRid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// =============================================================================
// Internal ID Enums (pub(crate))
// =============================================================================
// These enums enforce either all-names or all-RIDs addressing at compile time.

/// Generic resource identifier: either by name or by RID.
///
/// This is reused across resource reference types (including databases) to avoid
/// duplicating identical `ByName`/`ByRid` enums per resource.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) enum ResourceIdentifierType {
    /// Reference by user-provided resource name.
    ByName(ResourceName),
    /// Reference by internal RID.
    ByRid(ResourceRid),
}

impl ResourceIdentifierType {
    /// Creates a resource identifier by name.
    pub(crate) fn by_name(name: impl Into<ResourceName>) -> Self {
        Self::ByName(name.into())
    }

    /// Creates a resource identifier by RID.
    pub(crate) fn by_rid(rid: impl Into<ResourceRid>) -> Self {
        Self::ByRid(rid.into())
    }

    /// Returns the resource name if this is a name-based identifier.
    pub(crate) fn name(&self) -> Option<&str> {
        match self {
            Self::ByName(name) => Some(name.as_str()),
            Self::ByRid(_) => None,
        }
    }

    /// Returns the resource RID if this is a RID-based identifier.
    pub(crate) fn rid(&self) -> Option<&str> {
        match self {
            Self::ByName(_) => None,
            Self::ByRid(rid) => Some(rid.as_str()),
        }
    }

    /// Returns `true` if this is a name-based identifier.
    pub(crate) fn is_by_name(&self) -> bool {
        matches!(self, Self::ByName(_))
    }

    /// Returns `true` if this is a RID-based identifier.
    pub(crate) fn is_by_rid(&self) -> bool {
        matches!(self, Self::ByRid(_))
    }
}

// =============================================================================
// ParsedResourceId (pub(crate))
// =============================================================================

/// Parsed components of a Cosmos DB RID.
///
/// RIDs encode the resource hierarchy. This struct extracts the individual
/// components for validation and path construction.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) struct ParsedResourceId {
    /// The database RID component (if present).
    database_rid: Option<ResourceRid>,
    /// The container/collection RID component (if present).
    container_rid: Option<ResourceRid>,
    /// The document/item RID component (if present).
    document_rid: Option<ResourceRid>,
}

#[cfg(test)]
impl ParsedResourceId {
    /// Creates an empty parsed resource ID.
    pub(crate) fn empty() -> Self {
        Self {
            database_rid: None,
            container_rid: None,
            document_rid: None,
        }
    }

    /// Creates a parsed resource ID for a database.
    pub(crate) fn database(database_rid: ResourceRid) -> Self {
        Self {
            database_rid: Some(database_rid),
            container_rid: None,
            document_rid: None,
        }
    }

    /// Creates a parsed resource ID for a container.
    pub(crate) fn container(database_rid: ResourceRid, container_rid: ResourceRid) -> Self {
        Self {
            database_rid: Some(database_rid),
            container_rid: Some(container_rid),
            document_rid: None,
        }
    }

    /// Creates a parsed resource ID for a document.
    pub(crate) fn document(
        database_rid: ResourceRid,
        container_rid: ResourceRid,
        document_rid: ResourceRid,
    ) -> Self {
        Self {
            database_rid: Some(database_rid),
            container_rid: Some(container_rid),
            document_rid: Some(document_rid),
        }
    }

    /// Returns the database RID component.
    pub(crate) fn database_rid(&self) -> Option<&ResourceRid> {
        self.database_rid.as_ref()
    }

    /// Returns the container RID component.
    pub(crate) fn container_rid(&self) -> Option<&ResourceRid> {
        self.container_rid.as_ref()
    }

    /// Returns the document RID component.
    pub(crate) fn document_rid(&self) -> Option<&ResourceRid> {
        self.document_rid.as_ref()
    }
}

// =============================================================================
// RID Parsing Utilities
// =============================================================================

/// Cosmos DB RID encoding/decoding utilities.
///
/// RIDs are encoded using standard Base64 with `=` padding, but with `/`
/// replaced by `-` for transport safety. The binary representation uses
/// big-endian byte ordering with a hierarchical structure:
///
/// | Offset | Size    | Field              |
/// |--------|---------|--------------------|
/// | 0–3    | 4 bytes | Database ID        |
/// | 4–7    | 4 bytes | Collection ID      |
/// | 8–15   | 8 bytes | Sub-resource ID    |
/// | 16–19  | 4 bytes | Attachment ID      |
///
/// Valid RID byte lengths: 3 (offer), 4 (database), 8 (collection),
/// 16 (document/SP/trigger/UDF/PKRange), 20 (attachment).
#[cfg(test)]
use base64::{engine::general_purpose::STANDARD, Engine as _};

/// Decodes a Cosmos DB RID string into its raw bytes.
///
/// RIDs use standard Base64 with `-` substituted for `/`.
#[cfg(test)]
fn decode_rid(rid: &str) -> Result<Vec<u8>, RidParseError> {
    if rid.is_empty() {
        return Err(RidParseError::Empty);
    }
    if !rid.len().is_multiple_of(4) {
        return Err(RidParseError::InvalidLength);
    }
    let b64 = rid.replace('-', "/");
    STANDARD
        .decode(&b64)
        .map_err(|_| RidParseError::InvalidBase64)
}

/// Encodes raw bytes into a Cosmos DB RID string.
///
/// Uses standard Base64 with `/` replaced by `-`.
#[cfg(test)]
fn encode_rid(bytes: &[u8]) -> String {
    STANDARD.encode(bytes).replace('/', "-")
}

/// Extracts the database RID string from a container (collection) RID string.
///
/// A container RID is 8 bytes: the first 4 bytes encode the parent database ID.
/// This function extracts those 4 bytes and re-encodes them as a database RID.
///
/// # Errors
///
/// Returns `Err` if the input is not a valid container RID (must decode to
/// exactly 8 bytes with `buffer.len() % 4 == 0`).
///
/// # Example
///
/// ```ignore
/// let db_rid = extract_database_rid_from_container_rid("dbs-rid==").unwrap();
/// ```
#[cfg(test)]
pub(crate) fn extract_database_rid_from_container_rid(
    container_rid: &str,
) -> Result<ResourceRid, RidParseError> {
    let bytes = decode_rid(container_rid)?;
    if bytes.len() < 8 || bytes.len() % 4 != 0 {
        return Err(RidParseError::InvalidLength);
    }
    // First 4 bytes are the database ID
    let db_bytes = &bytes[0..4];
    Ok(ResourceRid::new(encode_rid(db_bytes)))
}

/// Extracts the container (collection) RID string from a document or sub-resource RID string.
///
/// A document RID is 16 bytes: the first 8 bytes encode the parent container
/// (which itself encodes the database in its first 4 bytes).
///
/// # Errors
///
/// Returns `Err` if the input is not a valid document/sub-resource RID
/// (must decode to at least 16 bytes with `buffer.len() % 4 == 0`).
#[cfg(test)]
pub(crate) fn extract_container_rid_from_document_rid(
    document_rid: &str,
) -> Result<ResourceRid, RidParseError> {
    let bytes = decode_rid(document_rid)?;
    if bytes.len() < 16 || bytes.len() % 4 != 0 {
        return Err(RidParseError::InvalidLength);
    }
    // First 8 bytes are the container ID (which includes the database ID)
    let container_bytes = &bytes[0..8];
    Ok(ResourceRid::new(encode_rid(container_bytes)))
}

/// Parses a RID string into its hierarchical components.
///
/// Determines the resource level based on byte length and populates
/// the appropriate fields of `ParsedResourceId`.
///
/// # Errors
///
/// Returns `Err` if the input is not a valid Cosmos DB RID.
#[cfg(test)]
pub(crate) fn parse_rid(rid: &str) -> Result<ParsedResourceId, RidParseError> {
    let bytes = decode_rid(rid)?;
    let len = bytes.len();

    // Offer RIDs are 3 bytes — not relevant for our resource hierarchy
    if len == 3 {
        return Ok(ParsedResourceId::empty());
    }

    if len % 4 != 0 {
        return Err(RidParseError::InvalidLength);
    }

    let mut parsed = ParsedResourceId::empty();

    if len >= 4 {
        let db_rid = encode_rid(&bytes[0..4]);
        parsed.database_rid = Some(ResourceRid::new(db_rid));
    }

    if len >= 8 {
        let container_rid = encode_rid(&bytes[0..8]);
        parsed.container_rid = Some(ResourceRid::new(container_rid));
    }

    if len >= 16 {
        let document_rid = encode_rid(&bytes[0..16]);
        parsed.document_rid = Some(ResourceRid::new(document_rid));
    }

    Ok(parsed)
}

/// Errors that can occur when parsing a Cosmos DB RID.
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg(test)]
pub(crate) enum RidParseError {
    /// The RID string is empty.
    Empty,
    /// The RID string length is not a multiple of 4 (invalid Base64 padding).
    InvalidLength,
    /// The RID string contains invalid Base64 characters.
    InvalidBase64,
}

#[cfg(test)]
impl std::fmt::Display for RidParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "RID string is empty"),
            Self::InvalidLength => write!(f, "RID has invalid byte length"),
            Self::InvalidBase64 => write!(f, "RID contains invalid Base64"),
        }
    }
}

#[cfg(test)]
impl std::error::Error for RidParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resource_name_from_str() {
        let name = ResourceName::from("mydb");
        assert_eq!(name.as_str(), "mydb");
    }

    #[test]
    fn resource_name_from_string() {
        let name = ResourceName::from(String::from("mydb"));
        assert_eq!(name.as_str(), "mydb");
    }

    #[test]
    fn resource_rid_from_str() {
        let rid = ResourceRid::from("abc123");
        assert_eq!(rid.as_str(), "abc123");
    }

    #[test]
    fn database_id_by_name() {
        let id = ResourceIdentifierType::ByName(ResourceName::from("testdb"));
        assert_eq!(id.name(), Some("testdb"));
        assert_eq!(id.rid(), None);
    }

    #[test]
    fn database_id_by_rid() {
        let id = ResourceIdentifierType::ByRid(ResourceRid::from("abc123"));
        assert_eq!(id.name(), None);
        assert_eq!(id.rid(), Some("abc123"));
    }

    #[test]
    fn parsed_resource_id_database() {
        let parsed = ParsedResourceId::database(ResourceRid::from("db123"));
        assert_eq!(parsed.database_rid().map(|r| r.as_str()), Some("db123"));
        assert!(parsed.container_rid().is_none());
        assert!(parsed.document_rid().is_none());
    }

    #[test]
    fn parsed_resource_id_container() {
        let parsed =
            ParsedResourceId::container(ResourceRid::from("db123"), ResourceRid::from("coll456"));
        assert_eq!(parsed.database_rid().map(|r| r.as_str()), Some("db123"));
        assert_eq!(parsed.container_rid().map(|r| r.as_str()), Some("coll456"));
        assert!(parsed.document_rid().is_none());
    }

    #[test]
    fn parsed_resource_id_document() {
        let parsed = ParsedResourceId::document(
            ResourceRid::from("db123"),
            ResourceRid::from("coll456"),
            ResourceRid::from("doc789"),
        );
        assert_eq!(parsed.database_rid().map(|r| r.as_str()), Some("db123"));
        assert_eq!(parsed.container_rid().map(|r| r.as_str()), Some("coll456"));
        assert_eq!(parsed.document_rid().map(|r| r.as_str()), Some("doc789"));
    }

    // ===== RID parsing tests =====

    #[test]
    fn decode_and_encode_rid_roundtrip() {
        // A database RID is 4 bytes → 8 chars in base64
        let db_bytes: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let encoded = encode_rid(&db_bytes);
        let decoded = decode_rid(&encoded).unwrap();
        assert_eq!(decoded, db_bytes);
    }

    #[test]
    fn decode_rid_replaces_dash_with_slash() {
        // Manually create a base64 string that contains '/'
        // then replace with '-' and verify decoding works
        let bytes: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
        let b64 = STANDARD.encode(bytes);
        // Standard base64 of [0xFF, 0xFF, 0xFF, 0xFF] is "////\nw==" or similar
        let cosmos_rid = b64.replace('/', "-");
        let decoded = decode_rid(&cosmos_rid).unwrap();
        assert_eq!(decoded, bytes);
    }

    #[test]
    fn encode_rid_replaces_slash_with_dash() {
        let bytes: [u8; 4] = [0xFF, 0xFF, 0xFF, 0xFF];
        let encoded = encode_rid(&bytes);
        assert!(!encoded.contains('/'), "encoded RID should not contain '/'");
    }

    #[test]
    fn decode_rid_empty_returns_error() {
        assert_eq!(decode_rid(""), Err(RidParseError::Empty));
    }

    #[test]
    fn decode_rid_invalid_length_returns_error() {
        assert_eq!(decode_rid("abc"), Err(RidParseError::InvalidLength));
    }

    #[test]
    fn extract_database_rid_from_container_rid_valid() {
        // Create a container RID: 8 bytes = 4 db bytes + 4 collection bytes
        let mut container_bytes = [0u8; 8];
        container_bytes[0..4].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D]); // db
        container_bytes[4..8].copy_from_slice(&[0x80, 0x01, 0x02, 0x03]); // collection (high bit set)
        let container_rid = encode_rid(&container_bytes);

        let db_rid = extract_database_rid_from_container_rid(&container_rid).unwrap();

        // Verify the database RID is just the first 4 bytes
        let expected_db_rid = encode_rid(&[0x0A, 0x0B, 0x0C, 0x0D]);
        assert_eq!(db_rid.as_str(), expected_db_rid);
    }

    #[test]
    fn extract_database_rid_from_short_rid_returns_error() {
        // A database RID (4 bytes) is too short to be a container RID
        let db_bytes: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let db_rid = encode_rid(&db_bytes);
        assert_eq!(
            extract_database_rid_from_container_rid(&db_rid),
            Err(RidParseError::InvalidLength)
        );
    }

    #[test]
    fn extract_container_rid_from_document_rid_valid() {
        // Create a document RID: 16 bytes
        let mut doc_bytes = [0u8; 16];
        doc_bytes[0..4].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D]); // db
        doc_bytes[4..8].copy_from_slice(&[0x80, 0x01, 0x02, 0x03]); // collection
        doc_bytes[8..16].copy_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00]); // doc

        let doc_rid = encode_rid(&doc_bytes);
        let container_rid = extract_container_rid_from_document_rid(&doc_rid).unwrap();

        let expected = encode_rid(&doc_bytes[0..8]);
        assert_eq!(container_rid.as_str(), expected);
    }

    #[test]
    fn parse_rid_database() {
        let db_bytes: [u8; 4] = [0x01, 0x02, 0x03, 0x04];
        let rid_str = encode_rid(&db_bytes);
        let parsed = parse_rid(&rid_str).unwrap();

        assert!(parsed.database_rid().is_some());
        assert!(parsed.container_rid().is_none());
        assert!(parsed.document_rid().is_none());
    }

    #[test]
    fn parse_rid_container() {
        let mut bytes = [0u8; 8];
        bytes[0..4].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D]);
        bytes[4..8].copy_from_slice(&[0x80, 0x01, 0x02, 0x03]);
        let rid_str = encode_rid(&bytes);
        let parsed = parse_rid(&rid_str).unwrap();

        assert!(parsed.database_rid().is_some());
        assert!(parsed.container_rid().is_some());
        assert!(parsed.document_rid().is_none());

        // Verify we can extract database from the parsed container RID
        let container_rid = parsed.container_rid().unwrap().as_str();
        let db_rid = extract_database_rid_from_container_rid(container_rid).unwrap();
        assert_eq!(db_rid.as_str(), parsed.database_rid().unwrap().as_str());
    }

    #[test]
    fn parse_rid_document() {
        let mut bytes = [0u8; 16];
        bytes[0..4].copy_from_slice(&[0x0A, 0x0B, 0x0C, 0x0D]);
        bytes[4..8].copy_from_slice(&[0x80, 0x01, 0x02, 0x03]);
        bytes[8..16].copy_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x00]);
        let rid_str = encode_rid(&bytes);
        let parsed = parse_rid(&rid_str).unwrap();

        assert!(parsed.database_rid().is_some());
        assert!(parsed.container_rid().is_some());
        assert!(parsed.document_rid().is_some());
    }
}
