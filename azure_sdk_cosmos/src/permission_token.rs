use azure_sdk_core::errors::AzureError;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct PermissionToken<'a> {
    permission_type: Cow<'a, str>,
    ver: Cow<'a, str>,
    sig: Cow<'a, str>,
}

impl<'a> std::convert::TryFrom<&'a str> for PermissionToken<'a> {
    type Error = AzureError;
    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = s.split("&").collect();

        Ok(Self {
            permission_type: Cow::Borrowed(tokens[0]),
            ver: Cow::Borrowed(tokens[0]),
            sig: Cow::Borrowed(tokens[0]),
        })
    }
}
