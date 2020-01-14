use crate::permission::CosmosPermission;
use crate::Permission;
use crate::{
    activity_id_from_headers, alt_content_path_from_headers, content_path_from_headers,
    request_charge_from_headers,
};
use azure_sdk_core::errors::AzureError;
use azure_sdk_core::session_token_from_headers;
use http::HeaderMap;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq)]
pub struct ListPermissionsResponse<'a> {
    pub permissions: Vec<Permission<'a, Cow<'a, str>>>,
    pub charge: f64,
    pub activity_id: uuid::Uuid,
    pub session_token: String,
    pub content_path: String,
    pub alt_content_path: String,
}

impl<'a> std::convert::TryFrom<(&HeaderMap, &[u8])> for ListPermissionsResponse<'a> {
    type Error = AzureError;
    fn try_from(value: (&HeaderMap, &[u8])) -> Result<Self, Self::Error> {
        let headers = value.0;
        let body = value.1;

        debug!("headers == {:#?}", headers);
        debug!("body == {:#?}", std::str::from_utf8(body)?);

        #[derive(Debug, Deserialize)]
        struct Response<'b> {
            _rid: String,
            #[serde(rename = "Permissions")]
            permissions: Vec<CosmosPermission<'b>>,
            _count: u32,
        }

        // first get the Cosmos REST API permission
        let response: Response<'_> = serde_json::from_slice(body)?;
        debug!("response == {:#?}", response);

        // now convert every Cosmos REST API permission
        // into the SDK struct
        let permissions = response
            .permissions
            .into_iter()
            .map(Permission::try_from)
            .collect::<Result<Vec<Permission<'_, Cow<'_, str>>>, AzureError>>()?;

        Ok(Self {
            permissions,
            charge: request_charge_from_headers(headers)?,
            activity_id: activity_id_from_headers(headers)?,
            session_token: session_token_from_headers(headers)?,
            content_path: content_path_from_headers(headers)?.to_owned(),
            alt_content_path: alt_content_path_from_headers(headers)?.to_owned(),
        })
    }
}
