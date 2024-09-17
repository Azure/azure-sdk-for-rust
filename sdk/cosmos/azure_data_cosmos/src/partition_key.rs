#[derive(Debug, Clone)]
pub struct PartitionKey(Vec<PartitionKeyValue>); // TODO: A partition key can be any JSON value. Should we use serde_json::Value or define a custom enum?

impl PartitionKey {
    pub(crate) fn into_header_value(self) -> azure_core::Result<String> {
        // We're going to write our JSON manually, because we need to escape non-ASCII characters in strings, because we're putting this value in an HTTP header.
        let mut json = String::new();
        let mut utf_buf = [0; 2]; // A buffer for encoding UTF-16 characters.
        json.push('[');
        for key in self.0 {
            match key.0 {
                serde_json::Value::String(string_key) => {
                    // We have to manually escape the string to ensure non-ASCII characters are escaped.
                    // Rust's escape_default function escapes characters as \u{XXXXX} (allowing a full Unicode code point in the "{}").
                    // However, JSON doesn't support that and requires encoding the UTF-16 code units separately using `\uXXXX`.
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
                x => {
                    json.push_str(serde_json::to_string(&x)?.as_str());
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

#[derive(Debug, Clone)]
pub struct PartitionKeyValue(serde_json::Value);

impl PartitionKeyValue {
    pub const NULL: PartitionKeyValue = PartitionKeyValue(serde_json::Value::Null);
}

pub struct NullPartitionKey;

impl From<NullPartitionKey> for PartitionKeyValue {
    fn from(_: NullPartitionKey) -> Self {
        PartitionKeyValue(serde_json::Value::Null)
    }
}

macro_rules! impl_from_json {
    ($source_type: ty) => {
        impl From<$source_type> for PartitionKeyValue {
            fn from(value: $source_type) -> Self {
                PartitionKeyValue(serde_json::Value::from(value))
            }
        }
    };
}

impl_from_json!(&str);
impl_from_json!(String);
impl_from_json!(bool);
impl_from_json!(f32);
impl_from_json!(f64);
impl_from_json!(i16);
impl_from_json!(i32);
impl_from_json!(i64);
impl_from_json!(i8);
impl_from_json!(isize);
impl_from_json!(u16);
impl_from_json!(u32);
impl_from_json!(u64);
impl_from_json!(u8);
impl_from_json!(usize);
impl_from_json!(serde_json::Value);

impl<T: Into<PartitionKeyValue>> From<Option<T>> for PartitionKeyValue {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(t) => t.into(),
            None => PartitionKeyValue(serde_json::Value::Null),
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
    use crate::{NullPartitionKey, PartitionKey, PartitionKeyValue};

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
    pub fn serde_json_values() {
        assert_eq!(
            key_to_string(serde_json::Value::String("my_partition_key".into())),
            r#"["my_partition_key"]"#
        );
        assert_eq!(
            key_to_string(serde_json::Value::Number(
                serde_json::Number::from_f64(4.2).unwrap()
            )),
            r#"[4.2]"#
        );
        assert_eq!(key_to_string(serde_json::Value::Null), r#"[null]"#);
    }

    #[test]
    pub fn null_value() {
        assert_eq!(key_to_string(PartitionKeyValue::NULL), r#"[null]"#);
        assert_eq!(key_to_string(NullPartitionKey), r#"[null]"#);
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
