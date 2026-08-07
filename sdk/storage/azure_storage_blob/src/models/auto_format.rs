// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{
    headers::{self, Headers, CONTENT_TYPE},
    response::{RawResponse, ResponseBody},
    DeserializeWith, Format, XmlFormat,
};
use serde::{de::DeserializeOwned, Deserialize};

use crate::generated::models::ListBlobsResponse;

const ARROW_CONTENT_TYPE: &str = "application/vnd.apache.arrow.stream";

/// Selects deserialization based on the `content-type` header.
#[derive(Debug, Clone)]
pub struct AutoFormat;

impl Format for AutoFormat {
    fn deserialize<T: DeserializeOwned, S: AsRef<[u8]>>(body: S) -> azure_core::Result<T> {
        XmlFormat::deserialize(body)
    }
}

impl DeserializeWith<AutoFormat> for ListBlobsResponse {
    fn deserialize_with(body: ResponseBody) -> azure_core::Result<Self> {
        body.xml()
    }

    fn deserialize_from(response: &RawResponse) -> azure_core::Result<Self> {
        let ct = response
            .headers()
            .get_optional_str(&CONTENT_TYPE)
            .unwrap_or_default();
        let media_type = ct.split(';').next().unwrap_or_default().trim();
        if media_type.eq_ignore_ascii_case(ARROW_CONTENT_TYPE) {
            crate::arrow_decode::decode_arrow_list_blobs(response.body())
        } else {
            azure_core::xml::from_xml(response.body())
        }
    }
}

/// Extracts the continuation token from a list_blobs response in either Arrow or XML format.
pub(crate) fn decode_next_marker(
    headers: &Headers,
    bytes: &[u8],
) -> azure_core::Result<Option<String>> {
    if is_arrow(headers) {
        crate::arrow_decode::arrow_next_marker(bytes)
    } else {
        #[derive(Deserialize)]
        struct ListBlobsPage {
            #[serde(rename = "NextMarker")]
            next_marker: Option<String>,
        }
        let page: ListBlobsPage = azure_core::xml::from_xml(bytes)?;
        Ok(page.next_marker.filter(|m| !m.is_empty()))
    }
}

fn is_arrow(headers: &Headers) -> bool {
    headers
        .get_optional_str(&headers::CONTENT_TYPE)
        .is_some_and(|ct| {
            ct.split(';')
                .next()
                .unwrap_or_default()
                .trim()
                .eq_ignore_ascii_case(ARROW_CONTENT_TYPE)
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers_with_content_type(ct: &str) -> Headers {
        let mut headers = Headers::new();
        headers.insert(CONTENT_TYPE, ct.to_string());
        headers
    }

    #[test]
    fn decode_next_marker_xml_with_marker() {
        let headers = headers_with_content_type("application/xml");
        let body = br#"<?xml version="1.0" encoding="utf-8"?><EnumerationResults><NextMarker>token123</NextMarker></EnumerationResults>"#;
        let result = decode_next_marker(&headers, body).unwrap();
        assert_eq!(result, Some("token123".to_string()));
    }

    #[test]
    fn decode_next_marker_xml_empty_marker() {
        let headers = headers_with_content_type("application/xml");
        let body = br#"<?xml version="1.0" encoding="utf-8"?><EnumerationResults><NextMarker></NextMarker></EnumerationResults>"#;
        let result = decode_next_marker(&headers, body).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn decode_next_marker_xml_no_marker() {
        let headers = headers_with_content_type("application/xml");
        let body =
            br#"<?xml version="1.0" encoding="utf-8"?><EnumerationResults></EnumerationResults>"#;
        let result = decode_next_marker(&headers, body).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn is_arrow_detects_arrow_content_type() {
        let headers = headers_with_content_type("application/vnd.apache.arrow.stream");
        assert!(is_arrow(&headers));
    }

    #[test]
    fn is_arrow_with_charset_parameter() {
        let headers =
            headers_with_content_type("application/vnd.apache.arrow.stream; charset=utf-8");
        assert!(is_arrow(&headers));
    }

    #[test]
    fn is_arrow_xml_returns_false() {
        let headers = headers_with_content_type("application/xml");
        assert!(!is_arrow(&headers));
    }

    #[test]
    fn is_arrow_missing_header_returns_false() {
        let headers = Headers::new();
        assert!(!is_arrow(&headers));
    }
}
