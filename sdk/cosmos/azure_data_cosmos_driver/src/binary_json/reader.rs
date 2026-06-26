// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos binary JSON **decoder** (`binary` → [`serde_json::Value`]).
//!
//! The decoder parses **untrusted** service bytes, so every step is
//! bounds-checked and returns a [`BinaryError`] rather than panicking; a
//! malformed or truncated buffer must fail gracefully. Multi-byte integers and
//! length prefixes are little-endian, matching the service.
//!
//! # Phase status
//!
//! Implemented so far: the `Reader` cursor infrastructure, the scalar value
//! forms ([`null`](serde_json::Value::Null), booleans, literal and fixed-width
//! numbers — including the extended `Int8`/`Int16`/`Int32`/`Int64`/`UInt32`
//! and `Float32`/`Float64` forms — and the common string forms (system
//! strings, encoded-length strings, `StrL1`/`StrL2`/`StrL4`, and reference
//! strings `StrR1`–`StrR4`), and **containers** (arrays `0xE0`–`0xE7` and
//! objects `0xE8`–`0xEF`) with a nesting-depth guard. User strings
//! (`0x40`–`0x67`) are recognized but report
//! [`BinaryError::UnsupportedUserString`] because they reference an external
//! dictionary the data plane does not supply. `Float16` (`0xCF`) and the
//! extended `UInt8` (`0xD7`) have no JSON node type and are rejected as
//! [`BinaryError::InvalidMarker`]. The remaining forms surface as
//! [`BinaryError::InvalidMarker`] until their sub-phase lands:
//!
//! - **P1d-2 (done):** GUID strings (`0x75`–`0x77`).
//! - **P1d-3 (done):** base64 strings (`0x71`–`0x74`).
//! - **P1d-4 (done):** compressed strings (`0x78`–`0x7F`).
//! - **P1d-5:** GUID value (`0xD3`), binary (`0xDD`–`0xDF`), and uniform
//!   number arrays (`0xF0`–`0xF3`).

use base64::Engine;
use serde_json::{Map, Value};

use super::markers::{
    ARR0, ARR1, ARR_L1, ARR_L2, ARR_L4, ARR_LC1, ARR_LC2, ARR_LC4, BASE64_STRING_LENGTH1,
    BASE64_STRING_LENGTH2, BASE64_URL_STRING_LENGTH1, BASE64_URL_STRING_LENGTH2,
    COMPRESSED_DATE_TIME_STRING, COMPRESSED_LOWERCASE_HEX_STRING, COMPRESSED_UPPERCASE_HEX_STRING,
    DOUBLE_QUOTED_LOWERCASE_GUID_STRING, ENCODED_STRING_LENGTH_MASK, ENCODED_STRING_LENGTH_MAX,
    ENCODED_STRING_LENGTH_MIN, FALSE, FLOAT32, FLOAT64, INT16, INT32, INT64, INT8, LITERAL_INT_MAX,
    LITERAL_INT_MIN, LOWERCASE_GUID_STRING, NULL, NUMBER_DOUBLE, NUMBER_INT16, NUMBER_INT32,
    NUMBER_INT64, NUMBER_UINT64, NUMBER_UINT8, OBJ0, OBJ1, OBJ_L1, OBJ_L2, OBJ_L4, OBJ_LC1,
    OBJ_LC2, OBJ_LC4, PACKED_4BIT_STRING, PACKED_5BIT_STRING, PACKED_6BIT_STRING,
    PACKED_7BIT_STRING_LENGTH1, PACKED_7BIT_STRING_LENGTH2, STR_L1, STR_L2, STR_L4, STR_R1, STR_R2,
    STR_R3, STR_R4, SYSTEM_STRING_1BYTE_MAX, SYSTEM_STRING_1BYTE_MIN, TRUE, UINT32,
    UPPERCASE_GUID_STRING, USER_STRING_1BYTE_MAX, USER_STRING_1BYTE_MIN, USER_STRING_2BYTE_MAX,
    USER_STRING_2BYTE_MIN,
};
use super::system_strings::system_string_for_marker;
use super::{is_binary, BinaryError, Result};

/// Maximum container nesting depth the decoder will descend before returning
/// [`BinaryError::DepthLimitExceeded`]. This mirrors the .NET Cosmos JSON
/// stack's `JsonObjectState.JsonMaxNestingDepth` (256 simultaneously-open
/// containers), so the Rust decoder enforces the same nesting policy while
/// guarding against stack exhaustion from adversarial input.
const MAX_DEPTH: usize = 256;

/// Decodes a complete Cosmos binary JSON buffer into a [`serde_json::Value`].
///
/// The buffer must begin with the [`PREAMBLE`](super::PREAMBLE) byte (`0x80`); the single
/// top-level value that follows is decoded, and any bytes left over afterwards
/// are reported as [`BinaryError::TrailingBytes`].
///
/// # Errors
///
/// Returns a [`BinaryError`] if the buffer is not binary (missing preamble),
/// is truncated, contains an invalid or not-yet-supported type marker, holds a
/// malformed length, carries invalid UTF-8, or has trailing bytes.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::{decode, PREAMBLE};
///
/// // The binary form of `true` is the preamble followed by the `true` marker.
/// let value = decode(&[PREAMBLE, 0xD2]).unwrap();
/// assert_eq!(value, serde_json::Value::Bool(true));
/// ```
pub fn decode(buffer: &[u8]) -> Result<Value> {
    if !is_binary(buffer) {
        return Err(match buffer.first() {
            Some(&found) => BinaryError::MissingPreamble { found },
            None => BinaryError::UnexpectedEof { needed: 1 },
        });
    }

    // Start reading after the one-byte preamble. The reader keeps absolute
    // offsets (into `buffer`) so error positions account for the preamble.
    let mut reader = Reader {
        buf: buffer,
        pos: 1,
    };
    let value = reader.read_value(0)?;
    let remaining = buffer.len() - reader.pos;
    if remaining != 0 {
        return Err(BinaryError::TrailingBytes { remaining });
    }
    Ok(value)
}

/// A bounds-checked forward cursor over a binary JSON buffer.
///
/// `pos` is an absolute offset into `buf`; the first value begins at `pos == 1`
/// (just past the [`PREAMBLE`](super::PREAMBLE)). Every read advances `pos` only after verifying
/// the bytes are present, so the reader never indexes out of bounds.
struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Reads a single byte, advancing the cursor.
    fn read_u8(&mut self) -> Result<u8> {
        let byte = *self
            .buf
            .get(self.pos)
            .ok_or(BinaryError::UnexpectedEof { needed: 1 })?;
        self.pos += 1;
        Ok(byte)
    }

    /// Reads exactly `N` bytes into a fixed-size array, advancing the cursor.
    fn read_array<const N: usize>(&mut self) -> Result<[u8; N]> {
        let slice = self.read_bytes(N)?;
        let mut array = [0u8; N];
        array.copy_from_slice(slice);
        Ok(array)
    }

    /// Borrows the next `len` bytes, advancing the cursor.
    ///
    /// Returns [`BinaryError::UnexpectedEof`] if fewer than `len` bytes remain.
    /// This only ever slices the existing buffer, so an attacker-controlled
    /// `len` cannot trigger an allocation larger than the buffer.
    fn read_bytes(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .pos
            .checked_add(len)
            .ok_or(BinaryError::InvalidLength {
                detail: "length prefix overflows the address space",
            })?;
        let slice = self
            .buf
            .get(self.pos..end)
            .ok_or(BinaryError::UnexpectedEof {
                needed: end.saturating_sub(self.buf.len()),
            })?;
        self.pos = end;
        Ok(slice)
    }

    fn read_u16_le(&mut self) -> Result<u16> {
        Ok(u16::from_le_bytes(self.read_array()?))
    }

    fn read_u32_le(&mut self) -> Result<u32> {
        Ok(u32::from_le_bytes(self.read_array()?))
    }

    /// Reads a 3-byte little-endian unsigned integer (the `StrR3` offset width).
    fn read_u24_le(&mut self) -> Result<u32> {
        let [b0, b1, b2] = self.read_array::<3>()?;
        Ok(u32::from(b0) | (u32::from(b1) << 8) | (u32::from(b2) << 16))
    }

    fn read_u64_le(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.read_array()?))
    }

    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }

    fn read_i16_le(&mut self) -> Result<i16> {
        Ok(i16::from_le_bytes(self.read_array()?))
    }

    fn read_i32_le(&mut self) -> Result<i32> {
        Ok(i32::from_le_bytes(self.read_array()?))
    }

    fn read_i64_le(&mut self) -> Result<i64> {
        Ok(i64::from_le_bytes(self.read_array()?))
    }

    fn read_f32_le(&mut self) -> Result<f32> {
        Ok(f32::from_le_bytes(self.read_array()?))
    }

    fn read_f64_le(&mut self) -> Result<f64> {
        Ok(f64::from_le_bytes(self.read_array()?))
    }

    /// Reads a UTF-8 string of `len` bytes. `marker_offset` is the offset of the
    /// value's type marker, used for error reporting.
    fn read_string(&mut self, len: usize, marker_offset: usize) -> Result<String> {
        let bytes = self.read_bytes(len)?;
        std::str::from_utf8(bytes)
            .map(str::to_owned)
            .map_err(|_| BinaryError::InvalidUtf8 {
                offset: marker_offset,
            })
    }

    /// Reads one complete value at the current position.
    ///
    /// `depth` is the value's nesting depth (`0` for the top-level value);
    /// container children are read at `depth + 1`. Exceeding [`MAX_DEPTH`]
    /// returns [`BinaryError::DepthLimitExceeded`] rather than risking stack
    /// exhaustion on deeply nested adversarial input.
    fn read_value(&mut self, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(BinaryError::DepthLimitExceeded { limit: MAX_DEPTH });
        }

        // Offset of this value's type marker, captured before consuming it so
        // error positions point at the marker.
        let offset = self.pos;
        let marker = self.read_u8()?;

        match marker {
            NULL => Ok(Value::Null),
            FALSE => Ok(Value::Bool(false)),
            TRUE => Ok(Value::Bool(true)),

            // Literal integer: the value is encoded in the marker itself.
            m if (LITERAL_INT_MIN..LITERAL_INT_MAX).contains(&m) => Ok(int_value(i64::from(m))),

            // Fixed-width numbers (little-endian payloads).
            NUMBER_UINT8 => Ok(int_value(i64::from(self.read_u8()?))),
            NUMBER_INT16 => Ok(int_value(i64::from(self.read_i16_le()?))),
            NUMBER_INT32 => Ok(int_value(i64::from(self.read_i32_le()?))),
            NUMBER_INT64 => Ok(int_value(self.read_i64_le()?)),
            NUMBER_UINT64 => Ok(uint_value(self.read_u64_le()?)),
            NUMBER_DOUBLE => double_value(self.read_f64_le()?),

            // Extended fixed-width numbers (value follows the marker, no length
            // prefix). These are part of the Cosmos extended type system; each
            // has a natural JSON-number projection. `Float16` (0xCF) and the
            // extended `UInt8` (0xD7) have no JSON node type in the service and
            // therefore fall through to the catch-all as InvalidMarker.
            INT8 => Ok(int_value(i64::from(self.read_i8()?))),
            INT16 => Ok(int_value(i64::from(self.read_i16_le()?))),
            INT32 => Ok(int_value(i64::from(self.read_i32_le()?))),
            INT64 => Ok(int_value(self.read_i64_le()?)),
            UINT32 => Ok(int_value(i64::from(self.read_u32_le()?))),
            FLOAT32 => double_value(f64::from(self.read_f32_le()?)),
            FLOAT64 => double_value(self.read_f64_le()?),

            // 1-byte system string: index into the fixed dictionary.
            m if (SYSTEM_STRING_1BYTE_MIN..SYSTEM_STRING_1BYTE_MAX).contains(&m) => {
                let s = system_string_for_marker(m)
                    .ok_or(BinaryError::InvalidMarker { marker: m, offset })?;
                Ok(Value::String(s.to_owned()))
            }

            // Encoded-length string: the length is carried in the marker.
            m if (ENCODED_STRING_LENGTH_MIN..ENCODED_STRING_LENGTH_MAX).contains(&m) => {
                let len = usize::from(m & ENCODED_STRING_LENGTH_MASK);
                Ok(Value::String(self.read_string(len, offset)?))
            }

            // Length-prefixed strings.
            STR_L1 => {
                let len = usize::from(self.read_u8()?);
                Ok(Value::String(self.read_string(len, offset)?))
            }
            STR_L2 => {
                let len = usize::from(self.read_u16_le()?);
                Ok(Value::String(self.read_string(len, offset)?))
            }
            STR_L4 => {
                let len = self.read_u32_le()? as usize;
                Ok(Value::String(self.read_string(len, offset)?))
            }

            // GUID strings: a 16-byte encoded form expanded to the canonical
            // 36-character hex text. The lowercase/uppercase variants differ
            // only in hex case; the double-quoted variant additionally wraps
            // the text in literal quote characters (the original JSON string
            // value included the quotes).
            LOWERCASE_GUID_STRING => Ok(Value::String(self.read_guid_string(false, false)?)),
            UPPERCASE_GUID_STRING => Ok(Value::String(self.read_guid_string(true, false)?)),
            DOUBLE_QUOTED_LOWERCASE_GUID_STRING => {
                Ok(Value::String(self.read_guid_string(false, true)?))
            }

            // Base64 strings: the raw (already base64-decoded) bytes are stored
            // inline; decoding re-encodes them to the JSON string. The width of
            // the group-count prefix (1 vs 2 bytes) and the alphabet (standard
            // vs URL-safe) depend on the marker.
            BASE64_STRING_LENGTH1 => Ok(Value::String(self.read_base64_string(1, false)?)),
            BASE64_STRING_LENGTH2 => Ok(Value::String(self.read_base64_string(2, false)?)),
            BASE64_URL_STRING_LENGTH1 => Ok(Value::String(self.read_base64_string(1, true)?)),
            BASE64_URL_STRING_LENGTH2 => Ok(Value::String(self.read_base64_string(2, true)?)),

            // Compressed strings. The 4-bit table forms map each nibble through
            // a fixed character set; the packed N-bit forms unpack N-bit values
            // (optionally offset by a base character). All decode to ASCII text.
            COMPRESSED_LOWERCASE_HEX_STRING => Ok(Value::String(
                self.read_table_string(compression::LOWERCASE_HEX)?,
            )),
            COMPRESSED_UPPERCASE_HEX_STRING => Ok(Value::String(
                self.read_table_string(compression::UPPERCASE_HEX)?,
            )),
            COMPRESSED_DATE_TIME_STRING => Ok(Value::String(
                self.read_table_string(compression::DATE_TIME)?,
            )),
            PACKED_4BIT_STRING => Ok(Value::String(self.read_packed_string(4, true, 1)?)),
            PACKED_5BIT_STRING => Ok(Value::String(self.read_packed_string(5, true, 1)?)),
            PACKED_6BIT_STRING => Ok(Value::String(self.read_packed_string(6, true, 1)?)),
            PACKED_7BIT_STRING_LENGTH1 => Ok(Value::String(self.read_packed_string(7, false, 1)?)),
            PACKED_7BIT_STRING_LENGTH2 => Ok(Value::String(self.read_packed_string(7, false, 2)?)),

            // User strings reference an external string dictionary that the
            // Cosmos data plane does not supply, so they cannot be resolved to
            // text. We still consume the id bytes (1-byte vs 2-byte form) so the
            // error reflects the correct id, then report it as unsupported.
            m if (USER_STRING_1BYTE_MIN..USER_STRING_1BYTE_MAX).contains(&m) => {
                let id = usize::from(m - USER_STRING_1BYTE_MIN);
                Err(BinaryError::UnsupportedUserString { id })
            }
            m if (USER_STRING_2BYTE_MIN..USER_STRING_2BYTE_MAX).contains(&m) => {
                // Two-byte form: id = one_byte_count + low_byte + (high * 256),
                // where `high` is the marker's offset from USER_STRING_2BYTE_MIN
                // and `low` is the byte that follows. Mirrors .NET
                // TryGetUserStringId.
                let one_byte_count = usize::from(USER_STRING_1BYTE_MAX - USER_STRING_1BYTE_MIN);
                let low = usize::from(self.read_u8()?);
                let high = usize::from(m - USER_STRING_2BYTE_MIN);
                let id = one_byte_count + low + high * 256;
                Err(BinaryError::UnsupportedUserString { id })
            }

            // Reference strings point back to an earlier string's byte offset in
            // the buffer. The offset width grows with the marker (1..4 bytes).
            STR_R1 => {
                let target = usize::from(self.read_u8()?);
                self.resolve_reference(target)
            }
            STR_R2 => {
                let target = usize::from(self.read_u16_le()?);
                self.resolve_reference(target)
            }
            STR_R3 => {
                let target = self.read_u24_le()? as usize;
                self.resolve_reference(target)
            }
            STR_R4 => {
                let target = self.read_u32_le()? as usize;
                self.resolve_reference(target)
            }

            // Arrays.
            ARR0 => Ok(Value::Array(Vec::new())),
            ARR1 => {
                let item = self.read_value(depth + 1)?;
                Ok(Value::Array(vec![item]))
            }
            ARR_L1 => self.read_array_value(1, false, depth),
            ARR_L2 => self.read_array_value(2, false, depth),
            ARR_L4 => self.read_array_value(4, false, depth),
            ARR_LC1 => self.read_array_value(1, true, depth),
            ARR_LC2 => self.read_array_value(2, true, depth),
            ARR_LC4 => self.read_array_value(4, true, depth),

            // Objects.
            OBJ0 => Ok(Value::Object(Map::new())),
            OBJ1 => {
                let (name, value) = self.read_member(depth + 1)?;
                let mut map = Map::new();
                map.insert(name, value);
                Ok(Value::Object(map))
            }
            OBJ_L1 => self.read_object_value(1, false, depth),
            OBJ_L2 => self.read_object_value(2, false, depth),
            OBJ_L4 => self.read_object_value(4, false, depth),
            OBJ_LC1 => self.read_object_value(1, true, depth),
            OBJ_LC2 => self.read_object_value(2, true, depth),
            OBJ_LC4 => self.read_object_value(4, true, depth),

            // Every other (valid-but-not-yet-implemented or genuinely invalid)
            // marker is reported as invalid. User/reference strings and the
            // exotic string/number forms are filled in by later P1 sub-phases
            // (see the module-level docs).
            other => Err(BinaryError::InvalidMarker {
                marker: other,
                offset,
            }),
        }
    }

    /// Reads a 1-, 2-, or 4-byte little-endian length or count field.
    fn read_len(&mut self, width: usize) -> Result<usize> {
        match width {
            1 => Ok(usize::from(self.read_u8()?)),
            2 => Ok(usize::from(self.read_u16_le()?)),
            // The only other width the callers pass is 4.
            _ => Ok(self.read_u32_le()? as usize),
        }
    }

    /// Computes the absolute end offset of a `payload_len`-byte payload starting
    /// at the current position, verifying it fits within the buffer.
    fn bounded_end(&self, payload_len: usize) -> Result<usize> {
        let end = self
            .pos
            .checked_add(payload_len)
            .ok_or(BinaryError::InvalidLength {
                detail: "container length overflows the address space",
            })?;
        if end > self.buf.len() {
            return Err(BinaryError::UnexpectedEof {
                needed: end - self.buf.len(),
            });
        }
        Ok(end)
    }

    /// Reads a length-prefixed array body. `width` is the length/count prefix
    /// width in bytes (1, 2, or 4); when `has_count` is set, a count field of
    /// the same width follows the length and is validated against the number of
    /// items actually decoded.
    fn read_array_value(&mut self, width: usize, has_count: bool, depth: usize) -> Result<Value> {
        let payload_len = self.read_len(width)?;
        let count = if has_count {
            Some(self.read_len(width)?)
        } else {
            None
        };
        let end = self.bounded_end(payload_len)?;

        let mut items = Vec::new();
        while self.pos < end {
            let item = self.read_value(depth + 1)?;
            if self.pos > end {
                return Err(BinaryError::InvalidLength {
                    detail: "array element extends past the array's declared length",
                });
            }
            items.push(item);
        }

        if let Some(expected) = count {
            if items.len() != expected {
                return Err(BinaryError::InvalidLength {
                    detail: "array item count does not match its declared count",
                });
            }
        }
        Ok(Value::Array(items))
    }

    /// Reads a length-prefixed object body, mirroring [`read_array_value`] but
    /// decoding name/value member pairs. The declared count (when present) is
    /// the number of members, validated against the number actually decoded.
    ///
    /// [`read_array_value`]: Reader::read_array_value
    fn read_object_value(&mut self, width: usize, has_count: bool, depth: usize) -> Result<Value> {
        let payload_len = self.read_len(width)?;
        let count = if has_count {
            Some(self.read_len(width)?)
        } else {
            None
        };
        let end = self.bounded_end(payload_len)?;

        let mut map = Map::new();
        let mut members = 0usize;
        while self.pos < end {
            let (name, value) = self.read_member(depth + 1)?;
            if self.pos > end {
                return Err(BinaryError::InvalidLength {
                    detail: "object member extends past the object's declared length",
                });
            }
            map.insert(name, value);
            members += 1;
        }

        if let Some(expected) = count {
            if members != expected {
                return Err(BinaryError::InvalidLength {
                    detail: "object member count does not match its declared count",
                });
            }
        }
        Ok(Value::Object(map))
    }

    /// Reads one object member: a string name followed by its value. The name
    /// must decode to a string; any other form is reported as an
    /// [`BinaryError::InvalidMarker`] at the name's marker offset, since a
    /// non-string is not valid in a property-name position.
    fn read_member(&mut self, depth: usize) -> Result<(String, Value)> {
        let name_offset = self.pos;
        let name = self.read_value(depth)?;
        let name = match name {
            Value::String(s) => s,
            _ => {
                // The byte at `name_offset` was necessarily present (the
                // `read_value` above consumed it), so this index is in bounds.
                let marker = self.buf[name_offset];
                return Err(BinaryError::InvalidMarker {
                    marker,
                    offset: name_offset,
                });
            }
        };
        let value = self.read_value(depth)?;
        Ok((name, value))
    }

    /// Reads a GUID string: the 16-byte encoded form (following the marker)
    /// expanded to the canonical `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` hex text.
    ///
    /// This is a straight sequential hex dump of the 16 bytes (not the .NET
    /// `Guid` mixed-endian layout), mirroring .NET `DecodeGuidStringValue`.
    /// `uppercase` selects the hex case. When `quoted`, the original JSON string
    /// included literal quote characters, so they are re-added around the text.
    fn read_guid_string(&mut self, uppercase: bool, quoted: bool) -> Result<String> {
        const DASH_POSITIONS: [usize; 4] = [4, 6, 8, 10];
        let bytes = self.read_array::<16>()?;
        let digits = if uppercase {
            b"0123456789ABCDEF"
        } else {
            b"0123456789abcdef"
        };

        // 36 hex/dash chars, plus the two optional surrounding quotes.
        let mut out = String::with_capacity(if quoted { 38 } else { 36 });
        if quoted {
            out.push('"');
        }
        for (index, byte) in bytes.iter().enumerate() {
            // A dash precedes the byte at each group boundary (after bytes 4, 6,
            // 8, and 10), producing the 8-4-4-4-12 grouping.
            if DASH_POSITIONS.contains(&index) {
                out.push('-');
            }
            out.push(char::from(digits[usize::from(byte >> 4)]));
            out.push(char::from(digits[usize::from(byte & 0x0F)]));
        }
        if quoted {
            out.push('"');
        }
        Ok(out)
    }

    /// Reads a base64 string. The inline payload is the **raw** (already
    /// base64-decoded) bytes; this re-encodes them to the original base64 text.
    ///
    /// `length_width` is the width (1 or 2 bytes, little-endian) of the
    /// group-count prefix that precedes a 1-byte padding field; `url_safe`
    /// selects the URL-safe alphabet. The group count times four is the padded
    /// base64 length, and the padding byte records how many `=` characters the
    /// original text carried (or, when greater than 2, that padding was omitted,
    /// in which case the encoded length shrinks accordingly). Mirrors .NET
    /// `ConvertBytesToBase64String`.
    fn read_base64_string(&mut self, length_width: usize, url_safe: bool) -> Result<String> {
        let groups = self.read_len(length_width)?;
        let padding = self.read_u8()?;

        // Padded length is always a multiple of four; `effective_padding` is the
        // literal `=` count (0..=2), or `!padding` when padding was omitted.
        let padded_len = groups.checked_mul(4).ok_or(BinaryError::InvalidLength {
            detail: "base64 length overflows the address space",
        })?;
        let omitted = padding > 2;
        let effective_padding = usize::from(if omitted { !padding } else { padding });
        let final_len = padded_len
            .checked_sub(if omitted { effective_padding } else { 0 })
            .ok_or(BinaryError::InvalidLength {
                detail: "base64 padding exceeds the encoded length",
            })?;
        let raw_len = padded_len
            .checked_sub(effective_padding)
            .ok_or(BinaryError::InvalidLength {
                detail: "base64 padding exceeds the encoded length",
            })?
            .checked_mul(3)
            .ok_or(BinaryError::InvalidLength {
                detail: "base64 length overflows the address space",
            })?
            / 4;

        let raw = self.read_bytes(raw_len)?;
        let engine = if url_safe {
            &base64::engine::general_purpose::URL_SAFE
        } else {
            &base64::engine::general_purpose::STANDARD
        };
        let mut encoded = engine.encode(raw);

        // The padded encoding may carry trailing `=`; keep only the original
        // text length (this drops padding the service chose to omit).
        if final_len > encoded.len() {
            return Err(BinaryError::InvalidLength {
                detail: "base64 encoded length is shorter than the declared length",
            });
        }
        encoded.truncate(final_len);
        Ok(encoded)
    }

    /// Reads a 4-bit table-compressed string (lowercase hex, uppercase hex, or
    /// date-time). A 1-byte prefix gives the decoded character count `len`; the
    /// payload is `ceil(len / 2)` bytes, each holding two 4-bit indices into
    /// `table` (low nibble first, then high nibble), mirroring .NET
    /// `Decode4BitCharacterStringValue`.
    fn read_table_string(&mut self, table: &[u8; 16]) -> Result<String> {
        let len = usize::from(self.read_u8()?);
        let byte_count = len.div_ceil(2);
        let bytes = self.read_bytes(byte_count)?;

        let mut out = String::with_capacity(len);
        for (index, &byte) in bytes.iter().enumerate() {
            // Low nibble is the first character of the pair.
            out.push(char::from(table[usize::from(byte & 0x0F)]));
            // The final byte of an odd-length string contributes only its low
            // nibble; its high nibble is padding and must be zero.
            let produced_low_only = index == byte_count - 1 && len % 2 == 1;
            if produced_low_only {
                if byte >> 4 != 0 {
                    return Err(BinaryError::InvalidLength {
                        detail: "compressed string has non-zero padding nibble",
                    });
                }
            } else {
                out.push(char::from(table[usize::from(byte >> 4)]));
            }
        }
        Ok(out)
    }

    /// Reads a packed N-bit compressed string. A length prefix (`length_width`
    /// bytes, little-endian) gives the decoded character count `len`; the
    /// payload is `ceil(len * bits / 8)` bytes holding `len` little-endian
    /// `bits`-wide values. When `has_base`, a 1-byte base character precedes the
    /// payload and is added to every unpacked value. Mirrors .NET
    /// `DecodeCompressedStringValue`.
    fn read_packed_string(
        &mut self,
        bits: u32,
        has_base: bool,
        length_width: usize,
    ) -> Result<String> {
        let len = self.read_len(length_width)?;
        let base = if has_base { self.read_u8()? } else { 0 };
        let byte_count = (len * bits as usize).div_ceil(8);
        let bytes = self.read_bytes(byte_count)?;

        // Unpack `len` values of `bits` bits each, least-significant bit first,
        // from a contiguous little-endian bit stream.
        let mask = (1u32 << bits) - 1;
        let mut out = String::with_capacity(len);
        let mut bit_pos = 0usize;
        for _ in 0..len {
            let byte_index = bit_pos / 8;
            let bit_offset = bit_pos % 8;
            // A value spans at most two bytes for bits <= 8; read a little-endian
            // 16-bit window so the value is always fully covered.
            let lo = u32::from(bytes[byte_index]);
            let hi = bytes.get(byte_index + 1).map_or(0, |&b| u32::from(b));
            let window = lo | (hi << 8);
            let value = (window >> bit_offset) & mask;
            // Each unpacked value is a byte; `+ base` yields the ASCII char.
            let ch = (value as u8).wrapping_add(base);
            out.push(char::from(ch));
            bit_pos += bits as usize;
        }
        Ok(out)
    }

    /// Resolves a reference string ([`STR_R1`]–[`STR_R4`]) whose `target` is an
    /// absolute byte offset into the buffer (the same frame as [`Reader::pos`],
    /// where the [`PREAMBLE`](super::PREAMBLE) is offset `0`).
    ///
    /// The target must lie within the buffer and hold a string that is **not**
    /// itself a reference string; this mirrors .NET's
    /// `IsValidReferenceStringTarget` and makes reference chains (and therefore
    /// cycles) impossible, so the lookup terminates without recursion guards.
    /// The referenced string is decoded from a fresh cursor positioned at
    /// `target`, leaving `self` untouched.
    ///
    /// [`STR_R1`]: super::markers::STR_R1
    /// [`STR_R4`]: super::markers::STR_R4
    fn resolve_reference(&self, target: usize) -> Result<Value> {
        let marker = *self
            .buf
            .get(target)
            .ok_or(BinaryError::UnresolvedReference { target })?;

        // The target must be a string, and must not itself be a reference
        // string (no chains/cycles).
        let is_string = (SYSTEM_STRING_1BYTE_MIN..NUMBER_UINT64).contains(&marker);
        let is_reference = (STR_R1..=STR_R4).contains(&marker);
        if !is_string || is_reference {
            return Err(BinaryError::UnresolvedReference { target });
        }

        // Decode the referenced string from its own cursor. It is a single
        // string value, so depth does not grow and a bare reader suffices.
        let mut sub = Reader {
            buf: self.buf,
            pos: target,
        };
        sub.read_value(0)
    }
}

/// Wraps a signed integer that fits in `i64` as a JSON number.
fn int_value(n: i64) -> Value {
    Value::Number(n.into())
}

/// Wraps an unsigned 64-bit integer as a JSON number (used for `UInt64` values
/// that may exceed `i64::MAX`).
fn uint_value(n: u64) -> Value {
    Value::Number(n.into())
}

/// Wraps a `double` as a JSON number, rejecting non-finite values that JSON
/// cannot represent.
fn double_value(n: f64) -> Result<Value> {
    serde_json::Number::from_f64(n)
        .map(Value::Number)
        .ok_or(BinaryError::InvalidNumber {
            detail: "non-finite double (NaN or infinity)",
        })
}

/// Character lookup tables for the 4-bit table-compressed string forms.
///
/// Each table maps a 4-bit nibble (`0x0`–`0xF`) to one ASCII byte, transcribed
/// verbatim from the .NET `StringCompressionLookupTables` `list` arrays
/// (`JsonBinaryEncoding.Chars.cs`).
mod compression {
    /// Lowercase hexadecimal digits (`CompressedLowercaseHexString`).
    pub(super) const LOWERCASE_HEX: &[u8; 16] = b"0123456789abcdef";

    /// Uppercase hexadecimal digits (`CompressedUppercaseHexString`).
    pub(super) const UPPERCASE_HEX: &[u8; 16] = b"0123456789ABCDEF";

    /// Date-time character set (`CompressedDateTimeString`): space, digits, and
    /// the `:`, `-`, `.`, `T`, `Z` separators used in ISO-8601 timestamps.
    pub(super) const DATE_TIME: &[u8; 16] = b" 0123456789:-.TZ";
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::markers;
    use crate::binary_json::vectors::SCALAR_VECTORS;
    use crate::binary_json::PREAMBLE;

    /// Helper: prepend the preamble to a value's marker+payload bytes.
    fn buf(value_bytes: &[u8]) -> Vec<u8> {
        let mut v = vec![PREAMBLE];
        v.extend_from_slice(value_bytes);
        v
    }

    /// The real decoder reproduces every golden scalar vector's JSON.
    #[test]
    fn decodes_golden_scalar_corpus() {
        for vector in SCALAR_VECTORS {
            let decoded = decode(vector.binary).unwrap_or_else(|e| {
                panic!("case {}: decode failed: {e}", vector.name);
            });
            let expected: Value = serde_json::from_str(vector.json).unwrap();
            assert_eq!(decoded, expected, "case {}", vector.name);
        }
    }

    #[test]
    fn decodes_null_and_booleans() {
        assert_eq!(decode(&buf(&[markers::NULL])).unwrap(), Value::Null);
        assert_eq!(decode(&buf(&[markers::FALSE])).unwrap(), Value::Bool(false));
        assert_eq!(decode(&buf(&[markers::TRUE])).unwrap(), Value::Bool(true));
    }

    #[test]
    fn decodes_literal_integers() {
        for n in 0u8..32 {
            let value = decode(&buf(&[n])).unwrap();
            assert_eq!(value, serde_json::json!(n), "literal int {n}");
        }
    }

    #[test]
    fn decodes_fixed_width_numbers() {
        // UInt8.
        assert_eq!(
            decode(&buf(&[markers::NUMBER_UINT8, 200])).unwrap(),
            serde_json::json!(200),
        );
        // Int16 = -1000 (little-endian).
        let mut int16 = vec![markers::NUMBER_INT16];
        int16.extend_from_slice(&(-1000i16).to_le_bytes());
        assert_eq!(decode(&buf(&int16)).unwrap(), serde_json::json!(-1000));
        // Int32 = 70000.
        let mut int32 = vec![markers::NUMBER_INT32];
        int32.extend_from_slice(&70_000i32.to_le_bytes());
        assert_eq!(decode(&buf(&int32)).unwrap(), serde_json::json!(70_000));
        // Int64 = a large negative value.
        let mut int64 = vec![markers::NUMBER_INT64];
        int64.extend_from_slice(&(-5_000_000_000i64).to_le_bytes());
        assert_eq!(
            decode(&buf(&int64)).unwrap(),
            serde_json::json!(-5_000_000_000i64),
        );
        // UInt64 beyond i64::MAX must round-trip as an unsigned number.
        let big = u64::MAX - 1;
        let mut uint64 = vec![markers::NUMBER_UINT64];
        uint64.extend_from_slice(&big.to_le_bytes());
        assert_eq!(decode(&buf(&uint64)).unwrap(), serde_json::json!(big));
        // Double.
        let mut dbl = vec![markers::NUMBER_DOUBLE];
        dbl.extend_from_slice(&3.5f64.to_le_bytes());
        assert_eq!(decode(&buf(&dbl)).unwrap(), serde_json::json!(3.5));
    }

    #[test]
    fn rejects_non_finite_double() {
        let mut nan = vec![markers::NUMBER_DOUBLE];
        nan.extend_from_slice(&f64::NAN.to_le_bytes());
        assert_eq!(
            decode(&buf(&nan)),
            Err(BinaryError::InvalidNumber {
                detail: "non-finite double (NaN or infinity)",
            }),
        );
    }

    #[test]
    fn decodes_string_forms() {
        // System string "id" (index 12).
        assert_eq!(
            decode(&buf(&[markers::SYSTEM_STRING_1BYTE_MIN + 12])).unwrap(),
            serde_json::json!("id"),
        );
        // Encoded-length string "hi" (length 2 baked into the marker).
        assert_eq!(
            decode(&buf(&[markers::ENCODED_STRING_LENGTH_MIN | 2, b'h', b'i'])).unwrap(),
            serde_json::json!("hi"),
        );
        // Empty encoded-length string (marker == ENCODED_STRING_LENGTH_MIN).
        assert_eq!(
            decode(&buf(&[markers::ENCODED_STRING_LENGTH_MIN])).unwrap(),
            serde_json::json!(""),
        );
        // StrL1 string "hello".
        let mut str_l1 = vec![markers::STR_L1, 5];
        str_l1.extend_from_slice(b"hello");
        assert_eq!(decode(&buf(&str_l1)).unwrap(), serde_json::json!("hello"));
        // StrL2 string of length 300.
        let long: String = "a".repeat(300);
        let mut str_l2 = vec![markers::STR_L2];
        str_l2.extend_from_slice(&300u16.to_le_bytes());
        str_l2.extend_from_slice(long.as_bytes());
        assert_eq!(decode(&buf(&str_l2)).unwrap(), serde_json::json!(long));
    }

    #[test]
    fn rejects_missing_preamble() {
        assert_eq!(
            decode(b"{}"),
            Err(BinaryError::MissingPreamble { found: b'{' }),
        );
    }

    #[test]
    fn rejects_empty_buffer() {
        assert_eq!(decode(&[]), Err(BinaryError::UnexpectedEof { needed: 1 }));
    }

    #[test]
    fn rejects_trailing_bytes() {
        // Preamble + `null` marker + one extra byte.
        assert_eq!(
            decode(&[PREAMBLE, markers::NULL, 0x00]),
            Err(BinaryError::TrailingBytes { remaining: 1 }),
        );
    }

    #[test]
    fn rejects_truncated_number() {
        // Int32 marker but only two payload bytes present.
        assert_eq!(
            decode(&[PREAMBLE, markers::NUMBER_INT32, 0x01, 0x02]),
            Err(BinaryError::UnexpectedEof { needed: 2 }),
        );
    }

    #[test]
    fn rejects_truncated_string() {
        // StrL1 claims 5 bytes but only 2 follow.
        assert_eq!(
            decode(&[PREAMBLE, markers::STR_L1, 5, b'h', b'i']),
            Err(BinaryError::UnexpectedEof { needed: 3 }),
        );
    }

    #[test]
    fn rejects_invalid_utf8() {
        // StrL1 of length 1 carrying a lone continuation byte (0xFF).
        assert!(matches!(
            decode(&[PREAMBLE, markers::STR_L1, 1, 0xFF]),
            Err(BinaryError::InvalidUtf8 { .. }),
        ));
    }

    #[test]
    fn deferred_markers_report_invalid_for_now() {
        // The GUID value marker (0xD3) is valid in the format but implemented
        // in a later sub-phase (P1d-5); until then it surfaces as InvalidMarker.
        // The offset points at the marker (index 1, just past the preamble).
        assert_eq!(
            decode(&[PREAMBLE, markers::GUID]),
            Err(BinaryError::InvalidMarker {
                marker: markers::GUID,
                offset: 1,
            }),
        );
    }

    #[test]
    fn decodes_extended_integers() {
        // Int8 = -5.
        let mut int8 = vec![markers::INT8];
        int8.extend_from_slice(&(-5i8).to_le_bytes());
        assert_eq!(decode(&buf(&int8)).unwrap(), serde_json::json!(-5));
        // Int16 = -1000.
        let mut int16 = vec![markers::INT16];
        int16.extend_from_slice(&(-1000i16).to_le_bytes());
        assert_eq!(decode(&buf(&int16)).unwrap(), serde_json::json!(-1000));
        // Int32 = -70000.
        let mut int32 = vec![markers::INT32];
        int32.extend_from_slice(&(-70_000i32).to_le_bytes());
        assert_eq!(decode(&buf(&int32)).unwrap(), serde_json::json!(-70_000));
        // Int64 = a large negative value.
        let mut int64 = vec![markers::INT64];
        int64.extend_from_slice(&(-5_000_000_000i64).to_le_bytes());
        assert_eq!(
            decode(&buf(&int64)).unwrap(),
            serde_json::json!(-5_000_000_000i64),
        );
        // UInt32 near u32::MAX must round-trip as a positive number.
        let big = u32::MAX - 1;
        let mut uint32 = vec![markers::UINT32];
        uint32.extend_from_slice(&big.to_le_bytes());
        assert_eq!(decode(&buf(&uint32)).unwrap(), serde_json::json!(big));
    }

    #[test]
    fn decodes_extended_floats() {
        // Float32 = 1.5 (exactly representable).
        let mut f32v = vec![markers::FLOAT32];
        f32v.extend_from_slice(&1.5f32.to_le_bytes());
        assert_eq!(decode(&buf(&f32v)).unwrap(), serde_json::json!(1.5));
        // Float64 = -2.25.
        let mut f64v = vec![markers::FLOAT64];
        f64v.extend_from_slice(&(-2.25f64).to_le_bytes());
        assert_eq!(decode(&buf(&f64v)).unwrap(), serde_json::json!(-2.25));
    }

    #[test]
    fn rejects_non_finite_extended_float() {
        // Float32 carrying infinity has no JSON representation.
        let mut inf = vec![markers::FLOAT32];
        inf.extend_from_slice(&f32::INFINITY.to_le_bytes());
        assert_eq!(
            decode(&buf(&inf)),
            Err(BinaryError::InvalidNumber {
                detail: "non-finite double (NaN or infinity)",
            }),
        );
    }

    #[test]
    fn float16_and_extended_uint8_have_no_json_node() {
        // Float16 (0xCF) and the extended UInt8 (0xD7) map to no JSON node type
        // in the service, so the decoder rejects them as invalid markers.
        assert_eq!(
            decode(&[PREAMBLE, markers::FLOAT16, 0x00, 0x00]),
            Err(BinaryError::InvalidMarker {
                marker: markers::FLOAT16,
                offset: 1,
            }),
        );
        assert_eq!(
            decode(&[PREAMBLE, markers::UINT8, 0x00]),
            Err(BinaryError::InvalidMarker {
                marker: markers::UINT8,
                offset: 1,
            }),
        );
    }

    #[test]
    fn rejects_truncated_extended_number() {
        // Int32 marker with only two payload bytes present.
        assert_eq!(
            decode(&[PREAMBLE, markers::INT32, 0x01, 0x02]),
            Err(BinaryError::UnexpectedEof { needed: 2 }),
        );
    }

    #[test]
    fn decodes_guid_strings() {
        // 16 encoded bytes 0x00..0x0F expand to the canonical hex text.
        let encoded: [u8; 16] = std::array::from_fn(|i| i as u8);
        let mut lower = vec![markers::LOWERCASE_GUID_STRING];
        lower.extend_from_slice(&encoded);
        assert_eq!(
            decode(&buf(&lower)).unwrap(),
            serde_json::json!("00010203-0405-0607-0809-0a0b0c0d0e0f"),
        );
        // Uppercase variant differs only in hex case.
        let mut upper = vec![markers::UPPERCASE_GUID_STRING];
        upper.extend_from_slice(&encoded);
        assert_eq!(
            decode(&buf(&upper)).unwrap(),
            serde_json::json!("00010203-0405-0607-0809-0A0B0C0D0E0F"),
        );
        // Double-quoted variant wraps the text in literal quotes.
        let mut quoted = vec![markers::DOUBLE_QUOTED_LOWERCASE_GUID_STRING];
        quoted.extend_from_slice(&encoded);
        assert_eq!(
            decode(&buf(&quoted)).unwrap(),
            serde_json::json!("\"00010203-0405-0607-0809-0a0b0c0d0e0f\""),
        );
    }

    #[test]
    fn rejects_truncated_guid_string() {
        // GUID string marker claims 16 encoded bytes but only 4 follow.
        let mut bytes = vec![markers::LOWERCASE_GUID_STRING];
        bytes.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::UnexpectedEof { needed: 12 }),
        );
    }

    /// Builds a Base64StringLength1 token: marker, 1-byte group count, 1-byte
    /// padding, then the raw bytes.
    fn base64_len1(groups: u8, padding: u8, raw: &[u8]) -> Vec<u8> {
        let mut v = vec![markers::BASE64_STRING_LENGTH1, groups, padding];
        v.extend_from_slice(raw);
        v
    }

    #[test]
    fn decodes_base64_strings_with_literal_padding() {
        // "foo" -> "Zm9v" (3 raw bytes, one group, no padding).
        assert_eq!(
            decode(&buf(&base64_len1(1, 0, b"foo"))).unwrap(),
            serde_json::json!("Zm9v"),
        );
        // "fo" -> "Zm8=" (2 raw bytes, one group, one '=').
        assert_eq!(
            decode(&buf(&base64_len1(1, 1, b"fo"))).unwrap(),
            serde_json::json!("Zm8="),
        );
        // "A" -> "QQ==" (1 raw byte, one group, two '=').
        assert_eq!(
            decode(&buf(&base64_len1(1, 2, b"A"))).unwrap(),
            serde_json::json!("QQ=="),
        );
    }

    #[test]
    fn decodes_base64_string_with_omitted_padding() {
        // "A" encoded without the trailing "==": padding byte is !2 (253),
        // shrinking the final length from 4 to 2, so the text is "QQ".
        assert_eq!(
            decode(&buf(&base64_len1(1, !2u8, b"A"))).unwrap(),
            serde_json::json!("QQ"),
        );
    }

    #[test]
    fn decodes_base64_string_length2() {
        // "foobar" -> "Zm9vYmFy" (6 raw bytes, two groups, no padding); the
        // group count is a 2-byte little-endian field.
        let mut bytes = vec![markers::BASE64_STRING_LENGTH2];
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.push(0); // padding
        bytes.extend_from_slice(b"foobar");
        assert_eq!(decode(&buf(&bytes)).unwrap(), serde_json::json!("Zm9vYmFy"),);
    }

    #[test]
    fn decodes_base64_url_string() {
        // Bytes [0xFB, 0xFF, 0xFE] encode to "+//+" in standard base64 and
        // "-__-" in the URL-safe alphabet.
        let mut std_bytes = vec![markers::BASE64_STRING_LENGTH1, 1, 0];
        std_bytes.extend_from_slice(&[0xFB, 0xFF, 0xFE]);
        assert_eq!(decode(&buf(&std_bytes)).unwrap(), serde_json::json!("+//+"),);
        let mut url_bytes = vec![markers::BASE64_URL_STRING_LENGTH1, 1, 0];
        url_bytes.extend_from_slice(&[0xFB, 0xFF, 0xFE]);
        assert_eq!(decode(&buf(&url_bytes)).unwrap(), serde_json::json!("-__-"),);
    }

    #[test]
    fn rejects_truncated_base64_string() {
        // Declares one group (3 raw bytes) but only one byte follows.
        assert_eq!(
            decode(&buf(&base64_len1(1, 0, b"f"))),
            Err(BinaryError::UnexpectedEof { needed: 2 }),
        );
    }

    #[test]
    fn decodes_table_compressed_strings() {
        // Lowercase hex "1a2b": chars [1, a, 2, b] pack low-nibble-first into
        // bytes [0xA1, 0xB2] (len 4).
        let lower = [markers::COMPRESSED_LOWERCASE_HEX_STRING, 4, 0xA1, 0xB2];
        assert_eq!(decode(&buf(&lower)).unwrap(), serde_json::json!("1a2b"));
        // Uppercase hex with the same bytes yields uppercase digits.
        let upper = [markers::COMPRESSED_UPPERCASE_HEX_STRING, 4, 0xA1, 0xB2];
        assert_eq!(decode(&buf(&upper)).unwrap(), serde_json::json!("1A2B"));
        // Odd length: a single 'f' (index 15) occupies one byte's low nibble;
        // the high nibble must be zero.
        let odd = [markers::COMPRESSED_LOWERCASE_HEX_STRING, 1, 0x0F];
        assert_eq!(decode(&buf(&odd)).unwrap(), serde_json::json!("f"));
        // Date-time set: "2024-01" -> chars 2,0,2,4,-,0,1 (7 chars).
        // Indices: '2'=3,'0'=1,'2'=3,'4'=5,'-'=12,'0'=1,'1'=2.
        // Packed pairs (low,high): (3,1)->0x13, (3,5)->0x53, (12,1)->0x1C,
        // then trailing '1'=2 alone -> 0x02.
        let dt = [
            markers::COMPRESSED_DATE_TIME_STRING,
            7,
            0x13,
            0x53,
            0x1C,
            0x02,
        ];
        assert_eq!(decode(&buf(&dt)).unwrap(), serde_json::json!("2024-01"));
    }

    #[test]
    fn rejects_table_compressed_string_with_padding_nibble() {
        // Odd length 1 but the (only) byte's high nibble is non-zero padding.
        let bad = [markers::COMPRESSED_LOWERCASE_HEX_STRING, 1, 0x1F];
        assert!(matches!(
            decode(&buf(&bad)),
            Err(BinaryError::InvalidLength { .. }),
        ));
    }

    #[test]
    fn decodes_packed_compressed_strings() {
        // 7-bit "Hi" (values 0x48, 0x69) packs to [0xC8, 0x34], no base char.
        let p7 = [markers::PACKED_7BIT_STRING_LENGTH1, 2, 0xC8, 0x34];
        assert_eq!(decode(&buf(&p7)).unwrap(), serde_json::json!("Hi"));
        // 4-bit packed "0123" relative to base '0': values 0..3 pack to
        // [0x10, 0x32].
        let p4 = [markers::PACKED_4BIT_STRING, 4, b'0', 0x10, 0x32];
        assert_eq!(decode(&buf(&p4)).unwrap(), serde_json::json!("0123"));
        // 5-bit packed "abc" relative to base 'a': values 0..2 pack to
        // [0x20, 0x08].
        let p5 = [markers::PACKED_5BIT_STRING, 3, b'a', 0x20, 0x08];
        assert_eq!(decode(&buf(&p5)).unwrap(), serde_json::json!("abc"));
        // 6-bit packed "abcd" relative to base 'a': values 0..3 pack to
        // [0x40, 0x20, 0x0C].
        let p6 = [markers::PACKED_6BIT_STRING, 4, b'a', 0x40, 0x20, 0x0C];
        assert_eq!(decode(&buf(&p6)).unwrap(), serde_json::json!("abcd"));
    }

    #[test]
    fn decodes_packed_7bit_length2() {
        // Packed7BitStringLength2 uses a 2-byte little-endian length prefix.
        let mut bytes = vec![markers::PACKED_7BIT_STRING_LENGTH2];
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&[0xC8, 0x34]); // "Hi"
        assert_eq!(decode(&buf(&bytes)).unwrap(), serde_json::json!("Hi"));
    }

    #[test]
    fn rejects_truncated_compressed_string() {
        // 7-bit length 4 needs ceil(4*7/8) = 4 payload bytes; only one follows.
        assert_eq!(
            decode(&[PREAMBLE, markers::PACKED_7BIT_STRING_LENGTH1, 4, 0x00]),
            Err(BinaryError::UnexpectedEof { needed: 3 }),
        );
    }

    #[test]
    fn user_strings_report_unsupported() {
        // 1-byte user string: id == marker - USER_STRING_1BYTE_MIN.
        assert_eq!(
            decode(&buf(&[markers::USER_STRING_1BYTE_MIN + 3])),
            Err(BinaryError::UnsupportedUserString { id: 3 }),
        );
        // The very first 1-byte user string id is 0.
        assert_eq!(
            decode(&buf(&[markers::USER_STRING_1BYTE_MIN])),
            Err(BinaryError::UnsupportedUserString { id: 0 }),
        );
        // 2-byte user string: id == one_byte_count + low + high * 256, where
        // one_byte_count = USER_STRING_1BYTE_MAX - USER_STRING_1BYTE_MIN (32),
        // high = marker - USER_STRING_2BYTE_MIN, low = following byte.
        let one_byte_count =
            usize::from(markers::USER_STRING_1BYTE_MAX - markers::USER_STRING_1BYTE_MIN);
        assert_eq!(
            decode(&buf(&[markers::USER_STRING_2BYTE_MIN, 5])),
            Err(BinaryError::UnsupportedUserString {
                id: one_byte_count + 5,
            }),
        );
        assert_eq!(
            decode(&buf(&[markers::USER_STRING_2BYTE_MIN + 1, 5])),
            Err(BinaryError::UnsupportedUserString {
                id: one_byte_count + 5 + 256,
            }),
        );
    }

    #[test]
    fn decodes_reference_string() {
        // Buffer: preamble, then a StrL1 "hello" at offset 1, then a StrR1 that
        // points back to offset 1. The top-level value is the array [<the
        // string>, <the reference>] so both resolve to "hello".
        //
        // Layout (absolute offsets):
        //   0: PREAMBLE
        //   1: ARR_L1
        //   2: payload length (7)
        //   3: STR_L1, 4: len 5, 5..10: "hello"   (string token at offset 3)
        //  10: STR_R1, 11: target offset 3
        let mut payload = vec![markers::STR_L1, 5];
        payload.extend_from_slice(b"hello");
        payload.push(markers::STR_R1);
        payload.push(3); // absolute offset of the StrL1 token
        let mut bytes = vec![markers::ARR_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!(["hello", "hello"]),
        );
    }

    #[test]
    fn reference_string_to_system_string_resolves() {
        // StrR1 may target a system string. Place the 1-byte system string for
        // "id" (idx 12) at offset 1, then reference it.
        let id_name = markers::SYSTEM_STRING_1BYTE_MIN + 12;
        let mut payload = vec![id_name];
        payload.push(markers::STR_R1);
        payload.push(3); // offset of `id_name` within the full buffer
        let mut bytes = vec![markers::ARR_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!(["id", "id"]),
        );
    }

    #[test]
    fn rejects_out_of_range_reference() {
        // StrR1 target points past the end of the buffer.
        assert_eq!(
            decode(&[PREAMBLE, markers::STR_R1, 200]),
            Err(BinaryError::UnresolvedReference { target: 200 }),
        );
    }

    #[test]
    fn rejects_reference_to_non_string() {
        // StrR1 target (offset 4) lands on a literal-int marker, not a string.
        //   0: PREAMBLE
        //   1: ARR_L1, 2: len 4
        //   3: literal int 0  <- NOT a string
        //   4: STR_R1, 5: target 3
        let payload = [0x00u8, markers::STR_R1, 3];
        let mut bytes = vec![markers::ARR_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::UnresolvedReference { target: 3 }),
        );
    }

    #[test]
    fn rejects_reference_to_reference() {
        // A StrR1 that targets another StrR1 is rejected (no chains/cycles).
        //   0: PREAMBLE
        //   1: ARR_L1, 2: len 4
        //   3: STR_R1, 4: target 3 (self-reference)
        //   5: STR_R1, 6: target 3
        let payload = [markers::STR_R1, 3, markers::STR_R1, 3];
        let mut bytes = vec![markers::ARR_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::UnresolvedReference { target: 3 }),
        );
    }

    #[test]
    fn decodes_empty_containers() {
        assert_eq!(
            decode(&buf(&[markers::ARR0])).unwrap(),
            serde_json::json!([])
        );
        assert_eq!(
            decode(&buf(&[markers::OBJ0])).unwrap(),
            serde_json::json!({}),
        );
    }

    #[test]
    fn decodes_single_item_containers() {
        // [true]
        assert_eq!(
            decode(&buf(&[markers::ARR1, markers::TRUE])).unwrap(),
            serde_json::json!([true]),
        );
        // {"id": true} — the name is the 1-byte system string for "id" (idx 12).
        let id_name = markers::SYSTEM_STRING_1BYTE_MIN + 12;
        assert_eq!(
            decode(&buf(&[markers::OBJ1, id_name, markers::TRUE])).unwrap(),
            serde_json::json!({ "id": true }),
        );
    }

    #[test]
    fn decodes_length_prefixed_array() {
        // ArrL1 [0, 1, null]: three 1-byte scalar elements.
        let payload = [0x00u8, 0x01, markers::NULL];
        let mut bytes = vec![markers::ARR_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!([0, 1, null]),
        );
    }

    #[test]
    fn decodes_length_and_count_array() {
        // ArrLC1 [0, 1, null]: payload length 3, count 3.
        let payload = [0x00u8, 0x01, markers::NULL];
        let mut bytes = vec![markers::ARR_LC1, payload.len() as u8, 3u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!([0, 1, null]),
        );
    }

    #[test]
    fn decodes_length_prefixed_object() {
        let id_name = markers::SYSTEM_STRING_1BYTE_MIN + 12; // "id"
        let type_name = markers::SYSTEM_STRING_1BYTE_MIN + 27; // "type"
        let payload = [id_name, 0x00, type_name, 0x01];
        let mut bytes = vec![markers::OBJ_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!({ "id": 0, "type": 1 }),
        );
    }

    #[test]
    fn decodes_length_and_count_object() {
        let id_name = markers::SYSTEM_STRING_1BYTE_MIN + 12;
        let type_name = markers::SYSTEM_STRING_1BYTE_MIN + 27;
        let payload = [id_name, 0x00, type_name, 0x01];
        let mut bytes = vec![markers::OBJ_LC1, payload.len() as u8, 2u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!({ "id": 0, "type": 1 }),
        );
    }

    #[test]
    fn decodes_nested_containers() {
        let id_name = markers::SYSTEM_STRING_1BYTE_MIN + 12;
        // Outer ArrL1 wrapping `[0]` then `{"id": 1}`.
        let payload = [markers::ARR1, 0x00, markers::OBJ1, id_name, 0x01];
        let mut bytes = vec![markers::ARR_L1, payload.len() as u8];
        bytes.extend_from_slice(&payload);
        assert_eq!(
            decode(&buf(&bytes)).unwrap(),
            serde_json::json!([[0], { "id": 1 }]),
        );
    }

    #[test]
    fn rejects_count_mismatch() {
        // ArrLC1 declares count 5 but only one item fits in the 1-byte payload.
        let mut bytes = vec![markers::ARR_LC1, 1u8, 5u8];
        bytes.push(0x00);
        assert!(matches!(
            decode(&buf(&bytes)),
            Err(BinaryError::InvalidLength { .. }),
        ));
    }

    #[test]
    fn rejects_element_past_declared_length() {
        // ArrL1 declares payload length 1, but its single element is an Int16
        // (3 bytes) that runs past the declared region.
        let mut bytes = vec![markers::ARR_L1, 1u8, markers::NUMBER_INT16];
        bytes.extend_from_slice(&5i16.to_le_bytes());
        assert!(matches!(
            decode(&buf(&bytes)),
            Err(BinaryError::InvalidLength { .. }),
        ));
    }

    #[test]
    fn rejects_non_string_object_key() {
        // OBJ1 whose name slot is a literal integer (0x00) rather than a string.
        assert_eq!(
            decode(&buf(&[markers::OBJ1, 0x00, markers::TRUE])),
            Err(BinaryError::InvalidMarker {
                marker: 0x00,
                offset: 2,
            }),
        );
    }

    #[test]
    fn accepts_max_depth_nesting() {
        // MAX_DEPTH nested single-item arrays around a scalar leaf is exactly at
        // the limit and must decode successfully.
        let mut bytes = vec![markers::ARR1; MAX_DEPTH];
        bytes.push(0x00); // literal int 0 leaf
        let mut expected = serde_json::json!(0);
        for _ in 0..MAX_DEPTH {
            expected = Value::Array(vec![expected]);
        }
        assert_eq!(decode(&buf(&bytes)).unwrap(), expected);
    }

    #[test]
    fn rejects_excessive_nesting() {
        // One level beyond MAX_DEPTH trips the depth guard.
        let mut bytes = vec![markers::ARR1; MAX_DEPTH + 1];
        bytes.push(0x00);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::DepthLimitExceeded { limit: MAX_DEPTH }),
        );
    }
}
