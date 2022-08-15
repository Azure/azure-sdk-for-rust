use super::PermissionToken;
use crate::resources::Resource;

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// The permission model.
///
/// A permission has an authorization token associated with a user for authorized
/// access to a specific resource. It is used to manage access to collections, documents,
/// attachments, stored procedures, triggers, and user-defined functions for a particular user.
/// You can learn more about permissions [here](https://docs.microsoft.com/rest/api/cosmos-db/permissions).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Permission {
    ///  The unique name that identifies the permission.
    pub id: String,
    #[serde(flatten)]
    /// The access mode on the resource for the user
    ///
    /// Represented as both "permissionMode" and "resource" in the JSON representation.
    pub permission_mode: PermissionMode,
    #[serde(rename = "_rid")]
    rid: String,
    /// The last updated timestamp of the resource.
    ///
    /// Represented as "_ts" in the JSON representation.
    #[serde(rename = "_ts")]
    pub timestamp: u64,
    /// The unique addressable URI for the resource.
    ///
    /// Represented as "_self" in the JSON representation.
    #[serde(rename = "_self")]
    pub uri: String,
    /// The resource etag required for optimistic concurrency control.
    ///
    /// Represented as "_etag" in the JSON representation.
    #[serde(rename = "_etag")]
    pub etag: String,
    /// The resource token for the particular resource and user.
    ///
    /// Represented as "_token" in the JSON representation.
    #[serde(rename = "_token")]
    pub permission_token: PermissionToken,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
#[serde(tag = "permissionMode", content = "resource")]
/// The access mode on the resource for the user along with the full
/// addressable path of the resource associated with the permission
///
/// Constructing a `PermissionMode` manually is error prone. Use one of the constructor methods
/// (i.e., [`PermissionMode::read`] or [`PermissionMode::all`]) or get a permission directly
/// from a resource (e.g., `Collection::read_permission`).
pub enum PermissionMode {
    /// read, write, and delete access
    All(Cow<'static, str>),
    /// read access only
    Read(Cow<'static, str>),
}

impl PermissionMode {
    /// Read permission for a given resource
    pub fn read<T: Resource + ?Sized>(resource: &T) -> Self {
        PermissionMode::Read(Cow::Owned(resource.uri().to_owned()))
    }

    /// Read, write, and delete permissions for a given resource
    pub fn all<T: Resource + ?Sized>(resource: &T) -> Self {
        PermissionMode::All(Cow::Owned(resource.uri().to_owned()))
    }

    /// The kind of permission mode as a string. Either "All" or "Read".
    pub fn kind(&self) -> &str {
        match self {
            Self::All(_) => "All",
            Self::Read(_) => "Read",
        }
    }

    /// The full addressable path of the resource associated with the permission
    pub fn resource(&self) -> &str {
        match self {
            Self::All(s) => s.as_ref(),
            Self::Read(s) => s.as_ref(),
        }
    }
}

impl std::convert::TryFrom<&[u8]> for Permission {
    type Error = azure_core::error::Error;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        use azure_core::error::ResultExt;
        serde_json::from_slice::<Self>(slice).with_context(
            azure_core::error::ErrorKind::DataConversion,
            || {
                format!(
                    "could not convert json '{}' into Permission",
                    std::str::from_utf8(slice).unwrap_or("<NON-UTF8>")
                )
            },
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::permission::AuthorizationToken;

    const PERMISSION_JSON: &str = r#"{
    "id": "a_permission",
    "permissionMode": "Read",
    "resource": "dbs/volcanodb/colls/volcano1",
    "_rid": "Sl8fAG8cXgBn6Ju2GqNsAA==",
    "_ts": 1449604760,
    "_self": "dbs\/Sl8fAA==\/users\/Sl8fAG8cXgA=\/permissions\/Sl8fAG8cXgBn6Ju2GqNsAA==\/",
    "_etag": "\"00000e00-0000-0000-0000-566736980000\"",
    "_token": "type=resource&ver=1.0&sig=ocPyc9QQFybITu1EqzX0kg==;w+WR1aWafB3+yZq5JSoBwgz78XDlU+k9Xiqvc+Q7TlAl1P4h4t721Cn5cjhZ9h3TSd2\/MJLy+wG+YkhDL9UlGkVv05RZGy2fMaLGdeQkWc7TShkc\/M2boPc3GXq2yiERKl5CN4AZWSOcrFhOFuuTOqF4ZdBlflmNudaakodr\/8qTip0i+a7moz1Jkc5+9iLAsDFyqTR1sirp7kAVNFbiqPdYTjNkvZUHF3nYYmRskOg=;"
} "#;

    #[test]
    fn parse_permission() {
        let permission: Permission = serde_json::from_str(PERMISSION_JSON).unwrap();

        assert_eq!(
            permission.permission_token,
            AuthorizationToken::Resource("ocPyc9QQFybITu1EqzX0kg==;w+WR1aWafB3+yZq5JSoBwgz78XDlU+k9Xiqvc+Q7TlAl1P4h4t721Cn5cjhZ9h3TSd2/MJLy+wG+YkhDL9UlGkVv05RZGy2fMaLGdeQkWc7TShkc/M2boPc3GXq2yiERKl5CN4AZWSOcrFhOFuuTOqF4ZdBlflmNudaakodr/8qTip0i+a7moz1Jkc5+9iLAsDFyqTR1sirp7kAVNFbiqPdYTjNkvZUHF3nYYmRskOg=;".to_owned()).into()
        );
        assert_eq!(
            permission.permission_mode,
            PermissionMode::Read(r#"dbs/volcanodb/colls/volcano1"#.into())
        );
    }
}
