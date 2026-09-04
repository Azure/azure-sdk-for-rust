// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Native `serde` deserializer for Cosmos binary JSON (`binary` → `T`).
//!
//! [`from_slice`] drives a target type's own [`Deserialize`](serde::Deserialize)
//! implementation directly off the binary buffer, without building an
//! intermediate [`serde_json::Value`]. Objects, arrays, and plain scalars are
//! streamed straight into the visitor, with plain strings borrowed directly
//! from the buffer. The rarer wire forms (GUID, base64, compressed, and
//! reference strings, binary blobs, and uniform number arrays) are decoded
//! through [`decode`](super::decode) and forwarded to the visitor.

use serde::de::{DeserializeSeed, Deserializer, IntoDeserializer, MapAccess, SeqAccess, Visitor};
use serde::forward_to_deserialize_any;

use super::markers::NULL;
use super::reader::{ContainerHeader, Frame, Reader, ScalarToken};
use super::{is_binary, BinaryError, Result};

/// Maximum container nesting depth, mirroring the reference decoder's
/// [`MAX_DEPTH`](super::reader) so both paths reject the same
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
    ///
    /// The decoded value is normalized first: `Value`'s own deserializer maps a
    /// `Number(f64)` straight to `visit_f64`, so an exotic form (notably a
    /// service-only uniform `Float64` array, `0xF0..`) would otherwise decode
    /// its integral members as floats while the native path decodes them as
    /// integers. It also lets a typed integer sequence (e.g. `Vec<u64>`)
    /// deserialize instead of erroring.
    fn deserialize_via_value<V>(&mut self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        let mut value = self.reader.read_value(self.depth)?;
        super::normalize_integral_floats(&mut value);
        value
            .deserialize_any(visitor)
            .map_err(|e| BinaryError::Custom(e.to_string()))
    }

    /// Deserializes a value into an **integer** target.
    ///
    /// Mirrors [`deserialize_any`](Deserializer::deserialize_any) but coerces an
    /// integral-valued finite `Double` into the integer visitor instead of
    /// rejecting it: the service stores any integer at or beyond `2^53` as a
    /// `Double`, so a wide integer sent exactly is echoed back as one. A
    /// non-integral double still errors via `visit_f64`.
    ///
    /// `signed` routes the double to the matching visitor; routing every
    /// non-negative value through `visit_u64` would break signed targets, which
    /// reject a `u64` above `i64::MAX`.
    fn deserialize_integer<V>(&mut self, visitor: V, signed: bool) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        if self.depth > MAX_DEPTH {
            return Err(BinaryError::DepthLimitExceeded { limit: MAX_DEPTH });
        }

        // A container is never a number; defer to the container path so an
        // integer field receiving an array/object yields the standard type error.
        if let Some(header) = self.reader.read_container_header()? {
            self.depth += 1;
            let result = match header {
                ContainerHeader::Array(frame) => visitor.visit_seq(SeqStream::new(self, frame)),
                ContainerHeader::Object(frame) => visitor.visit_map(MapStream::new(self, frame)),
            };
            self.depth -= 1;
            return result;
        }

        if let Some(token) = self.reader.try_read_native_scalar()? {
            return match token {
                ScalarToken::Null => visitor.visit_unit(),
                ScalarToken::Bool(b) => visitor.visit_bool(b),
                ScalarToken::I64(i) => visitor.visit_i64(i),
                ScalarToken::U64(u) => visitor.visit_u64(u),
                ScalarToken::F64(f) => {
                    if !f.is_finite() {
                        return Err(BinaryError::InvalidNumber {
                            detail: "non-finite double (NaN or infinity)",
                        });
                    }
                    // Integral double coerces to the integer target, routed by
                    // signedness; the cast saturates so `2^63`/`2^64` map back to
                    // `i64::MAX`/`u64::MAX`.
                    //
                    // TODO(cosmos/binary-json): intentionally lossy. An integer
                    // at or beyond `2^53` is sent exactly but persisted as a
                    // double, so the value read back may DIFFER from the value
                    // sent (see `wide_u64_sent_exactly_is_read_back_..`). Revisit
                    // if the backend preserves `UInt64` natively.
                    if f.fract() == 0.0 {
                        // The endpoints are DELIBERATELY inside these inclusive
                        // ranges: `i64::MAX as f64` / `u64::MAX as f64` round up
                        // to `2^63` / `2^64` — the very doubles the service stores
                        // for those maxima — so the saturating cast lands them
                        // back on the sent value. A double strictly beyond the
                        // endpoint falls through to `visit_f64` and errors.
                        if signed {
                            if (i64::MIN as f64..=i64::MAX as f64).contains(&f) {
                                return visitor.visit_i64(f as i64);
                            }
                        } else if (0.0..=u64::MAX as f64).contains(&f) {
                            return visitor.visit_u64(f as u64);
                        }
                    }
                    visitor.visit_f64(f)
                }
                ScalarToken::Str(s) => visitor.visit_borrowed_str(s),
            };
        }

        self.deserialize_via_value(visitor)
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
                ScalarToken::F64(f) => {
                    // Reject non-finite doubles (`NaN`/`±∞`) so the native
                    // deserializer agrees with the reference `decode`, which
                    // maps a non-finite `Double` to `BinaryError::InvalidNumber`
                    // (JSON has no representation for these).
                    if !f.is_finite() {
                        return Err(BinaryError::InvalidNumber {
                            detail: "non-finite double (NaN or infinity)",
                        });
                    }
                    // An integral `Double` decodes as an integer, matching the
                    // service's text rendering. This is the *untyped* path, so
                    // without it a `serde_json::Value` from a binary page holds
                    // `Float(3.0)` where a text page holds `PosInt(3)`, and
                    // `Number`'s `PartialEq` is variant-sensitive. A float
                    // target still gets its `f64`: serde's float visitors accept
                    // an integer visit.
                    //
                    // This also decides `#[serde(untagged)]` variant selection,
                    // which buffers through `deserialize_any`. Deliberate: the
                    // same enum picks the integer variant over a text page, so
                    // the two encodings agree rather than diverging.
                    match super::integral_double(f) {
                        Some(super::IntegralDouble::Unsigned(unsigned)) => {
                            visitor.visit_u64(unsigned)
                        }
                        Some(super::IntegralDouble::Signed(signed)) => visitor.visit_i64(signed),
                        None => visitor.visit_f64(f),
                    }
                }
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
        //
        // Normalized for the same reason as `deserialize_via_value`: `Value`'s
        // deserializer would otherwise hand an integral `Double` in a variant
        // field to `visit_f64`, disagreeing with the native path.
        let mut value = self.reader.read_value(self.depth)?;
        super::normalize_integral_floats(&mut value);
        value
            .into_deserializer()
            .deserialize_enum(name, variants, visitor)
            .map_err(|e| BinaryError::Custom(e.to_string()))
    }

    // Integer targets coerce an integral `Double` into the visitor, routed by
    // signedness (see [`BinaryDeserializer::deserialize_integer`]); other types
    // use the standard `deserialize_any` dispatch below.
    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, true)
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, true)
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, true)
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, true)
    }

    fn deserialize_i128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, true)
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, false)
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, false)
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, false)
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, false)
    }

    fn deserialize_u128<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_integer(visitor, false)
    }

    forward_to_deserialize_any! {
        bool f32 f64 char str string
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

    /// After a count-framed container is fully produced, verify the declared
    /// count consumed exactly the framed byte span. A declared count smaller
    /// than the bytes the container spans would otherwise let the native path
    /// silently under-read and reinterpret the leftover bytes as the parent's
    /// next element/member — the reference decoder rejects this with
    /// `InvalidLength`, so `from_slice` must too. Skipped for `Arr1`/`Obj1`
    /// (`exact_end == false`), whose `end` is a buffer-length sentinel.
    fn validate_exact_end(&self) -> Result<()> {
        if self.frame.exact_end && self.de.reader.pos != self.frame.end {
            return Err(BinaryError::InvalidLength {
                detail: "array declared count does not span its declared length",
            });
        }
        Ok(())
    }
}

impl<'de> SeqAccess<'de> for SeqStream<'_, 'de> {
    type Error = BinaryError;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        if self.finished() {
            self.validate_exact_end()?;
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

    /// See [`SeqStream::validate_exact_end`]: rejects a declared member count
    /// that does not span the object's framed byte length, matching the
    /// reference decoder.
    fn validate_exact_end(&self) -> Result<()> {
        if self.frame.exact_end && self.de.reader.pos != self.frame.end {
            return Err(BinaryError::InvalidLength {
                detail: "object declared count does not span its declared length",
            });
        }
        Ok(())
    }
}

impl<'de> MapAccess<'de> for MapStream<'_, 'de> {
    type Error = BinaryError;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        if self.finished() {
            self.validate_exact_end()?;
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
    use crate::binary_json::{decode, markers, to_vec};
    use serde::{Deserialize, Serialize};
    use serde_json::json;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct Product {
        id: String,
        count: u64,
        tags: Vec<String>,
        in_stock: bool,
    }

    /// Builds a `Double` buffer to simulate the service echo: the service stores
    /// any integer at or beyond `2^53` as an IEEE-754 `Double`, so a wide integer
    /// sent exactly comes back in this form.
    fn double_buffer(value: f64) -> Vec<u8> {
        let mut bytes = vec![crate::binary_json::PREAMBLE, markers::NUMBER_DOUBLE];
        bytes.extend_from_slice(&value.to_le_bytes());
        bytes
    }

    #[test]
    fn wide_u64_echoed_as_double_coerces_back_to_u64() {
        // The service echoes `u64::MAX` as the double `2^64`; the float→int cast
        // saturates it back to `u64::MAX`, so this wide value round-trips.
        let echoed_max = double_buffer(u64::MAX as f64); // 2^64
        let back: u64 = from_slice(&echoed_max).unwrap();
        assert_eq!(back, u64::MAX);

        // `u64::MAX - 1` shares the same double (`2^64`), so it also comes back as
        // `u64::MAX` — a silent precision loss, asserted here so it is explicit.
        let echoed_lossy = double_buffer((u64::MAX - 1) as f64); // also 2^64
        let back_lossy: u64 = from_slice(&echoed_lossy).unwrap();
        assert_ne!(back_lossy, u64::MAX - 1);
        assert_eq!(back_lossy, u64::MAX);

        // A wide value that IS exactly f64-representable survives intact.
        let exact = 1u64 << 60; // exactly representable
        let echoed_exact = double_buffer(exact as f64);
        let back_exact: u64 = from_slice(&echoed_exact).unwrap();
        assert_eq!(back_exact, exact);
    }

    /// `#[serde(untagged)]` buffers through `deserialize_any`, so the
    /// integral-`Double` coercion decides which variant an integral number
    /// selects. Binary must pick the same variant text does, or the two
    /// encodings disagree for the same stored document.
    #[test]
    fn untagged_variant_selection_agrees_between_binary_and_text() {
        #[derive(Deserialize, PartialEq, Debug)]
        #[serde(untagged)]
        enum Value {
            Int(i64),
            Float(f64),
        }

        // Float variant declared second, so this would pass vacuously if the
        // order were reversed.
        let from_binary: Value = from_slice(&double_buffer(3.0)).unwrap();
        let from_text: Value = serde_json::from_str("3").unwrap();
        assert_eq!(from_binary, Value::Int(3));
        assert_eq!(from_binary, from_text);

        // A fractional `Double` still reaches the float variant on both paths.
        let fractional_binary: Value = from_slice(&double_buffer(2.5)).unwrap();
        let fractional_text: Value = serde_json::from_str("2.5").unwrap();
        assert_eq!(fractional_binary, Value::Float(2.5));
        assert_eq!(fractional_binary, fractional_text);
    }

    /// Signed targets must route through `visit_i64`, not `visit_u64` (which a
    /// signed visitor rejects above `i64::MAX`).
    #[test]
    fn echoed_double_coerces_into_signed_targets_across_the_i64_range() {
        // `i64::MAX` is stored as the double `2^63`; the cast saturates back.
        let back: i64 = from_slice(&double_buffer(i64::MAX as f64)).unwrap();
        assert_eq!(back, i64::MAX);

        // `i64::MIN` is exactly representable, so it survives intact.
        let back: i64 = from_slice(&double_buffer(i64::MIN as f64)).unwrap();
        assert_eq!(back, i64::MIN);

        // A wide positive value well inside the signed range round-trips exactly.
        let exact = 1i64 << 60;
        let back: i64 = from_slice(&double_buffer(exact as f64)).unwrap();
        assert_eq!(back, exact);

        // A double beyond the signed range is a genuine type error, not a
        // saturating coercion into `i64::MAX`.
        let too_wide: Result<i64> = from_slice(&double_buffer(u64::MAX as f64));
        assert!(
            too_wide.is_err(),
            "2^64 must not coerce into an i64 field, got {too_wide:?}"
        );

        // A negative double must not coerce into an unsigned field.
        let negative: Result<u64> = from_slice(&double_buffer(-7.0));
        assert!(
            negative.is_err(),
            "a negative double must not coerce into u64, got {negative:?}"
        );
    }

    /// End-to-end wide-`u64` precision boundary: a value above `i64::MAX` is sent
    /// exactly (as a `UInt64` token) but persisted as a `Double`, so the value
    /// read back **differs** from the value sent.
    #[test]
    fn wide_u64_sent_exactly_is_read_back_lossily_after_service_double_conversion() {
        // A u64 above i64::MAX that is NOT exactly representable as f64
        // (`2^63 + 1` rounds to `2^63`).
        let sent: u64 = (i64::MAX as u64) + 2;

        // 1. The encoder sends it exactly: the wire is a full-precision UInt64.
        let sent_wire = to_vec(&sent).unwrap();
        assert_eq!(
            sent_wire[1],
            markers::NUMBER_UINT64,
            "a u64 above i64::MAX must be sent as the exact UInt64 form"
        );
        // The exact UInt64 wire round-trips back to the same value locally.
        let sent_roundtrip: u64 = from_slice(&sent_wire).unwrap();
        assert_eq!(sent_roundtrip, sent);

        // 2. The service echoes the value back in Double form (it cannot persist
        //    a >= 2^53 integer exactly). Simulate that conversion.
        let service_echo = double_buffer(sent as f64);

        // 3. The deserializer coerces the returned double into the u64 field, but
        //    the value differs from what was sent — precision was lost at the
        //    service. Callers must be aware of this for wide integers.
        let received: u64 = from_slice(&service_echo).unwrap();
        assert_ne!(
            received, sent,
            "sent and received must differ: the service stored a lossy double"
        );
        assert_eq!(
            received,
            1u64 << 63,
            "the value collapses to the nearest representable double (2^63)"
        );
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

    /// An integral `Double` coerces into an integer field, but a **fractional**
    /// `Double` is still a genuine type error (never silently truncated).
    #[test]
    fn integral_double_coerces_but_fractional_double_is_rejected() {
        // A fractional double must not coerce into an integer field.
        let bytes = to_vec(&json!(3.5)).unwrap();
        let as_int: Result<u64> = from_slice(&bytes);
        assert!(
            as_int.is_err(),
            "a fractional double must not coerce into u64, got {as_int:?}"
        );

        // An integral double reads cleanly as the unsigned integer.
        let bytes = to_vec(&json!(3.0)).unwrap();
        let as_uint: u64 = from_slice(&bytes).unwrap();
        assert_eq!(as_uint, 3);

        // A negative integral double reads as the signed integer.
        let bytes = to_vec(&json!(-7.0)).unwrap();
        let as_int: i64 = from_slice(&bytes).unwrap();
        assert_eq!(as_int, -7);
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

    #[test]
    fn non_finite_double_is_rejected_like_decode() {
        // A hand-crafted buffer carrying a non-finite `Double` (NaN / +∞).
        // JSON cannot represent these, so the native `from_slice` path must
        // reject them with the same `InvalidNumber` error as the reference
        // `decode`, rather than accepting a `NaN`/`inf` the way an unguarded
        // `visit_f64` would.
        for bits in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            let mut bytes = vec![crate::binary_json::PREAMBLE, markers::NUMBER_DOUBLE];
            bytes.extend_from_slice(&bits.to_le_bytes());

            let decoded = decode(&bytes);
            assert!(
                matches!(decoded, Err(BinaryError::InvalidNumber { .. })),
                "decode must reject non-finite double, got {decoded:?}"
            );

            let native: Result<f64> = from_slice(&bytes);
            assert!(
                matches!(native, Err(BinaryError::InvalidNumber { .. })),
                "from_slice must reject non-finite double, got {native:?}"
            );
        }
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
        // Every golden vector `decode` accepts must also deserialize through the
        // native path, yielding the identical value.
        for vector in crate::binary_json::vectors::golden_vectors() {
            let native: serde_json::Value = from_slice(&vector.binary)
                .unwrap_or_else(|e| panic!("from_slice failed for vector {}: {e}", vector.name));
            assert_eq!(
                native,
                decode(&vector.binary).unwrap(),
                "from_slice must equal decode for vector {}",
                vector.name
            );
        }
    }
}
