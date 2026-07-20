// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Native `serde` serializer for Cosmos binary JSON (`T` → `binary`).
//!
//! [`to_vec`] drives a value's own [`Serialize`] implementation straight into
//! binary bytes, without building an intermediate [`serde_json::Value`]. It
//! produces the same bytes as [`encode`](super::encode) does for the equivalent
//! [`serde_json::Value`].
//!
//! # Enum representation
//!
//! Enums use serde's externally tagged convention (matching `serde_json`): a
//! unit variant serializes as its name string, and newtype, tuple, and struct
//! variants serialize as a single-key object `{ "Variant": <payload> }`.

use serde::{ser, Serialize};

use super::writer::{
    encode_container, encode_f64, encode_i64, encode_string, encode_u64, ARRAY_LC_MARKERS,
    OBJECT_LC_MARKERS,
};
use super::{
    markers::{FALSE, NULL, TRUE},
    BinaryError, Result, PREAMBLE,
};

/// Serializes a value into a complete Cosmos binary JSON buffer.
///
/// The returned buffer begins with the [`PREAMBLE`] byte (`0x80`) and can be
/// round-tripped back through [`decode`](super::decode). This is the native
/// serde entry point mirroring [`serde_json::to_vec`]; it produces the same
/// bytes as `encode(&serde_json::to_value(value)?)` without building the
/// intermediate [`serde_json::Value`].
///
/// # Errors
///
/// Returns [`BinaryError::Custom`] if the value's
/// [`Serialize`] implementation fails.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::binary_json::{decode, to_vec};
///
/// let value = serde_json::json!({ "id": "1", "count": 7 });
/// let bytes = to_vec(&value).unwrap();
/// assert_eq!(decode(&bytes).unwrap(), value);
/// ```
pub fn to_vec<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>> {
    let mut out = vec![PREAMBLE];
    value.serialize(BinarySerializer { out: &mut out })?;
    Ok(out)
}

/// Serializes a single value (its type marker and payload) into `out`.
///
/// Scalars append directly; compound types delegate to a [`ContainerBuilder`]
/// that buffers children and frames them on `end`.
struct BinarySerializer<'a> {
    out: &'a mut Vec<u8>,
}

impl<'a> ser::Serializer for BinarySerializer<'a> {
    type Ok = ();
    type Error = BinaryError;

    type SerializeSeq = ContainerBuilder<'a>;
    type SerializeTuple = ContainerBuilder<'a>;
    type SerializeTupleStruct = ContainerBuilder<'a>;
    type SerializeTupleVariant = ContainerBuilder<'a>;
    type SerializeMap = ContainerBuilder<'a>;
    type SerializeStruct = ContainerBuilder<'a>;
    type SerializeStructVariant = ContainerBuilder<'a>;

    fn serialize_bool(self, v: bool) -> Result<()> {
        self.out.push(if v { TRUE } else { FALSE });
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_i64(i64::from(v))
    }

    fn serialize_i64(self, v: i64) -> Result<()> {
        encode_i64(v, self.out);
        Ok(())
    }

    fn serialize_i128(self, v: i128) -> Result<()> {
        // Cosmos binary JSON has no 128-bit integer; encode through i64 when it
        // fits (matching serde_json) and fail otherwise instead of truncating.
        i64::try_from(v)
            .map(|v| self.serialize_i64(v))
            .unwrap_or_else(|_| {
                Err(BinaryError::Custom(format!(
                    "i128 value {v} out of range for Cosmos binary JSON (max i64)"
                )))
            })
    }

    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_u64(u64::from(v))
    }

    fn serialize_u64(self, v: u64) -> Result<()> {
        encode_u64(v, self.out);
        Ok(())
    }

    fn serialize_u128(self, v: u128) -> Result<()> {
        // See `serialize_i128`.
        u64::try_from(v)
            .map(|v| self.serialize_u64(v))
            .unwrap_or_else(|_| {
                Err(BinaryError::Custom(format!(
                    "u128 value {v} out of range for Cosmos binary JSON (max u64)"
                )))
            })
    }

    fn serialize_f32(self, v: f32) -> Result<()> {
        self.serialize_f64(f64::from(v))
    }

    fn serialize_f64(self, v: f64) -> Result<()> {
        encode_f64(v, self.out);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        let mut buf = [0u8; 4];
        self.serialize_str(v.encode_utf8(&mut buf))
    }

    fn serialize_str(self, v: &str) -> Result<()> {
        encode_string(v, self.out);
        Ok(())
    }

    fn serialize_bytes(self, v: &[u8]) -> Result<()> {
        // Mirror serde_json: a byte slice serializes as an array of byte values,
        // keeping `to_vec` identical to `encode(&serde_json::to_value(v))`.
        // Note this makes byte-heavy fields larger than text (each byte becomes
        // a number element); the `Binary*` blob form would shrink them but break
        // that parity invariant, so keep byte-heavy items on the text path.
        use serde::ser::SerializeSeq;
        let mut seq = self.serialize_seq(Some(v.len()))?;
        for byte in v {
            seq.serialize_element(byte)?;
        }
        seq.end()
    }

    fn serialize_none(self) -> Result<()> {
        self.out.push(NULL);
        Ok(())
    }

    fn serialize_some<T: Serialize + ?Sized>(self, value: &T) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_unit(self) -> Result<()> {
        self.out.push(NULL);
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        // Externally tagged: a unit variant is just its name string.
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        // Transparent: a newtype struct serializes as its inner value.
        value.serialize(self)
    }

    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        value: &T,
    ) -> Result<()> {
        // Externally tagged: `{ "Variant": <inner> }`.
        let mut body = Vec::new();
        encode_string(variant, &mut body);
        value.serialize(BinarySerializer { out: &mut body })?;
        encode_container(OBJECT_LC_MARKERS, 1, &body, self.out);
        Ok(())
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<ContainerBuilder<'a>> {
        Ok(ContainerBuilder::new(self.out, ContainerKind::Array))
    }

    fn serialize_tuple(self, _len: usize) -> Result<ContainerBuilder<'a>> {
        Ok(ContainerBuilder::new(self.out, ContainerKind::Array))
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<ContainerBuilder<'a>> {
        Ok(ContainerBuilder::new(self.out, ContainerKind::Array))
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<ContainerBuilder<'a>> {
        // Externally tagged: `{ "Variant": [ ... ] }`. The outer object is
        // framed when the inner array is finished, in `end`.
        Ok(ContainerBuilder::new_variant(
            self.out,
            ContainerKind::Array,
            variant,
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<ContainerBuilder<'a>> {
        Ok(ContainerBuilder::new(self.out, ContainerKind::Object))
    }

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<ContainerBuilder<'a>> {
        Ok(ContainerBuilder::new(self.out, ContainerKind::Object))
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
        _len: usize,
    ) -> Result<ContainerBuilder<'a>> {
        // Externally tagged: `{ "Variant": { ... } }`.
        Ok(ContainerBuilder::new_variant(
            self.out,
            ContainerKind::Object,
            variant,
        ))
    }
}

/// Whether a [`ContainerBuilder`] frames its contents as an array or an object.
enum ContainerKind {
    Array,
    Object,
}

/// Serializes map keys, which must project to a string (a property name).
///
/// Mirrors `serde_json`'s map-key handling: strings, chars, and integers are
/// accepted (integers are stringified); every other type is rejected so a
/// non-string key surfaces at serialization time rather than producing a buffer
/// that only fails to decode later.
struct MapKeySerializer<'a> {
    out: &'a mut Vec<u8>,
}

fn key_must_be_a_string() -> BinaryError {
    BinaryError::Custom("map key must serialize to a string".to_owned())
}

impl ser::Serializer for MapKeySerializer<'_> {
    type Ok = ();
    type Error = BinaryError;

    type SerializeSeq = ser::Impossible<(), BinaryError>;
    type SerializeTuple = ser::Impossible<(), BinaryError>;
    type SerializeTupleStruct = ser::Impossible<(), BinaryError>;
    type SerializeTupleVariant = ser::Impossible<(), BinaryError>;
    type SerializeMap = ser::Impossible<(), BinaryError>;
    type SerializeStruct = ser::Impossible<(), BinaryError>;
    type SerializeStructVariant = ser::Impossible<(), BinaryError>;

    fn serialize_str(self, v: &str) -> Result<()> {
        encode_string(v, self.out);
        Ok(())
    }

    fn serialize_char(self, v: char) -> Result<()> {
        encode_string(v.encode_utf8(&mut [0u8; 4]), self.out);
        Ok(())
    }

    fn serialize_i8(self, v: i8) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_i16(self, v: i16) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_i32(self, v: i32) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_i64(self, v: i64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_i128(self, v: i128) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_u8(self, v: u8) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_u16(self, v: u16) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_u32(self, v: u32) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_u64(self, v: u64) -> Result<()> {
        self.serialize_str(&v.to_string())
    }
    fn serialize_u128(self, v: u128) -> Result<()> {
        self.serialize_str(&v.to_string())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        variant: &'static str,
    ) -> Result<()> {
        self.serialize_str(variant)
    }

    fn serialize_newtype_struct<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_bool(self, _v: bool) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_f32(self, _v: f32) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_f64(self, _v: f64) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_bytes(self, _v: &[u8]) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_none(self) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, _value: &T) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_unit(self) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()> {
        Err(key_must_be_a_string())
    }
    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        Err(key_must_be_a_string())
    }
    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(key_must_be_a_string())
    }
    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(key_must_be_a_string())
    }
    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        Err(key_must_be_a_string())
    }
    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(key_must_be_a_string())
    }
}

/// Buffers a compound value's children, then frames them on `end`.
///
/// Children are serialized into `buffer` (a scratch buffer); `end` emits the
/// appropriate `LC*` marker, byte length, and element count into the parent
/// `out`, then appends the buffer. When `variant` is set, the framed container
/// is itself wrapped in a single-key `{ "Variant": <container> }` object to
/// realize serde's externally-tagged enum representation.
struct ContainerBuilder<'a> {
    out: &'a mut Vec<u8>,
    buffer: Vec<u8>,
    count: usize,
    kind: ContainerKind,
    variant: Option<&'static str>,
}

impl<'a> ContainerBuilder<'a> {
    fn new(out: &'a mut Vec<u8>, kind: ContainerKind) -> Self {
        Self {
            out,
            buffer: Vec::new(),
            count: 0,
            kind,
            variant: None,
        }
    }

    fn new_variant(out: &'a mut Vec<u8>, kind: ContainerKind, variant: &'static str) -> Self {
        Self {
            out,
            buffer: Vec::new(),
            count: 0,
            kind,
            variant: Some(variant),
        }
    }

    /// Serializes `value` into the scratch buffer and bumps the element count.
    fn push_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        value.serialize(BinarySerializer {
            out: &mut self.buffer,
        })?;
        self.count += 1;
        Ok(())
    }

    /// Serializes `key` into the scratch buffer **without** bumping the count.
    /// Object element count tracks key/value *pairs*, so only the value bumps.
    ///
    /// Map keys must project to a string; non-string keys are rejected via
    /// [`MapKeySerializer`] (matching `serde_json`).
    fn push_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<()> {
        key.serialize(MapKeySerializer {
            out: &mut self.buffer,
        })
    }

    /// Serializes a struct field name (a static string) into the scratch buffer.
    fn push_field_name(&mut self, key: &'static str) {
        encode_string(key, &mut self.buffer);
    }

    /// Frames the buffered container into the parent buffer, wrapping it in a
    /// single-key object first when this builder represents an enum variant.
    fn finish(self) -> Result<()> {
        let markers = match self.kind {
            ContainerKind::Array => ARRAY_LC_MARKERS,
            ContainerKind::Object => OBJECT_LC_MARKERS,
        };
        match self.variant {
            None => encode_container(markers, self.count, &self.buffer, self.out),
            Some(variant) => {
                // Build the inner container, then wrap it in `{ variant: inner }`.
                let mut inner = Vec::new();
                encode_container(markers, self.count, &self.buffer, &mut inner);
                let mut wrapper = Vec::new();
                encode_string(variant, &mut wrapper);
                wrapper.extend_from_slice(&inner);
                encode_container(OBJECT_LC_MARKERS, 1, &wrapper, self.out);
            }
        }
        Ok(())
    }
}

impl ser::SerializeSeq for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

impl ser::SerializeTuple for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_element<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

impl ser::SerializeTupleStruct for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

impl ser::SerializeTupleVariant for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

impl ser::SerializeMap for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_key<T: Serialize + ?Sized>(&mut self, key: &T) -> Result<()> {
        self.push_key(key)
    }

    fn serialize_value<T: Serialize + ?Sized>(&mut self, value: &T) -> Result<()> {
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

impl ser::SerializeStruct for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.push_field_name(key);
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

impl ser::SerializeStructVariant for ContainerBuilder<'_> {
    type Ok = ();
    type Error = BinaryError;

    fn serialize_field<T: Serialize + ?Sized>(
        &mut self,
        key: &'static str,
        value: &T,
    ) -> Result<()> {
        self.push_field_name(key);
        self.push_element(value)
    }

    fn end(self) -> Result<()> {
        self.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::binary_json::{decode, encode};
    use serde::{Deserialize, Serialize};
    use serde_json::json;
    use std::collections::BTreeMap;

    /// Asserts that the native serializer produces exactly the same bytes as
    /// the `Value`-based encoder for the given JSON value.
    fn assert_parity(value: serde_json::Value) {
        let native = to_vec(&value).unwrap();
        let via_value = encode(&value);
        assert_eq!(
            native, via_value,
            "native to_vec must match encode(&Value) for {value:?}"
        );
        // And the bytes must round-trip back to the original value.
        assert_eq!(decode(&native).unwrap(), value);
    }

    #[test]
    fn map_with_integer_keys_stringifies_them() {
        // serde_json stringifies integer map keys; the native serializer does
        // the same so the container's property names stay strings on the wire.
        let mut map = BTreeMap::new();
        map.insert(1u32, "a");
        map.insert(2u32, "b");
        let bytes = to_vec(&map).unwrap();
        assert_eq!(decode(&bytes).unwrap(), json!({ "1": "a", "2": "b" }));
    }

    #[test]
    fn map_with_non_string_keys_is_rejected() {
        // A bool key has no string projection and must be rejected at
        // serialization time (matching serde_json), not silently encoded.
        let mut map = BTreeMap::new();
        map.insert(true, 1);
        map.insert(false, 2);
        let err = to_vec(&map).unwrap_err();
        assert!(
            matches!(err, BinaryError::Custom(ref m) if m.contains("map key")),
            "expected a map-key error, got {err:?}"
        );
    }

    #[test]
    fn parity_scalars() {
        assert_parity(json!(null));
        assert_parity(json!(true));
        assert_parity(json!(false));
        assert_parity(json!(0));
        assert_parity(json!(31));
        assert_parity(json!(32));
        assert_parity(json!(-1));
        assert_parity(json!(i64::MAX));
        assert_parity(json!(u64::MAX));
        assert_parity(json!(1.5));
        assert_parity(json!("hello"));
        assert_parity(json!(""));
    }

    #[test]
    fn parity_long_string() {
        // Exercise the length-prefixed string forms beyond the encoded-length
        // range (< 64 bytes).
        assert_parity(json!("x".repeat(100)));
        assert_parity(json!("y".repeat(70_000)));
    }

    #[test]
    fn parity_arrays_and_objects() {
        assert_parity(json!([]));
        assert_parity(json!([1, 2, 3]));
        assert_parity(json!({}));
        assert_parity(json!({ "id": "doc-1", "count": 42, "nested": { "ok": true } }));
        assert_parity(json!({ "a": [1, { "b": [true, null, "x"] }], "c": 3.5 }));
    }

    #[test]
    fn to_vec_begins_with_preamble() {
        let bytes = to_vec(&json!({ "id": "1" })).unwrap();
        assert_eq!(bytes.first(), Some(&PREAMBLE));
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Product {
        id: String,
        count: u64,
        tags: Vec<String>,
    }

    #[test]
    fn typed_struct_round_trips() {
        // The native serializer preserves struct *declaration* order (like
        // `serde_json::to_vec`), whereas `serde_json::to_value` sorts object
        // keys (its `Map` is a `BTreeMap` without `preserve_order`). So the
        // bytes intentionally differ from the `Value` encoder for named
        // structs; the correctness bar is a faithful round-trip.
        let product = Product {
            id: "p1".to_owned(),
            count: 7,
            tags: vec!["a".to_owned(), "b".to_owned()],
        };
        let native = to_vec(&product).unwrap();
        let decoded: Product = serde_json::from_value(decode(&native).unwrap()).unwrap();
        assert_eq!(decoded, product);
    }

    #[test]
    fn typed_struct_preserves_field_declaration_order() {
        // Field order on the wire must be id, count, tags (declaration order),
        // matching `serde_json::to_vec`, not the alphabetized `to_value` order.
        // The decoder normalizes keys into a sorted map, so assert on the raw
        // wire bytes: each field-name string is embedded verbatim, so their
        // byte offsets reflect emission order.
        let product = Product {
            id: "p1".to_owned(),
            count: 7,
            tags: vec![],
        };
        let bytes = to_vec(&product).unwrap();
        let offset = |needle: &str| {
            bytes
                .windows(needle.len())
                .position(|w| w == needle.as_bytes())
                .unwrap_or_else(|| panic!("field name {needle:?} not found in wire bytes"))
        };
        let (id_at, count_at, tags_at) = (offset("id"), offset("count"), offset("tags"));
        assert!(
            id_at < count_at && count_at < tags_at,
            "expected declaration order id < count < tags, got offsets \
             id={id_at}, count={count_at}, tags={tags_at}"
        );
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum Shape {
        Unit,
        Newtype(u32),
        Tuple(u8, u8),
        Struct { width: u32, height: u32 },
    }

    #[test]
    fn enum_variants_round_trip_externally_tagged() {
        // Externally-tagged variants round-trip through decode +
        // serde_json::from_value. (Byte-parity with the `Value` encoder is not
        // asserted for the struct variant, whose named fields keep declaration
        // order rather than the alphabetized `to_value` order.)
        for shape in [
            Shape::Unit,
            Shape::Newtype(5),
            Shape::Tuple(1, 2),
            Shape::Struct {
                width: 3,
                height: 4,
            },
        ] {
            let native = to_vec(&shape).unwrap();
            let decoded: Shape = serde_json::from_value(decode(&native).unwrap()).unwrap();
            assert_eq!(decoded, shape);
        }
    }

    #[test]
    fn unit_variant_serializes_as_name_string() {
        // A unit variant is externally tagged as its bare name string.
        assert_eq!(
            decode(&to_vec(&Shape::Unit).unwrap()).unwrap(),
            json!("Unit")
        );
    }

    #[test]
    fn newtype_variant_serializes_as_tagged_object() {
        // `{ "Newtype": 5 }`.
        assert_eq!(
            decode(&to_vec(&Shape::Newtype(5)).unwrap()).unwrap(),
            json!({ "Newtype": 5 })
        );
    }

    /// A tiny deterministic LCG so the generative parity test needs no external
    /// RNG dependency and reproduces the same values on every run.
    struct Lcg(u64);

    impl Lcg {
        fn next_u64(&mut self) -> u64 {
            // Numerical Recipes LCG constants.
            self.0 = self.0.wrapping_mul(6364136223846793005).wrapping_add(1);
            self.0
        }

        fn below(&mut self, n: u64) -> u64 {
            self.next_u64() % n
        }
    }

    /// Builds a random JSON value up to `depth` levels deep. Object keys are
    /// generated so that both encode paths see identical `serde_json::Value`
    /// input (same key ordering), keeping the byte-parity assertion valid.
    fn random_value(rng: &mut Lcg, depth: u32) -> serde_json::Value {
        // At depth 0 only scalars are produced to bound recursion.
        let arms = if depth == 0 { 6 } else { 8 };
        match rng.below(arms) {
            0 => serde_json::Value::Null,
            1 => json!(rng.next_u64().is_multiple_of(2)),
            2 => json!(rng.below(64) as i64), // hits the literal-int range
            3 => json!((rng.next_u64() as i64).wrapping_sub(i64::MAX / 2)), // wide i64
            4 => json!((rng.next_u64() as f64) / 7.0), // double
            5 => {
                let len = rng.below(80) as usize; // spans encoded-length + StrL1
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
                    // Distinct, deterministic keys; Map orders them for us.
                    map.insert(format!("k{i}"), random_value(rng, depth - 1));
                }
                serde_json::Value::Object(map)
            }
        }
    }

    #[test]
    fn generative_parity_native_matches_value_encoder() {
        // Property: for any `serde_json::Value`, the native serializer emits the
        // exact same bytes as the `Value` encoder, and those bytes round-trip.
        // (Parity holds because both paths observe the same `Value` — identical
        // key ordering — unlike typed structs, which preserve declaration order.)
        let mut rng = Lcg(0x1234_5678_9abc_def0);
        for _ in 0..2_000 {
            let value = random_value(&mut rng, 4);
            let native = to_vec(&value).unwrap();
            assert_eq!(
                native,
                encode(&value),
                "native to_vec diverged from encode(&Value) for {value:?}"
            );
            assert_eq!(
                decode(&native).unwrap(),
                value,
                "round-trip mismatch for {value:?}"
            );
        }
    }
}
