use super::{ApiVersion, OdataMetadataLevel};
use crate::operations::{header_time_value, header_value, EchoContent};
use azure_core::{Error, Request, Response};
use chrono::Utc;
use http::HeaderValue;

#[derive(Debug, Clone)]
pub struct CreateTableOptions {
    api_version: Option<ApiVersion>,
    echo_content: Option<EchoContent>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for CreateTableOptions {
    fn default() -> Self {
        Self {
            api_version: Some(ApiVersion::default()),
            echo_content: Some(EchoContent::ReturnContent),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl CreateTableOptions {
    setters! {
        api_version: ApiVersion => Some(api_version),
        echo_content: EchoContent => Some(echo_content),
        odata_metadata_level: OdataMetadataLevel  => Some(odata_metadata_level),
    }

    pub(crate) fn expected_status_code(&self) -> http::StatusCode {
        match &self.echo_content {
            Some(value) => match value {
                EchoContent::ReturnNoContent => http::StatusCode::NO_CONTENT,
                EchoContent::ReturnContent => http::StatusCode::CREATED,
            },
            None => http::StatusCode::CREATED,
        }
    }

    pub fn decorate_request(&self, request: &mut Request, table_name: &str) -> Result<(), Error> {
        let headers = request.headers_mut();
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
        headers.append("Prefer", header_value::<EchoContent>(&self.echo_content)?);
        headers.append("x-ms-date", header_time_value(Utc::now())?);
        headers.append(
            "x-ms-version",
            header_value::<ApiVersion>(&self.api_version)?,
        );
        headers.append(
            "Accept",
            header_value::<OdataMetadataLevel>(&self.odata_metadata_level)?,
        );

        #[derive(serde::Serialize)]
        struct CreateTableRequest<'a> {
            #[serde(rename = "TableName")]
            pub table_name: &'a str,
        }
        let body = CreateTableRequest { table_name };
        let bytes = bytes::Bytes::from(serde_json::to_string(&body)?);
        headers.append("Content-Length", HeaderValue::from(bytes.len()));

        let md5 = base64::encode(&md5::compute(bytes.as_ref())[..]);
        headers.append("Content-MD5", HeaderValue::from_str(md5.as_str()).unwrap());

        *request.body_mut() = bytes.into();
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTableResponse {
    #[serde(rename = "odata.metadata")]
    pub odata_metadata: Option<String>,
    #[serde(rename = "odata.type")]
    pub odata_type: Option<String>,
    #[serde(rename = "odata.id")]
    pub odata_id: Option<String>,
    #[serde(rename = "odata.editLink")]
    pub odata_edit_link: Option<String>,
    #[serde(rename = "TableName")]
    pub table_name: String,
}

impl CreateTableResponse {
    pub(crate) async fn try_from(response: Response) -> Result<Self, Error> {
        let body = azure_core::collect_pinned_stream(response.deconstruct().2).await?;
        let response = serde_json::from_slice(&body)?;
        Ok(response)
    }
}
