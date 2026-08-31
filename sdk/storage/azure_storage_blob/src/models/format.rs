// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::{
    arrow::{decode_arrow_list_blobs, decode_arrow_list_blobs_hierarchy, wire_format, WireFormat},
    models::{ListBlobsHierarchicalResponse, ListBlobsResponse},
};
use azure_core::{
    http::{response::ResponseBody, DeserializeWith, Format, RawResponse},
    Result,
};
use serde::de::DeserializeOwned;

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
            WireFormat::Arrow => decode_arrow_list_blobs_hierarchy(response.body()),
            WireFormat::Xml => azure_core::xml::from_xml(response.body()),
        }
    }
}
