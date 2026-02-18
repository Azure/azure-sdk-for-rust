// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Partition key types for Cosmos DB operations.

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};
use std::{
    borrow::Cow,
    hash::{Hash, Hasher},
};

/// Header name for partition key.
pub(crate) const PARTITION_KEY: HeaderName =
    HeaderName::from_static("x-ms-documentdb-partitionkey");

/// Header name to enable cross-partition queries.
pub(crate) const QUERY_ENABLE_CROSS_PARTITION: HeaderName =
    HeaderName::from_static("x-ms-documentdb-query-enablecrosspartition");

// =============================================================================
// FiniteF64
// =============================================================================

/// A finite f64 value that can be used in hashed collections.
///
/// Guarantees:
/// - No NaN values (construction panics for NaN)
/// - -0.0 and +0.0 are normalized to +0.0 for consistent hashing
///
/// This allows `PartitionKey` to implement `Hash` and `Eq`, which is required
/// for `ItemReference` to be usable in hashed collections.
#[derive(Clone, Copy, Debug)]
struct FiniteF64(f64);

impl FiniteF64 {
    /// Creates a new FiniteF64 from a value.
    ///
    /// # Panics
    ///
    /// Panics if the value is NaN.
    fn new(value: f64) -> Self {
        assert!(
            !value.is_nan(),
            "NaN is not allowed in partition key values"
        );
        // Normalize -0.0 to +0.0 for consistent hashing
        let normalized = if value == 0.0 { 0.0 } else { value };
        Self(normalized)
    }

    /// Returns the underlying f64 value.
    fn value(&self) -> f64 {
        self.0
    }
}

impl PartialEq for FiniteF64 {
    fn eq(&self, other: &Self) -> bool {
        // Safe: no NaN means reflexivity holds
        self.0 == other.0
    }
}

impl Eq for FiniteF64 {}

impl Hash for FiniteF64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Safe: zeros normalized, no NaN → equal values have equal bits
        self.0.to_bits().hash(state)
    }
}

// =============================================================================
// PartitionKeyValue
// =============================================================================

/// Represents a value for a single partition key.
///
/// You shouldn't need to construct this type directly. The various implementations
/// of [`Into<PartitionKey>`] will handle it for you.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub(crate) struct PartitionKeyValue(InnerPartitionKeyValue);

// We don't want to expose the implementation details of PartitionKeyValue, so we use
// this inner private enum to store the data.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum InnerPartitionKeyValue {
    Null,
    String(Cow<'static, str>),
    Number(FiniteF64),
    Bool(bool),
}

impl From<InnerPartitionKeyValue> for PartitionKeyValue {
    fn from(value: InnerPartitionKeyValue) -> Self {
        PartitionKeyValue(value)
    }
}

impl From<&'static str> for PartitionKeyValue {
    fn from(value: &'static str) -> Self {
        InnerPartitionKeyValue::String(Cow::Borrowed(value)).into()
    }
}

impl From<String> for PartitionKeyValue {
    fn from(value: String) -> Self {
        InnerPartitionKeyValue::String(Cow::Owned(value)).into()
    }
}

impl From<&String> for PartitionKeyValue {
    fn from(value: &String) -> Self {
        InnerPartitionKeyValue::String(Cow::Owned(value.clone())).into()
    }
}

impl From<Cow<'static, str>> for PartitionKeyValue {
    fn from(value: Cow<'static, str>) -> Self {
        InnerPartitionKeyValue::String(value).into()
    }
}

macro_rules! impl_from_number {
    ($source_type:ty) => {
        impl From<$source_type> for PartitionKeyValue {
            fn from(value: $source_type) -> Self {
                InnerPartitionKeyValue::Number(FiniteF64::new(value as f64)).into()
            }
        }
    };
}

impl_from_number!(i8);
impl_from_number!(i16);
impl_from_number!(i32);
impl_from_number!(i64);
impl_from_number!(isize);
impl_from_number!(u8);
impl_from_number!(u16);
impl_from_number!(u32);
impl_from_number!(u64);
impl_from_number!(usize);
impl_from_number!(f32);
impl_from_number!(f64);

impl From<bool> for PartitionKeyValue {
    fn from(value: bool) -> Self {
        InnerPartitionKeyValue::Bool(value).into()
    }
}

impl<T: Into<PartitionKeyValue>> From<Option<T>> for PartitionKeyValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => InnerPartitionKeyValue::Null.into(),
        }
    }
}

/// A partition key used to identify the target partition for an operation.
///
/// Supports both single and hierarchical partition keys (HPK).
///
/// # Examples
///
/// Single partition key:
/// ```
/// use azure_data_cosmos_driver::models::PartitionKey;
///
/// let pk = PartitionKey::from("my-partition");
/// let pk_num = PartitionKey::from(42);
/// ```
///
/// Hierarchical partition key (tuple):
/// ```
/// use azure_data_cosmos_driver::models::PartitionKey;
///
/// let pk = PartitionKey::from(("tenant-1", "user-123"));
/// let pk3 = PartitionKey::from(("region", "tenant", 42));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct PartitionKey(Vec<PartitionKeyValue>);

impl Default for PartitionKey {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl PartitionKey {
    /// An empty partition key, used to signal a cross-partition operation.
    pub const EMPTY: PartitionKey = PartitionKey(Vec::new());

    /// Creates a new partition key from a single value.
    pub(crate) fn new(value: impl Into<PartitionKeyValue>) -> Self {
        Self(vec![value.into()])
    }

    /// Returns true if this partition key is empty (cross-partition).
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the number of components in this partition key.
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl AsHeaders for PartitionKey {
    type Error = azure_core::Error;
    type Iter = std::iter::Once<(HeaderName, HeaderValue)>;

    fn as_headers(&self) -> Result<Self::Iter, Self::Error> {
        // We have to do some manual JSON serialization here.
        // The partition key is sent in an HTTP header, when used to set the partition key for a query.
        // It's not safe to use non-ASCII characters in HTTP headers, and serde_json will not escape non-ASCII characters if they are otherwise valid as UTF-8.
        // So, we do some conversion by hand, with the help of Rust's own `encode_utf16` method which gives us the necessary code points for non-ASCII values, and produces surrogate pairs as needed.

        // Quick shortcut for empty partition keys list, which also prevents a bug when we pop the trailing comma for an empty list.
        if self.0.is_empty() {
            // An empty partition key means a cross partition query
            return Ok(std::iter::once((
                QUERY_ENABLE_CROSS_PARTITION,
                HeaderValue::from_static("True"),
            )));
        }

        let mut json = String::new();
        let mut utf_buf = [0; 2]; // A buffer for encoding UTF-16 characters.
        json.push('[');
        for key in &self.0 {
            match &key.0 {
                InnerPartitionKeyValue::Null => json.push_str("null"),
                InnerPartitionKeyValue::String(ref string_key) => {
                    json.push('"');
                    for char in string_key.chars() {
                        match char {
                            '\x08' => json.push_str(r#"\b"#),
                            '\x0c' => json.push_str(r#"\f"#),
                            '\n' => json.push_str(r#"\n"#),
                            '\r' => json.push_str(r#"\r"#),
                            '\t' => json.push_str(r#"\t"#),
                            '"' => json.push_str(r#"\""#),
                            '\\' => json.push_str(r#"\\"#),
                            c if c.is_ascii() && !c.is_control() => json.push(c),
                            c if c.is_ascii() => {
                                // Remaining ASCII control characters (< 0x20) must be \uXXXX-escaped.
                                json.push_str(&format!("\\u{:04x}", c as u32));
                            }
                            c => {
                                let encoded = c.encode_utf16(&mut utf_buf);
                                for code_unit in encoded {
                                    json.push_str(&format!(r#"\u{:04x}"#, code_unit));
                                }
                            }
                        }
                    }
                    json.push('"');
                }
                InnerPartitionKeyValue::Number(num) => {
                    // Format number - integers without decimal, floats with decimal
                    let val = num.value();
                    if val.fract() == 0.0 && val.abs() < (i64::MAX as f64) {
                        json.push_str(&format!("{}", val as i64));
                    } else {
                        json.push_str(&format!("{}", val));
                    }
                }
                InnerPartitionKeyValue::Bool(b) => {
                    json.push_str(if *b { "true" } else { "false" });
                }
            }

            json.push(',');
        }

        // Pop the trailing ','
        json.pop();
        json.push(']');

        Ok(std::iter::once((
            PARTITION_KEY,
            HeaderValue::from_cow(json),
        )))
    }
}

// Single value conversions
impl<T: Into<PartitionKeyValue>> From<T> for PartitionKey {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl From<()> for PartitionKey {
    fn from(_: ()) -> Self {
        PartitionKey::EMPTY
    }
}

impl From<Vec<PartitionKeyValue>> for PartitionKey {
    /// Creates a [`PartitionKey`] from a vector of partition key components.
    ///
    /// This is useful when the partition key structure is determined at runtime,
    /// such as when working with multiple containers with different schemas or
    /// building partition keys from configuration.
    ///
    /// # Panics
    ///
    /// Panics if the vector contains more than 3 elements, as Cosmos DB supports
    /// a maximum of 3 hierarchical partition key levels.
    fn from(values: Vec<PartitionKeyValue>) -> Self {
        assert!(
            values.len() <= 3,
            "Partition keys can have at most 3 levels, got {}",
            values.len()
        );
        PartitionKey(values)
    }
}

// Tuple conversions for hierarchical partition keys
impl<T1, T2> From<(T1, T2)> for PartitionKey
where
    T1: Into<PartitionKeyValue>,
    T2: Into<PartitionKeyValue>,
{
    fn from((v1, v2): (T1, T2)) -> Self {
        Self(vec![v1.into(), v2.into()])
    }
}

impl<T1, T2, T3> From<(T1, T2, T3)> for PartitionKey
where
    T1: Into<PartitionKeyValue>,
    T2: Into<PartitionKeyValue>,
    T3: Into<PartitionKeyValue>,
{
    fn from((v1, v2, v3): (T1, T2, T3)) -> Self {
        Self(vec![v1.into(), v2.into(), v3.into()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_partition_key() {
        let pk = PartitionKey::from("test");
        assert_eq!(pk.len(), 1);
        assert!(!pk.is_empty());
    }

    #[test]
    fn numeric_partition_key() {
        let pk1 = PartitionKey::from(42);
        let pk2 = PartitionKey::from(42i64);
        let pk3 = PartitionKey::from(1.5f64);
        assert_eq!(pk1.len(), 1);
        assert_eq!(pk2.len(), 1);
        assert_eq!(pk3.len(), 1);
    }

    #[test]
    fn hierarchical_partition_key() {
        let pk = PartitionKey::from(("tenant", "user", 42));
        assert_eq!(pk.len(), 3);
    }

    #[test]
    fn empty_partition_key() {
        let pk = PartitionKey::EMPTY;
        assert!(pk.is_empty());
        assert_eq!(pk.len(), 0);
    }

    #[test]
    fn default_is_empty() {
        let pk = PartitionKey::default();
        assert_eq!(pk, PartitionKey::EMPTY);
    }

    #[test]
    fn unit_converts_to_empty() {
        let pk = PartitionKey::from(());
        assert_eq!(pk, PartitionKey::EMPTY);
        assert!(pk.is_empty());
        assert_eq!(pk.len(), 0);
    }

    #[test]
    fn null_partition_key_value() {
        let pk = PartitionKey::from(None::<String>);
        assert_eq!(pk.len(), 1);
    }

    #[test]
    #[should_panic(expected = "at most 3 levels")]
    fn too_many_levels() {
        let values = vec![
            PartitionKeyValue::from("a"),
            PartitionKeyValue::from("b"),
            PartitionKeyValue::from("c"),
            PartitionKeyValue::from("d"),
        ];
        let _pk = PartitionKey::from(values);
    }
}
