// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cosmos binary JSON **decoder** (`binary` -> [`serde_json::Value`]).
//!
//! Every step is bounds-checked and returns a [`BinaryError`] rather than
//! panicking, so a malformed or truncated buffer fails gracefully. Multi-byte
//! integers and length prefixes are little-endian, matching the service.
//!
//! The decoder handles every value form the service can emit:
//! [`null`](serde_json::Value::Null), booleans, all literal, fixed-width, and
//! extended numbers, every string form (system, user, reference, encoded-length,
//! length-prefixed, GUID, base64, and compressed), the GUID value, binary blobs,
//! containers, and uniform number arrays. Two cases have no JSON representation
//! and are reported as errors: user strings (`0x40`-`0x67`) report
//! [`BinaryError::UnsupportedUserString`] (they reference an external dictionary
//! the data plane does not supply), and `Float16` (`0xCF`) plus the standalone
//! extended `UInt8` (`0xD7`) report [`BinaryError::InvalidMarker`].

use base64::Engine;
use serde_json::{Map, Value};

use std::cell::Cell;

use super::markers::{
    ARR0, ARR1, ARR_ARR_NUM_C1C1, ARR_ARR_NUM_C2C2, ARR_L1, ARR_L2, ARR_L4, ARR_LC1, ARR_LC2,
    ARR_LC4, ARR_NUM_C1, ARR_NUM_C2, BASE64_STRING_LENGTH1, BASE64_STRING_LENGTH2,
    BASE64_URL_STRING_LENGTH1, BASE64_URL_STRING_LENGTH2, BINARY_1BYTE_LENGTH, BINARY_2BYTE_LENGTH,
    BINARY_4BYTE_LENGTH, COMPRESSED_DATE_TIME_STRING, COMPRESSED_LOWERCASE_HEX_STRING,
    COMPRESSED_UPPERCASE_HEX_STRING, DOUBLE_QUOTED_LOWERCASE_GUID_STRING,
    ENCODED_STRING_LENGTH_MASK, ENCODED_STRING_LENGTH_MAX, ENCODED_STRING_LENGTH_MIN, FALSE,
    FLOAT32, FLOAT64, GUID, INT16, INT32, INT64, INT8, LITERAL_INT_MAX, LITERAL_INT_MIN,
    LOWERCASE_GUID_STRING, NULL, NUMBER_DOUBLE, NUMBER_INT16, NUMBER_INT32, NUMBER_INT64,
    NUMBER_UINT64, NUMBER_UINT8, OBJ0, OBJ1, OBJ_L1, OBJ_L2, OBJ_L4, OBJ_LC1, OBJ_LC2, OBJ_LC4,
    PACKED_4BIT_STRING, PACKED_5BIT_STRING, PACKED_6BIT_STRING, PACKED_7BIT_STRING_LENGTH1,
    PACKED_7BIT_STRING_LENGTH2, STR_L1, STR_L2, STR_L4, STR_R1, STR_R2, STR_R3, STR_R4,
    SYSTEM_STRING_1BYTE_MAX, SYSTEM_STRING_1BYTE_MIN, TRUE, UINT32, UINT8, UPPERCASE_GUID_STRING,
    USER_STRING_1BYTE_MAX, USER_STRING_1BYTE_MIN, USER_STRING_2BYTE_MAX, USER_STRING_2BYTE_MIN,
};
use super::system_strings::system_string_for_marker;
use super::{is_binary, BinaryError, Result};

/// Maximum container nesting depth the decoder will descend before returning
/// [`BinaryError::DepthLimitExceeded`]. This mirrors the .NET Cosmos JSON
/// stack's `JsonObjectState.JsonMaxNestingDepth` (256 simultaneously-open
/// containers), so the Rust decoder enforces the same nesting policy while
/// guarding against stack exhaustion from adversarial input.
const MAX_DEPTH: usize = 256;

/// A single native scalar token read directly from the buffer, used by the
/// native serde deserializer ([`super::de`]) to feed a visitor without
/// materializing a [`serde_json::Value`]. Only the common, cheaply-decodable
/// forms are represented here; exotic string/number forms fall back to
/// [`Reader::read_value`] in the deserializer.
pub(super) enum ScalarToken<'a> {
    /// `null`.
    Null,
    /// `true` / `false`.
    Bool(bool),
    /// A signed integer (literal, fixed-width, or extended).
    I64(i64),
    /// An unsigned integer that does not fit `i64` (`NumberUInt64`).
    U64(u64),
    /// A double.
    F64(f64),
    /// A plain UTF-8 string borrowed directly from the buffer (system,
    /// encoded-length, or `StrL1/2/4` form).
    Str(&'a str),
}

/// Framing for a container being streamed by the native deserializer: either a
/// known element/member `count`, or a byte `end` offset to read until.
pub(super) struct Frame {
    /// Declared element/member count, when the marker carries one.
    pub(super) count: Option<usize>,
    /// Absolute buffer offset at which the container's payload ends.
    pub(super) end: usize,
    /// Whether `end` is an authoritative byte boundary derived from a
    /// length prefix (`true` for the empty `Arr0`/`Obj0` and the length-framed
    /// `L*`/`LC*` markers) versus a sentinel (`false` for `Arr1`/`Obj1`, whose
    /// single element has no length prefix so `end` is set to the buffer length
    /// and must not be treated as the container's true end). When `true`, the
    /// streaming deserializer asserts the framed byte span is fully consumed,
    /// matching the reference decoder's length + count validation.
    pub(super) exact_end: bool,
}

/// The two container shapes the native deserializer streams.
pub(super) enum ContainerHeader {
    /// An array; stream `visit_seq`.
    Array(Frame),
    /// An object; stream `visit_map`.
    Object(Frame),
}

/// Width in bytes of a little-endian length or count field.
///
/// Length- and count-prefixed forms encode their field in 1, 2, or 4 bytes
/// depending on the marker; carrying this as an enum instead of a raw `usize`
/// keeps the width total and makes the accepted set explicit.
#[derive(Clone, Copy)]
enum FieldWidth {
    /// 1-byte field.
    One,
    /// 2-byte field.
    Two,
    /// 4-byte field.
    Four,
}

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
    let mut reader = Reader::new(buffer, 1);
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
pub(super) struct Reader<'a> {
    pub(super) buf: &'a [u8],
    pub(super) pos: usize,
    /// Remaining budget, in bytes, for text materialized by **reference-string**
    /// resolution ([`STR_R1`](super::markers::STR_R1)-[`STR_R4`](super::markers::STR_R4)).
    ///
    /// Each reference decodes a *fresh owned copy* of its target string, so a
    /// crafted buffer (one long string plus many short references to it) can
    /// expand to O(S^2) aggregate output from a size-`S` buffer even though every
    /// individual length prefix is buffer-bounded. This shared counter caps the
    /// total reference-expanded bytes for one decode; exceeding it fails with
    /// [`BinaryError::InvalidLength`]. Non-reference strings are backed 1:1 by
    /// buffer bytes and are not charged. Shared (via [`Cell`]) across every
    /// reference resolution in the value tree because they all run on the same
    /// reader instance.
    ref_budget: Cell<usize>,
}

/// The reference-string expansion budget for a buffer of `buf_len` bytes.
///
/// References normally *shrink* payloads (they replace a repeated string with a
/// few offset bytes), so a legitimate buffer never approaches this. The cap is
/// generous -- a multiple of the buffer size, floored at a small constant so
/// tiny buffers still permit some expansion -- while still bounding the
/// adversarial O(S^2) blow-up to O(S).
fn reference_budget(buf_len: usize) -> usize {
    const FLOOR: usize = 64 * 1024;
    const FACTOR: usize = 16;
    buf_len.saturating_mul(FACTOR).max(FLOOR)
}

impl<'a> Reader<'a> {
    /// Creates a reader positioned at `pos` within `buf`.
    pub(super) fn new(buf: &'a [u8], pos: usize) -> Self {
        let ref_budget = Cell::new(reference_budget(buf.len()));
        Self {
            buf,
            pos,
            ref_budget,
        }
    }

    /// Returns the next byte without advancing the cursor.
    pub(super) fn peek_u8(&self) -> Result<u8> {
        self.buf
            .get(self.pos)
            .copied()
            .ok_or(BinaryError::UnexpectedEof { needed: 1 })
    }

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
        Ok(slice
            .try_into()
            .expect("read_bytes returns exactly N bytes or fails"))
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
        let [b0, b1, b2] = self.read_bytes(3)? else {
            unreachable!("read_bytes(3) returns exactly 3 bytes or fails")
        };
        Ok(u32::from(*b0) | (u32::from(*b1) << 8) | (u32::from(*b2) << 16))
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
    pub(super) fn read_value(&mut self, depth: usize) -> Result<Value> {
        if depth > MAX_DEPTH {
            return Err(BinaryError::DepthLimitExceeded { limit: MAX_DEPTH });
        }
        // Offset of this value's type marker, captured before consuming it so
        // error positions point at the marker.
        let offset = self.pos;
        let marker = self.read_u8()?;

        // Only the container markers recurse (into `read_value`, directly or via
        // the array/object/member helpers). Every other marker is a
        // non-recursive leaf, decoded in `read_leaf_value` — a separate,
        // never-inlined frame. Keeping the leaf-decoding locals out of this
        // function keeps `read_value`'s own frame small, so descending to
        // `MAX_DEPTH` stays well within an ordinary thread stack instead of
        // requiring an oversized one.
        match marker {
            // Arrays.
            ARR0 => Ok(Value::Array(Vec::new())),
            ARR1 => {
                let item = self.read_value(depth + 1)?;
                Ok(Value::Array(vec![item]))
            }
            ARR_L1 => self.read_array_value(FieldWidth::One, false, depth),
            ARR_L2 => self.read_array_value(FieldWidth::Two, false, depth),
            ARR_L4 => self.read_array_value(FieldWidth::Four, false, depth),
            ARR_LC1 => self.read_array_value(FieldWidth::One, true, depth),
            ARR_LC2 => self.read_array_value(FieldWidth::Two, true, depth),
            ARR_LC4 => self.read_array_value(FieldWidth::Four, true, depth),

            // Objects.
            OBJ0 => Ok(Value::Object(Map::new())),
            OBJ1 => {
                let (name, value) = self.read_member(depth + 1)?;
                let mut map = Map::new();
                map.insert(name, value);
                Ok(Value::Object(map))
            }
            OBJ_L1 => self.read_object_value(FieldWidth::One, false, depth),
            OBJ_L2 => self.read_object_value(FieldWidth::Two, false, depth),
            OBJ_L4 => self.read_object_value(FieldWidth::Four, false, depth),
            OBJ_LC1 => self.read_object_value(FieldWidth::One, true, depth),
            OBJ_LC2 => self.read_object_value(FieldWidth::Two, true, depth),
            OBJ_LC4 => self.read_object_value(FieldWidth::Four, true, depth),

            // Every non-container marker is a leaf value.
            _ => self.read_leaf_value(marker, offset),
        }
    }

    /// Decodes a single non-container ("leaf") value: scalars, all string
    /// forms, numbers, GUIDs, binary blobs, uniform number arrays, user-string
    /// references, and back-references.
    ///
    /// This is deliberately **not inlined** into [`read_value`](Self::read_value)
    /// and never recurses back into it, so its (comparatively large) stack frame
    /// is paid only once at each leaf rather than at every level of a deeply
    /// nested container. That is what lets the recursive descent reach
    /// [`MAX_DEPTH`] on an ordinary thread stack.
    ///
    /// `offset` is the position of `marker`, used for error reporting.
    #[inline(never)]
    fn read_leaf_value(&mut self, marker: u8, offset: usize) -> Result<Value> {
        match marker {
            NULL => Ok(Value::Null),
            FALSE => Ok(Value::Bool(false)),
            TRUE => Ok(Value::Bool(true)),

            // Literal integer: the value is encoded in the marker itself.
            LITERAL_INT_MIN..LITERAL_INT_MAX => Ok(int_value(i64::from(marker))),

            // Fixed-width numbers (little-endian payloads), both the
            // self-describing `NUMBER_*` markers and the extended Cosmos
            // `INT*`/`UINT*`/`FLOAT*` markers, decode through a shared helper.
            // `Float16` (0xCF) and the extended `UInt8` (0xD7) have no JSON node
            // type in the service and are intentionally *not* routed here, so
            // they fall through to the catch-all as InvalidMarker.
            NUMBER_UINT8 | NUMBER_INT16 | NUMBER_INT32 | NUMBER_INT64 | NUMBER_UINT64
            | NUMBER_DOUBLE | INT8 | INT16 | INT32 | INT64 | UINT32 | FLOAT32 | FLOAT64 => {
                self.read_number_value(marker, offset)
            }

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
            BASE64_STRING_LENGTH1 => Ok(Value::String(
                self.read_base64_string(FieldWidth::One, false)?,
            )),
            BASE64_STRING_LENGTH2 => Ok(Value::String(
                self.read_base64_string(FieldWidth::Two, false)?,
            )),
            BASE64_URL_STRING_LENGTH1 => Ok(Value::String(
                self.read_base64_string(FieldWidth::One, true)?,
            )),
            BASE64_URL_STRING_LENGTH2 => Ok(Value::String(
                self.read_base64_string(FieldWidth::Two, true)?,
            )),

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
            PACKED_4BIT_STRING => Ok(Value::String(self.read_packed_string(
                4,
                true,
                FieldWidth::One,
            )?)),
            PACKED_5BIT_STRING => Ok(Value::String(self.read_packed_string(
                5,
                true,
                FieldWidth::One,
            )?)),
            PACKED_6BIT_STRING => Ok(Value::String(self.read_packed_string(
                6,
                true,
                FieldWidth::One,
            )?)),
            PACKED_7BIT_STRING_LENGTH1 => Ok(Value::String(self.read_packed_string(
                7,
                false,
                FieldWidth::One,
            )?)),
            PACKED_7BIT_STRING_LENGTH2 => Ok(Value::String(self.read_packed_string(
                7,
                false,
                FieldWidth::Two,
            )?)),

            // The GUID *value* is 16 bytes interpreted as a .NET `Guid`
            // (mixed-endian) and rendered as the canonical lowercase text. This
            // differs from the GUID *strings* above, which are a straight hex
            // dump. JSON has no GUID type, so it maps to a string.
            GUID => Ok(Value::String(self.read_guid_value()?)),

            // Binary blobs have no JSON representation; the raw bytes are mapped
            // to a standard base64 string (the conventional JSON byte encoding).
            BINARY_1BYTE_LENGTH => self.read_binary(FieldWidth::One),
            BINARY_2BYTE_LENGTH => self.read_binary(FieldWidth::Two),
            BINARY_4BYTE_LENGTH => self.read_binary(FieldWidth::Four),

            // Uniform number arrays: a typed, marker-shared sequence of bare
            // numbers (`ArrNumC*`) or a sequence of such arrays (`ArrArrNumC*`).
            ARR_NUM_C1 => self.read_uniform_number_array(FieldWidth::One),
            ARR_NUM_C2 => self.read_uniform_number_array(FieldWidth::Two),
            ARR_ARR_NUM_C1C1 => self.read_uniform_array_of_number_arrays(FieldWidth::One),
            ARR_ARR_NUM_C2C2 => self.read_uniform_array_of_number_arrays(FieldWidth::Two),

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

            // Any other byte is not a valid type marker.
            other => Err(BinaryError::InvalidMarker {
                marker: other,
                offset,
            }),
        }
    }

    /// Attempts to read the next value as a native scalar token, consuming it
    /// only when it is one of the cheaply-decodable forms.
    ///
    /// Returns `Ok(Some(_))` (advancing the cursor) for `null`, booleans, every
    /// literal/fixed-width/extended number, system strings, and plain
    /// UTF-8 strings (encoded-length and `StrL1/2/4`). Returns `Ok(None)`
    /// **without advancing** for any other marker -- containers and the exotic
    /// string/number forms -- which the deserializer handles via a container
    /// stream or the [`read_value`](Self::read_value) fallback respectively.
    pub(super) fn try_read_native_scalar(&mut self) -> Result<Option<ScalarToken<'a>>> {
        let offset = self.pos;
        let marker = self.peek_u8()?;
        let token = match marker {
            NULL => ScalarToken::Null,
            FALSE => ScalarToken::Bool(false),
            TRUE => ScalarToken::Bool(true),

            // Literal integer: the value is encoded in the marker itself.
            m if (LITERAL_INT_MIN..LITERAL_INT_MAX).contains(&m) => ScalarToken::I64(i64::from(m)),

            // Fixed-width and extended numbers all project to a JSON number.
            NUMBER_UINT8 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::I64(i64::from(self.read_u8()?))));
            }
            NUMBER_INT16 | INT16 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::I64(i64::from(self.read_i16_le()?))));
            }
            NUMBER_INT32 | INT32 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::I64(i64::from(self.read_i32_le()?))));
            }
            NUMBER_INT64 | INT64 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::I64(self.read_i64_le()?)));
            }
            NUMBER_UINT64 => {
                self.pos += 1;
                let v = self.read_u64_le()?;
                // Prefer the signed projection when it fits, matching the
                // `Value` decoder's number handling.
                return Ok(Some(match i64::try_from(v) {
                    Ok(i) => ScalarToken::I64(i),
                    Err(_) => ScalarToken::U64(v),
                }));
            }
            NUMBER_DOUBLE | FLOAT64 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::F64(self.read_f64_le()?)));
            }
            INT8 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::I64(i64::from(self.read_i8()?))));
            }
            UINT32 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::I64(i64::from(self.read_u32_le()?))));
            }
            FLOAT32 => {
                self.pos += 1;
                return Ok(Some(ScalarToken::F64(f64::from(self.read_f32_le()?))));
            }

            // 1-byte system string: borrow the static dictionary entry.
            m if (SYSTEM_STRING_1BYTE_MIN..SYSTEM_STRING_1BYTE_MAX).contains(&m) => {
                let s = system_string_for_marker(m)
                    .ok_or(BinaryError::InvalidMarker { marker: m, offset })?;
                self.pos += 1;
                return Ok(Some(ScalarToken::Str(s)));
            }

            // Encoded-length string: length carried in the marker.
            m if (ENCODED_STRING_LENGTH_MIN..ENCODED_STRING_LENGTH_MAX).contains(&m) => {
                self.pos += 1;
                let len = usize::from(m & ENCODED_STRING_LENGTH_MASK);
                return Ok(Some(ScalarToken::Str(self.read_str_slice(len, offset)?)));
            }

            // Length-prefixed strings.
            STR_L1 => {
                self.pos += 1;
                let len = usize::from(self.read_u8()?);
                return Ok(Some(ScalarToken::Str(self.read_str_slice(len, offset)?)));
            }
            STR_L2 => {
                self.pos += 1;
                let len = usize::from(self.read_u16_le()?);
                return Ok(Some(ScalarToken::Str(self.read_str_slice(len, offset)?)));
            }
            STR_L4 => {
                self.pos += 1;
                let len = self.read_u32_le()? as usize;
                return Ok(Some(ScalarToken::Str(self.read_str_slice(len, offset)?)));
            }

            // Not a native scalar: leave the cursor untouched.
            _ => return Ok(None),
        };
        // Single-byte tokens (null/bool/literal-int): consume just the marker.
        self.pos += 1;
        Ok(Some(token))
    }

    /// Borrows a `len`-byte UTF-8 slice directly from the buffer, advancing the
    /// cursor. `marker_offset` positions a UTF-8 error.
    fn read_str_slice(&mut self, len: usize, marker_offset: usize) -> Result<&'a str> {
        let bytes = self.read_bytes(len)?;
        std::str::from_utf8(bytes).map_err(|_| BinaryError::InvalidUtf8 {
            offset: marker_offset,
        })
    }

    /// Attempts to read a standard array/object container header, consuming the
    /// marker and length/count prefix only when the marker is one of the
    /// streamable container forms (`Arr0/1/L*/LC*`, `Obj0/1/L*/LC*`).
    ///
    /// Returns `Ok(None)` **without advancing** for anything else (including the
    /// uniform number-array markers, which have no per-element framing and are
    /// handled via the [`read_value`](Self::read_value) fallback).
    pub(super) fn read_container_header(&mut self) -> Result<Option<ContainerHeader>> {
        let marker = self.peek_u8()?;
        let header = match marker {
            ARR0 => ContainerHeader::Array(Frame {
                count: Some(0),
                end: self.pos + 1,
                exact_end: true,
            }),
            ARR1 => {
                // A one-element array with no length prefix: the element
                // follows immediately, so stream by count and let the element
                // read advance the cursor.
                self.pos += 1;
                return Ok(Some(ContainerHeader::Array(Frame {
                    count: Some(1),
                    end: self.buf.len(),
                    exact_end: false,
                })));
            }
            ARR_L1 | ARR_L2 | ARR_L4 | ARR_LC1 | ARR_LC2 | ARR_LC4 => {
                let (count, end) = self.read_container_frame(marker, [ARR_L1, ARR_L2, ARR_L4])?;
                return Ok(Some(ContainerHeader::Array(Frame {
                    count,
                    end,
                    exact_end: true,
                })));
            }
            OBJ0 => ContainerHeader::Object(Frame {
                count: Some(0),
                end: self.pos + 1,
                exact_end: true,
            }),
            OBJ1 => {
                self.pos += 1;
                return Ok(Some(ContainerHeader::Object(Frame {
                    count: Some(1),
                    end: self.buf.len(),
                    exact_end: false,
                })));
            }
            OBJ_L1 | OBJ_L2 | OBJ_L4 | OBJ_LC1 | OBJ_LC2 | OBJ_LC4 => {
                let (count, end) = self.read_container_frame(marker, [OBJ_L1, OBJ_L2, OBJ_L4])?;
                return Ok(Some(ContainerHeader::Object(Frame {
                    count,
                    end,
                    exact_end: true,
                })));
            }
            _ => return Ok(None),
        };
        // Empty-container markers (`Arr0`/`Obj0`): consume just the marker.
        self.pos += 1;
        Ok(Some(header))
    }

    /// Parses the length (and optional count) prefix shared by the `L*`/`LC*`
    /// array and object markers, consuming the marker and prefixes. `l_markers`
    /// are the three length-only markers (width 1/2/4) for this container kind;
    /// the `LC*` (length + count) markers sit three positions above their `L*`
    /// counterparts.
    fn read_container_frame(
        &mut self,
        marker: u8,
        l_markers: [u8; 3],
    ) -> Result<(Option<usize>, usize)> {
        let [l1, l2, l4] = l_markers;
        let width = if marker == l1 || marker == l1 + 3 {
            FieldWidth::One
        } else if marker == l2 || marker == l2 + 3 {
            FieldWidth::Two
        } else {
            FieldWidth::Four
        };
        let has_count = marker == l1 + 3 || marker == l2 + 3 || marker == l4 + 3;
        self.pos += 1; // consume the marker
        let payload_len = self.read_len(width)?;
        let count = if has_count {
            Some(self.read_len(width)?)
        } else {
            None
        };
        let end = self.bounded_end(payload_len)?;
        Ok((count, end))
    }

    /// Reads a 1-, 2-, or 4-byte little-endian length or count field.
    fn read_len(&mut self, width: FieldWidth) -> Result<usize> {
        match width {
            FieldWidth::One => Ok(usize::from(self.read_u8()?)),
            FieldWidth::Two => Ok(usize::from(self.read_u16_le()?)),
            FieldWidth::Four => Ok(self.read_u32_le()? as usize),
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
    fn read_array_value(
        &mut self,
        width: FieldWidth,
        has_count: bool,
        depth: usize,
    ) -> Result<Value> {
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
    fn read_object_value(
        &mut self,
        width: FieldWidth,
        has_count: bool,
        depth: usize,
    ) -> Result<Value> {
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
        // Capture the name's type marker before decoding so a non-string name
        // can be reported without re-indexing the buffer.
        let name_marker = self.peek_u8()?;
        let name = self.read_value(depth)?;
        let name = match name {
            Value::String(s) => s,
            _ => {
                return Err(BinaryError::InvalidMarker {
                    marker: name_marker,
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
            compression::UPPERCASE_HEX
        } else {
            compression::LOWERCASE_HEX
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
    fn read_base64_string(&mut self, length_width: FieldWidth, url_safe: bool) -> Result<String> {
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
        length_width: FieldWidth,
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

    /// Reads a GUID value: 16 bytes interpreted as a .NET `Guid` and rendered as
    /// the canonical lowercase `xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx` text.
    ///
    /// The .NET `Guid` memory layout is mixed-endian: the first three groups
    /// (4, 2, 2 bytes) are little-endian integers, while the final 8 bytes are
    /// taken in order. This matches `Guid`'s in-memory representation that the
    /// service writes, and differs from the GUID *string* forms (which dump the
    /// 16 bytes sequentially).
    fn read_guid_value(&mut self) -> Result<String> {
        let b = self.read_array::<16>()?;
        Ok(format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            // Data1 (little-endian u32).
            b[3], b[2], b[1], b[0],
            // Data2 (little-endian u16).
            b[5], b[4],
            // Data3 (little-endian u16).
            b[7], b[6],
            // Data4 (sequential).
            b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15],
        ))
    }

    /// Reads a binary blob: a `length_width`-byte little-endian length followed
    /// by that many raw bytes, mapped to a standard base64 [`Value::String`]
    /// (JSON has no native binary type).
    fn read_binary(&mut self, length_width: FieldWidth) -> Result<Value> {
        let len = self.read_len(length_width)?;
        let bytes = self.read_bytes(len)?;
        let encoded = base64::engine::general_purpose::STANDARD.encode(bytes);
        Ok(Value::String(encoded))
    }

    /// Reads one **bare** number value of the given uniform-array item type.
    ///
    /// Inside a uniform number array the item type marker is shared, so each
    /// element is just the little-endian value with no per-item marker.
    /// `marker_offset` is the offset of the array's item-type marker, used to
    /// report an unsupported item type. Mirrors the uniform-array branch of
    /// .NET `TryGetNumberValue`.
    ///
    /// A uniform-array item type must be one of the **extended**
    /// `INT*`/`UINT*`/`FLOAT*` markers. The self-describing `NUMBER_*` markers
    /// never appear as a uniform-array item type, so they are rejected here
    /// rather than accepted through the shared [`read_number_value`] decoder,
    /// keeping the decoder no more permissive than the service.
    ///
    /// [`read_number_value`]: Self::read_number_value
    fn read_bare_number(&mut self, item_marker: u8, marker_offset: usize) -> Result<Value> {
        match item_marker {
            INT8 | UINT8 | INT16 | INT32 | INT64 | UINT32 | FLOAT32 | FLOAT64 => {
                self.read_number_value(item_marker, marker_offset)
            }
            other => Err(BinaryError::InvalidMarker {
                marker: other,
                offset: marker_offset,
            }),
        }
    }

    /// Decodes the little-endian payload of a fixed-width number `marker` into a
    /// JSON number [`Value`]. The marker has already been consumed; the payload
    /// is read from the current position. `marker_offset` locates the marker for
    /// error reporting.
    ///
    /// Shared by [`read_value`](Self::read_value) (self-describing top-level
    /// numbers) and [`read_bare_number`](Self::read_bare_number) (uniform-array
    /// items using the extended `INT*`/`UINT*`/`FLOAT*` markers). Each caller
    /// only forwards the markers valid in its context; any other marker is
    /// reported as [`BinaryError::InvalidMarker`].
    fn read_number_value(&mut self, marker: u8, marker_offset: usize) -> Result<Value> {
        match marker {
            NUMBER_UINT8 | UINT8 => Ok(int_value(i64::from(self.read_u8()?))),
            NUMBER_INT16 | INT16 => Ok(int_value(i64::from(self.read_i16_le()?))),
            NUMBER_INT32 | INT32 => Ok(int_value(i64::from(self.read_i32_le()?))),
            NUMBER_INT64 | INT64 => Ok(int_value(self.read_i64_le()?)),
            NUMBER_UINT64 => Ok(uint_value(self.read_u64_le()?)),
            UINT32 => Ok(int_value(i64::from(self.read_u32_le()?))),
            INT8 => Ok(int_value(i64::from(self.read_i8()?))),
            NUMBER_DOUBLE | FLOAT64 => double_value(self.read_f64_le()?),
            FLOAT32 => double_value(f64::from(self.read_f32_le()?)),
            other => Err(BinaryError::InvalidMarker {
                marker: other,
                offset: marker_offset,
            }),
        }
    }

    /// Reads a uniform number array (`ArrNumC1`/`ArrNumC2`). The prefix is the
    /// shared item-type marker followed by a `count_width`-byte little-endian
    /// item count; the body is that many bare numbers of the shared type.
    fn read_uniform_number_array(&mut self, count_width: FieldWidth) -> Result<Value> {
        let item_marker_offset = self.pos;
        let item_marker = self.read_u8()?;
        let count = self.read_len(count_width)?;

        let mut items = Vec::with_capacity(count.min(1024));
        for _ in 0..count {
            items.push(self.read_bare_number(item_marker, item_marker_offset)?);
        }
        Ok(Value::Array(items))
    }

    /// Reads a uniform array of uniform number arrays (`ArrArrNumC1C1` /
    /// `ArrArrNumC2C2`). The prefix is the inner-array type marker, the shared
    /// number item-type marker, the per-inner-array number count, then the outer
    /// array count (each a `count_width`-byte little-endian field). The body is
    /// `outer_count` inner arrays, each holding `inner_count` bare numbers.
    fn read_uniform_array_of_number_arrays(&mut self, count_width: FieldWidth) -> Result<Value> {
        // Inner-array type marker (ArrNumC1/ArrNumC2); consumed but not needed
        // beyond confirming the structure, since the shared number type follows.
        let _inner_array_marker = self.read_u8()?;
        let item_marker_offset = self.pos;
        let item_marker = self.read_u8()?;
        let inner_count = self.read_len(count_width)?;
        let outer_count = self.read_len(count_width)?;

        // When `inner_count == 0` each inner array reads zero body bytes, so
        // `outer_count` (up to `u16::MAX` for the `C2C2` form) empty arrays
        // could be produced from a handful of input bytes. Bound the produced
        // element count by the bytes remaining, keeping decode output
        // proportional to input for this otherwise-unbacked branch (`decode`
        // runs on untrusted response bytes).
        if inner_count == 0 && outer_count > self.buf.len().saturating_sub(self.pos) {
            return Err(BinaryError::InvalidLength {
                detail: "uniform array of empty number arrays declares more elements than remaining bytes",
            });
        }

        let mut outer = Vec::with_capacity(outer_count.min(1024));
        for _ in 0..outer_count {
            let mut inner = Vec::with_capacity(inner_count.min(1024));
            for _ in 0..inner_count {
                inner.push(self.read_bare_number(item_marker, item_marker_offset)?);
            }
            outer.push(Value::Array(inner));
        }
        Ok(Value::Array(outer))
    }

    /// Resolves a reference string ([`STR_R1`]-[`STR_R4`]) whose `target` is an
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
        // The target is guaranteed a non-reference string, so this sub-read
        // resolves no further references and needs no shared budget.
        let mut sub = Reader::new(self.buf, target);
        let value = sub.read_value(0)?;

        // Charge the materialized text against the shared reference-expansion
        // budget so many references to one large string cannot amplify a
        // size-`S` buffer into O(S^2) aggregate output.
        if let Value::String(s) = &value {
            let remaining = self.ref_budget.get();
            let cost = s.len();
            if cost > remaining {
                return Err(BinaryError::InvalidLength {
                    detail: "reference-string expansion exceeds the decode budget",
                });
            }
            self.ref_budget.set(remaining - cost);
        }
        Ok(value)
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
/// Each table maps a 4-bit nibble (`0x0`-`0xF`) to one ASCII byte, transcribed
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
    use crate::binary_json::vectors::golden_vectors;
    use crate::binary_json::PREAMBLE;

    /// Helper: prepend the preamble to a value's marker+payload bytes.
    fn buf(value_bytes: &[u8]) -> Vec<u8> {
        let mut v = vec![PREAMBLE];
        v.extend_from_slice(value_bytes);
        v
    }

    /// The decoder reproduces every golden vector's JSON.
    #[test]
    fn decodes_golden_corpus() {
        for vector in golden_vectors() {
            let decoded = decode(&vector.binary).unwrap_or_else(|e| {
                panic!("case {}: decode failed: {e}", vector.name);
            });
            let expected: Value = serde_json::from_str(&vector.json).unwrap();
            assert_eq!(decoded, expected, "case {}", vector.name);
        }
    }

    #[test]
    fn decodes_literal_integers() {
        for n in 0u8..32 {
            let value = decode(&buf(&[n])).unwrap();
            assert_eq!(value, serde_json::json!(n), "literal int {n}");
        }
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
    fn reserved_and_invalid_markers_are_rejected() {
        // 0xFF is the explicit Invalid marker; 0xD4 is a reserved/empty slot.
        // Neither has a value form, so both report InvalidMarker at the marker
        // offset (index 1, just past the preamble).
        assert_eq!(
            decode(&[PREAMBLE, markers::INVALID]),
            Err(BinaryError::InvalidMarker {
                marker: markers::INVALID,
                offset: 1,
            }),
        );
        assert_eq!(
            decode(&[PREAMBLE, 0xD4]),
            Err(BinaryError::InvalidMarker {
                marker: 0xD4,
                offset: 1,
            }),
        );
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
    fn rejects_truncated_base64_string() {
        // Declares one group (3 raw bytes) but only one byte follows.
        assert_eq!(
            decode(&buf(&base64_len1(1, 0, b"f"))),
            Err(BinaryError::UnexpectedEof { needed: 2 }),
        );
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
    fn rejects_truncated_compressed_string() {
        // 7-bit length 4 needs ceil(4*7/8) = 4 payload bytes; only one follows.
        assert_eq!(
            decode(&[PREAMBLE, markers::PACKED_7BIT_STRING_LENGTH1, 4, 0x00]),
            Err(BinaryError::UnexpectedEof { needed: 3 }),
        );
    }

    #[test]
    fn rejects_truncated_guid_value() {
        // GUID value needs 16 bytes; only four follow.
        let mut bytes = vec![markers::GUID];
        bytes.extend_from_slice(&[0x01, 0x02, 0x03, 0x04]);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::UnexpectedEof { needed: 12 }),
        );
    }

    #[test]
    fn rejects_truncated_binary_blob() {
        // Declares 10 bytes but only 2 follow.
        let mut bytes = vec![markers::BINARY_1BYTE_LENGTH, 10];
        bytes.extend_from_slice(&[0x01, 0x02]);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::UnexpectedEof { needed: 8 }),
        );
    }

    #[test]
    fn rejects_uniform_array_with_invalid_item_type() {
        // The shared item-type marker (here NULL) is not a number type.
        let bytes = [markers::ARR_NUM_C1, markers::NULL, 1, 0x00];
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::InvalidMarker {
                marker: markers::NULL,
                // Marker sits at index 2: preamble (0) + ArrNumC1 (1) + item (2).
                offset: 2,
            }),
        );
    }

    #[test]
    fn rejects_uniform_array_with_self_describing_number_item_type() {
        // A uniform-array item type must be an extended `INT*`/`UINT*`/`FLOAT*`
        // marker. The self-describing `NUMBER_*` markers never appear in this
        // position, so `NUMBER_UINT8` must be rejected rather than decoded.
        let bytes = [markers::ARR_NUM_C1, markers::NUMBER_UINT8, 1, 0x00];
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::InvalidMarker {
                marker: markers::NUMBER_UINT8,
                // Marker sits at index 2: preamble (0) + ArrNumC1 (1) + item (2).
                offset: 2,
            }),
        );
    }

    #[test]
    fn rejects_truncated_uniform_array() {
        // Declares three Int32 items but only one value's worth of bytes follow.
        let mut bytes = vec![markers::ARR_NUM_C1, markers::INT32, 3];
        bytes.extend_from_slice(&1i32.to_le_bytes());
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::UnexpectedEof { needed: 4 }),
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
        // the limit and must decode successfully. The recursive descent keeps a
        // small per-level frame (leaf decoding lives in a separate non-inlined
        // frame), so this stays well within an ordinary thread stack.
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
        // One level beyond MAX_DEPTH trips the depth guard. The recursion keeps
        // a small per-level frame, so reaching the guard does not stress the
        // ordinary test-harness stack.
        let mut bytes = vec![markers::ARR1; MAX_DEPTH + 1];
        bytes.push(0x00);
        assert_eq!(
            decode(&buf(&bytes)),
            Err(BinaryError::DepthLimitExceeded { limit: MAX_DEPTH }),
        );
    }
}
