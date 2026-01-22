use std::borrow::Cow;

use azure_core::{error::ErrorKind, fmt::SafeDebug, Error};
use serde::{Deserialize, Serialize};

// Cosmos' patch operations are _similar_ to JSON Patch (RFC 6902) in structure, but have different operations.

/// Represents a patch document for Cosmos DB.
///
/// A patch document is made up of a collection of patch operations.
/// Each operation describes how to modify a property of a document.
/// See <https://learn.microsoft.com/en-us/azure/cosmos-db/partial-document-update> for more information on patch operations.
///
/// # Examples
///
/// To build up a patch document, use [`PatchDocument::default`] to create an empty document,
/// then use the various `with_` methods to add operations.
///
/// ```rust
/// # use azure_data_cosmos::models::{PatchDocument, PatchOperation};
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let patch = PatchDocument::default()
///     .with_add("/color", "silver")?
///     .with_move("/from", "/to")?;
/// # assert_eq!(patch, PatchDocument {
/// #     condition: None,
/// #     operations: vec![
/// #         PatchOperation::Add {
/// #             path: "/color".into(),
/// #             value: serde_json::Value::String("silver".to_string()),
/// #        },
/// #        PatchOperation::Move {
/// #            from: "/from".into(),
/// #            to: "/to".into(),
/// #        },
/// #    ],
/// # });
/// # Ok(())
/// # }
/// ```
#[derive(Default, SafeDebug, Serialize, Deserialize, PartialEq, Eq)]
#[safe(true)]
pub struct PatchDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<Cow<'static, str>>,
    pub operations: Vec<PatchOperation>,
}

impl PatchDocument {
    /// Adds a condition, which determines whether or not the patch should be applied.
    ///
    /// The value is an SQL-like filter predicate as a string. For example, `from c where c.taskNum = 3`.
    pub fn with_condition(mut self, condition: impl Into<Cow<'static, str>>) -> Self {
        self.condition = Some(condition.into());
        self
    }

    /// Adds a new "add" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `path` - The path to the property to add.
    /// * `value` - The value to add.
    pub fn with_add(
        mut self,
        path: impl Into<Cow<'static, str>>,
        value: impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        let path = path.into();
        let value = serde_json::to_value(value)?;
        self.operations.push(PatchOperation::Add { path, value });
        Ok(self)
    }

    /// Adds a new "increment" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `path` - The path to the property to increment.
    /// * `value` - The amount to increment by. Integers can be specified directly in this parameter. Use [`serde_json::Number::from_f64`] to create a floating-point number.
    pub fn with_increment(
        mut self,
        path: impl Into<Cow<'static, str>>,
        value: impl ToJsonNumber,
    ) -> azure_core::Result<Self> {
        let path = path.into();
        let value = value.to_json_number()?;
        self.operations
            .push(PatchOperation::Increment { path, value });
        Ok(self)
    }

    /// Adds a new "remove" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `path` - The path to the property to remove.
    pub fn with_remove(mut self, path: impl Into<Cow<'static, str>>) -> azure_core::Result<Self> {
        let path = path.into();
        self.operations.push(PatchOperation::Remove { path });
        Ok(self)
    }

    /// Adds a new "replace" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `path` - The path to the property to remove.
    /// * `value` - The value to replace the property with.
    pub fn with_replace(
        mut self,
        path: impl Into<Cow<'static, str>>,
        value: impl Serialize,
    ) -> azure_core::Result<Self> {
        let path = path.into();
        let value = serde_json::to_value(value)?;
        self.operations
            .push(PatchOperation::Replace { path, value });
        Ok(self)
    }

    /// Adds a new "set" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `path` - The path to the property to remove.
    /// * `value` - The value to set the property to.
    pub fn with_set(
        mut self,
        path: impl Into<Cow<'static, str>>,
        value: impl Serialize,
    ) -> azure_core::Result<Self> {
        let path = path.into();
        let value = serde_json::to_value(value)?;
        self.operations.push(PatchOperation::Set { path, value });
        Ok(self)
    }

    /// Adds a new "move" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `from` - The path to the property to move.
    /// * `to` - The path to move the property to.
    pub fn with_move(
        mut self,
        from: impl Into<Cow<'static, str>>,
        to: impl Into<Cow<'static, str>>,
    ) -> azure_core::Result<Self> {
        let from = from.into();
        let to = to.into();
        self.operations.push(PatchOperation::Move { from, to });
        Ok(self)
    }
}

#[derive(SafeDebug, Serialize, Deserialize, PartialEq, Eq)]
#[safe(true)]
#[serde(tag = "op")]
#[serde(rename_all = "camelCase")]
pub enum PatchOperation {
    Add {
        path: Cow<'static, str>,
        value: serde_json::Value,
    },
    #[serde(rename = "incr")]
    Increment {
        path: Cow<'static, str>,
        value: serde_json::Number,
    },
    Remove {
        path: Cow<'static, str>,
    },
    Replace {
        path: Cow<'static, str>,
        value: serde_json::Value,
    },
    Set {
        path: Cow<'static, str>,
        value: serde_json::Value,
    },
    Move {
        from: Cow<'static, str>,
        #[serde(rename = "path")]
        to: Cow<'static, str>,
    },
}

/// A trait that represents a type that can be converted to a JSON number.
///
/// This trait allows APIs to accept any integer or floating point number.
/// Note that "NaN" and "Infinity" are not valid JSON numbers, so this conversion is fallible.
pub trait ToJsonNumber {
    /// Converts the type to a JSON number.
    ///
    /// Returns an error if the number cannot be represented as a JSON number.
    ///
    /// For example:
    /// * NaN and Infinity are not valid JSON numbers.
    /// * Integers larger than u64::MAX cannot be represented as JSON numbers by serde_json.
    fn to_json_number(self) -> azure_core::Result<serde_json::Number>;
}

// We're not implementing ToJsonNumber for f32 because there's are precision issues when converting f32 to f64.
// serde_json::Number only supports f64, so we'll expect users to convert f32 to f64 before calling to_json_number.

impl ToJsonNumber for f64 {
    fn to_json_number(self) -> azure_core::Result<serde_json::Number> {
        serde_json::Number::from_f64(self).ok_or_else(|| {
            Error::with_message_fn(ErrorKind::DataConversion, || {
                format!("{} is not a valid JSON number", self)
            })
        })
    }
}

impl ToJsonNumber for serde_json::Number {
    fn to_json_number(self) -> azure_core::Result<serde_json::Number> {
        Ok(self)
    }
}

macro_rules! to_json_number_for_int {
    ($t: ty) => {
        impl ToJsonNumber for $t {
            fn to_json_number(self) -> azure_core::Result<serde_json::Number> {
                Ok(serde_json::Number::from(self))
            }
        }
    };
}

to_json_number_for_int!(u8);
to_json_number_for_int!(i8);
to_json_number_for_int!(u16);
to_json_number_for_int!(i16);
to_json_number_for_int!(u32);
to_json_number_for_int!(i32);
to_json_number_for_int!(u64);
to_json_number_for_int!(i64);
to_json_number_for_int!(usize);
to_json_number_for_int!(isize);

#[cfg(test)]
mod tests {
    use serde::Serialize;

    use crate::models::{PatchDocument, ToJsonNumber};

    #[derive(Serialize)]
    struct TestStruct {
        foo: String,
        bar: i32,
    }

    #[test]
    pub fn serialize_empty_patch_document() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default();

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(serialized, "{\"operations\":[]}");
        Ok(())
    }

    #[test]
    pub fn serialize_condition() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_condition("from c where c.value = 0");

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"condition\":\"from c where c.value = 0\",\"operations\":[]}"
        );
        Ok(())
    }

    #[test]
    pub fn serialize_add() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_add(
            "/parent",
            TestStruct {
                foo: "test".to_string(),
                bar: 42,
            },
        )?;

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"add\",\"path\":\"/parent\",\"value\":{\"bar\":42,\"foo\":\"test\"}}]}"
        );

        Ok(())
    }

    #[test]
    pub fn serialize_increment() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default()
            .with_increment("/count", 1)?
            .with_increment("/sum", 4.2)?;
        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"incr\",\"path\":\"/count\",\"value\":1},{\"op\":\"incr\",\"path\":\"/sum\",\"value\":4.2}]}",
        );

        Ok(())
    }

    #[test]
    pub fn serialize_remove() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_remove("/value")?;

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"remove\",\"path\":\"/value\"}]}"
        );

        Ok(())
    }

    #[test]
    pub fn serialize_replace() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_replace(
            "/parent",
            TestStruct {
                foo: "test".to_string(),
                bar: 42,
            },
        )?;

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"replace\",\"path\":\"/parent\",\"value\":{\"bar\":42,\"foo\":\"test\"}}]}"
        );

        Ok(())
    }

    #[test]
    pub fn serialize_set() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_set(
            "/parent",
            TestStruct {
                foo: "test".to_string(),
                bar: 42,
            },
        )?;

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"set\",\"path\":\"/parent\",\"value\":{\"bar\":42,\"foo\":\"test\"}}]}"
        );

        Ok(())
    }

    #[test]
    pub fn serialize_move() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_move("/from", "/to")?;

        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"move\",\"from\":\"/from\",\"path\":\"/to\"}]}"
        );

        Ok(())
    }

    #[test]
    pub fn cosmos_docs_example() -> Result<(), Box<dyn std::error::Error>> {
        const TEST_DOC: &str = r#"{
            "operations": [
                { "op": "add", "path": "/color", "value": "silver" },
                { "op": "remove", "path": "/used" },
                { "op": "set", "path": "/price", "value": 355.45 },
                { "op": "incr", "path": "/inventory/quantity", "value": 10 },
                { "op": "add", "path": "/tags/-", "value": "featured-bikes" },
                { "op": "move", "from": "/color", "path": "/inventory/color" }
            ]
        }"#;

        let doc: PatchDocument = serde_json::from_str(TEST_DOC)?;

        assert_eq!(
            doc,
            PatchDocument::default()
                .with_add("/color", "silver")?
                .with_remove("/used")?
                .with_set("/price", 355.45)?
                .with_increment("/inventory/quantity", 10)?
                .with_add("/tags/-", "featured-bikes")?
                .with_move("/color", "/inventory/color")?
        );
        Ok(())
    }

    #[test]
    pub fn cosmos_docs_conditional_patch_example() -> Result<(), Box<dyn std::error::Error>> {
        const TEST_DOC: &str = r#"{
            "condition": "from c where c.Address.ZipCode = '98101'",
            "operations": [
                {
                    "op":"replace",
                    "path":"/Address/ZipCode",
                    "value":98107
                }
            ]
        }"#;

        let doc: PatchDocument = serde_json::from_str(TEST_DOC)?;

        assert_eq!(
            doc,
            PatchDocument::default()
                .with_condition("from c where c.Address.ZipCode = '98101'")
                .with_replace("/Address/ZipCode", 98107)?
        );
        Ok(())
    }

    #[test]
    pub fn to_json_number_f64() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            serde_json::Number::from_f64(4.2f64).unwrap(),
            (4.2f64).to_json_number()?
        );
        assert_eq!(
            "NaN is not a valid JSON number",
            format!("{}", (f64::NAN).to_json_number().unwrap_err())
        );
        assert_eq!(
            "inf is not a valid JSON number",
            format!("{}", (f64::INFINITY).to_json_number().unwrap_err())
        );
        assert_eq!(
            "-inf is not a valid JSON number",
            format!("{}", (f64::NEG_INFINITY).to_json_number().unwrap_err())
        );
        Ok(())
    }

    #[test]
    pub fn to_json_number_valid_ints() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(serde_json::Number::from(42u8), (42u8).to_json_number()?);
        assert_eq!(serde_json::Number::from(-42i8), (-42i8).to_json_number()?);
        assert_eq!(serde_json::Number::from(42u16), (42u16).to_json_number()?);
        assert_eq!(serde_json::Number::from(-42i16), (-42i16).to_json_number()?);
        assert_eq!(serde_json::Number::from(42u32), (42u32).to_json_number()?);
        assert_eq!(serde_json::Number::from(-42i32), (-42i32).to_json_number()?);
        assert_eq!(serde_json::Number::from(42u64), (42u64).to_json_number()?);
        assert_eq!(serde_json::Number::from(-42i64), (-42i64).to_json_number()?);
        assert_eq!(
            serde_json::Number::from(42usize),
            (42usize).to_json_number()?
        );
        assert_eq!(
            serde_json::Number::from(-42isize),
            (-42isize).to_json_number()?
        );
        Ok(())
    }
}
