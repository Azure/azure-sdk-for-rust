use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(tag = "type")]
pub enum Security {
    #[serde(rename = "apiKey")]
    ApiKey {
        name: String,
        #[serde(rename = "in")]
        in_: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "oauth2")]
    Oauth2 {
        flow: Flow,
        #[serde(rename = "authorizationUrl", skip_serializing_if = "Option::is_none")]
        authorization_url: Option<String>,
        #[serde(rename = "tokenUrl")]
        #[serde(skip_serializing_if = "Option::is_none")]
        token_url: Option<String>,
        /// The available scopes for the OAuth2 security scheme.
        #[serde(default, skip_serializing_if = "IndexMap::is_empty")]
        scopes: IndexMap<String, String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
    #[serde(rename = "basic")]
    Basic {
        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,
    },
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum Flow {
    Implicit,
    Password,
    Application,
    AccessCode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use indexmap::IndexMap;

    #[test]
    fn api_deserializes() {
        let json = r#"{"type":"apiKey", "name":"foo", "in": "query"}"#;
        assert_eq!(
            serde_json::from_str::<Security>(json).unwrap(),
            Security::ApiKey {
                name: "foo".into(),
                in_: "query".into(),
                description: None,
            }
        );
    }

    #[test]
    fn api_serializes() {
        let json = r#"{"type":"apiKey","name":"foo","in":"query"}"#;
        assert_eq!(
            serde_json::to_string(&Security::ApiKey {
                name: "foo".into(),
                in_: "query".into(),
                description: None,
            })
            .unwrap(),
            json
        );
    }

    #[test]
    fn basic_deserializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(
            serde_json::from_str::<Security>(json).unwrap(),
            Security::Basic { description: None }
        );
    }

    #[test]
    fn basic_serializes() {
        let json = r#"{"type":"basic"}"#;
        assert_eq!(json, serde_json::to_string(&Security::Basic { description: None }).unwrap());
    }

    #[test]
    fn oauth_deserializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = IndexMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            serde_json::from_str::<Security>(json).unwrap(),
            Security::Oauth2 {
                flow: Flow::Implicit,
                authorization_url: Some("foo/bar".into()),
                token_url: None,
                scopes,
                description: None,
            }
        );
    }

    #[test]
    fn oauth_serializes() {
        let json = r#"{"type":"oauth2","flow":"implicit","authorizationUrl":"foo/bar","scopes":{"foo":"bar"}}"#;
        let mut scopes = IndexMap::new();
        scopes.insert("foo".into(), "bar".into());
        assert_eq!(
            json,
            serde_json::to_string(&Security::Oauth2 {
                flow: Flow::Implicit,
                authorization_url: Some("foo/bar".into()),
                token_url: None,
                scopes,
                description: None,
            })
            .unwrap()
        );
    }
}
