use crate::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// https://swagger.io/docs/specification/using-ref/
/// https://github.com/OAI/OpenAPI-Specification/blob/master/versions/2.0.md#referenceObject
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum ReferenceOr<T> {
    Reference {
        #[serde(rename = "$ref")]
        reference: Reference,

        // $ref with sibling elements are not OpenAPI spec compliant
        // https://github.com/ctaggart/autorust_openapi/issues/13
        #[serde(skip_serializing_if = "Option::is_none")]
        title: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
        // specifying "type" feels like a bug in the spec
        #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
        type_: Option<DataType>,
        #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
        read_only: Option<bool>,

        /// flattens client model property or parameter
        /// https://github.com/Azure/autorest/blob/master/docs/extensions/readme.md#x-ms-client-flatten
        #[serde(rename = "x-ms-client-flatten", skip_serializing_if = "Option::is_none")]
        x_ms_client_flatten: Option<bool>,

        #[serde(skip_serializing_if = "Option::is_none")]
        xml: Option<MsXml>,

        #[serde(rename = "x-nullable", skip_serializing_if = "Option::is_none")]
        x_nullable: Option<bool>,
    },
    Item(T),
}

impl<T> ReferenceOr<T> {
    pub fn from_reference(reference: Reference) -> Self {
        Self::Reference {
            reference,
            title: None,
            description: None,
            x_ms_client_flatten: None,
            type_: None,
            read_only: None,
            xml: None,
            x_nullable: None,
        }
    }
}

/// a `$ref` URI
/// https://swagger.io/docs/specification/using-ref/
/// examples:
///   "$ref": "#/definitions/CloudError"
///   "$ref": "../../../../../common-types/resource-management/v1/types.json#/parameters/ApiVersionParameter"
///   "$ref": "#/parameters/privateCloudName"
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Reference {
    pub file: Option<String>,
    pub path: Vec<String>,
    pub name: Option<String>,
}

impl Reference {
    pub fn parse(str: &str) -> Result<Self, serde_json::Error> {
        let str = format!("\"{}\"", str);
        serde_json::from_str(&str)
    }
    pub fn from_file(file: &str) -> Self {
        Self {
            file: Some(file.to_owned()),
            path: Vec::new(),
            name: None,
        }
    }
}

impl<'de> Deserialize<'de> for Reference {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ReferenceVisitor)
    }
}

struct ReferenceVisitor;

impl<'de> serde::de::Visitor<'de> for ReferenceVisitor {
    type Value = Reference;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string conforming to the reference spec")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let reference = match s.find("#/") {
            None => Reference {
                file: Some(s.to_owned()),
                path: Vec::new(),
                name: None,
            },
            Some(i) => {
                let (file, s) = if i == 0 {
                    (None, &s[i + 2..])
                } else {
                    (Some(s[0..i].to_string()), &s[i + 2..])
                };

                let mut path: Vec<String> = s.split('/').map(str::to_owned).collect();
                let name = path.pop();
                Reference { file, path, name }
            }
        };
        Ok(reference)
    }
}

impl Serialize for Reference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut str = self.file.clone().unwrap_or_default();
        let path = &self.path.join("/");
        if !path.is_empty() {
            str.push_str("#/");
            str.push_str(path);
        }
        if let Some(name) = self.name.as_ref() {
            str.push('/');
            str.push_str(name);
        }
        serializer.serialize_str(&str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Parameter, ReferenceOr};

    #[test]
    fn can_parse_common_types() {
        let json = r#""../../../../../common-types/resource-management/v1/types.json#/parameters/SubscriptionIdParameter""#;
        let reference = serde_json::from_str::<Reference>(json).unwrap();
        assert_eq!(
            reference,
            Reference {
                file: Some("../../../../../common-types/resource-management/v1/types.json".to_owned()),
                path: vec!["parameters".to_owned()],
                name: Some("SubscriptionIdParameter".to_owned()),
            }
        );
    }

    #[test]
    fn can_parse_clouderror() {
        let json = r##""#/definitions/CloudError""##;
        let reference = serde_json::from_str::<Reference>(json).unwrap();
        assert_eq!(
            reference,
            Reference {
                file: None,
                path: vec!["definitions".to_owned()],
                name: Some("CloudError".to_owned()),
            }
        );
    }

    #[test]
    fn can_parse_example() {
        let json = r#""./examples/Authorizations_CreateOrUpdate.json""#;
        let reference = serde_json::from_str::<Reference>(json).unwrap();
        assert_eq!(
            reference,
            Reference {
                file: Some("./examples/Authorizations_CreateOrUpdate.json".to_owned()),
                path: vec![],
                name: None,
            }
        );
    }

    #[test]
    fn deserializes() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            serde_json::from_str::<ReferenceOr<Parameter>>(json).unwrap(),
            ReferenceOr::<Parameter>::from_reference(Reference::from_file("foo/bar"))
        );
    }

    #[test]
    fn serializes() {
        let json = r#"{"$ref":"foo/bar"}"#;
        assert_eq!(
            json,
            serde_json::to_string(&ReferenceOr::<Parameter>::from_reference(Reference::from_file("foo/bar"))).unwrap()
        );
    }
}
