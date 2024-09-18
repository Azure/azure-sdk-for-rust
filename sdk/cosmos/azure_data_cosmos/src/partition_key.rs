/// Specifies a partition key value, usually used when querying a specific partition.
#[derive(Debug, Clone)]
pub struct PartitionKey(Vec<PartitionKeyValue>);

impl PartitionKey {
    pub(crate) fn into_header_value(self) -> azure_core::Result<String> {
        // We have to do some manual JSON serialization here.
        // The partition key is sent in an HTTP header, when used to set the partition key for a query.
        // It's not safe to use non-ASCII characters in HTTP headers, and serde_json will not escape non-ASCII characters if they are otherwise valid as UTF-8.
        // So, we do some conversion by hand, with the help of Rust's own `encode_utf16` method which gives us the necessary code points for non-ASCII values, and produces surrogate pairs as needed.

        let mut json = String::new();
        let mut utf_buf = [0; 2]; // A buffer for encoding UTF-16 characters.
        json.push('[');
        for key in self.0 {
            match key.0 {
                InnerPartitionKeyValue::Null => json.push_str("null"),
                InnerPartitionKeyValue::String(string_key) => {
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
                InnerPartitionKeyValue::Number(num) => {
                    json.push_str(serde_json::to_string(&serde_json::Value::Number(num))?.as_str());
                }
            }

            json.push(',');
        }

        // Pop the trailing ','
        json.pop();
        json.push(']');

        Ok(json)
    }
}

/// Represents a value for a single partition key.
#[derive(Debug, Clone)]
pub struct PartitionKeyValue(InnerPartitionKeyValue);

impl PartitionKeyValue {
    /// A value that represents a 'null' partition key.
    pub const NULL: PartitionKeyValue = PartitionKeyValue(InnerPartitionKeyValue::Null);
}

// We don't want to expose the implementation details of PartitionKeyValue (specifically the use of serde_json::Number), so we use this inner private enum to store the data.
#[derive(Debug, Clone)]
enum InnerPartitionKeyValue {
    Null,
    String(String),
    Number(serde_json::Number), // serde_json::Number has special integer handling, so we'll use that.
}

impl From<InnerPartitionKeyValue> for PartitionKeyValue {
    fn from(value: InnerPartitionKeyValue) -> Self {
        PartitionKeyValue(value)
    }
}

impl From<&str> for PartitionKeyValue {
    fn from(value: &str) -> Self {
        InnerPartitionKeyValue::String(value.into()).into()
    }
}

impl From<String> for PartitionKeyValue {
    fn from(value: String) -> Self {
        InnerPartitionKeyValue::String(value).into()
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

impl<T: Into<PartitionKeyValue>> From<T> for PartitionKey {
    fn from(value: T) -> Self {
        PartitionKey(vec![value.into()])
    }
}

impl<T: Into<PartitionKeyValue>> From<Vec<T>> for PartitionKey {
    fn from(value: Vec<T>) -> Self {
        PartitionKey(value.into_iter().map(Into::into).collect())
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

impl_from_tuple!(0 A);
impl_from_tuple!(0 A 1 B);
impl_from_tuple!(0 A 1 B 2 C);
impl_from_tuple!(0 A 1 B 2 C 3 D);
impl_from_tuple!(0 A 1 B 2 C 3 D 4 E);
impl_from_tuple!(0 A 1 B 2 C 3 D 4 E 5 F);
impl_from_tuple!(0 A 1 B 2 C 3 D 4 E 5 F 6 G);
impl_from_tuple!(0 A 1 B 2 C 3 D 4 E 5 F 6 G 7 H);

#[cfg(test)]
mod tests {
    use crate::{PartitionKey, PartitionKeyValue};

    fn key_to_string(v: impl Into<PartitionKey>) -> String {
        v.into().into_header_value().unwrap()
    }

    #[test]
    pub fn static_str() {
        assert_eq!(key_to_string("my_partition_key"), r#"["my_partition_key"]"#);
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
        assert_eq!(key_to_string(PartitionKeyValue::NULL), r#"[null]"#);
    }

    #[test]
    pub fn non_ascii_string() {
        let key = PartitionKey::from("smile 😀");
        assert_eq!(
            key.into_header_value().unwrap().as_str(),
            r#"["smile \ud83d\ude00"]"#
        );
    }

    #[test]
    pub fn tuple() {
        assert_eq!(
            key_to_string((42u8, "my_partition_key", 4.2f64, PartitionKeyValue::NULL)),
            r#"[42,"my_partition_key",4.2,null]"#
        );
    }
}
