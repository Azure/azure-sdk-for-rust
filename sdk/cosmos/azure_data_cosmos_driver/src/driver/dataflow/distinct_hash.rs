// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Structural 128-bit hash of a JSON value, an exact port of the Cosmos DB
//! backend / .NET SDK `DistinctHash`
//! (`Microsoft.Azure.Cosmos.Query.Core.Pipeline.Distinct.DistinctHash`).
//!
//! The backend represents a complex (array/object) `ORDER BY` resume value as
//! this hash, so producing a byte-identical hash is what lets the structured
//! `resumeFilter` seek correctly from an array/object boundary — including
//! across a partition split or merge.
//!
//! Structurally-equal values hash equal regardless of object property order
//! (property hashes are XOR-combined). The 128-bit output is the raw
//! `UInt128` whose low/high 64-bit halves match .NET's `UInt128.GetLow()` /
//! `GetHigh()`; hashing a `UInt128` or `DoubleEx` feeds its little-endian
//! memory bytes to [`murmurhash3_128`], exactly as .NET's
//! `MemoryMarshal.AsBytes` does on a little-endian host.
//!
//! This operates on [`OrderByItem`] (the driver's typed JSON key model), so
//! it is reusable by future `DISTINCT` / `GROUP BY` work: any
//! `serde_json::Value` bridges in via [`OrderByItem::from_json`].

use super::order_by::{OrderByItem, OrderByNumber};
use crate::models::murmur_hash::murmurhash3_128;

/// Combines a `UInt128`'s low and high 64-bit words into a `u128`, matching
/// .NET's `UInt128.Create(low, high)`.
const fn seed(low: u64, high: u64) -> u128 {
    ((high as u128) << 64) | (low as u128)
}

/// Root and per-type hash seeds, verbatim from .NET `DistinctHash`
/// (`HashSeeds` / `RootHashSeed` and `CosmosNumberHasher.HashSeeds`).
mod seeds {
    use super::seed;

    pub(super) const ROOT: u128 = seed(0xbfc2359eafc0e2b7, 0x8846e00284c4cf1f);
    pub(super) const NULL: u128 = seed(0x1380f68bb3b0cfe4, 0x156c918bf564ee48);
    pub(super) const FALSE: u128 = seed(0xc1be517fe893b40c, 0xe9fc8a4c531cd0dd);
    pub(super) const TRUE: u128 = seed(0xf86d4abf9a412e74, 0x788488365c8a985d);
    pub(super) const STRING: u128 = seed(0x61f53f0a44204cfb, 0x09481be8ef4b56dd);
    pub(super) const ARRAY: u128 = seed(0xfa573b014c4dc18e, 0xa014512c858eb115);
    pub(super) const OBJECT: u128 = seed(0x77b285ac511aef30, 0x3dcf187245822449);
    pub(super) const ARRAY_INDEX: u128 = seed(0xfe057204216db999, 0x5b1cc3178bd9c593);
    pub(super) const PROPERTY_NAME: u128 = seed(0xc915dde058492a8a, 0x7c8be2eba72e4634);
    pub(super) const NUMBER64: u128 = seed(0x2400e8b894ce9c2a, 0x790be1eabd7b9481);
}

/// Structural 128-bit `DistinctHash` of a JSON value, seeded from the root —
/// the canonical entry point used to hash a complex `ORDER BY` resume value.
pub(crate) fn distinct_hash(item: &OrderByItem) -> u128 {
    structural_hash(item, seeds::ROOT)
}

/// Hashes an element under `seed`. The type seed is always mixed in first
/// (`Murmur(type_seed_bytes, seed)`); .NET's `RootCache` is just memoization
/// of this exact expression for `seed == ROOT`, so no root special-case is
/// needed here.
fn structural_hash(item: &OrderByItem, seed: u128) -> u128 {
    match item {
        // Undefined is ignored while hashing: it returns the seed untouched.
        OrderByItem::Undefined => seed,
        OrderByItem::Null => type_hash(seeds::NULL, seed),
        OrderByItem::Boolean(false) => type_hash(seeds::FALSE, seed),
        OrderByItem::Boolean(true) => type_hash(seeds::TRUE, seed),
        OrderByItem::Number(number) => hash_number(*number, seed),
        OrderByItem::String(value) => {
            let hash = type_hash(seeds::STRING, seed);
            murmurhash3_128(value.as_bytes(), hash)
        }
        OrderByItem::Array(items) => hash_array(items, seed),
        OrderByItem::Object(properties) => hash_object(properties, seed),
    }
}

/// `Murmur(type_seed's little-endian bytes, seed)` — the first mix applied to
/// every element, distinguishing e.g. an empty array from an empty object.
fn type_hash(type_seed: u128, seed: u128) -> u128 {
    murmurhash3_128(&type_seed.to_le_bytes(), seed)
}

fn hash_array(items: &[OrderByItem], seed: u128) -> u128 {
    let mut hash = type_hash(seeds::ARRAY, seed);
    for (index, item) in items.iter().enumerate() {
        // Undefined items are skipped, but the position index still advances.
        if !matches!(item, OrderByItem::Undefined) {
            let item_seed = seeds::ARRAY_INDEX.wrapping_add(index as u128);
            let item_hash = structural_hash(item, item_seed);
            hash = murmurhash3_128(&item_hash.to_le_bytes(), hash);
        }
    }
    hash
}

fn hash_object(properties: &[(String, OrderByItem)], seed: u128) -> u128 {
    let hash = type_hash(seeds::OBJECT, seed);
    // Each property value is seeded with a hash of its name, so property order
    // can be XOR-folded (order-independent) without duplicate values cancelling.
    let name_seed = murmurhash3_128(&seeds::STRING.to_le_bytes(), seeds::PROPERTY_NAME);
    let mut intermediate: u128 = 0;
    for (name, value) in properties {
        if !matches!(value, OrderByItem::Undefined) {
            let name_hash = murmurhash3_128(name.as_bytes(), name_seed);
            intermediate ^= structural_hash(value, name_hash);
        }
    }
    // Only fold in the properties for a non-empty object, so `{}` keeps a
    // distinct hash. `intermediate` is unsigned, so `!= 0` matches .NET's
    // `if (intermediateHash > 0)` exactly.
    if intermediate != 0 {
        murmurhash3_128(&intermediate.to_le_bytes(), hash)
    } else {
        hash
    }
}

fn hash_number(number: OrderByNumber, seed: u128) -> u128 {
    let hash = type_hash(seeds::NUMBER64, seed);
    let (double_value, extra_bits) = number_to_double_ex(number);
    // .NET hashes the `DoubleEx` struct's raw bytes: `[double LE][u16 LE]`,
    // 10 bytes total (`[StructLayout(Sequential, Pack = 2)]`).
    let mut bytes = [0u8; 10];
    bytes[..8].copy_from_slice(&double_value.to_le_bytes());
    bytes[8..].copy_from_slice(&extra_bits.to_le_bytes());
    murmurhash3_128(&bytes, hash)
}

/// Converts an `ORDER BY` number to the backend's `Number64.DoubleEx`
/// `(double, extraBits)` pair. Cosmos `Number64` is either a signed `long` or
/// a `double`, so an integer within `i64` range takes the exact long path and
/// anything larger (only reachable via a `u64` above `i64::MAX`) or a float
/// takes the double path.
fn number_to_double_ex(number: OrderByNumber) -> (f64, u16) {
    match number {
        OrderByNumber::I64(value) => long_to_double_ex(value),
        OrderByNumber::U64(value) if value <= i64::MAX as u64 => long_to_double_ex(value as i64),
        // Above i64::MAX, Number64 can only carry it as a double.
        OrderByNumber::U64(value) => (value as f64, 0),
        OrderByNumber::F64(value) => (value, 0),
    }
}

/// Exact port of `Number64.DoubleEx`'s `implicit operator DoubleEx(long)`:
/// small integers become a plain `double`, while an integer whose significant
/// bits span more than 52 positions is encoded losslessly across the double's
/// mantissa plus 16 `extraBits`.
fn long_to_double_ex(value: i64) -> (f64, u16) {
    // long.MinValue is special-cased: Math.Abs would overflow.
    if value == i64::MIN {
        return (value as f64, 0);
    }

    let abs_value = value.unsigned_abs();
    if abs_value != 0 {
        let msb_index = 63 - abs_value.leading_zeros() as i32;
        let lsb_index = abs_value.trailing_zeros() as i32;
        // Only when the value needs more than a double's 52-bit mantissa
        // (both spanning past bit 52 and covering more than 52 bit positions)
        // do we take the extended-precision path.
        if msb_index > 52 && (msb_index - lsb_index) > 52 {
            let exponent_value = msb_index;
            let exponent_bits: i64 = ((exponent_value as i64) + 1023) << 52;
            // Shift in u64 to match .NET's unchecked `long` shift (the mask
            // then clears the implicit leading 1, keeping 62 mantissa bits).
            let mantissa = (abs_value << (62 - exponent_value)) & 0x3FFF_FFFF_FFFF_FFFF;
            let extra_bits = ((mantissa & 0x3FF) << 6) as u16;
            let mantissa = (mantissa >> 10) as i64;
            let mut value_bits = exponent_bits | mantissa;
            if value < 0 {
                value_bits = (value_bits as u64 | 0x8000_0000_0000_0000) as i64;
            }
            return (f64::from_bits(value_bits as u64), extra_bits);
        }
    }

    (value as f64, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Parses a `.NET`-style `"89-8E-..."` 16-byte hash dump into a `u128`.
    /// The dump is the `UInt128` in big-endian order (most-significant byte
    /// first), so `from_be_bytes` recovers the value.
    fn expected(dump: &str) -> u128 {
        let mut bytes = [0u8; 16];
        for (i, part) in dump.split('-').enumerate() {
            bytes[i] = u8::from_str_radix(part, 16).expect("valid hex byte");
        }
        u128::from_be_bytes(bytes)
    }

    fn repeated_string_key() -> String {
        // Kept as a repeat to avoid a long unknown word in the spell checker.
        "asdf".repeat(6)
    }

    // ── Mandatory .NET baseline vectors ──────────────────────────────────

    #[test]
    fn baseline_null() {
        assert_eq!(
            distinct_hash(&OrderByItem::Null),
            expected("89-8E-FB-F2-68-0D-AE-A3-9E-24-D6-AE-66-53-41-7D")
        );
    }

    #[test]
    fn baseline_false() {
        assert_eq!(
            distinct_hash(&OrderByItem::Boolean(false)),
            expected("6E-88-AA-42-42-F0-17-B7-70-5F-12-58-1A-61-11-8F")
        );
    }

    #[test]
    fn baseline_true() {
        assert_eq!(
            distinct_hash(&OrderByItem::Boolean(true)),
            expected("CD-4D-51-AB-86-9F-07-1F-F0-19-C3-E4-A5-5C-01-25")
        );
    }

    #[test]
    fn baseline_empty_string() {
        assert_eq!(
            distinct_hash(&OrderByItem::String(String::new())),
            expected("A7-84-46-CF-28-70-29-A5-85-B3-95-A1-61-A2-A9-A1")
        );
    }

    #[test]
    fn baseline_string() {
        assert_eq!(
            distinct_hash(&OrderByItem::String(repeated_string_key())),
            expected("8F-12-43-68-40-1E-0B-91-AD-58-0C-A5-42-E0-35-DB")
        );
    }

    #[test]
    fn baseline_empty_array() {
        assert_eq!(
            distinct_hash(&OrderByItem::Array(Vec::new())),
            expected("7D-54-F7-29-FF-4A-63-FF-A2-EF-3C-A3-59-53-80-82")
        );
    }

    #[test]
    fn baseline_mixed_array() {
        let array = OrderByItem::Array(vec![
            OrderByItem::Null,
            OrderByItem::Boolean(false),
            OrderByItem::Boolean(true),
            OrderByItem::Number(1234_i64.into()),
            OrderByItem::String(repeated_string_key()),
        ]);
        assert_eq!(
            distinct_hash(&array),
            expected("A9-40-79-16-DE-27-F3-16-CC-5A-8E-4E-B9-AF-D5-E6")
        );
    }

    #[test]
    fn baseline_empty_object() {
        assert_eq!(
            distinct_hash(&OrderByItem::Object(Vec::new())),
            expected("14-B8-6A-60-AA-01-A7-38-14-38-74-3E-80-B9-65-D8")
        );
    }

    #[test]
    fn baseline_object() {
        let object = OrderByItem::Object(vec![
            ("null".to_owned(), OrderByItem::Null),
            ("false".to_owned(), OrderByItem::Boolean(false)),
            ("true".to_owned(), OrderByItem::Boolean(true)),
            (
                "cosmosNumber".to_owned(),
                OrderByItem::Number(1234_i64.into()),
            ),
            (
                "cosmosString".to_owned(),
                OrderByItem::String(repeated_string_key()),
            ),
        ]);
        assert_eq!(
            distinct_hash(&object),
            expected("3B-6A-A9-F4-F4-3A-AE-C0-E4-8E-BE-2B-C0-20-D5-5C")
        );
    }

    #[test]
    fn baseline_number_1234() {
        assert_eq!(
            distinct_hash(&OrderByItem::Number(1234_i64.into())),
            expected("A9-4B-F6-13-35-C9-FB-A4-2C-28-D7-D9-89-5D-14-34")
        );
    }

    // ── Structural properties ────────────────────────────────────────────

    #[test]
    fn object_hash_is_property_order_independent() {
        let forward = OrderByItem::Object(vec![
            ("a".to_owned(), OrderByItem::Number(1_i64.into())),
            ("b".to_owned(), OrderByItem::String("x".to_owned())),
            ("c".to_owned(), OrderByItem::Null),
        ]);
        let reversed = OrderByItem::Object(vec![
            ("c".to_owned(), OrderByItem::Null),
            ("b".to_owned(), OrderByItem::String("x".to_owned())),
            ("a".to_owned(), OrderByItem::Number(1_i64.into())),
        ]);
        assert_eq!(distinct_hash(&forward), distinct_hash(&reversed));
    }

    #[test]
    fn array_hash_is_order_sensitive() {
        let forward = OrderByItem::Array(vec![
            OrderByItem::Boolean(true),
            OrderByItem::Boolean(false),
        ]);
        let swapped = OrderByItem::Array(vec![
            OrderByItem::Boolean(false),
            OrderByItem::Boolean(true),
        ]);
        assert_ne!(distinct_hash(&forward), distinct_hash(&swapped));
    }

    #[test]
    fn distinct_scalars_hash_differently() {
        assert_ne!(
            distinct_hash(&OrderByItem::Null),
            distinct_hash(&OrderByItem::Boolean(false))
        );
        assert_ne!(
            distinct_hash(&OrderByItem::Number(1_i64.into())),
            distinct_hash(&OrderByItem::Number(2_i64.into()))
        );
    }

    // ── DoubleEx conversion (hand-computed from the .NET operator) ────────
    //
    // These assert the `long -> DoubleEx` conversion (double bits + extra
    // bits) hand-derived from .NET's operator. There is deliberately no
    // full end-to-end `distinct_hash` baseline vector for an *extended*
    // DoubleEx value (extra bits != 0): no published .NET hash dump was
    // available for one, and deriving an "expected" value from this crate's
    // own implementation would be a circular self-test, not an independent
    // baseline. `adjacent_large_integers_hash_differently` still guards the
    // lossless path structurally. TODO: add extended-DoubleEx baseline
    // vectors here once a published .NET dump is available.

    #[test]
    fn double_ex_small_integer_is_plain_double() {
        assert_eq!(long_to_double_ex(1234), (1234.0_f64, 0));
        assert_eq!(long_to_double_ex(-1234), (-1234.0_f64, 0));
        assert_eq!(long_to_double_ex(0), (0.0_f64, 0));
    }

    #[test]
    fn double_ex_min_value_special_case() {
        assert_eq!(long_to_double_ex(i64::MIN), (i64::MIN as f64, 0));
    }

    #[test]
    fn double_ex_two_pow_53_plus_one() {
        // 2^53 + 1: msb 53, lsb 0 -> extended path. Hand-computed:
        // doubleValue = 2^53 (0x4340_0000_0000_0000), extraBits = 0x8000.
        let (double_value, extra_bits) = long_to_double_ex(9_007_199_254_740_993);
        assert_eq!(double_value.to_bits(), 0x4340_0000_0000_0000);
        assert_eq!(extra_bits, 0x8000);
    }

    #[test]
    fn double_ex_i64_max() {
        // i64::MAX = 2^63 - 1: msb 62, lsb 0 -> extended path. Hand-computed:
        // valueBits = 0x43DF_FFFF_FFFF_FFFF, extraBits = 0xFFC0.
        let (double_value, extra_bits) = long_to_double_ex(i64::MAX);
        assert_eq!(double_value.to_bits(), 0x43DF_FFFF_FFFF_FFFF);
        assert_eq!(extra_bits, 0xFFC0);
    }

    #[test]
    fn double_ex_negative_extended_sets_sign_bit() {
        // The negative of the 2^53+1 case: identical magnitude, sign bit set.
        let (positive, positive_extra) = long_to_double_ex(9_007_199_254_740_993);
        let (negative, negative_extra) = long_to_double_ex(-9_007_199_254_740_993);
        assert_eq!(negative_extra, positive_extra);
        assert_eq!(
            negative.to_bits(),
            positive.to_bits() | 0x8000_0000_0000_0000
        );
    }

    #[test]
    fn adjacent_large_integers_hash_differently() {
        // Would collide only if the value were routed through a lossy f64.
        assert_ne!(
            distinct_hash(&OrderByItem::Number(9_007_199_254_740_992_i64.into())),
            distinct_hash(&OrderByItem::Number(9_007_199_254_740_993_i64.into()))
        );
    }

    #[test]
    fn u64_above_i64_max_uses_double_path() {
        // A u64 beyond i64::MAX can only be a Number64 double.
        let big = u64::MAX;
        assert_eq!(
            number_to_double_ex(OrderByNumber::U64(big)),
            (big as f64, 0)
        );
    }
}
