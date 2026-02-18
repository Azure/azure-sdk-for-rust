// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};

use crate::constants;
use crate::hash::{get_hashed_partition_key_string, EffectivePartitionKey, InnerPartitionKeyValue};
use crate::models::PartitionKeyKind;

/// Specifies a partition key value, usually used when querying a specific partition.
///
/// # Specifying a partition key
///
/// Most APIs that require a partition key will accept `impl Into<PartitionKey>`, giving you a few options on how to specify your partition key.
///
/// A single, non-hierarchical, partition key can be specified using the underlying type itself:
///
/// ```rust,no_run
/// # use azure_data_cosmos::clients::ContainerClient;
/// # let container_client: ContainerClient = panic!("this is a non-running example");
/// container_client.query_items::<serde_json::Value>(
///     "SELECT * FROM c",
///     "a single string partition key",
///     None).unwrap();
/// container_client.query_items::<serde_json::Value>(
///     "SELECT * FROM c",
///     42, // A numeric partition key
///     None).unwrap();
/// ```
///
/// Hierarchical partition keys can be specified using tuples:
///
/// ```rust,no_run
/// # use azure_data_cosmos::clients::ContainerClient;
/// # let container_client: ContainerClient = panic!("this is a non-running example");
/// container_client.query_items::<serde_json::Value>(
///     "SELECT * FROM c",
///     ("parent", "child"),
///     None).unwrap();
/// ```
///
/// Null values can be represented in one of two ways.
/// First, you can use the value [`PartitionKey::NULL`]:
///
/// ```rust,no_run
/// # use azure_data_cosmos::{clients::ContainerClient, PartitionKey};
/// # let container_client: ContainerClient = panic!("this is a non-running example");
/// container_client.query_items::<serde_json::Value>(
///     "SELECT * FROM c",
///     PartitionKey::NULL,
///     None).unwrap();
/// container_client.query_items::<serde_json::Value>(
///     "SELECT * FROM c",
///     ("a", PartitionKey::NULL, "b"), // A null value within a hierarchical partition key.
///     None).unwrap();
/// ```
///
/// Undefined partition key values can be represented using [`PartitionKey::UNDEFINED`].
/// This is used to refer to items where the partition key property is absent from the document.
/// This is distinct from `null` (where the property exists but has a JSON null value).
///
/// ```rust,no_run
/// # use azure_data_cosmos::{clients::ContainerClient, PartitionKey};
/// # let container_client: ContainerClient = panic!("this is a non-running example");
/// # async {
/// container_client.read_item::<serde_json::Value>(
///     PartitionKey::UNDEFINED,
///     "item_without_partition_key_property",
///     None).await.unwrap();
/// # };
/// ```
///
/// Or, if you have an [`Option<T>`], for some `T` that is valid as a partition key, it will automatically be serialized as `null` if it has the value [`Option::None`]:
///
/// ```rust,no_run
/// # use azure_data_cosmos::clients::ContainerClient;
/// # let container_client: ContainerClient = panic!("this is a non-running example");
/// let my_partition_key: Option<String> = None;
/// container_client.query_items::<serde_json::Value>(
///     "SELECT * FROM c",
///     my_partition_key,
///     None).unwrap();
/// ```
///
/// If you want to create your [`PartitionKey`] and store it in a variable, use [`PartitionKey::from()`]
///
/// ```rust
/// # use azure_data_cosmos::PartitionKey;
/// let partition_key_1 = PartitionKey::from("simple_string");
/// let partition_key_2 = PartitionKey::from(("parent", "child", 42));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionKey(Vec<PartitionKeyValue>);

impl PartitionKey {
    /// A single null partition key value, which can be used as the sole partition key or as part of a hierarchical partition key.
    pub const NULL: PartitionKeyValue = PartitionKeyValue(InnerPartitionKeyValue::Null);

    /// A single undefined partition key value, used to target items where the partition key property is absent from the document.
    ///
    /// This is distinct from [`PartitionKey::NULL`], which targets items where the partition key property exists but has a JSON `null` value.
    /// An undefined value is serialized as `{}` (an empty JSON object) in the partition key header.
    /// For example, a single `UNDEFINED` value serializes to `[{}]`.
    pub const UNDEFINED: PartitionKeyValue = PartitionKeyValue(InnerPartitionKeyValue::Undefined);

    /// An empty list of partition key values, which is used to signal a cross-partition query, when querying a container.
    pub const EMPTY: PartitionKey = PartitionKey(Vec::new());

    #[allow(dead_code)]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns a hex string representation of the partition key hash.
    ///
    /// # Arguments
    /// * `kind` - The partition key kind (Hash or MultiHash)
    /// * `version` - The hash version (1 or 2)
    ///
    /// # Returns
    /// An `EffectivePartitionKey` representing the hashed partition key
    pub fn get_hashed_partition_key_string(
        &self,
        kind: PartitionKeyKind,
        version: u8,
    ) -> EffectivePartitionKey {
        let inner_values: Vec<&InnerPartitionKeyValue> = self.0.iter().map(|v| &v.0).collect();
        get_hashed_partition_key_string(&inner_values, kind, version)
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
                constants::QUERY_ENABLE_CROSS_PARTITION,
                HeaderValue::from_static("True"),
            )));
        }

        let mut json = String::new();
        let mut utf_buf = [0; 2]; // A buffer for encoding UTF-16 characters.
        json.push('[');
        for key in &self.0 {
            match key.0 {
                InnerPartitionKeyValue::Undefined => json.push_str("{}"),
                InnerPartitionKeyValue::Null => json.push_str("null"),
                InnerPartitionKeyValue::Bool(b) => json.push_str(if b { "true" } else { "false" }),
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
                            '\\' => json.push('\\'),
                            c if c.is_ascii() => json.push(c),
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
                InnerPartitionKeyValue::Number(ref num) => {
                    json.push_str(&num.to_string());
                }
                InnerPartitionKeyValue::Infinity => json.push_str("\"Infinity\""),
                InnerPartitionKeyValue::Undefined => json.push_str("undefined"),
            }

            json.push(',');
        }

        // Pop the trailing ',' (only if we actually wrote any values)
        if json.ends_with(',') {
            json.pop();
        }
        json.push(']');

        Ok(std::iter::once((
            constants::PARTITION_KEY,
            HeaderValue::from_cow(json),
        )))
    }
}

/// Represents a value for a single partition key.
///
/// You shouldn't need to construct this type directly. The various implementations of [`Into<PartitionKey>`] will handle it for you.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartitionKeyValue(InnerPartitionKeyValue);

impl From<InnerPartitionKeyValue> for PartitionKeyValue {
    fn from(value: InnerPartitionKeyValue) -> Self {
        PartitionKeyValue(value)
    }
}

impl From<&'static str> for PartitionKeyValue {
    fn from(value: &'static str) -> Self {
        InnerPartitionKeyValue::String(value.to_string()).into()
    }
}

impl From<String> for PartitionKeyValue {
    fn from(value: String) -> Self {
        InnerPartitionKeyValue::String(value).into()
    }
}

impl From<&String> for PartitionKeyValue {
    fn from(value: &String) -> Self {
        InnerPartitionKeyValue::String(value.clone()).into()
    }
}

impl From<Cow<'static, str>> for PartitionKeyValue {
    fn from(value: Cow<'static, str>) -> Self {
        InnerPartitionKeyValue::String(value.into_owned()).into()
    }
}

macro_rules! impl_from_number {
    ($source_type: ty) => {
        impl From<$source_type> for PartitionKeyValue {
            fn from(value: $source_type) -> Self {
                InnerPartitionKeyValue::Number(value as f64).into()
            }
        }
    };
}

impl_from_number!(i16);
impl_from_number!(i32);
impl_from_number!(i64);
impl_from_number!(i8);
impl_from_number!(isize);
impl_from_number!(u16);
impl_from_number!(u32);
impl_from_number!(u64);
impl_from_number!(u8);
impl_from_number!(usize);

impl From<f32> for PartitionKeyValue {
    /// Creates a [`PartitionKeyValue`] from an `f32`.
    ///
    /// WARNING: This extends the precision of the value from `f32` to `f64`.
    ///
    /// # Panics
    ///
    /// This method panics if given an Infinite or NaN value.
    fn from(value: f32) -> Self {
        assert!(
            !value.is_infinite() && !value.is_nan(),
            "value should be a non-infinite number"
        );
        InnerPartitionKeyValue::Number(value as f64).into()
    }
}

impl From<f64> for PartitionKeyValue {
    /// Creates a [`PartitionKeyValue`] from an `f64`.
    ///
    /// # Panics
    ///
    /// This method panics if given an Infinite or NaN value.
    fn from(value: f64) -> Self {
        assert!(
            !value.is_infinite() && !value.is_nan(),
            "value should be a non-infinite number"
        );
        InnerPartitionKeyValue::Number(value).into()
    }
}

impl<T: Into<PartitionKeyValue>> From<Option<T>> for PartitionKeyValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => t.into(),
            None => InnerPartitionKeyValue::Null.into(),
        }
    }
}

impl From<()> for PartitionKey {
    fn from(_: ()) -> Self {
        PartitionKey::EMPTY
    }
}

impl From<Vec<PartitionKeyValue>> for PartitionKey {
    /// Creates a [`PartitionKey`] from a vector of [`PartitionKeyValue`]s.
    ///
    /// This is useful when the partition key structure is determined at runtime,
    /// such as when working with multiple containers with different schemas or
    /// building partition keys from configuration.
    ///
    /// # Panics
    ///
    /// Panics if the vector contains more than 3 elements, as Cosmos DB supports
    /// a maximum of 3 hierarchical partition key levels.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use azure_data_cosmos::{PartitionKey, PartitionKeyValue};
    ///
    /// // Single-level partition key
    /// let keys = vec![PartitionKeyValue::from("tenant1")];
    /// let partition_key = PartitionKey::from(keys);
    ///
    /// // Multi-level partition key built at runtime
    /// let mut keys = vec![PartitionKeyValue::from("tenant1")];
    /// keys.push(PartitionKeyValue::from("region1"));
    /// let partition_key = PartitionKey::from(keys);
    /// ```
    fn from(values: Vec<PartitionKeyValue>) -> Self {
        assert!(
            values.len() <= 3,
            "Partition keys can have at most 3 levels, got {}",
            values.len()
        );
        PartitionKey(values)
    }
}

impl<T: Into<PartitionKeyValue>> From<T> for PartitionKey {
    fn from(value: T) -> Self {
        PartitionKey(vec![value.into()])
    }
}

macro_rules! impl_from_tuple {
    ($($n:tt $name:ident)*) => {
        impl<$($name: Into<PartitionKeyValue>),*> From<($($name,)*)> for PartitionKey {
            fn from(value: ($($name,)*)) -> Self {
                PartitionKey(vec![$(
                    value.$n.into()
                ),*])
            }
        }
    };
}

// CosmosDB hierarchical partition keys are up to 3 levels:
// https://learn.microsoft.com/en-us/azure/cosmos-db/hierarchical-partition-keys
impl_from_tuple!(0 A 1 B);
impl_from_tuple!(0 A 1 B 2 C);

#[cfg(test)]
mod tests {
    use crate::{constants, PartitionKey, PartitionKeyValue};
    use azure_core::http::headers::AsHeaders;

    fn key_to_string(v: impl Into<PartitionKey>) -> String {
        let key = v.into();
        let mut headers_iter = key.as_headers().unwrap();
        let (name, value) = headers_iter.next().unwrap();
        assert_eq!(constants::PARTITION_KEY, name);
        value.as_str().into()
    }

    /// Validates that a given value is `impl Into<QueryPartitionStrategy>` and works as-expected.
    fn key_to_single_string_partition_key(v: Option<impl Into<PartitionKey>>) -> Option<String> {
        v.map(|k| key_to_string(k))
    }

    #[test]
    pub fn static_str() {
        assert_eq!(key_to_string("my_partition_key"), r#"["my_partition_key"]"#);
        assert_eq!(
            key_to_single_string_partition_key(Some("my_partition_key")).as_deref(),
            Some(r#"["my_partition_key"]"#)
        );
    }

    #[test]
    pub fn integers() {
        assert_eq!(key_to_string(42u8), r#"[42]"#);
        assert_eq!(key_to_string(42u16), r#"[42]"#);
        assert_eq!(key_to_string(42u32), r#"[42]"#);
        assert_eq!(key_to_string(42u64), r#"[42]"#);
        assert_eq!(key_to_string(42usize), r#"[42]"#);
        assert_eq!(key_to_string(42i8), r#"[42]"#);
        assert_eq!(key_to_string(42i16), r#"[42]"#);
        assert_eq!(key_to_string(42i32), r#"[42]"#);
        assert_eq!(key_to_string(42i64), r#"[42]"#);
        assert_eq!(key_to_string(42isize), r#"[42]"#);
    }

    #[test]
    pub fn floats() {
        // The f32 gets up-cast to f64, which results in a rounding issue.
        // It's serde_json's default behavior, so we expect it, even if it isn't ideal.
        assert_eq!(key_to_string(4.2f32), r#"[4.199999809265137]"#);
        assert_eq!(key_to_string(4.2f64), r#"[4.2]"#);
    }

    #[test]
    pub fn options() {
        let some: Option<&str> = Some("my_partition_key");
        let none: Option<&str> = None;
        assert_eq!(key_to_string(some), r#"["my_partition_key"]"#);
        assert_eq!(key_to_string(none), r#"[null]"#);
    }

    #[test]
    fn from_vec_empty() {
        let keys: Vec<PartitionKeyValue> = vec![];
        let partition_key = PartitionKey::from(keys);
        assert_eq!(Vec::<PartitionKeyValue>::new(), partition_key.0);

        let mut headers_iter = partition_key.as_headers().unwrap();
        let (name, value) = headers_iter.next().unwrap();
        assert_eq!(constants::QUERY_ENABLE_CROSS_PARTITION, name);
        assert_eq!("True", value.as_str());
    }

    #[test]
    fn from_vec_single() {
        let keys = vec![PartitionKeyValue::from("tenant1")];
        let partition_key = PartitionKey::from(keys);
        assert_eq!(key_to_string(partition_key), r#"["tenant1"]"#);
    }

    #[test]
    fn from_vec_double() {
        let keys = vec![
            PartitionKeyValue::from("tenant1"),
            PartitionKeyValue::from("region1"),
        ];
        let partition_key = PartitionKey::from(keys);
        assert_eq!(key_to_string(partition_key), r#"["tenant1","region1"]"#);
    }

    #[test]
    fn from_vec_triple() {
        let keys = vec![
            PartitionKeyValue::from("tenant1"),
            PartitionKeyValue::from("region1"),
            PartitionKeyValue::from("user1"),
        ];
        let partition_key = PartitionKey::from(keys);
        assert_eq!(
            key_to_string(partition_key),
            r#"["tenant1","region1","user1"]"#
        );
    }

    #[test]
    fn from_vec_mixed_types() {
        let keys = vec![
            PartitionKeyValue::from("tenant1"),
            PartitionKeyValue::from(42i64),
            PartitionKeyValue::from(123.45f64),
        ];
        let partition_key = PartitionKey::from(keys);
        assert_eq!(key_to_string(partition_key), r#"["tenant1",42,123.45]"#);
    }

    #[test]
    #[should_panic(expected = "Partition keys can have at most 3 levels, got 4")]
    fn from_vec_too_many() {
        let keys = vec![
            PartitionKeyValue::from("a"),
            PartitionKeyValue::from("b"),
            PartitionKeyValue::from("c"),
            PartitionKeyValue::from("d"),
        ];
        let _partition_key = PartitionKey::from(keys);
    }

    #[test]
    fn null_value() {
        assert_eq!(key_to_string(PartitionKey::NULL), r#"[null]"#);
        assert_eq!(
            key_to_string((PartitionKey::NULL, PartitionKey::NULL, PartitionKey::NULL)),
            r#"[null,null,null]"#
        );
    }

    #[test]
    pub fn non_ascii_string() {
        let key = PartitionKey::from("smile 😀");
        assert_eq!(key_to_string(key), r#"["smile \ud83d\ude00"]"#);
    }

    #[test]
    pub fn tuple() {
        assert_eq!(
            key_to_string((42u8, "my_partition_key", PartitionKey::NULL)),
            r#"[42,"my_partition_key",null]"#
        );
    }

    #[test]
    pub fn empty() {
        let partition_key = PartitionKey::from(());
        assert_eq!(Vec::<PartitionKeyValue>::new(), partition_key.0);

        let mut headers_iter = partition_key.as_headers().unwrap();
        let (name, value) = headers_iter.next().unwrap();
        assert_eq!(constants::QUERY_ENABLE_CROSS_PARTITION, name);
        assert_eq!("True", value.as_str());
    }

    /// Helper to get the partition key header value (not cross-partition header).
    fn key_to_pk_header(v: impl Into<PartitionKey>) -> (String, String) {
        let key = v.into();
        let mut headers_iter = key.as_headers().unwrap();
        let (name, value) = headers_iter.next().unwrap();
        (name.as_str().to_string(), value.as_str().to_string())
    }

    #[test]
    fn undefined_single() {
        // A single UNDEFINED value should produce [{}] via the partition key header,
        // where {} is the wire representation of an undefined partition key component.
        let (name, value) = key_to_pk_header(PartitionKey::UNDEFINED);
        assert_eq!(constants::PARTITION_KEY.as_str(), name);
        assert_eq!("[{}]", value);
    }

    #[test]
    fn undefined_all_in_hierarchical() {
        // All UNDEFINED values in a hierarchical key should produce [{},{}].
        let (name, value) = key_to_pk_header((PartitionKey::UNDEFINED, PartitionKey::UNDEFINED));
        assert_eq!(constants::PARTITION_KEY.as_str(), name);
        assert_eq!("[{},{}]", value);
    }

    #[test]
    fn undefined_mixed_with_values() {
        // UNDEFINED values should be serialized as {} in the JSON array.
        assert_eq!(
            key_to_string(("parent", PartitionKey::UNDEFINED)),
            r#"["parent",{}]"#
        );
        assert_eq!(
            key_to_string((PartitionKey::UNDEFINED, "child")),
            r#"[{},"child"]"#
        );
    }

    #[test]
    fn undefined_distinct_from_null() {
        // UNDEFINED produces [{}] while NULL produces [null].
        let (undef_name, undef_value) = key_to_pk_header(PartitionKey::UNDEFINED);
        let null_value = key_to_string(PartitionKey::NULL);
        assert_eq!(constants::PARTITION_KEY.as_str(), undef_name);
        assert_eq!("[{}]", undef_value);
        assert_eq!("[null]", null_value);
    }

    #[test]
    fn undefined_distinct_from_empty() {
        // UNDEFINED sends the partition key header with `[{}]`, while EMPTY sends the cross-partition header.
        let (undef_name, undef_value) = key_to_pk_header(PartitionKey::UNDEFINED);
        assert_eq!(constants::PARTITION_KEY.as_str(), undef_name);
        assert_eq!("[{}]", undef_value);

        let empty = PartitionKey::EMPTY;
        let mut headers_iter = empty.as_headers().unwrap();
        let (empty_name, empty_value) = headers_iter.next().unwrap();
        assert_eq!(constants::QUERY_ENABLE_CROSS_PARTITION, empty_name);
        assert_eq!("True", empty_value.as_str());
    }

    #[test]
    fn undefined_in_vec() {
        let keys = vec![PartitionKeyValue::from("tenant1"), PartitionKey::UNDEFINED];
        let partition_key = PartitionKey::from(keys);
        assert_eq!(key_to_string(partition_key), r#"["tenant1",{}]"#);
    }
}
