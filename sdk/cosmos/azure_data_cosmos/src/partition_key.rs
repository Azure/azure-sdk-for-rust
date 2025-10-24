// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;

use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};

use crate::constants;

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
#[derive(Debug, Clone)]
pub struct PartitionKey(Vec<PartitionKeyValue>);

impl PartitionKey {
    /// A single null partition key value, which can be used as the sole partition key or as part of a hierarchical partition key.
    pub const NULL: PartitionKeyValue = PartitionKeyValue(InnerPartitionKeyValue::Null);

    /// An empty list of partition key values, which is used to signal a cross-partition query, when querying a container.
    pub const EMPTY: PartitionKey = PartitionKey(Vec::new());

    #[cfg_attr(not(feature = "preview_query_engine"), allow(dead_code))]
    pub(crate) fn is_empty(&self) -> bool {
        self.0.is_empty()
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
                    json.push_str(
                        serde_json::to_string(&serde_json::Value::Number(num.clone()))?.as_str(),
                    );
                }
            }

            json.push(',');
        }

        // Pop the trailing ','
        json.pop();
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

// We don't want to expose the implementation details of PartitionKeyValue (specifically the use of serde_json::Number), so we use this inner private enum to store the data.
#[derive(Debug, Clone, PartialEq, Eq)]
enum InnerPartitionKeyValue {
    Null,
    String(Cow<'static, str>),
    Number(serde_json::Number), // serde_json::Number has special integer handling, so we'll use that.
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
        InnerPartitionKeyValue::String(value.clone()).into()
    }
}

macro_rules! impl_from_number {
    ($source_type: ty) => {
        impl From<$source_type> for PartitionKeyValue {
            fn from(value: $source_type) -> Self {
                InnerPartitionKeyValue::Number(serde_json::Number::from(value)).into()
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
        InnerPartitionKeyValue::Number(
            serde_json::Number::from_f64(value as f64)
                .expect("value should be a non-infinite number"),
        )
        .into()
    }
}

impl From<f64> for PartitionKeyValue {
    /// Creates a [`PartitionKeyValue`] from an `f64`.
    ///
    /// # Panics
    ///
    /// This method panics if given an Infinite or NaN value.
    fn from(value: f64) -> Self {
        InnerPartitionKeyValue::Number(
            serde_json::Number::from_f64(value).expect("value should be a non-infinite number"),
        )
        .into()
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
    pub fn null_value() {
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
}
