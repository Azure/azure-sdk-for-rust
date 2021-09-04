use super::{header_value, ApiVersion, EchoContent, OdataMetadataLevel, TableEntity};
use azure_core::{Error, HTTPHeaderError, Request, Response};
use chrono::{Duration, Utc};
use http::HeaderValue;
use serde::de::DeserializeOwned;

pub struct InsertEntityOptions {
    // Optional. The timeout parameter is expressed in seconds.
    timeout: Option<Duration>,
    api_version: Option<ApiVersion>,
    echo_content: Option<EchoContent>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for InsertEntityOptions {
    fn default() -> Self {
        Self {
            timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
            echo_content: Some(EchoContent::ReturnContent),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl InsertEntityOptions {
    setters! {
        timeout: Duration => Some(timeout),
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

    pub fn decorate_request<'b, ENTITY: serde::Serialize + TableEntity<'b>>(
        &self,
        request: &mut Request,
        table_entity: &'b ENTITY,
    ) -> Result<(), HTTPHeaderError> {
        let headers = request.headers_mut();
        headers.append("Content-Type", HeaderValue::from_static("application/json"));
        headers.append("Prefer", header_value::<EchoContent>(&self.echo_content)?);
        headers.append(
            "x-ms-version",
            header_value::<ApiVersion>(&self.api_version)?,
        );
        headers.append(
            "Accept",
            header_value::<OdataMetadataLevel>(&self.odata_metadata_level)?,
        );
        headers.append(
            "x-ms-date",
            HeaderValue::from_str(
                Utc::now()
                    .format("%a, %d %h %Y %T GMT")
                    .to_string()
                    .as_str(),
            )?,
        );

        let serialized = serde_json::to_string(&table_entity).unwrap();

        headers.append(
            "Content-Length",
            HeaderValue::from(serialized.as_bytes().len()),
        );

        request.set_body(bytes::Bytes::from(serialized).into());
        Ok(())
    }
}

#[derive(Default, Debug, Clone, PartialEq, serde_derive::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsertEntityResponse<ENTITY> {
    /// odata_metadata fields
    #[serde(rename = "odata.metadata")]
    pub odata_metadata: Option<String>,
    #[serde(rename = "odata.type")]
    pub odata_type: Option<String>,
    #[serde(rename = "odata.id")]
    pub odata_id: Option<String>,
    #[serde(rename = "odata.etag")]
    pub odata_etag: Option<String>,
    #[serde(rename = "odata.editLink")]
    pub odata_edit_link: Option<String>,
    #[serde(rename = "Timestamp@odata.type")]
    pub timestamp_odata_type: Option<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: Option<String>,

    #[serde(flatten)]
    pub model: ENTITY,
}
