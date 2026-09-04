// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#[cfg(feature = "arrow")]
use crate::arrow::{
    decode_arrow_list_blobs, decode_arrow_list_blobs_hierarchy, decode_arrow_next_marker,
};
use crate::models::{ListBlobsHierarchicalResponse, ListBlobsResponse};
use azure_core::{
    error::{Error, ErrorKind},
    http::{
        headers::{self, Headers},
        response::ResponseBody,
        DeserializeWith, Format, RawResponse,
    },
    Result,
};
use serde::{de::DeserializeOwned, Deserialize};

const ARROW_CONTENT_TYPE: &str = "application/vnd.apache.arrow.stream";
const XML_CONTENT_TYPE: &str = "application/xml";

#[derive(Clone, Copy)]
enum WireFormat {
    #[cfg(feature = "arrow")]
    Arrow,
    Xml,
}

fn wire_format(headers: &Headers) -> Result<WireFormat> {
    let Some(content_type) = headers.get_optional_str(&headers::CONTENT_TYPE) else {
        return Ok(WireFormat::Xml);
    };
    let media_type = content_type.split(';').next().unwrap_or_default().trim();
    if media_type.eq_ignore_ascii_case(ARROW_CONTENT_TYPE) {
        #[cfg(feature = "arrow")]
        return Ok(WireFormat::Arrow);

        #[cfg(not(feature = "arrow"))]
        return Err(Error::with_message(
            ErrorKind::DataConversion,
            "received an Apache Arrow response, but the `arrow` feature is disabled",
        ));
    }
    if media_type.eq_ignore_ascii_case(XML_CONTENT_TYPE) {
        Ok(WireFormat::Xml)
    } else {
        Err(Error::with_message(
            ErrorKind::DataConversion,
            format!("unsupported list blobs Content-Type: {content_type}"),
        ))
    }
}

pub(crate) fn decode_next_marker(headers: &Headers, bytes: &[u8]) -> Result<Option<String>> {
    match wire_format(headers)? {
        #[cfg(feature = "arrow")]
        WireFormat::Arrow => decode_arrow_next_marker(bytes),
        WireFormat::Xml => decode_xml_next_marker(bytes),
    }
}

fn decode_xml_next_marker(bytes: &[u8]) -> Result<Option<String>> {
    #[derive(Deserialize)]
    struct ListBlobsPage {
        #[serde(rename = "NextMarker")]
        next_marker: Option<String>,
    }

    let page: ListBlobsPage = azure_core::xml::from_xml(bytes)?;
    Ok(page.next_marker.filter(|marker| !marker.is_empty()))
}

/// Selects the list blobs deserializer from the response `Content-Type`.
#[derive(Debug, Clone)]
pub struct AutoFormat;

impl Format for AutoFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> Result<T> {
        azure_core::xml::from_xml(body.as_ref())
    }
}

impl DeserializeWith<AutoFormat> for ListBlobsResponse {
    fn deserialize_with(body: ResponseBody) -> Result<Self> {
        body.xml()
    }

    fn deserialize_from(response: RawResponse) -> Result<Self> {
        match wire_format(response.headers())? {
            #[cfg(feature = "arrow")]
            WireFormat::Arrow => decode_arrow_list_blobs(response.body()),
            WireFormat::Xml => azure_core::xml::from_xml(response.body()),
        }
    }
}

impl DeserializeWith<AutoFormat> for ListBlobsHierarchicalResponse {
    fn deserialize_with(body: ResponseBody) -> Result<Self> {
        body.xml()
    }

    fn deserialize_from(response: RawResponse) -> Result<Self> {
        match wire_format(response.headers())? {
            #[cfg(feature = "arrow")]
            WireFormat::Arrow => decode_arrow_list_blobs_hierarchy(response.body()),
            WireFormat::Xml => azure_core::xml::from_xml(response.body()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_content_type_defaults_to_xml() {
        assert!(matches!(wire_format(&Headers::new()), Ok(WireFormat::Xml)));
    }
}
