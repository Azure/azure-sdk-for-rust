use std::borrow::Cow;

use azure_core::Model;
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
///     .with_add("/color".into(), "silver")?
///     .with_move("/from".into(), "/to".into())?;
/// # assert_eq!(patch, PatchDocument {
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
#[derive(Model, Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct PatchDocument {
    pub operations: Vec<PatchOperation>,
}

impl PatchDocument {
    /// Adds a new "add" operation to the patch document.
    ///
    /// See the [type documentation](PatchDocument) for more information on patch operations.
    ///
    /// # Arguments
    /// * `path` - The path to the property to add.
    /// * `value` - The value to add.
    pub fn with_add(
        mut self,
        path: Cow<'static, str>,
        value: impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        self.operations.push(PatchOperation::Add {
            path,
            value: serde_json::to_value(value)?,
        });
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
        path: Cow<'static, str>,
        value: serde_json::Number,
    ) -> Result<Self, serde_json::Error> {
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
    pub fn with_remove(mut self, path: Cow<'static, str>) -> Result<Self, serde_json::Error> {
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
        path: Cow<'static, str>,
        value: impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        self.operations.push(PatchOperation::Replace {
            path,
            value: serde_json::to_value(value)?,
        });
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
        path: Cow<'static, str>,
        value: impl Serialize,
    ) -> Result<Self, serde_json::Error> {
        self.operations.push(PatchOperation::Set {
            path,
            value: serde_json::to_value(value)?,
        });
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
        from: Cow<'static, str>,
        to: Cow<'static, str>,
    ) -> Result<Self, serde_json::Error> {
        self.operations.push(PatchOperation::Move { from, to });
        Ok(self)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use serde::Serialize;
    use serde_json::Number;

    use crate::models::PatchDocument;

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
    pub fn serialize_add() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_add(
            "/parent".into(),
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
            .with_increment("/count".into(), Number::from(1))?
            .with_increment("/sum".into(), Number::from_f64(4.2).unwrap())?;
        let serialized = serde_json::to_string(&patch_document).unwrap();
        assert_eq!(
            serialized,
            "{\"operations\":[{\"op\":\"incr\",\"path\":\"/count\",\"value\":1},{\"op\":\"incr\",\"path\":\"/sum\",\"value\":4.2}]}",
        );

        Ok(())
    }

    #[test]
    pub fn serialize_remove() -> Result<(), Box<dyn std::error::Error>> {
        let patch_document = PatchDocument::default().with_remove("/value".into())?;

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
            "/parent".into(),
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
            "/parent".into(),
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
        let patch_document = PatchDocument::default().with_move("/from".into(), "/to".into())?;

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
                .with_add("/color".into(), "silver")?
                .with_remove("/used".into())?
                .with_set(
                    "/price".into(),
                    serde_json::Number::from_f64(355.45).unwrap()
                )?
                .with_increment("/inventory/quantity".into(), Number::from(10))?
                .with_add("/tags/-".into(), "featured-bikes")?
                .with_move("/color".into(), "/inventory/color".into())?
        );
        Ok(())
    }
}
