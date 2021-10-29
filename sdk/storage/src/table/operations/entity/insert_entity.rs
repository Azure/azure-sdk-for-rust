use crate::table::prelude::{
    header_time_value, header_value, ApiVersion, EchoContent, OdataMetadataLevel,
};
use azure_core::{Error, Request};
use chrono::Utc;
use http::HeaderValue;

use super::TableEntity;

pub struct InsertEntityOptions {
    // Optional. The timeout parameter is expressed in seconds.
    // timeout: Option<Duration>,
    api_version: Option<ApiVersion>,
    echo_content: Option<EchoContent>,
    odata_metadata_level: Option<OdataMetadataLevel>,
}

impl Default for InsertEntityOptions {
    fn default() -> Self {
        Self {
            // timeout: Default::default(),
            api_version: Some(ApiVersion::default()),
            echo_content: Some(EchoContent::ReturnContent),
            odata_metadata_level: Some(OdataMetadataLevel::FullMetadata),
        }
    }
}

impl InsertEntityOptions {
    setters! {
        // timeout: Duration => Some(timeout),
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
    ) -> Result<(), Error> {
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

        let serialized = serde_json::to_string(&table_entity)?;
        headers.append(
            "Content-Length",
            HeaderValue::from(serialized.as_bytes().len()),
        );
        request.set_body(bytes::Bytes::from(serialized).into());

        Ok(())
    }
}
