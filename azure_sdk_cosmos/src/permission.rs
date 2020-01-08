use crate::Resource;
use azure_sdk_core::errors::{AzureError, UnexpectedValue};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub trait PermissionName: std::fmt::Debug {
    fn name(&self) -> &str;
}

impl<'a, T> PermissionName for Permission<'a, T>
where
    T: Resource + Clone + std::fmt::Debug,
{
    fn name(&self) -> &str {
        &self.id
    }
}

impl<R> PermissionName for R
where
    R: AsRef<str> + std::fmt::Debug,
{
    fn name(&self) -> &str {
        self.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum PermissionMode<T: Resource> {
    All(T),
    Read(T),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Permission<'a, T>
where
    T: Resource + Clone,
{
    pub id: Cow<'a, str>,
    pub permission_mode: PermissionMode<T>,
    pub rid: Cow<'a, str>,
    pub ts: u64,
    pub _self: Cow<'a, str>,
    pub etag: Cow<'a, str>,
    pub token: Cow<'a, str>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct CosmosPermission<'a> {
    pub id: Cow<'a, str>,
    #[serde(rename = "permissionMode")]
    pub permission_mode: Cow<'a, str>,
    pub resource: Cow<'a, str>,
    pub _rid: Cow<'a, str>,
    pub _ts: u64,
    pub _self: Cow<'a, str>,
    pub _etag: Cow<'a, str>,
    pub _token: Cow<'a, str>,
}

impl<'a> std::convert::TryFrom<CosmosPermission<'a>> for Permission<'a, Cow<'a, str>> {
    type Error = AzureError;
    fn try_from(cosmos_permission: CosmosPermission<'a>) -> Result<Self, Self::Error> {
        let permission_mode: &str = &cosmos_permission.permission_mode;
        let permission_mode = match permission_mode {
            "All" => PermissionMode::All(cosmos_permission.resource.clone()),
            "Read" => PermissionMode::Read(cosmos_permission.resource.clone()),
            _ => {
                return Err(UnexpectedValue::new_multiple(
                    vec!["All".to_owned(), "Read".to_owned()],
                    permission_mode.to_owned(),
                )
                .into())
            }
        };

        Ok(Self {
            id: cosmos_permission.id,
            permission_mode,
            rid: cosmos_permission._rid,
            ts: cosmos_permission._ts,
            _self: cosmos_permission._self,
            etag: cosmos_permission._etag,
            token: cosmos_permission._token,
        })
    }
}

impl<'a, T> std::convert::From<Permission<'a, T>> for CosmosPermission<'a>
where
    T: Resource + Clone,
{
    fn from(permission: Permission<'a, T>) -> Self {
        let (permission_mode, resource) = match permission.permission_mode {
            PermissionMode::Read(resource) => ("Read", resource),
            PermissionMode::All(resource) => ("All", resource),
        };

        Self {
            id: permission.id,
            permission_mode: Cow::Borrowed(permission_mode),
            resource: Cow::Owned(resource.uri().to_owned()),
            _rid: permission.rid,
            _ts: permission.ts,
            _self: permission._self,
            _etag: permission.etag,
            _token: permission.token,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    const PERMISSION_JSON: &str = r#"{  
    "id": "a_permission",  
    "permissionMode": "Read",  
    "resource": "dbs/volcanodb/colls/volcano1",  
    "_rid": "Sl8fAG8cXgBn6Ju2GqNsAA==",  
    "_ts": 1449604760,  
    "_self": "dbs\/Sl8fAA==\/users\/Sl8fAG8cXgA=\/permissions\/Sl8fAG8cXgBn6Ju2GqNsAA==\/",  
    "_etag": "\"00000e00-0000-0000-0000-566736980000\"",  
    "_token": "type=resource&ver=1&sig=ocPyc9QQFybITu1EqzX0kg==;w+WR1aWafB3+yZq5JSoBwgz78XDlU+k9Xiqvc+Q7TlAl1P4h4t721Cn5cjhZ9h3TSd2\/MJLy+wG+YkhDL9UlGkVv05RZGy2fMaLGdeQkWc7TShkc\/M2boPc3GXq2yiERKl5CN4AZWSOcrFhOFuuTOqF4ZdBlflmNudaakodr\/8qTip0i+a7moz1Jkc5+9iLAsDFyqTR1sirp7kAVNFbiqPdYTjNkvZUHF3nYYmRskOg=;"  
} "#;

    #[test]
    fn parse_permission() {
        let cosmos_permission: CosmosPermission<'_> =
            serde_json::from_str(&PERMISSION_JSON).unwrap();

        let permission: Permission<'_, Cow<'_, str>> =
            cosmos_permission.clone().try_into().unwrap();
        assert_eq!(permission.token, cosmos_permission._token);
        assert_eq!(
            permission.permission_mode,
            PermissionMode::Read(Cow::Borrowed(r#"dbs/volcanodb/colls/volcano1"#))
        );
    }
}
