use crate::{CosmosError, PermissionToken};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Permission<'a> {
    pub id: Cow<'a, str>,
    #[serde(flatten)]
    pub permission_mode: PermissionMode<'a>,
    #[serde(rename = "_rid")]
    pub rid: Cow<'a, str>,
    #[serde(rename = "_ts")]
    pub ts: u64,
    pub _self: Cow<'a, str>,
    #[serde(rename = "_etag")]
    pub etag: Cow<'a, str>,
    #[serde(rename = "_token")]
    pub permission_token: PermissionToken,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "permissionMode", content = "resource")]
pub enum PermissionMode<'a> {
    All(Cow<'a, str>),
    Read(Cow<'a, str>),
}

impl<'a> PermissionMode<'a> {
    pub fn kind(&self) -> &str {
        match self {
            Self::All(_) => "All",
            Self::Read(_) => "Read",
        }
    }

    pub fn resource(&self) -> &str {
        match self {
            Self::All(s) => s.as_ref(),
            Self::Read(s) => s.as_ref(),
        }
    }
}

impl<'a> std::convert::TryFrom<&[u8]> for Permission<'a> {
    type Error = CosmosError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        Ok(serde_json::from_slice(slice)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let permission: Permission<'_> = serde_json::from_str(&PERMISSION_JSON).unwrap();

        assert_eq!(
            permission.permission_token,
            crate::AuthorizationToken::Resource("ocPyc9QQFybITu1EqzX0kg==;w+WR1aWafB3+yZq5JSoBwgz78XDlU+k9Xiqvc+Q7TlAl1P4h4t721Cn5cjhZ9h3TSd2/MJLy+wG+YkhDL9UlGkVv05RZGy2fMaLGdeQkWc7TShkc/M2boPc3GXq2yiERKl5CN4AZWSOcrFhOFuuTOqF4ZdBlflmNudaakodr/8qTip0i+a7moz1Jkc5+9iLAsDFyqTR1sirp7kAVNFbiqPdYTjNkvZUHF3nYYmRskOg=;".to_owned()).into()
        );
        assert_eq!(
            permission.permission_mode,
            PermissionMode::Read(r#"dbs/volcanodb/colls/volcano1"#.into())
        );
    }
}
