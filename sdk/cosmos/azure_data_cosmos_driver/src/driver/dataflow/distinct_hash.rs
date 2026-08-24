// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Structural, type-aware hashing of Cosmos JSON values.
//!
//! This is the deduplication identity used by cross-partition `DISTINCT`
//! ([`super::distinct`]), and is written to be reusable as a `GROUP BY` key
//! hash — nothing here knows about either stage.
//!
//! # Semantics
//!
//! Ported from .NET's `DistinctHash`, not Java's: Java funnels every scalar
//! through `ObjectOutputStream`, whose byte format is JVM-specific and not
//! reproducible outside a JVM. The *hash bytes* are therefore not compatible
//! with either peer, which is fine — a hash never leaves this process except
//! inside our own continuation token. The *behavior* matches both:
//!
//! - Every JSON type carries its own seed, so `null`, `false`, `""`, `[]`,
//!   and `{}` can never collide with each other.
//! - Arrays are **position-sensitive**: element `i` is seeded with
//!   `ARRAY_INDEX + i` and folded into a running chain, so `[1,2] != [2,1]`.
//! - Objects are **position-insensitive**: each property's hash is seeded by
//!   its key's hash and the results are XOR-folded, so
//!   `{"a":1,"b":2} == {"b":2,"a":1}`. XOR is safe here because a key is
//!   unique within a well-formed object, so no two properties can cancel.
//! - Numbers compare by value, not representation: `5` and `5.0` hash
//!   identically (see [`OrderByNumber`]), and `-0.0` is normalized to `0.0`.
//! - `undefined` is a value in its own right at the top level, distinct from
//!   `null`. It cannot appear *inside* a container here: `serde_json` has no
//!   `undefined`, and Cosmos drops undefined values from a result set rather
//!   than emitting them, so the array/object walks never have one to skip.
//!   (.NET's hasher skips undefined container members while still consuming
//!   the array index; there is nothing to mirror until `GROUP BY` introduces
//!   keys that can be undefined — see [`hash_undefined`].)
//!
//! # Recursion
//!
//! [`StructuralHasher::hash`] is depth-bounded ([`MAX_DEPTH`]) and returns a
//! typed error rather than overflowing the stack on a pathologically nested
//! payload. .NET relies on `EnsureSufficientExecutionStack` for the same
//! reason.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::murmur_hash::murmurhash3_128;

use super::order_by::OrderByNumber;

/// A 128-bit structural hash of a Cosmos JSON value.
///
/// Serialized as a lowercase 32-character hex string so a continuation token
/// stays human-inspectable and round-trips exactly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Hash128(u128);

impl Hash128 {
    #[cfg(test)]
    pub(crate) fn raw(self) -> u128 {
        self.0
    }
}

impl std::fmt::Display for Hash128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

impl Serialize for Hash128 {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Hash128 {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = <std::borrow::Cow<'de, str>>::deserialize(deserializer)?;
        // Exactly 32 hex digits, the width `Serialize` emits (which is always
        // lowercase; either case parses to the same value). A `Hash128` only
        // ever arrives from a token we minted, so a short, over-long, or
        // sign-prefixed form is hand-crafted and is rejected rather than
        // reinterpreted — the stance `planner::peel_distinct_resume` takes on
        // token shape.
        if text.len() != 32 || !text.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(serde::de::Error::custom(format!(
                "invalid 128-bit hash {text:?}: expected 32 hex digits"
            )));
        }
        u128::from_str_radix(&text, 16)
            .map(Hash128)
            .map_err(|_| serde::de::Error::custom(format!("invalid 128-bit hash {text:?}")))
    }
}

/// Maximum JSON nesting depth accepted by [`StructuralHasher::hash`].
///
/// Cosmos itself caps document nesting well below this, so exceeding it means
/// a hand-crafted or corrupt payload rather than real data.
const MAX_DEPTH: usize = 128;

/// Root seed, and one seed per JSON type so structurally different values with
/// identical raw bytes never collide. Values are .NET's `DistinctHash.HashSeeds`
/// verbatim, laid out as `(low, high)` halves of a `u128`.
mod seeds {
    const fn seed(low: u64, high: u64) -> u128 {
        ((high as u128) << 64) | (low as u128)
    }

    pub(super) const ROOT: u128 = seed(0xbfc2359eafc0e2b7, 0x8846e00284c4cf1f);
    /// Only reachable through `super::hash_undefined`; see its note.
    #[cfg(test)]
    pub(super) const UNDEFINED: u128 = seed(0x5d1f8b2a91c46e03, 0x2a7f04b6cd39e1a8);
    pub(super) const NULL: u128 = seed(0x1380f68bb3b0cfe4, 0x156c918bf564ee48);
    pub(super) const FALSE: u128 = seed(0xc1be517fe893b40c, 0xe9fc8a4c531cd0dd);
    pub(super) const TRUE: u128 = seed(0xf86d4abf9a412e74, 0x788488365c8a985d);
    pub(super) const STRING: u128 = seed(0x61f53f0a44204cfb, 0x09481be8ef4b56dd);
    pub(super) const NUMBER: u128 = seed(0x2400e8b894ce9c2a, 0x790be1eabd7b9481);
    pub(super) const ARRAY: u128 = seed(0xfa573b014c4dc18e, 0xa014512c858eb115);
    pub(super) const OBJECT: u128 = seed(0x77b285ac511aef30, 0x3dcf187245822449);
    pub(super) const ARRAY_INDEX: u128 = seed(0xfe057204216db999, 0x5b1cc3178bd9c593);
    pub(super) const PROPERTY_NAME: u128 = seed(0xc915dde058492a8a, 0x7c8be2eba72e4634);
}

/// Computes the structural hash of a JSON value using the canonical root seed.
///
/// `undefined` has no `serde_json` representation, so a caller that can
/// observe it (a missing projection) should use [`hash_undefined`] instead.
pub(crate) fn hash_value(value: &Value) -> crate::error::Result<Hash128> {
    StructuralHasher.hash(value, seeds::ROOT, 0).map(Hash128)
}

/// The hash of `undefined` — the value a projection yields when the path does
/// not exist. Deliberately distinct from the hash of `null`.
///
/// Cosmos drops `undefined` from a result set rather than emitting it, so no
/// `DISTINCT` payload can carry one today; this exists so the semantics are
/// pinned by a test before `GROUP BY`, whose keys *can* be undefined, starts
/// calling it.
#[cfg(test)]
pub(crate) fn hash_undefined() -> Hash128 {
    Hash128(murmurhash3_128(
        &seeds::UNDEFINED.to_le_bytes(),
        seeds::ROOT,
    ))
}

struct StructuralHasher;

impl StructuralHasher {
    fn hash(&self, value: &Value, seed: u128, depth: usize) -> crate::error::Result<u128> {
        if depth > MAX_DEPTH {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::CLIENT_DISTINCT_VALUE_TOO_DEEPLY_NESTED)
                .with_message(format!(
                    "value nests deeper than the {MAX_DEPTH}-level limit for DISTINCT hashing"
                ))
                .build());
        }

        Ok(match value {
            Value::Null => hash_u128(seeds::NULL, seed),
            Value::Bool(false) => hash_u128(seeds::FALSE, seed),
            Value::Bool(true) => hash_u128(seeds::TRUE, seed),
            Value::String(s) => {
                let hash = hash_u128(seeds::STRING, seed);
                murmurhash3_128(s.as_bytes(), hash)
            }
            Value::Number(n) => {
                let hash = hash_u128(seeds::NUMBER, seed);
                murmurhash3_128(&number_bytes(n), hash)
            }
            Value::Array(items) => {
                let mut hash = hash_u128(seeds::ARRAY, seed);
                for (index, item) in items.iter().enumerate() {
                    // `serde_json` has no `undefined`, so unlike .NET there is
                    // never an item to skip here — the index is always the
                    // element's real position either way.
                    let item_seed = seeds::ARRAY_INDEX.wrapping_add(index as u128);
                    let item_hash = self.hash(item, item_seed, depth + 1)?;
                    hash = hash_u128(item_hash, hash);
                }
                hash
            }
            Value::Object(map) => {
                let hash = hash_u128(seeds::OBJECT, seed);
                let mut folded: u128 = 0;
                for (key, property) in map {
                    let name_hash = murmurhash3_128(
                        key.as_bytes(),
                        hash_u128(seeds::STRING, seeds::PROPERTY_NAME),
                    );
                    folded ^= self.hash(property, name_hash, depth + 1)?;
                }
                // An empty object (and one whose properties happen to XOR to
                // zero) keeps just the type seed, matching .NET.
                if folded != 0 {
                    hash_u128(folded, hash)
                } else {
                    hash
                }
            }
        })
    }
}

/// Hashes a 128-bit value as its little-endian bytes under `seed`.
fn hash_u128(value: u128, seed: u128) -> u128 {
    murmurhash3_128(&value.to_le_bytes(), seed)
}

/// Canonical byte encoding of a JSON number, chosen so numerically equal
/// values encode identically regardless of how they were written.
///
/// Every integral value — signed, unsigned, or a float that happens to be
/// whole — widens to `i128`, which covers the full `i64` *and* `u64` ranges
/// without overlap. Encoding `i64` and `u64` separately as 8 two's-complement
/// bytes would make `-1` and `u64::MAX` share a byte pattern, silently
/// collapsing two distinct values; widening also makes `1e19` and
/// `10000000000000000000` agree, which 8-byte encodings could not.
///
/// Everything else encodes as IEEE-754 bits with `-0.0` normalized to `0.0`.
fn number_bytes(number: &serde_json::Number) -> [u8; 17] {
    const INTEGER_TAG: u8 = 0;
    const FLOAT_TAG: u8 = 1;

    let mut out = [0u8; 17];
    let integral: Option<i128> = match OrderByNumber::from_json_number(number) {
        OrderByNumber::I64(i) => Some(i as i128),
        OrderByNumber::U64(u) => Some(u as i128),
        OrderByNumber::F64(f) => integral_f64_as_i128(f),
    };
    match integral {
        Some(value) => {
            out[0] = INTEGER_TAG;
            out[1..].copy_from_slice(&value.to_le_bytes());
        }
        None => {
            let OrderByNumber::F64(f) = OrderByNumber::from_json_number(number) else {
                unreachable!("only the float variant can fail the integral conversion");
            };
            // `-0.0 == 0.0`, so normalize before touching the bit pattern.
            let f = if f == 0.0 { 0.0 } else { f };
            out[0] = FLOAT_TAG;
            out[1..9].copy_from_slice(&f.to_bits().to_le_bytes());
        }
    }
    out
}

/// Returns `f` as an `i128` when it is exactly integral and in range, so a
/// float-encoded whole number shares the integer encoding.
fn integral_f64_as_i128(f: f64) -> Option<i128> {
    const TWO_POW_127: f64 = 170_141_183_460_469_231_731_687_303_715_884_105_728.0;
    if !f.is_finite() || f.fract() != 0.0 || f >= TWO_POW_127 || f < -TWO_POW_127 {
        return None;
    }
    Some(f as i128)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use super::*;
    use serde_json::json;

    fn h(value: serde_json::Value) -> Hash128 {
        hash_value(&value).expect("value hashes")
    }

    // ── Type coverage ────────────────────────────────────────────────────
    //
    // Every JSON type must be self-consistent and mutually distinct. Mirrors
    // .NET `DistinctHashBaselineTests.ElementsHash` / `NumbersHash` and Java
    // `DistinctHashTest.{nullHash,booleanHash,integerHash,longHash,doubleHash,stringHash}`.

    #[test]
    fn each_type_hashes_deterministically() {
        for value in [
            json!(null),
            json!(true),
            json!(false),
            json!(""),
            json!("hello"),
            json!(0),
            json!(-1),
            json!(3.5),
            json!([]),
            json!({}),
            json!([1, 2, 3]),
            json!({"a": 1}),
        ] {
            assert_eq!(
                h(value.clone()),
                h(value.clone()),
                "unstable hash for {value}"
            );
        }
    }

    #[test]
    fn distinct_types_do_not_collide() {
        // `null`, `undefined`, `false`, `""`, `[]`, and `{}` are all "empty"
        // in some sense and are the classic collision trap.
        let hashes = [
            h(json!(null)),
            hash_undefined(),
            h(json!(false)),
            h(json!(true)),
            h(json!(0)),
            h(json!("")),
            h(json!([])),
            h(json!({})),
        ];
        for (i, a) in hashes.iter().enumerate() {
            for (j, b) in hashes.iter().enumerate() {
                if i != j {
                    assert_ne!(a, b, "types at {i} and {j} collide");
                }
            }
        }
    }

    #[test]
    fn undefined_is_not_null() {
        assert_ne!(hash_undefined(), h(json!(null)));
    }

    // ── Numeric equality ─────────────────────────────────────────────────

    /// Java `DistinctQueryTests.queryDocumentsForDistinctIntValues` asserts a
    /// document with `intprop: 5` and one with `intprop: 5.0` dedupe together.
    #[test]
    fn integer_and_float_forms_of_the_same_value_match() {
        assert_eq!(h(json!(5)), h(json!(5.0)));
        assert_eq!(h(json!(-7)), h(json!(-7.0)));
    }

    /// .NET normalizes `-0.0` to `0.0` in `CosmosNumberHasher`, but neither
    /// peer has a test for it.
    #[test]
    fn negative_zero_matches_positive_zero() {
        assert_eq!(h(json!(-0.0)), h(json!(0.0)));
        assert_eq!(h(json!(-0.0)), h(json!(0)));
    }

    #[test]
    fn different_numbers_differ() {
        assert_ne!(h(json!(5)), h(json!(6)));
        assert_ne!(h(json!(3.5)), h(json!(3.75)));
    }

    #[test]
    fn large_integers_beyond_f64_precision_stay_distinct() {
        // 2^53 and 2^53+1 are the same `f64`; the lossless integer encoding
        // must keep them apart.
        assert_ne!(
            h(json!(9_007_199_254_740_992i64)),
            h(json!(9_007_199_254_740_993i64))
        );
    }

    /// Regression: encoding `i64` and `u64` as bare 8-byte two's-complement
    /// patterns made `-1` and `u64::MAX` share an encoding, so DISTINCT would
    /// silently drop one of them.
    #[test]
    fn negative_integers_do_not_collide_with_large_unsigned_ones() {
        assert_ne!(h(json!(-1i64)), h(json!(u64::MAX)));
        assert_ne!(h(json!(i64::MIN)), h(json!(9_223_372_036_854_775_808u64)));
    }

    /// A whole number above `2^63` must hash the same whether it was written
    /// as an integer or in float form.
    #[test]
    fn integral_values_above_the_i64_range_still_compare_by_value() {
        assert_eq!(h(json!(10_000_000_000_000_000_000u64)), h(json!(1e19)));
    }

    #[test]
    fn very_large_floats_stay_distinct() {
        assert_ne!(h(json!(1.0e308)), h(json!(1.0e307)));
    }

    // ── Structural equality ──────────────────────────────────────────────

    /// Java `DistinctHashTest.arrayNodeHash` / `listHash`.
    #[test]
    fn array_order_matters() {
        assert_ne!(h(json!([1, 2])), h(json!([2, 1])));
        // Elements normalize like scalars do: `5` and `5.0` are one value.
        assert_eq!(h(json!([1, 2])), h(json!([1.0, 2.0])));
    }

    /// Java `DistinctHashTest.jsonObjectHash`. Untested in .NET, though its
    /// XOR fold is designed for exactly this.
    #[test]
    fn object_key_order_does_not_matter() {
        let a: Value = serde_json::from_str(r#"{"a":1,"b":2}"#).unwrap();
        let b: Value = serde_json::from_str(r#"{"b":2,"a":1}"#).unwrap();
        assert_eq!(h(a), h(b));
    }

    #[test]
    fn object_value_change_changes_the_hash() {
        assert_ne!(h(json!({"a": 1})), h(json!({"a": 2})));
        assert_ne!(h(json!({"a": 1})), h(json!({"b": 1})));
    }

    /// .NET `DistinctHashBaselineTests.WrappedElementsHash`: a value, that
    /// value in an array, and that value in an object must all differ.
    #[test]
    fn wrapping_a_value_changes_the_hash() {
        for value in [
            json!(null),
            json!(true),
            json!(42),
            json!("x"),
            json!([]),
            json!({}),
        ] {
            let bare = h(value.clone());
            let in_array = h(json!([value.clone()]));
            let in_object = h(json!({"prop": value.clone()}));
            assert_ne!(bare, in_array, "bare vs array for {value}");
            assert_ne!(bare, in_object, "bare vs object for {value}");
            assert_ne!(in_array, in_object, "array vs object for {value}");
        }
    }

    #[test]
    fn nested_structures_are_distinguished() {
        assert_ne!(h(json!({"a": {"b": 1}})), h(json!({"a": {"b": 2}})));
        assert_ne!(h(json!([[1], [2]])), h(json!([[2], [1]])));
        assert_eq!(
            h(json!({"a": [1, {"b": null}]})),
            h(json!({"a": [1, {"b": null}]}))
        );
    }

    #[test]
    fn empty_containers_differ_from_populated_ones() {
        assert_ne!(h(json!([])), h(json!([null])));
        assert_ne!(h(json!({})), h(json!({"a": null})));
    }

    // ── Strings ──────────────────────────────────────────────────────────
    //
    // .NET `DistinctQueryPipelineStageTests.MixedTypeTests` covers CJK values
    // and Arabic object keys.

    #[test]
    fn unicode_strings_are_handled() {
        assert_eq!(
            h(json!("敏捷的棕色狐狸跳过了懒狗")),
            h(json!("敏捷的棕色狐狸跳过了懒狗"))
        );
        assert_ne!(
            h(json!("敏捷的棕色狐狸跳过了懒狗")),
            h(json!("敏捷的棕色狐狸跳过了懶狗"))
        );
        assert_ne!(h(json!({"فوق": 1})), h(json!({"تحت": 1})));
    }

    #[test]
    fn long_strings_past_the_inline_threshold_are_distinguished() {
        let a = "x".repeat(4096);
        let b = format!("{}y", "x".repeat(4095));
        assert_ne!(h(json!(a)), h(json!(b)));
    }

    // ── Recursion guard ──────────────────────────────────────────────────

    #[test]
    fn excessive_nesting_is_a_typed_error_not_a_stack_overflow() {
        let mut value = json!(1);
        for _ in 0..(MAX_DEPTH + 10) {
            value = Value::Array(vec![value]);
        }
        let err = hash_value(&value).expect_err("must reject over-deep values");
        assert_eq!(
            err.status().sub_status(),
            Some(crate::error::SubStatusCode::CLIENT_DISTINCT_VALUE_TOO_DEEPLY_NESTED),
        );
    }

    #[test]
    fn nesting_at_the_limit_is_accepted() {
        let mut value = json!(1);
        for _ in 0..(MAX_DEPTH - 1) {
            value = Value::Array(vec![value]);
        }
        assert!(hash_value(&value).is_ok());
    }

    // ── Serialization ────────────────────────────────────────────────────

    #[test]
    fn hash_round_trips_through_json() {
        let hash = h(json!({"city": "Seattle"}));
        let text = serde_json::to_string(&hash).unwrap();
        assert_eq!(
            text.len(),
            34,
            "expected a quoted 32-char hex string, got {text}"
        );
        let restored: Hash128 = serde_json::from_str(&text).unwrap();
        assert_eq!(hash, restored);
    }

    #[test]
    fn malformed_hash_text_is_rejected() {
        assert!(serde_json::from_str::<Hash128>(r#""not-hex""#).is_err());
        // Short, over-long, and sign-prefixed forms are all rejected rather
        // than silently reinterpreted.
        assert!(serde_json::from_str::<Hash128>(r#""5""#).is_err());
        assert!(serde_json::from_str::<Hash128>(r#""+0000000000000000000000000000005""#).is_err());
        assert!(serde_json::from_str::<Hash128>(&format!(r#""{}""#, "0".repeat(33))).is_err());
    }

    #[test]
    fn ordering_is_total() {
        let a = h(json!(1));
        let b = h(json!(2));
        assert_eq!(a.raw().cmp(&a.raw()), Ordering::Equal);
        assert_eq!(a.cmp(&b), a.raw().cmp(&b.raw()));
    }
}
