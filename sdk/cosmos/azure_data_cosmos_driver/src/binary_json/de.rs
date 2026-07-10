// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Native `serde` **deserializer** for Cosmos binary JSON (`binary` → `T`).
//!
//! [`from_slice`] drives a target type's own [`Deserialize`](serde::Deserialize)
//! implementation directly off the binary buffer. The common, dominant forms —
//! objects, arrays, and plain scalars (null, booleans, every integer/float
//! width, and plain UTF-8 strings) — are streamed straight into the visitor
//! with **no intermediate [`serde_json::Value`]**: object/array structure is
//! yielded through [`SeqAccess`]/[`MapAccess`], and plain strings are handed to
//! the visitor as borrowed slices ([`visit_borrowed_str`](serde::de::Visitor::visit_borrowed_str))
//! that point into the buffer.
//!
//! # Completeness via a `Value` fallback
//!
//! The service can emit rarer wire forms — GUID / base64 / compressed /
//! reference strings, binary blobs, and uniform number arrays — and Rust
//! `enum`s deserialize from serde's externally-tagged shape. Rather than
//! re-implement all of those against the serde visitor surface, the
//! deserializer falls back to the proven [`decode`](super::decode) reader for a
//! single such value and forwards it through `serde_json::Value`'s own
//! deserializer. This keeps the native fast path small and guarantees the
//! decoder's completeness is inherited without duplication. Real Cosmos item
//! bodies are objects of plain scalars, so the fast path covers them entirely.
//!
//! # Relationship to [`decode`](super::decode)
//!
//! [`decode`](super::decode) (binary → [`serde_json::Value`]) remains the
//! reference decoder and the target of fuzzing and the parity corpus.
//! `from_slice` is the typed, allocation-light path used on the SDK read
//! boundary; `from_slice::<serde_json::Value>(buf)` is asserted to equal
//! `decode(buf)`.

use serde::de::{DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;

use super::markers::NULL;
use super::reader::{ContainerHeader, Frame, Reader, ScalarToken};
use super::{is_binary, BinaryError, Result};

/// Maximum container nesting depth, mirroring the reference decoder's
/// [`MAX_DEPTH`](super::reader) so both paths reject the same adversarial
/// nesting.
const MAX_DEPTH: usize = 256;

/// Deserializes a Cosmos binary JSON buffer into a value of type `T`.
///
/// The buffer must begin with the [`PREAMBLE`](super::PREAMBLE) byte (`0x80`);
/// the single top-level value that follows is decoded, and any trailing bytes
/// are reported as [`BinaryError::TrailingBytes`].
///
/// This is the native, allocation-light counterpart to
/// `decode(buf).and_then(serde_json::from_value)` — it drives `T::deserialize`
/// straight off the bytes, materializing a [`serde_json::Value`] only for the
/// rare exotic forms handled by the fallback (see the module docs).
///
/// # Errors
///
/// Returns a [`BinaryError`] if the buffer is not binary (missing preamble), is
/// truncated, contains an invalid marker, has trailing bytes, or if `T`'s
/// `Deserialize` implementation rejects the decoded shape.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::{from_slice, to_vec};
///
/// let bytes = to_vec(&serde_json::json!({ "id": "1", "count": 7 })).unwrap();
/// let value: serde_json::Value = from_slice(&bytes).unwrap();
/// assert_eq!(value, serde_json::json!({ "id": "1", "count": 7 }));
/// ```
pub fn from_slice<'de, T>(buffer: &'de [u8]) -> Result<T>
where
    T: serde::Deserialize<'de>,
{
    if !is_binary(buffer) {
        return Err(match buffer.first() {
            Some(&found) => BinaryError::MissingPreamble { found },
            None => BinaryError::UnexpectedEof { needed: 1 },
        });
    }

    let mut de = BinaryDeserializer {
        reader: Reader::new(buffer, 1),
        depth: 0,
    };
    let value = T::deserialize(&mut de)?;
    let remaining = buffer.len() - de.reader.pos;
    if remaining != 0 {
        return Err(BinaryError::TrailingBytes { remaining });
    }
    Ok(value)
}

/// A serde deserializer over a binary JSON buffer.
struct BinaryDeserializer<'de> {
    reader: Reader<'de>,
    depth: usize,
}

impl<'de> BinaryDeserializer<'de> {
    /// Reads the next value as an owned [`serde_json::Value`] (consuming its
    /// bytes) and forwards it through `Value`'s deserializer. Used for the
    /// exotic wire forms the native fast path does not handle.
    fn deserialize_via_value<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let value = self.reader.read_value(self.depth)?;
        value
            .deserialize_any(visitor)
            .map_err(|e| BinaryError::Custom(e.to_string()))
    }
}

impl<'de> Deserializer<'de> for &mut BinaryDeserializer<'de> {
    type Error = BinaryError;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.depth > MAX_DEPTH {
            return Err(BinaryError::DepthLimitExceeded { limit: MAX_DEPTH });
        }

        // Standard array/object container: stream it natively.
        if let Some(header) = self.reader.read_container_header()? {
            self.depth += 1;
            let result = match header {
                ContainerHeader::Array(frame) => visitor.visit_seq(SeqStream::new(self, frame)),
                ContainerHeader::Object(frame) => visitor.visit_map(MapStream::new(self, frame)),
            };
            self.depth -= 1;
            return result;
        }

        // Common scalar: feed the visitor directly, borrowing plain strings.
        if let Some(token) = self.reader.try_read_native_scalar()? {
            return match token {
                ScalarToken::Null => visitor.visit_unit(),
                ScalarToken::Bool(b) => visitor.visit_bool(b),
                ScalarToken::I64(i) => visitor.visit_i64(i),
                ScalarToken::U64(u) => visitor.visit_u64(u),
                ScalarToken::F64(f) => visitor.visit_f64(f),
                ScalarToken::Str(s) => visitor.visit_borrowed_str(s),
            };
        }

        // Exotic form (guid/base64/compressed/reference string, binary blob,
        // uniform number array): defer to the reference decoder + Value.
        self.deserialize_via_value(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.reader.peek_u8()? == NULL {
            // Consume the `null` marker, then report absence.
            self.reader.try_read_native_scalar()?;
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Newtype structs serialize transparently, so deserialize the inner
        // value straight through.
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_enum<V>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        // Enums use serde's externally-tagged shape (unit → name string,
        // others → single-key object). Decode the whole value and let
        // `serde_json::Value`'s enum deserializer apply the matching rule.
        let value = self.reader.read_value(self.depth)?;
        value
            .into_deserializer()
            .deserialize_enum(name, variants, visitor)
            .map_err(|e| BinaryError::Custom(e.to_string()))
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 i128 u8 u16 u32 u64 u128 f32 f64 char str string
        bytes byte_buf unit unit_struct seq tuple tuple_struct map struct
        identifier ignored_any
    }
}

/// Streams the elements of a binary array into a [`SeqAccess`], deserializing
/// each element natively (no intermediate `Value` for the array structure).
struct SeqStream<'a, 'de> {
    de: &'a mut BinaryDeserializer<'de>,
    frame: Frame,
    produced: usize,
}

impl<'a, 'de> SeqStream<'a, 'de> {
    fn new(de: &'a mut BinaryDeserializer<'de>, frame: Frame) -> Self {
        Self {
            de,
            frame,
            produced: 0,
        }
    }

    /// Whether all elements have been produced: by declared count when the
    /// marker carried one, else by reaching the payload's end offset.
    fn finished(&self) -> bool {
        match self.frame.count {
            Some(count) => self.produced >= count,
            None => self.de.reader.pos >= self.frame.end,
        }
    }
}

impl<'de> SeqAccess<'de> for SeqStream<'_, 'de> {
    type Error = BinaryError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished() {
            return Ok(None);
        }
        let value = seed.deserialize(&mut *self.de)?;
        self.produced += 1;
        if self.de.reader.pos > self.frame.end {
            return Err(BinaryError::InvalidLength {
                detail: "array element extends past the array's declared length",
            });
        }
        Ok(Some(value))
    }

    fn size_hint(&self) -> Option<usize> {
        self.frame
            .count
            .map(|count| count.saturating_sub(self.produced))
    }
}

/// Streams the members of a binary object into a [`MapAccess`], deserializing
/// each key and value natively.
struct MapStream<'a, 'de> {
    de: &'a mut BinaryDeserializer<'de>,
    frame: Frame,
    produced: usize,
}

impl<'a, 'de> MapStream<'a, 'de> {
    fn new(de: &'a mut BinaryDeserializer<'de>, frame: Frame) -> Self {
        Self {
            de,
            frame,
            produced: 0,
        }
    }

    fn finished(&self) -> bool {
        match self.frame.count {
            Some(count) => self.produced >= count,
            None => self.de.reader.pos >= self.frame.end,
        }
    }
}

impl<'de> MapAccess<'de> for MapStream<'_, 'de> {
    type Error = BinaryError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.finished() {
            return Ok(None);
        }
        // The property name is a string value; the seed (a field identifier or
        // `String`) consumes it through the normal scalar path.
        let key = seed.deserialize(&mut *self.de)?;
        Ok(Some(key))
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        let value = seed.deserialize(&mut *self.de)?;
        self.produced += 1;
        if self.de.reader.pos > self.frame.end {
            return Err(BinaryError::InvalidLength {
                detail: "object member extends past the object's declared length",
            });
        }
        Ok(value)
    }

    fn size_hint(&self) -> Option<usize> {
        self.frame
            .count
            .map(|count| count.saturating_sub(self.produced))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::{decode, to_vec};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    /// Round-trips a JSON value through `to_vec` → `from_slice::<Value>` and
    /// asserts equality, plus parity with the reference `decode`.
    fn assert_round_trip(value: serde_json::Value) {
        let bytes = to_vec(&value).unwrap();
        let via_native: serde_json::Value = from_slice(&bytes).unwrap();
        assert_eq!(
            via_native, value,
            "native from_slice mismatch for {value:?}"
        );
        assert_eq!(
            via_native,
            decode(&bytes).unwrap(),
            "from_slice must equal decode for {value:?}"
        );
    }

    #[test]
    fn scalars_round_trip() {
        assert_round_trip(json!(null));
        assert_round_trip(json!(true));
        assert_round_trip(json!(false));
        assert_round_trip(json!(0));
        assert_round_trip(json!(31));
        assert_round_trip(json!(32));
        assert_round_trip(json!(-1));
        assert_round_trip(json!(i64::MAX));
        assert_round_trip(json!(u64::MAX));
        assert_round_trip(json!(1.5));
        assert_round_trip(json!("hello"));
        assert_round_trip(json!(""));
        assert_round_trip(json!("x".repeat(100)));
    }

    #[test]
    fn containers_round_trip() {
        assert_round_trip(json!([]));
        assert_round_trip(json!([1, 2, 3]));
        assert_round_trip(json!({}));
        assert_round_trip(json!({ "id": "doc-1", "count": 42, "nested": { "ok": true } }));
        assert_round_trip(json!({ "a": [1, { "b": [true, null, "x"] }], "c": 3.5 }));
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Product {
        id: String,
        count: u64,
        tags: Vec<String>,
        in_stock: bool,
    }

    #[test]
    fn typed_struct_round_trips() {
        let product = Product {
            id: "p1".to_owned(),
            count: 7,
            tags: vec!["a".to_owned(), "b".to_owned()],
            in_stock: true,
        };
        let bytes = to_vec(&product).unwrap();
        let decoded: Product = from_slice(&bytes).unwrap();
        assert_eq!(decoded, product);
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct WithOption {
        id: String,
        note: Option<String>,
    }

    #[test]
    fn option_fields_round_trip() {
        for note in [Some("hi".to_owned()), None] {
            let value = WithOption {
                id: "x".to_owned(),
                note,
            };
            let bytes = to_vec(&value).unwrap();
            let decoded: WithOption = from_slice(&bytes).unwrap();
            assert_eq!(decoded, value);
        }
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Shape {
        Unit,
        Newtype(u32),
        Tuple(u8, u8),
        Struct { width: u32, height: u32 },
    }

    #[test]
    fn enum_variants_round_trip() {
        for shape in [
            Shape::Unit,
            Shape::Newtype(5),
            Shape::Tuple(1, 2),
            Shape::Struct {
                width: 3,
                height: 4,
            },
        ] {
            let bytes = to_vec(&shape).unwrap();
            let decoded: Shape = from_slice(&bytes).unwrap();
            assert_eq!(decoded, shape);
        }
    }

    #[test]
    fn hash_map_round_trips() {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        map.insert("alpha".to_owned(), 1u32);
        map.insert("beta".to_owned(), 2u32);
        let bytes = to_vec(&map).unwrap();
        let decoded: BTreeMap<String, u32> = from_slice(&bytes).unwrap();
        assert_eq!(decoded, map);
    }

    #[test]
    fn trailing_bytes_are_rejected() {
        let mut bytes = to_vec(&json!(true)).unwrap();
        bytes.push(0x00);
        let result: Result<serde_json::Value> = from_slice(&bytes);
        assert!(matches!(
            result,
            Err(BinaryError::TrailingBytes { remaining: 1 })
        ));
    }

    #[test]
    fn missing_preamble_is_rejected() {
        let result: Result<serde_json::Value> = from_slice(b"{}");
        assert!(matches!(result, Err(BinaryError::MissingPreamble { .. })));
    }

    /// A tiny deterministic LCG so the generative parity test needs no external
    /// RNG dependency and reproduces the same values on every run.
    struct Lcg(u64);

    impl Lcg {
        fn next_u64(&mut self) -> u64 {
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.0
        }

        fn below(&mut self, n: u64) -> u64 {
            self.next_u64() % n
        }
    }

    /// Builds a random JSON value up to `depth` levels deep, exercising every
    /// scalar form the native fast path handles plus nested containers.
    fn random_value(rng: &mut Lcg, depth: u32) -> serde_json::Value {
        let arms = if depth == 0 { 6 } else { 8 };
        match rng.below(arms) {
            0 => serde_json::Value::Null,
            1 => json!(rng.next_u64().is_multiple_of(2)),
            2 => json!(rng.below(64) as i64),
            3 => json!((rng.next_u64() as i64).wrapping_sub(i64::MAX / 2)),
            4 => json!((rng.next_u64() as f64) / 7.0),
            5 => {
                let len = rng.below(80) as usize;
                json!("s".repeat(len))
            }
            6 => {
                let n = rng.below(5) as usize;
                let items: Vec<_> = (0..n).map(|_| random_value(rng, depth - 1)).collect();
                serde_json::Value::Array(items)
            }
            _ => {
                let n = rng.below(5) as usize;
                let mut map = serde_json::Map::new();
                for i in 0..n {
                    map.insert(format!("k{i}"), random_value(rng, depth - 1));
                }
                serde_json::Value::Object(map)
            }
        }
    }

    #[test]
    fn generative_parity_from_slice_matches_decode() {
        // Property: for any value, the native `from_slice::<Value>` equals the
        // reference `decode`, and both equal the original.
        let mut rng = Lcg(0x0f1e_2d3c_4b5a_6978);
        for _ in 0..2_000 {
            let value = random_value(&mut rng, 4);
            let bytes = to_vec(&value).unwrap();
            let native: serde_json::Value = from_slice(&bytes).unwrap();
            assert_eq!(native, decode(&bytes).unwrap(), "parity for {value:?}");
            assert_eq!(native, value, "round-trip for {value:?}");
        }
    }

    #[test]
    fn exotic_forms_decode_via_fallback() {
        // A binary blob (`Binary1ByteLength`) is not one of the native fast-path
        // forms, so it must route through the `Value` fallback and yield the
        // same base64 string the reference decoder produces.
        // cSpell:ignore AQID
        let bytes = [
            crate::binary_json::PREAMBLE,
            crate::binary_json::markers::BINARY_1BYTE_LENGTH,
            4,
            0xDE,
            0xAD,
            0xBE,
            0xEF,
        ];
        let native: serde_json::Value = from_slice(&bytes).unwrap();
        assert_eq!(native, json!("3q2+7w=="));
        assert_eq!(native, decode(&bytes).unwrap());
    }

    #[test]
    fn corpus_vectors_deserialize_natively() {
        // Every golden scalar vector `decode` accepts must also deserialize
        // through the native path, yielding the identical value.
        for vector in crate::binary_json::vectors::SCALAR_VECTORS {
            let native: serde_json::Value = from_slice(vector.binary)
                .unwrap_or_else(|e| panic!("from_slice failed for vector {}: {e}", vector.name));
            assert_eq!(
                native,
                decode(vector.binary).unwrap(),
                "from_slice must equal decode for vector {}",
                vector.name
            );
        }
    }
}
