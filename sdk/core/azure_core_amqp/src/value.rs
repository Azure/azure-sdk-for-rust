// Copyright (c) Microsoft Corporation. All Rights reserved
// Licensed under the MIT license.

use azure_core::Uuid;
use std::borrow::Borrow;

#[cfg(feature = "ffi")]
use crate::error::Result;
#[cfg(feature = "ffi")]
use crate::fe2o3::error::Fe2o3SerializationError;
#[cfg(all(feature = "ffi", feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
#[cfg(feature = "ffi")]
use crate::{Deserializable, Serializable};
use std::time::SystemTime;

/// An AMQP symbol.
///
/// Symbols are used to identify a type of data. They are similar to strings, and represent symbolic values from a constrained domain.
#[derive(Debug, PartialEq, Clone, Default, Eq)]
pub struct AmqpSymbol(pub String);

impl PartialEq<AmqpSymbol> for str {
    fn eq(&self, other: &AmqpSymbol) -> bool {
        self == other.0.as_str()
    }
}

impl PartialEq<&str> for AmqpSymbol {
    fn eq(&self, other: &&str) -> bool {
        self.0 == *other
    }
}
impl PartialEq<AmqpSymbol> for &str {
    fn eq(&self, other: &AmqpSymbol) -> bool {
        *self == other.0
    }
}

impl PartialEq<&AmqpSymbol> for AmqpSymbol {
    fn eq(&self, other: &&AmqpSymbol) -> bool {
        self.0 == other.0
    }
}

impl PartialEq<AmqpSymbol> for &AmqpSymbol {
    fn eq(&self, other: &AmqpSymbol) -> bool {
        self.0 == other.0
    }
}

impl From<String> for AmqpSymbol {
    fn from(s: String) -> Self {
        AmqpSymbol(s)
    }
}

impl From<AmqpSymbol> for String {
    fn from(s: AmqpSymbol) -> Self {
        s.0
    }
}

impl From<&AmqpSymbol> for String {
    fn from(s: &AmqpSymbol) -> Self {
        s.0.clone()
    }
}

impl From<&str> for AmqpSymbol {
    fn from(s: &str) -> Self {
        AmqpSymbol(s.to_string())
    }
}

impl Borrow<str> for AmqpSymbol {
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

/// A sequence of AMQP values
#[derive(Debug, PartialEq, Clone, Default)]
pub struct AmqpList(pub Vec<AmqpValue>);

impl AmqpList {
    /// Creates a new AMQP List.
    pub fn new() -> Self {
        Self(Vec::new())
    }

    /// Creates a new AMQP List with the specified capacity.
    pub fn with_capacity(size: usize) -> Self {
        Self(Vec::with_capacity(size))
    }

    /// Returns the number of elements in the list.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the list contains no elements.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Appends an element to the back of the list.
    pub fn push(&mut self, value: AmqpValue) {
        self.0.push(value);
    }

    /// Returns an iterator over the elements of the list.
    pub fn iter(&self) -> impl Iterator<Item = &AmqpValue> {
        self.0.iter()
    }
}

impl From<Vec<AmqpValue>> for AmqpList {
    fn from(v: Vec<AmqpValue>) -> Self {
        AmqpList(v)
    }
}

/// AMQP Timestamp.
///
/// Represents a point in time.
///
/// Note: Internally, AMQP times are represented as signed milliseconds since
/// the UNIX epoch (1-1-1970).
///
/// They can also contain the value of -62_135_596_800_000 which is
/// January 1, 0001 represented as the number of milliseconds *BEFORE* the UNIX_EPOCH.
///
/// This time cannot be expressed as a `SystemTime`, so it is represented as `None`.
#[derive(Debug, PartialEq, Clone)]
pub struct AmqpTimestamp(pub Option<SystemTime>);

impl From<SystemTime> for AmqpTimestamp {
    fn from(v: SystemTime) -> Self {
        AmqpTimestamp(Some(v))
    }
}

/// An ordered mapping from distinct keys to values.
///
/// This is a simple implementation of a map that is backed by a vector.
/// It is not intended to be used for large maps, but rather for small maps where the order of the keys is important.
#[derive(Debug, PartialEq, Clone, Default)]
pub struct AmqpOrderedMap<K, V>
where
    K: PartialEq,
    V: Clone,
{
    inner: Vec<(K, V)>,
}

/// The descriptor of a described AMQP type.
///
/// The descriptor for an AMQP composite type. See the [AMQP specification](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-types-v1.0-os.html#doc-idp108672) for more information.
#[derive(Debug, PartialEq, Clone)]
pub enum AmqpDescriptor {
    /// A numeric code that identifies the type.
    Code(u64),

    /// A symbolic name that identifies the type.
    Name(AmqpSymbol),
}

impl From<u64> for AmqpDescriptor {
    fn from(v: u64) -> Self {
        AmqpDescriptor::Code(v)
    }
}

impl<T> From<T> for AmqpDescriptor
where
    T: Into<AmqpSymbol>,
{
    fn from(v: T) -> Self {
        AmqpDescriptor::Name(v.into())
    }
}

/// An AMQP Composite type.
///
/// This is a complex type that is composed of a descriptor and a value.
/// See the [AMQP specification](https://docs.oasis-open.org/amqp/core/v1.0/os/amqp-core-types-v1.0-os.html#doc-idp108672) for more information.
#[derive(Debug, PartialEq, Clone)]
pub struct AmqpDescribed {
    /// The descriptor of the described type.
    pub descriptor: AmqpDescriptor,

    /// The value of the described type.
    pub value: AmqpValue,
}

impl AmqpDescribed {
    /// Creates a new AMQP Described type from descriptor and value.
    pub fn new(descriptor: impl Into<AmqpDescriptor>, value: impl Into<AmqpValue>) -> Self {
        Self {
            descriptor: descriptor.into(),
            value: value.into(),
        }
    }
}

/// An AMQP Composite type.
///
/// This is a complex type that is composed of a descriptor and a value.
/// The descriptor is used to identify the type of the value.
/// The value is the actual value.
#[derive(Debug, PartialEq, Clone)]
#[cfg(feature = "ffi")]
pub struct AmqpComposite {
    descriptor: AmqpDescriptor,
    value: AmqpList,
}

#[cfg(feature = "ffi")]
impl AmqpComposite {
    /// Creates a new AMQP Composite type.
    ///
    /// # Arguments
    ///
    /// * `descriptor` - The descriptor of the composite type.
    /// * `value` - The value of the composite type.
    pub fn new(descriptor: impl Into<AmqpDescriptor>, value: impl Into<AmqpList>) -> Self {
        Self {
            descriptor: descriptor.into(),
            value: value.into(),
        }
    }

    /// Returns a reference to the descriptor.
    pub fn descriptor(&self) -> &AmqpDescriptor {
        &self.descriptor
    }

    /// Returns a reference to the value.
    pub fn value(&self) -> &AmqpList {
        &self.value
    }

    /// Returns a mutable reference to the value.
    pub fn value_mut(&mut self) -> &mut AmqpList {
        &mut self.value
    }
}

/// An AMQP value.
#[derive(Debug, PartialEq, Clone, Default)]
pub enum AmqpValue {
    /// A null value.
    #[default]
    Null,
    /// A boolean (true/false) value.
    Boolean(bool),
    /// An unsigned byte value.
    UByte(u8),
    /// An unsigned short value.
    UShort(u16),
    /// An unsigned integer value.
    UInt(u32),
    /// An unsigned long value.
    ULong(u64),
    /// A signed byte value.
    Byte(i8),
    /// A signed short value.
    Short(i16),
    /// A signed integer value.
    Int(i32),
    /// A signed long value.
    Long(i64),
    /// A 32-bit floating point value.
    Float(f32),
    /// A 64-bit floating point value.
    Double(f64),
    /// A single Unicode character.
    Char(char),
    /// A point in time.
    TimeStamp(AmqpTimestamp),
    /// A universally unique identifier.
    Uuid(Uuid),
    /// A sequence of octets.
    Binary(Vec<u8>),
    /// A sequence of Unicode characters.
    String(String),
    /// An AMQP Symbol.
    Symbol(AmqpSymbol),

    /// An IEEE 754-2008 Decimal128 value.
    Decimal128([u8; 16]),
    /// An IEEE 754-2008 Decimal64 value.
    Decimal64([u8; 8]),
    /// An IEEE 754-2008 Decimal32 value.
    Decimal32([u8; 4]),

    /// An ordered list of AMQP values.
    List(AmqpList),
    /// An ordered map of AMQP values.
    Map(AmqpOrderedMap<AmqpValue, AmqpValue>),
    /// An array of AMQP values.
    Array(Vec<AmqpValue>),
    /// A described value.
    Described(Box<AmqpDescribed>),

    /// An AMQP composite value.
    #[cfg(feature = "ffi")]
    Composite(Box<AmqpComposite>),
}

#[cfg(feature = "ffi")]
impl Serializable for AmqpValue {
    fn encoded_size(&self) -> Result<usize> {
        #[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
        {
            use crate::AmqpError;

            let fe2o3_value = fe2o3_amqp_types::primitives::Value::from(self.clone());
            serde_amqp::serialized_size(&fe2o3_value)
                .map_err(|e| AmqpError::from(Fe2o3SerializationError(e)))
        }
        #[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
        {
            unimplemented!("Serialization of AMQP values is not supported")
        }
    }

    #[allow(unused_variables)]
    fn serialize(&self, buffer: &mut [u8]) -> Result<()> {
        #[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
        {
            let fe2o3_value = fe2o3_amqp_types::primitives::Value::from(self.clone());
            let vec = serde_amqp::to_vec(&fe2o3_value).map_err(|e| {
                azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
            })?;
            let bytes = vec.as_slice();
            buffer.copy_from_slice(bytes);
            Ok(())
        }
        #[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
        {
            unimplemented!("Serialization of AMQP values is not supported")
        }
    }
}

#[cfg(feature = "ffi")]
impl Deserializable<AmqpValue> for AmqpValue {
    #[allow(unused_variables)]
    fn decode(data: &[u8]) -> Result<AmqpValue> {
        #[cfg(all(feature = "fe2o3_amqp", not(target_arch = "wasm32")))]
        {
            let fe2o3_value: fe2o3_amqp_types::primitives::Value = serde_amqp::from_slice(data)
                .map_err(|e| {
                    azure_core::Error::new(azure_core::error::ErrorKind::DataConversion, e)
                })?;
            Ok(fe2o3_value.into())
        }
        #[cfg(any(not(feature = "fe2o3_amqp"), target_arch = "wasm32"))]
        {
            unimplemented!("Deserialization of AMQP values is not supported")
        }
    }
}

impl<K, V> AmqpOrderedMap<K, V>
where
    K: PartialEq + Clone,
    V: Clone,
{
    /// Creates a new, empty `AmqpOrderedMap`.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Inserts a key-value pair into the map.
    pub fn insert(&mut self, key: K, value: V) {
        self.inner.push((key, value));
    }

    /// Gets a reference to the value corresponding to the key.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: Borrow<Q>,
        Q: PartialEq<K> + ?Sized,
    {
        self.inner
            .iter()
            .find_map(|(k, v)| if key.eq(k) { Some(v) } else { None })
    }

    /// Returns the number of elements in the map.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns true if the map contains no elements.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Removes a key from the map, returning the value at the key if the key was previously in the map.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: Borrow<Q>,
        Q: PartialEq<K> + ?Sized,
    {
        let index = self.inner.iter().position(|(k, _)| key.eq(k))?;
        Some(self.inner.remove(index).1)
    }

    /// Returns true if the map contains a value for the specified key.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: Borrow<Q>,
        Q: PartialEq<K> + ?Sized,
    {
        self.inner.iter().any(|(k, _)| key.eq(k))
    }

    /// Returns an iterator over the key-value pairs in the map.
    /// This iterator yields tuples of references to the keys and values.
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> + '_ {
        self.inner.iter().map(|(k, v)| (k, v))
    }
}

impl<K, V> IntoIterator for AmqpOrderedMap<K, V>
where
    K: PartialEq,
    V: Clone,
{
    type Item = (K, V);
    type IntoIter = <Vec<(K, V)> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

macro_rules! conversions_for_amqp_types {
    ($(($t:ty, $field:ident)),*) => {
        $(
            impl From<$t> for AmqpValue {
                fn from(v: $t) -> Self {
                    AmqpValue::$field(v)
                }
            }

            impl From<AmqpValue> for $t {
                fn from(v: AmqpValue) -> Self {
                    match v {
                        AmqpValue::$field(v) => v,
                        _ => panic!("Expected a {}", stringify!($t)),
                    }
                }
            }
            impl From<&AmqpValue> for $t {
                fn from(v: &AmqpValue) -> Self {
                    match v {
                        AmqpValue::$field(v) => v.clone(),
                        _ => panic!("Expected a {}", stringify!($t)),
                    }
                }
            }

            impl PartialEq<$t> for AmqpValue {
                fn eq(&self, other: &$t) -> bool {
                    match self {
                        AmqpValue::$field(v) => v == other,
                        _ => false,
                    }
                }
            }
            impl PartialEq<AmqpValue> for $t {
                fn eq(&self, other: &AmqpValue) -> bool {
                    match other {
                        AmqpValue::$field(v) => self == v,
                        _ => false,
                    }
                }
            }
        )*
    }
}

conversions_for_amqp_types!(
    (bool, Boolean),
    (u8, UByte),
    (u16, UShort),
    (u32, UInt),
    (u64, ULong),
    (i8, Byte),
    (i16, Short),
    (i32, Int),
    (i64, Long),
    (f32, Float),
    (f64, Double),
    (char, Char),
    (Uuid, Uuid),
    (Vec<u8>, Binary),
    (std::string::String, String),
    (AmqpSymbol, Symbol),
    (AmqpList, List),
    (Vec<AmqpValue>, Array),
    (AmqpOrderedMap<AmqpValue, AmqpValue>, Map),
    (AmqpTimestamp, TimeStamp),
    (Box<AmqpDescribed>, Described)
);

impl From<()> for AmqpValue {
    fn from(_: ()) -> Self {
        AmqpValue::Null
    }
}

impl From<AmqpValue> for () {
    fn from(v: AmqpValue) -> Self {
        match v {
            AmqpValue::Null => (),
            _ => panic!("Expected a null value"),
        }
    }
}

impl PartialEq<()> for AmqpValue {
    fn eq(&self, _: &()) -> bool {
        matches!(self, AmqpValue::Null)
    }
}

impl PartialEq<AmqpValue> for () {
    fn eq(&self, other: &AmqpValue) -> bool {
        other == self
    }
}

impl From<Box<AmqpDescribed>> for AmqpDescribed {
    fn from(b: Box<AmqpDescribed>) -> Self {
        *b
    }
}

impl From<&str> for AmqpValue {
    fn from(b: &str) -> Self {
        AmqpValue::String(b.to_string())
    }
}

impl From<Box<AmqpValue>> for AmqpValue {
    fn from(b: Box<AmqpValue>) -> Self {
        *b
    }
}

impl<K, V> From<Vec<(K, V)>> for AmqpOrderedMap<K, V>
where
    K: PartialEq,
    V: Clone,
{
    fn from(v: Vec<(K, V)>) -> Self {
        AmqpOrderedMap {
            inner: v.into_iter().collect(),
        }
    }
}

impl<K, V> FromIterator<(K, V)> for AmqpOrderedMap<K, V>
where
    K: PartialEq,
    V: Clone,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        AmqpOrderedMap {
            inner: iter.into_iter().collect(),
        }
    }
}

impl<V> FromIterator<V> for AmqpList
where
    V: Into<AmqpValue>,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        AmqpList(iter.into_iter().map(|v| v.into()).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::SystemTime;
    use std::vec;

    #[test]
    fn test_value_create_specific() {
        let uuid = Uuid::new_v4();
        let timestamp = SystemTime::now();
        let v1 = AmqpValue::Boolean(true);
        let v2 = AmqpValue::UByte(1);
        let v3 = AmqpValue::UShort(2);
        let v4 = AmqpValue::UInt(3);
        let v5 = AmqpValue::ULong(4);
        let v6 = AmqpValue::Byte(5);
        let v7 = AmqpValue::Short(6);
        let v8 = AmqpValue::Int(7);
        let v9 = AmqpValue::Long(8);
        let v10 = AmqpValue::Float(9.0);
        let v11 = AmqpValue::Double(10.0);
        let v12 = AmqpValue::Char('a');
        let v13 = AmqpValue::TimeStamp(AmqpTimestamp(Some(timestamp)));
        let v14 = AmqpValue::Uuid(uuid);
        let v15 = AmqpValue::Binary(vec![1, 2, 3]);
        let v16 = AmqpValue::String("hello".to_string());
        let v17 = AmqpValue::Symbol(AmqpSymbol("hello".to_string()));
        let v18 = AmqpValue::List(AmqpList(vec![AmqpValue::Int(1), AmqpValue::Int(2)]));
        let v19 = AmqpValue::Map(AmqpOrderedMap::new());
        let v20 = AmqpValue::Array(vec![AmqpValue::Int(1), AmqpValue::Int(2)]);
        let v21 = AmqpValue::Described(Box::new(AmqpDescribed {
            descriptor: AmqpDescriptor::Code(23),
            value: AmqpValue::Int(2),
        }));
        let v22 = AmqpValue::Described(Box::new(AmqpDescribed {
            descriptor: AmqpDescriptor::Name(AmqpSymbol("name".to_string())),
            value: AmqpValue::Int(2),
        }));

        assert_eq!(v1, AmqpValue::Boolean(true));
        assert_eq!(v2, AmqpValue::UByte(1));
        assert_eq!(v3, AmqpValue::UShort(2));
        assert_eq!(v4, AmqpValue::UInt(3));
        assert_eq!(v5, AmqpValue::ULong(4));
        assert_eq!(v6, AmqpValue::Byte(5));
        assert_eq!(v7, AmqpValue::Short(6));
        assert_eq!(v8, AmqpValue::Int(7));
        assert_eq!(v9, AmqpValue::Long(8));
        assert_eq!(v10, AmqpValue::Float(9.0));
        assert_eq!(v11, AmqpValue::Double(10.0));
        assert_eq!(v12, AmqpValue::Char('a'));
        assert_eq!(v13, AmqpValue::TimeStamp(AmqpTimestamp(Some(timestamp))));
        assert_eq!(v14, AmqpValue::Uuid(uuid));
        assert_eq!(v15, AmqpValue::Binary(vec![1, 2, 3]));
        assert_eq!(v16, AmqpValue::String("hello".to_string()));
        assert_eq!(v17, AmqpValue::Symbol(AmqpSymbol("hello".to_string())));
        assert_eq!(
            v18,
            AmqpValue::List(AmqpList(vec![AmqpValue::Int(1), AmqpValue::Int(2)]))
        );
        assert_eq!(v19, AmqpValue::Map(AmqpOrderedMap::new()));
        assert_eq!(
            v20,
            AmqpValue::Array(vec![AmqpValue::Int(1), AmqpValue::Int(2)])
        );
        assert_eq!(
            v21,
            AmqpValue::Described(Box::new(AmqpDescribed {
                descriptor: AmqpDescriptor::Code(23),
                value: AmqpValue::Int(2)
            }))
        );
        assert_eq!(
            v22,
            AmqpValue::Described(Box::new(AmqpDescribed {
                descriptor: AmqpDescriptor::Name("name".to_string().into()),
                value: AmqpValue::Int(2)
            }))
        );
    }

    /// Simple conversion tests for the AmqpValue enum
    /// This macro generates a test for each conversion from a specific type to AmqpValue and back
    /// The test checks that the conversion is correct in both directions
    /// The macro also generates a test for the conversion from the unit type to AmqpValue and back
    macro_rules! test_conversion {
        ($t:ty, $field:ident, $value:expr) => {
            let saved_value = $value;
            let v: AmqpValue = saved_value.clone().into();
            assert_eq!(v, AmqpValue::$field(saved_value.clone()));
            assert_eq!(AmqpValue::$field(saved_value.clone()), v);
            let b: $t = v.into();
            assert_eq!(b, saved_value);
        };
        () => {};
    }

    #[test]
    fn test_value_implicit_conversions() {
        test_conversion!(bool, Boolean, true);
        test_conversion!(u8, UByte, 1u8);
        test_conversion!(u16, UShort, 2u16);
        test_conversion!(u32, UInt, 3u32);
        test_conversion!(u64, ULong, 4u64);
        test_conversion!(i8, Byte, 5i8);
        test_conversion!(i16, Short, 6i16);
        test_conversion!(i32, Int, 7i32);
        test_conversion!(i64, Long, 8i64);
        test_conversion!(f32, Float, 9.0f32);
        test_conversion!(f64, Double, 10.0f64);
        test_conversion!(char, Char, 'a');
        test_conversion!(
            AmqpTimestamp,
            TimeStamp,
            AmqpTimestamp(Some(SystemTime::now()))
        );
        test_conversion!(Uuid, Uuid, Uuid::new_v4());
        test_conversion!(Vec<u8>, Binary, vec![1, 2, 3]);
        test_conversion!(String, String, "hello".to_string());
        test_conversion!(AmqpSymbol, Symbol, AmqpSymbol("hello".to_string()));
        test_conversion!(
            AmqpList,
            List,
            AmqpList(vec![AmqpValue::Int(1), AmqpValue::Float(2.75f32)])
        );
        test_conversion!(
            Vec<AmqpValue>,
            Array,
            vec![AmqpValue::Int(1), AmqpValue::Int(2)]
        );
        test_conversion!(
            AmqpOrderedMap<AmqpValue, AmqpValue>,
            Map,
            AmqpOrderedMap::new()
        );

        {
            let described = AmqpDescribed::new(23, 2u32);
            let v: AmqpValue = AmqpValue::Described(Box::new(described.clone()));
            assert_eq!(v, AmqpValue::Described(Box::new(described.clone())));
            assert_eq!(AmqpValue::Described(Box::new(described.clone())), v);
            let b: Box<AmqpDescribed> = v.into();
            assert_eq!(*b, described);
        }

        {
            let v: AmqpValue = AmqpValue::Null;
            assert_eq!(v, AmqpValue::Null);
            assert_eq!(AmqpValue::Null, v);
            let _: () = v.into();
        }
    }

    #[allow(clippy::approx_constant)]
    #[test]
    fn test_amqp_ordered_map() {
        let mut map = AmqpOrderedMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        assert_eq!(map.get(&"key1"), Some(&1));
        assert_eq!(map.get(&"key2"), Some(&2));
        assert_eq!(map.get(&"key3"), Some(&3));
        assert_eq!(map.get(&"key4"), None);

        assert_eq!(map.remove(&"key1"), Some(1));
        assert_eq!(map.remove(&"key1"), None);
        assert_eq!(map.get(&"key1"), None);
    }

    #[allow(clippy::approx_constant)]
    #[test]
    fn test_amqp_value() {
        // Test AmqpValue::Null
        let null_value: AmqpValue = AmqpValue::Null;
        assert_eq!(null_value, AmqpValue::Null);
        assert_eq!(AmqpValue::Null, null_value);
        let _: () = null_value.into();

        // Test AmqpValue::Boolean
        let bool_value: AmqpValue = AmqpValue::Boolean(true);
        assert_eq!(bool_value, AmqpValue::Boolean(true));
        assert_eq!(AmqpValue::Boolean(true), bool_value);
        let bool_val: bool = bool_value.into();
        assert!(bool_val);

        // Test AmqpValue::UByte
        let ubyte_value: AmqpValue = AmqpValue::UByte(255);
        assert_eq!(ubyte_value, AmqpValue::UByte(255));
        assert_eq!(AmqpValue::UByte(255), ubyte_value);
        let ubyte_val: u8 = ubyte_value.into();
        assert_eq!(ubyte_val, 255);

        // Test AmqpValue::UShort
        let ushort_value: AmqpValue = AmqpValue::UShort(65535);
        assert_eq!(ushort_value, AmqpValue::UShort(65535));
        assert_eq!(AmqpValue::UShort(65535), ushort_value);
        let ushort_val: u16 = ushort_value.into();
        assert_eq!(ushort_val, 65535);

        // Test AmqpValue::UInt
        let uint_value: AmqpValue = AmqpValue::UInt(4294967295);
        assert_eq!(uint_value, AmqpValue::UInt(4294967295));
        assert_eq!(AmqpValue::UInt(4294967295), uint_value);
        let uint_val: u32 = uint_value.into();
        assert_eq!(uint_val, 4294967295);

        // Test AmqpValue::ULong
        let ulong_value: AmqpValue = AmqpValue::ULong(18446744073709551615);
        assert_eq!(ulong_value, AmqpValue::ULong(18446744073709551615));
        assert_eq!(AmqpValue::ULong(18446744073709551615), ulong_value);
        let ulong_val: u64 = ulong_value.into();
        assert_eq!(ulong_val, 18446744073709551615);

        // Test AmqpValue::Byte
        let byte_value: AmqpValue = AmqpValue::Byte(-128);
        assert_eq!(byte_value, AmqpValue::Byte(-128));
        assert_eq!(AmqpValue::Byte(-128), byte_value);
        let byte_val: i8 = byte_value.into();
        assert_eq!(byte_val, -128);

        // Test AmqpValue::Short
        let short_value: AmqpValue = AmqpValue::Short(-32768);
        assert_eq!(short_value, AmqpValue::Short(-32768));
        assert_eq!(AmqpValue::Short(-32768), short_value);
        let short_val: i16 = short_value.into();
        assert_eq!(short_val, -32768);

        // Test AmqpValue::Int
        let int_value: AmqpValue = AmqpValue::Int(-2147483648);
        assert_eq!(int_value, AmqpValue::Int(-2147483648));
        assert_eq!(AmqpValue::Int(-2147483648), int_value);
        let int_val: i32 = int_value.into();
        assert_eq!(int_val, -2147483648);

        // Test AmqpValue::Long
        let long_value: AmqpValue = AmqpValue::Long(-9223372036854775808);
        assert_eq!(long_value, AmqpValue::Long(-9223372036854775808));
        assert_eq!(AmqpValue::Long(-9223372036854775808), long_value);
        let long_val: i64 = long_value.into();
        assert_eq!(long_val, -9223372036854775808);

        // Test AmqpValue::Float
        let float_value: AmqpValue = AmqpValue::Float(3.14);
        assert_eq!(float_value, AmqpValue::Float(3.14));
        assert_eq!(AmqpValue::Float(3.14), float_value);
        let float_val: f32 = float_value.into();
        assert_eq!(float_val, 3.14);

        // Test AmqpValue::Double
        let double_value: AmqpValue = AmqpValue::Double(3.14159);
        assert_eq!(double_value, AmqpValue::Double(3.14159));
        assert_eq!(AmqpValue::Double(3.14159), double_value);
        let double_val: f64 = double_value.into();
        assert_eq!(double_val, 3.14159);

        // Test AmqpValue::Char
        let char_value: AmqpValue = AmqpValue::Char('a');
        assert_eq!(char_value, AmqpValue::Char('a'));
        assert_eq!(AmqpValue::Char('a'), char_value);
        let char_val: char = char_value.into();
        assert_eq!(char_val, 'a');

        // Test AmqpValue::TimeStamp
        let timestamp = SystemTime::now();
        let timestamp_value: AmqpValue = AmqpValue::TimeStamp(AmqpTimestamp(Some(timestamp)));
        assert_eq!(
            timestamp_value,
            AmqpValue::TimeStamp(AmqpTimestamp(Some(timestamp)))
        );
        assert_eq!(
            AmqpValue::TimeStamp(AmqpTimestamp(Some(timestamp))),
            timestamp_value
        );
        let timestamp_val: AmqpTimestamp = timestamp_value.into();
        assert_eq!(timestamp_val, AmqpTimestamp(Some(timestamp)));

        // Test AmqpValue::Uuid
        let uuid = Uuid::new_v4();
        let uuid_value: AmqpValue = AmqpValue::Uuid(uuid);
        assert_eq!(uuid_value, AmqpValue::Uuid(uuid));
        assert_eq!(AmqpValue::Uuid(uuid), uuid_value);
        let uuid_val: Uuid = uuid_value.into();
        assert_eq!(uuid_val, uuid);

        // Test AmqpValue::Binary
        let binary_value: AmqpValue = AmqpValue::Binary(vec![1, 2, 3]);
        assert_eq!(binary_value, AmqpValue::Binary(vec![1, 2, 3]));
        assert_eq!(AmqpValue::Binary(vec![1, 2, 3]), binary_value);
        let binary_val: Vec<u8> = binary_value.into();
        assert_eq!(binary_val, vec![1, 2, 3]);

        // Test AmqpValue::String
        let string_value: AmqpValue = AmqpValue::String("hello".to_string());
        assert_eq!(string_value, AmqpValue::String("hello".to_string()));
        assert_eq!(AmqpValue::String("hello".to_string()), string_value);
        let string_val: String = string_value.into();
        assert_eq!(string_val, "hello");

        // Test AmqpValue::Symbol
        let symbol_value: AmqpValue = AmqpValue::Symbol(AmqpSymbol("hello".to_string()));
        assert_eq!(
            symbol_value,
            AmqpValue::Symbol(AmqpSymbol("hello".to_string()))
        );
        assert_eq!(
            AmqpValue::Symbol(AmqpSymbol("hello".to_string())),
            symbol_value
        );
        let symbol_val: AmqpSymbol = symbol_value.into();
        assert_eq!(symbol_val, AmqpSymbol("hello".to_string()));

        // Test AmqpValue::List
        let list_value: AmqpValue =
            AmqpValue::List(AmqpList(vec![AmqpValue::Int(1), AmqpValue::Int(2)]));
        assert_eq!(
            list_value,
            AmqpValue::List(AmqpList(vec![AmqpValue::Int(1), AmqpValue::Int(2)]))
        );
        assert_eq!(
            AmqpValue::List(AmqpList(vec![AmqpValue::Int(1), AmqpValue::Int(2)])),
            list_value
        );
        let list_val: AmqpList = list_value.into();
        assert_eq!(
            list_val,
            AmqpList(vec![AmqpValue::Int(1), AmqpValue::Int(2)])
        );

        // Test AmqpValue::Map
        let map_value: AmqpValue = AmqpValue::Map(AmqpOrderedMap::new());
        assert_eq!(map_value, AmqpValue::Map(AmqpOrderedMap::new()));
        assert_eq!(AmqpValue::Map(AmqpOrderedMap::new()), map_value);
        let map_val: AmqpOrderedMap<AmqpValue, AmqpValue> = map_value.into();
        assert_eq!(map_val, AmqpOrderedMap::new());

        // Test AmqpValue::Array
        let array_value: AmqpValue = AmqpValue::Array(vec![AmqpValue::Int(1), AmqpValue::Int(2)]);
        assert_eq!(
            array_value,
            AmqpValue::Array(vec![AmqpValue::Int(1), AmqpValue::Int(2)])
        );
        assert_eq!(
            AmqpValue::Array(vec![AmqpValue::Int(1), AmqpValue::Int(2)]),
            array_value
        );
        let array_val: Vec<AmqpValue> = array_value.into();
        assert_eq!(array_val, vec![AmqpValue::Int(1), AmqpValue::Int(2)]);

        // Test AmqpValue::Described
        let described_value: AmqpValue = AmqpValue::Described(Box::new(AmqpDescribed {
            descriptor: AmqpDescriptor::Code(23),
            value: AmqpValue::Int(2),
        }));
        assert_eq!(
            described_value,
            AmqpValue::Described(Box::new(AmqpDescribed {
                descriptor: AmqpDescriptor::Code(23),
                value: AmqpValue::Int(2),
            }))
        );
        assert_eq!(
            AmqpValue::Described(Box::new(AmqpDescribed {
                descriptor: AmqpDescriptor::Code(23),
                value: AmqpValue::Int(2),
            })),
            described_value
        );
        let described_val: Box<AmqpDescribed> = described_value.into();
        assert_eq!(*described_val, AmqpDescribed::new(23, 2i32));
    }

    #[test]
    #[cfg(feature = "ffi")]
    fn amqp_composite() {
        let composite =
            AmqpComposite::new(0x270, AmqpList::from(vec![AmqpValue::from("String value")]));
        assert_eq!(composite.descriptor(), &AmqpDescriptor::Code(0x270));
        assert_eq!(
            composite.value(),
            &AmqpList::from(vec![AmqpValue::from("String value")])
        );
    }
}
