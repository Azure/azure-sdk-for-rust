use crate::{Operation, Parameter, ReferenceOr};
use serde::{Deserialize, Serialize};

/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#path-item-object
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Default)]
pub struct PathItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Operation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<Operation>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<ReferenceOr<Parameter>>,
}

impl PathItem {
    /// Returns all operations
    pub fn operations(&self) -> impl Iterator<Item = &Operation> {
        vec![
            self.get.as_ref(),
            self.post.as_ref(),
            self.put.as_ref(),
            self.patch.as_ref(),
            self.delete.as_ref(),
            self.options.as_ref(),
            self.head.as_ref(),
        ]
        .into_iter()
        .filter_map(|x| x)
    }
}
