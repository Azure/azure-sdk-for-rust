// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub mod json;
#[cfg(feature = "xml")]
pub mod xml;

use std::sync::Arc;

use azure_core::{
    base64::{
        self,
        option::{deserialize, serialize},
    },
    http::{headers::Headers, AsyncRawResponse, ClientOptions, Pipeline, StatusCode, Transport},
    time::{self, OffsetDateTime},
    Bytes,
};
use azure_core_test::http::MockHttpClient;
use futures::FutureExt as _;
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

fn create_pipeline<F>(count: usize, f: F) -> azure_core::Result<Pipeline>
where
    F: Fn(&List) -> azure_core::Result<Bytes>,
{
    let mut list = List {
        name: Some("t0123456789abcdef".into()),
        ..Default::default()
    };
    let mut items = Vec::with_capacity(count);
    let now = OffsetDateTime::now_utc();
    for i in 0..count {
        let name = format!("testItem{i}");
        let hash = base64::encode(&name).into_bytes();
        items.push(ListItem {
            name: Some(name),
            properties: Some(ListItemProperties {
                etag: Some(i.to_string().into()),
                creation_time: Some(now),
                last_modified: Some(now),
                content_md5: Some(hash),
            }),
        });
    }
    list.container = Some(ListItemsContainer { items: Some(items) });

    let body = f(&list)?;
    println!("Serialized {count} items in {} bytes", body.len());

    let client = Arc::new(MockHttpClient::new(move |_| {
        let body = body.clone();
        async move {
            // Yield simulates an expected network call but kills performance by ~45%.
            tokio::task::yield_now().await;
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                Headers::new(),
                body,
            ))
        }
        .boxed()
    }));
    let options = ClientOptions {
        transport: Some(Transport::new(client)),
        ..Default::default()
    };
    let pipeline = Pipeline::new(
        Some("perf"),
        Some("0.1.0"),
        options,
        Vec::new(),
        Vec::new(),
        None,
    );
    Ok(pipeline)
}
