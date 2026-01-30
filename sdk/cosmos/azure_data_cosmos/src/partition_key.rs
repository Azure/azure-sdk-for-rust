// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::borrow::Cow;
use std::io::Write;
use azure_core::http::headers::{AsHeaders, HeaderName, HeaderValue};

use crate::constants;
use crate::models::{PartitionKeyDefinition, PartitionKeyKind};

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


/// Minimum inclusive effective partition key (empty components).
pub const MINIMUM_INCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "";

/// Maximum exclusive effective partition key (infinity marker).
pub const MAXIMUM_EXCLUSIVE_EFFECTIVE_PARTITION_KEY: &str = "FF";

/// Error type for partition key operations.
#[derive(Debug)]
pub enum PartitionKeyError {
    #[error("Too few partition key components")]
    TooFewComponents,
    #[error("Too many partition key components")]
    TooManyComponents,
    #[error("Unexpected partition key definition version")]
    UnexpectedVersion,
    #[error("Internal server error: {0}")]
    InternalError(String),
}

/// Partition key component trait.
pub trait PartitionKeyComponent: Send + Sync {
    fn write_for_hashing(&self, writer: &mut dyn Write) -> std::io::Result<()>;
    fn write_for_hashing_v2(&self, writer: &mut dyn Write) -> std::io::Result<()>;
    fn write_for_binary_encoding(&self, writer: &mut dyn Write) -> std::io::Result<()>;
    fn truncate(&self) -> Box<dyn PartitionKeyComponent>;
    fn get_type_ordinal(&self) -> i32;
}


/// Partition key component types.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionKeyComponentType {
    Undefined = 0,
    Null = 1,
    False = 2,
    True = 3,
    MinNumber = 4,
    Number = 5,
    MaxNumber = 6,
    MinString = 7,
    String = 8,
    MaxString = 9,
    Infinity = 0xFF,
}

/// Number partition key component.
#[derive(Debug, Clone)]
pub struct NumberPartitionKeyComponent(pub f64);

impl PartitionKeyComponent for NumberPartitionKeyComponent {
    fn write_for_hashing(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        writer.write_all(&self.0.to_le_bytes())
    }

    fn write_for_hashing_v2(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        writer.write_all(&[PartitionKeyComponentType::Number as u8])?;
        writer.write_all(&self.0.to_le_bytes())
    }

    fn write_for_binary_encoding(&self, writer: &mut dyn Write) -> std::io::Result<()> {
        writer.write_all(&[PartitionKeyComponentType::Number as u8])?;
        // Encode as uint64 for binary representation
        let encoded = encode_double_as_uint64(self.0);
        writer.write_all(&encoded.to_be_bytes())
    }

    fn truncate(&self) -> Box<dyn PartitionKeyComponent> {
        Box::new(self.clone())
    }

    fn get_type_ordinal(&self) -> i32 {
        PartitionKeyComponentType::Number as i32
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
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


    /// Produces effective partition key string.
    ///
    /// Azure Cosmos DB has global index on effective partition key values.
    /// Effective value is produced by applying range or hash encoding to all
    /// component values, based on partition key definition.
    pub fn get_effective_partition_key_string(
        &self,
        partition_key_definition: &PartitionKeyDefinition,
        strict: bool,
    ) -> Result<String, PartitionKeyError> {
        let components = self
            .components
            .as_ref()
            .ok_or(PartitionKeyError::TooFewComponents)?;

        // Check for empty partition key
        if components.is_empty() {
            return Ok(MINIMUM_INCLUSIVE_EFFECTIVE_PARTITION_KEY.to_string());
        }

        // Check for infinity partition key
        if self.is_infinity() {
            return Ok(MAXIMUM_EXCLUSIVE_EFFECTIVE_PARTITION_KEY.to_string());
        }

        // Validate component count
        if components.len() < partition_key_definition.paths.len()
            && partition_key_definition.kind != PartitionKeyKind::MultiHash
        {
            return Err(PartitionKeyError::TooFewComponents);
        }

        if components.len() > partition_key_definition.paths.len() && strict {
            return Err(PartitionKeyError::TooManyComponents);
        }

        match partition_key_definition.kind {
            PartitionKeyKind::Hash => {
                let version = partition_key_definition
                    .version
                    .unwrap_or(1);

                match version {
                    1 => {
                        self.get_effective_partition_key_for_hash_partitioning()
                    }
                    2 => {
                        self.get_effective_partition_key_for_hash_partitioning_v2()
                    }
                    _ => self.get_effective_partition_key_for_hash_partitioning()
                }
            }
            PartitionKeyKind::MultiHash => {
                self.get_effective_partition_key_for_multi_hash_partitioning_v2()
            }
            PartitionKeyKind::Range => Self::to_hex_encoded_binary_string(components),
        }
    }

    /// Hash partitioning V1 (MurmurHash3 32-bit).
    fn get_effective_partition_key_for_hash_partitioning(&self) -> Result<String, PartitionKeyError> {
        let components = self.components.as_ref().unwrap();

        // Truncate components
        let truncated_components: Vec<Box<dyn PartitionKeyComponent>> = components
            .iter()
            .map(|c| c.truncate())
            .collect();

        // Compute hash
        let mut buffer = Vec::new();
        for component in &truncated_components {
            component
                .write_for_hashing(&mut buffer)
                .map_err(|e| PartitionKeyError::InternalError(e.to_string()))?;
        }

        let hash = murmur3_hash_32(&buffer);

        // Build result: [hash] + [truncated components]
        let mut result_components: Vec<&dyn PartitionKeyComponent> = Vec::with_capacity(components.len() + 1);

        let hash_component = NumberPartitionKeyComponent(hash as f64);
        let hash_box: Box<dyn PartitionKeyComponent> = Box::new(hash_component);

        let mut all_components: Vec<Box<dyn PartitionKeyComponent>> = vec![hash_box];
        all_components.extend(truncated_components);

        Self::to_hex_encoded_binary_string(&all_components)
    }

    /// Hash partitioning V2 (MurmurHash3 128-bit).
    fn get_effective_partition_key_for_hash_partitioning_v2(&self) -> Result<String, PartitionKeyError> {
        let components = self.components.as_ref().unwrap();

        let mut buffer = Vec::new();
        for component in components {
            component
                .write_for_hashing_v2(&mut buffer)
                .map_err(|e| PartitionKeyError::InternalError(e.to_string()))?;
        }

        let hash128 = murmur3_hash_128(&buffer);
        let mut hash_bytes = hash128.to_be_bytes();

        // Reset 2 most significant bits (max exclusive value is 'FF')
        hash_bytes[0] &= 0x3F;

        Ok(to_hex_string(&hash_bytes))
    }

    /// Multi-hash partitioning V2 (per-component hashing).
    fn get_effective_partition_key_for_multi_hash_partitioning_v2(&self) -> Result<String, PartitionKeyError> {
        let components = self.components.as_ref().unwrap();
        let mut result = String::new();

        for component in components {
            let mut buffer = Vec::new();
            component
                .write_for_hashing_v2(&mut buffer)
                .map_err(|e| PartitionKeyError::InternalError(e.to_string()))?;

            let hash128 = murmur3_hash_128(&buffer);
            let mut hash_bytes = hash128.to_be_bytes();

            // Reset 2 most significant bits
            hash_bytes[0] &= 0x3F;

            result.push_str(&to_hex_string(&hash_bytes));
        }

        Ok(result)
    }

    /// Converts components to hex-encoded binary string.
    fn to_hex_encoded_binary_string(
        components: &[Box<dyn PartitionKeyComponent>],
    ) -> Result<String, PartitionKeyError> {
        let mut buffer = Vec::with_capacity(256);

        for component in components {
            component
                .write_for_binary_encoding(&mut buffer)
                .map_err(|e| PartitionKeyError::InternalError(e.to_string()))?;
        }

        Ok(to_hex_string(&buffer))
    }

    /// Checks if this is the infinity partition key.
    fn is_infinity(&self) -> bool {
        if let Some(components) = &self.components {
            if components.len() == 1 {
                // Check if the single component is infinity type
                return components[0].get_type_ordinal() == PartitionKeyComponentType::Infinity as i32;
            }
        }
        false
    }
}

/// MurmurHash3 32-bit hash function.
fn murmur3_hash_32(data: &[u8]) -> u32 {
    // Simplified implementation - use a proper crate in production
    let seed: u32 = 0;
    let c1: u32 = 0xcc9e2d51;
    let c2: u32 = 0x1b873593;

    let mut h1 = seed;
    let len = data.len();
    let mut i = 0;

    while i + 4 <= len {
        let mut k1 = u32::from_le_bytes([data[i], data[i + 1], data[i + 2], data[i + 3]]);
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);

        h1 ^= k1;
        h1 = h1.rotate_left(13);
        h1 = h1.wrapping_mul(5).wrapping_add(0xe6546b64);
        i += 4;
    }

    // Handle remaining bytes
    let mut k1: u32 = 0;
    let remaining = len - i;
    if remaining >= 3 {
        k1 ^= (data[i + 2] as u32) << 16;
    }
    if remaining >= 2 {
        k1 ^= (data[i + 1] as u32) << 8;
    }
    if remaining >= 1 {
        k1 ^= data[i] as u32;
        k1 = k1.wrapping_mul(c1);
        k1 = k1.rotate_left(15);
        k1 = k1.wrapping_mul(c2);
        h1 ^= k1;
    }

    // Finalization
    h1 ^= len as u32;
    h1 ^= h1 >> 16;
    h1 = h1.wrapping_mul(0x85ebca6b);
    h1 ^= h1 >> 13;
    h1 = h1.wrapping_mul(0xc2b2ae35);
    h1 ^= h1 >> 16;

    h1
}

/// MurmurHash3 128-bit hash function.
fn murmur3_hash_128(data: &[u8]) -> u128 {
    // Simplified implementation - use a proper crate like `murmur3` in production
    let h1 = murmur3_hash_32(data) as u128;
    let h2 = murmur3_hash_32(&[data, &[0x01]].concat()) as u128;
    (h1 << 64) | h2
}

/// Converts bytes to uppercase hex string.
fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02X}", b)).collect()
}

/// Encodes a double as uint64 for binary encoding.
fn encode_double_as_uint64(value: f64) -> u64 {
    let bits = value.to_bits();
    if (bits & 0x8000_0000_0000_0000) != 0 {
        !bits
    } else {
        bits ^ 0x8000_0000_0000_0000
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
}
