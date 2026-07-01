// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Type-marker byte constants for the Cosmos binary JSON format.
//!
//! Every value in a Cosmos binary JSON buffer is introduced by a single
//! **type-marker** byte that selects how the following bytes are interpreted.
//! The byte values here are the authoritative wire constants and **must** match
//! the service byte-for-byte; they are transcribed from the .NET reference
//! implementation `Microsoft.Azure.Cosmos/src/Json/JsonBinaryEncoding.TypeMarker.cs`
//! (see [`crate::binary_json`] module docs for the cross-reference).
//!
//! The marker space is partitioned into contiguous ranges. Range boundaries are
//! exposed as `*_MIN` (inclusive) / `*_MAX` (**exclusive**) pairs mirroring the
//! .NET `InRange(value, min, max)` convention (`value >= min && value < max`),
//! so a Rust range check is `(MIN..MAX).contains(&marker)`.
//!
//! ```text
//! 0x00..0x20  literal integer (value == marker)
//! 0x20..0x40  1-byte system string (index into the fixed dictionary)
//! 0x40..0x60  1-byte user string (index into the per-buffer dictionary)
//! 0x60..0x68  2-byte user string
//! 0x68..0x80  base64 / GUID / compressed strings
//! 0x80..0xC0  encoded-length string (length == marker & 0x7F); 0x80 is also the preamble
//! 0xC0..0xC8  variable-length / reference strings, NumberUInt64
//! 0xC8..0xD0  fixed-width numbers
//! 0xD0..0xE0  null / bool / guid / sized ints / binary
//! 0xE0..0xE8  arrays
//! 0xE8..0xF0  objects
//! 0xF0..0xF8  uniform (typed) number arrays
//! 0xF8..=0xFF special values (0xFF == Invalid)
//! ```

// ─────────────────────────────────────────────────────────────────────────────
// [0x00, 0x20): Encoded literal integer (32 values)
// ─────────────────────────────────────────────────────────────────────────────

/// First marker whose value *is* the encoded integer (`value == marker`).
pub const LITERAL_INT_MIN: u8 = 0x00;
/// Exclusive upper bound of the literal-integer range (`LITERAL_INT_MIN + 32`).
pub const LITERAL_INT_MAX: u8 = 0x20;

// ─────────────────────────────────────────────────────────────────────────────
// [0x20, 0x40): Encoded 1-byte system string (32 values)
// ─────────────────────────────────────────────────────────────────────────────

/// First marker for a 1-byte-encoded system string (index into the fixed
/// system-string dictionary).
pub const SYSTEM_STRING_1BYTE_MIN: u8 = 0x20;
/// Exclusive upper bound of the 1-byte system-string range.
pub const SYSTEM_STRING_1BYTE_MAX: u8 = 0x40;

// ─────────────────────────────────────────────────────────────────────────────
// [0x40, 0x60): Encoded 1-byte user string (32 values)
// ─────────────────────────────────────────────────────────────────────────────

/// First marker for a 1-byte-encoded user string (index into the per-buffer
/// user-string dictionary).
pub const USER_STRING_1BYTE_MIN: u8 = 0x40;
/// Exclusive upper bound of the 1-byte user-string range.
pub const USER_STRING_1BYTE_MAX: u8 = 0x60;

// ─────────────────────────────────────────────────────────────────────────────
// [0x60, 0x68): Encoded 2-byte user string (8 values)
// ─────────────────────────────────────────────────────────────────────────────

/// First marker for a 2-byte-encoded user string.
pub const USER_STRING_2BYTE_MIN: u8 = 0x60;
/// Exclusive upper bound of the 2-byte user-string range.
pub const USER_STRING_2BYTE_MAX: u8 = 0x68;

// ─────────────────────────────────────────────────────────────────────────────
// [0x68, 0x80): base64 / GUID / compressed string values
// ─────────────────────────────────────────────────────────────────────────────

/// Standard base64-encoded string, length encoded in 1 byte.
pub const BASE64_STRING_LENGTH1: u8 = 0x71;
/// Standard base64-encoded string, length encoded in 2 bytes.
pub const BASE64_STRING_LENGTH2: u8 = 0x72;
/// URL-safe base64-encoded string, length encoded in 1 byte.
pub const BASE64_URL_STRING_LENGTH1: u8 = 0x73;
/// URL-safe base64-encoded string, length encoded in 2 bytes.
pub const BASE64_URL_STRING_LENGTH2: u8 = 0x74;
/// GUID string with only lowercase characters.
pub const LOWERCASE_GUID_STRING: u8 = 0x75;
/// GUID string with only uppercase characters.
pub const UPPERCASE_GUID_STRING: u8 = 0x76;
/// Double-quoted lowercase GUID string (ETag form).
pub const DOUBLE_QUOTED_LOWERCASE_GUID_STRING: u8 = 0x77;

/// Compressed string: lowercase hexadecimal digits packed as 4-bit characters.
pub const COMPRESSED_LOWERCASE_HEX_STRING: u8 = 0x78;
/// Compressed string: uppercase hexadecimal digits packed as 4-bit characters.
pub const COMPRESSED_UPPERCASE_HEX_STRING: u8 = 0x79;
/// Compressed string: date-time character set packed as 4-bit characters.
pub const COMPRESSED_DATE_TIME_STRING: u8 = 0x7A;
/// Compressed string: 4-bit packed characters relative to a base value.
pub const PACKED_4BIT_STRING: u8 = 0x7B;
/// Compressed string: 5-bit packed characters relative to a base value.
pub const PACKED_5BIT_STRING: u8 = 0x7C;
/// Compressed string: 6-bit packed characters relative to a base value.
pub const PACKED_6BIT_STRING: u8 = 0x7D;
/// Compressed string: 7-bit packed characters, length encoded in 1 byte.
pub const PACKED_7BIT_STRING_LENGTH1: u8 = 0x7E;
/// Compressed string: 7-bit packed characters, length encoded in 2 bytes.
pub const PACKED_7BIT_STRING_LENGTH2: u8 = 0x7F;

// ─────────────────────────────────────────────────────────────────────────────
// [0x80, 0xC0): Encoded-length string (64 values)
// ─────────────────────────────────────────────────────────────────────────────

/// First marker for an encoded-length string (`length == marker & 0x7F`).
///
/// `0x80` itself is also the **buffer preamble** byte (see
/// [`crate::binary_json::PREAMBLE`]); a zero-length string at the start of a
/// buffer is therefore indistinguishable from the preamble, so the preamble is
/// always consumed first.
pub const ENCODED_STRING_LENGTH_MIN: u8 = 0x80;
/// Exclusive upper bound of the encoded-length string range
/// (`ENCODED_STRING_LENGTH_MIN + 64`).
pub const ENCODED_STRING_LENGTH_MAX: u8 = 0xC0;

/// Mask applied to an encoded-length string marker to recover its length:
/// `length = marker & ENCODED_STRING_LENGTH_MASK`.
pub const ENCODED_STRING_LENGTH_MASK: u8 = 0x7F;

// ─────────────────────────────────────────────────────────────────────────────
// [0xC0, 0xC8): Variable-length and reference strings, NumberUInt64
// ─────────────────────────────────────────────────────────────────────────────

/// Length-prefixed string with a 1-byte length.
pub const STR_L1: u8 = 0xC0;
/// Length-prefixed string with a 2-byte length.
pub const STR_L2: u8 = 0xC1;
/// Length-prefixed string with a 4-byte length.
pub const STR_L4: u8 = 0xC2;
/// Reference string addressed by a 1-byte offset to an earlier string.
pub const STR_R1: u8 = 0xC3;
/// Reference string addressed by a 2-byte offset to an earlier string.
pub const STR_R2: u8 = 0xC4;
/// Reference string addressed by a 3-byte offset to an earlier string.
pub const STR_R3: u8 = 0xC5;
/// Reference string addressed by a 4-byte offset to an earlier string.
pub const STR_R4: u8 = 0xC6;
/// 8-byte unsigned integer.
pub const NUMBER_UINT64: u8 = 0xC7;

// ─────────────────────────────────────────────────────────────────────────────
// [0xC8, 0xD0): Fixed-width number values
// ─────────────────────────────────────────────────────────────────────────────

/// 1-byte unsigned integer.
pub const NUMBER_UINT8: u8 = 0xC8;
/// 2-byte signed integer.
pub const NUMBER_INT16: u8 = 0xC9;
/// 4-byte signed integer.
pub const NUMBER_INT32: u8 = 0xCA;
/// 8-byte signed integer.
pub const NUMBER_INT64: u8 = 0xCB;
/// Double-precision floating-point number (the canonical JSON number form).
pub const NUMBER_DOUBLE: u8 = 0xCC;
/// Single-precision (32-bit) floating-point number.
pub const FLOAT32: u8 = 0xCD;
/// Double-precision (64-bit) floating-point number.
pub const FLOAT64: u8 = 0xCE;
/// Half-precision (16-bit) floating-point number.
pub const FLOAT16: u8 = 0xCF;

// ─────────────────────────────────────────────────────────────────────────────
// [0xD0, 0xE0): null / bool / guid / sized ints / binary
// ─────────────────────────────────────────────────────────────────────────────

/// JSON `null`.
pub const NULL: u8 = 0xD0;
/// JSON `false`.
pub const FALSE: u8 = 0xD1;
/// JSON `true`.
pub const TRUE: u8 = 0xD2;
/// Raw 16-byte GUID value.
pub const GUID: u8 = 0xD3;
/// 1-byte unsigned integer value.
pub const UINT8: u8 = 0xD7;
/// 1-byte signed integer value.
pub const INT8: u8 = 0xD8;
/// 2-byte signed integer value.
pub const INT16: u8 = 0xD9;
/// 4-byte signed integer value.
pub const INT32: u8 = 0xDA;
/// 8-byte signed integer value.
pub const INT64: u8 = 0xDB;
/// 4-byte unsigned integer value.
pub const UINT32: u8 = 0xDC;
/// Binary payload with a 1-byte length prefix.
pub const BINARY_1BYTE_LENGTH: u8 = 0xDD;
/// Binary payload with a 2-byte length prefix.
pub const BINARY_2BYTE_LENGTH: u8 = 0xDE;
/// Binary payload with a 4-byte length prefix.
pub const BINARY_4BYTE_LENGTH: u8 = 0xDF;

// ─────────────────────────────────────────────────────────────────────────────
// [0xE0, 0xE8): Array type markers
// ─────────────────────────────────────────────────────────────────────────────

/// Empty array.
pub const ARR0: u8 = 0xE0;
/// Single-item array.
pub const ARR1: u8 = 0xE1;
/// Array with a 1-byte byte-length prefix.
pub const ARR_L1: u8 = 0xE2;
/// Array with a 2-byte byte-length prefix.
pub const ARR_L2: u8 = 0xE3;
/// Array with a 4-byte byte-length prefix.
pub const ARR_L4: u8 = 0xE4;
/// Array with a 1-byte byte-length prefix followed by a 1-byte item count.
pub const ARR_LC1: u8 = 0xE5;
/// Array with a 2-byte byte-length prefix followed by a 2-byte item count.
pub const ARR_LC2: u8 = 0xE6;
/// Array with a 4-byte byte-length prefix followed by a 4-byte item count.
pub const ARR_LC4: u8 = 0xE7;

// ─────────────────────────────────────────────────────────────────────────────
// [0xE8, 0xF0): Object type markers
// ─────────────────────────────────────────────────────────────────────────────

/// Empty object.
pub const OBJ0: u8 = 0xE8;
/// Single-property object.
pub const OBJ1: u8 = 0xE9;
/// Object with a 1-byte byte-length prefix.
pub const OBJ_L1: u8 = 0xEA;
/// Object with a 2-byte byte-length prefix.
pub const OBJ_L2: u8 = 0xEB;
/// Object with a 4-byte byte-length prefix.
pub const OBJ_L4: u8 = 0xEC;
/// Object with a 1-byte byte-length prefix followed by a 1-byte property count.
pub const OBJ_LC1: u8 = 0xED;
/// Object with a 2-byte byte-length prefix followed by a 2-byte property count.
pub const OBJ_LC2: u8 = 0xEE;
/// Object with a 4-byte byte-length prefix followed by a 4-byte property count.
pub const OBJ_LC4: u8 = 0xEF;

// ─────────────────────────────────────────────────────────────────────────────
// [0xF0, 0xF8): Uniform (typed) number arrays
// ─────────────────────────────────────────────────────────────────────────────

/// Uniform number array with a 1-byte item count.
pub const ARR_NUM_C1: u8 = 0xF0;
/// Uniform number array with a 2-byte item count.
pub const ARR_NUM_C2: u8 = 0xF1;
/// Array (1-byte item count) of uniform number arrays (1-byte item count).
pub const ARR_ARR_NUM_C1C1: u8 = 0xF2;
/// Array (2-byte item count) of uniform number arrays (2-byte item count).
pub const ARR_ARR_NUM_C2C2: u8 = 0xF3;

// ─────────────────────────────────────────────────────────────────────────────
// [0xF8, 0xFF]: Special values
// ─────────────────────────────────────────────────────────────────────────────

/// Reserved marker used to signal an invalid type marker.
pub const INVALID: u8 = 0xFF;

#[cfg(test)]
mod tests {
    use super::*;

    /// Pin the range boundaries so an accidental edit to one constant can't
    /// silently shift a whole class of markers. The `*_MAX` of one range must
    /// equal the `*_MIN` of the next (the ranges are contiguous).
    #[test]
    fn ranges_are_contiguous() {
        assert_eq!(LITERAL_INT_MAX, SYSTEM_STRING_1BYTE_MIN);
        assert_eq!(SYSTEM_STRING_1BYTE_MAX, USER_STRING_1BYTE_MIN);
        assert_eq!(USER_STRING_1BYTE_MAX, USER_STRING_2BYTE_MIN);
        assert_eq!(ENCODED_STRING_LENGTH_MIN, 0x80);
        assert_eq!(ENCODED_STRING_LENGTH_MAX, 0xC0);
        assert_eq!(ENCODED_STRING_LENGTH_MAX, STR_L1);
    }

    /// Each range spans the documented number of values.
    #[test]
    fn ranges_have_expected_widths() {
        assert_eq!(LITERAL_INT_MAX - LITERAL_INT_MIN, 32);
        assert_eq!(SYSTEM_STRING_1BYTE_MAX - SYSTEM_STRING_1BYTE_MIN, 32);
        assert_eq!(USER_STRING_1BYTE_MAX - USER_STRING_1BYTE_MIN, 32);
        assert_eq!(USER_STRING_2BYTE_MAX - USER_STRING_2BYTE_MIN, 8);
        assert_eq!(ENCODED_STRING_LENGTH_MAX - ENCODED_STRING_LENGTH_MIN, 64);
    }

    /// Spot-check the individually named markers against their authoritative
    /// .NET byte values. A failure here means the transcription drifted.
    #[test]
    fn named_markers_match_dotnet() {
        // Variable-length / reference strings + NumberUInt64.
        assert_eq!(STR_L1, 0xC0);
        assert_eq!(STR_L2, 0xC1);
        assert_eq!(STR_L4, 0xC2);
        assert_eq!(STR_R1, 0xC3);
        assert_eq!(STR_R4, 0xC6);
        assert_eq!(NUMBER_UINT64, 0xC7);

        // Fixed-width numbers.
        assert_eq!(NUMBER_UINT8, 0xC8);
        assert_eq!(NUMBER_INT16, 0xC9);
        assert_eq!(NUMBER_INT32, 0xCA);
        assert_eq!(NUMBER_INT64, 0xCB);
        assert_eq!(NUMBER_DOUBLE, 0xCC);
        assert_eq!(FLOAT16, 0xCF);

        // Singletons.
        assert_eq!(NULL, 0xD0);
        assert_eq!(FALSE, 0xD1);
        assert_eq!(TRUE, 0xD2);
        assert_eq!(GUID, 0xD3);

        // Containers.
        assert_eq!(ARR0, 0xE0);
        assert_eq!(ARR_LC4, 0xE7);
        assert_eq!(OBJ0, 0xE8);
        assert_eq!(OBJ_LC4, 0xEF);

        // Sentinel.
        assert_eq!(INVALID, 0xFF);
    }

    /// The encoded-length mask recovers the length stored in the marker.
    #[test]
    fn encoded_length_mask_recovers_length() {
        // 0x80 | 5 == 0x85; masking off the high bit yields 5.
        assert_eq!(
            (ENCODED_STRING_LENGTH_MIN | 5) & ENCODED_STRING_LENGTH_MASK,
            5
        );
        assert_eq!(0x85 & ENCODED_STRING_LENGTH_MASK, 5);
    }
}
