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
//! This is phase **P1a**: the [`Reader`] cursor infrastructure plus the scalar
//! value forms ([`null`](serde_json::Value::Null), booleans, literal and
//! fixed-width numbers, and the common string forms — system strings,
//! encoded-length strings, and `StrL1`/`StrL2`/`StrL4`). The remaining forms
//! surface as [`BinaryError::InvalidMarker`] until their sub-phase lands:
//!
//! - **P1b:** containers (arrays `0xE0`–`0xE7`, objects `0xE8`–`0xEF`).
//! - **P1b:** user strings (`0x40`–`0x67`) and reference strings
//!   (`StrR1`–`StrR4`, `0xC3`–`0xC6`).
//! - **P1c:** exotic strings (base64 / GUID / compressed, `0x68`–`0x7F`),
//!   `Float16`/`Float32`/`Float64` (`0xCD`–`0xCF`), sized ints (`0xD7`–`0xDC`),
//!   binary (`0xDD`–`0xDF`), and uniform number arrays (`0xF0`–`0xF3`).

use serde_json::Value;

use super::markers::{
    ENCODED_STRING_LENGTH_MASK, ENCODED_STRING_LENGTH_MAX, ENCODED_STRING_LENGTH_MIN,
    LITERAL_INT_MAX, LITERAL_INT_MIN, NUMBER_DOUBLE, NUMBER_INT16, NUMBER_INT32, NUMBER_INT64,
    NUMBER_UINT64, NUMBER_UINT8, STR_L1, STR_L2, STR_L4, SYSTEM_STRING_1BYTE_MAX,
    SYSTEM_STRING_1BYTE_MIN, TRUE,
};
use super::markers::{FALSE, NULL};
use super::system_strings::system_string_for_marker;
use super::{is_binary, BinaryError, Result};

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
    let value = reader.read_value()?;
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

    fn read_u64_le(&mut self) -> Result<u64> {
        Ok(u64::from_le_bytes(self.read_array()?))
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
    fn read_value(&mut self) -> Result<Value> {
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

            // Every other (valid-but-not-yet-implemented or genuinely invalid)
            // marker is reported as invalid. Containers, user/reference strings,
            // and the exotic string/number forms are filled in by later P1
            // sub-phases (see the module-level docs).
            other => Err(BinaryError::InvalidMarker {
                marker: other,
                offset,
            }),
        }
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
        // An object marker is valid in the format but implemented in P1b; until
        // then it surfaces as InvalidMarker. The offset points at the marker
        // (index 1, just past the preamble).
        assert_eq!(
            decode(&[PREAMBLE, markers::OBJ0]),
            Err(BinaryError::InvalidMarker {
                marker: markers::OBJ0,
                offset: 1,
            }),
        );
    }
}
