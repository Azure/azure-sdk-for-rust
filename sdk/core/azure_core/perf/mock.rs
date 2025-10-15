// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod json;
#[cfg(feature = "xml")]
pub mod xml;

use azure_core::{
    base64::option::{deserialize, serialize},
    time::{self, OffsetDateTime},
};
use serde::{Deserialize, Serialize};

const DEFAULT_COUNT: usize = 25;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct List {
    #[serde(default, rename = "name")]
    name: Option<String>,

    #[serde(default, rename = "container")]
    container: Option<ListItemsContainer>,

    #[serde(default, rename = "next")]
    next: Option<String>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListItemsContainer {
    #[serde(default, rename = "items")]
    items: Option<Vec<ListItem>>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListItem {
    #[serde(default, rename = "name")]
    name: Option<String>,

    #[serde(default, rename = "properties")]
    properties: Option<ListItemProperties>,
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct ListItemProperties {
    #[serde(default, rename = "etag")]
    etag: Option<azure_core::http::Etag>,

    #[serde(default, rename = "creationTime", with = "time::rfc7231::option")]
    creation_time: Option<OffsetDateTime>,

    #[serde(default, rename = "lastModified", with = "time::rfc7231::option")]
    last_modified: Option<OffsetDateTime>,

    #[serde(
        default,
        rename = "contentMD5",
        serialize_with = "serialize",
        deserialize_with = "deserialize"
    )]
    content_md5: Option<Vec<u8>>,
}
