// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use arrow::{
    array::builder::StringBuilder,
    datatypes::{DataType, Field, Schema},
    ipc::writer::StreamWriter,
};
use azure_core::{
    http::{
        headers::{Headers, ACCEPT, CONTENT_TYPE},
        pager::{PagerContinuation, PagerOptions},
        AsyncRawResponse, ClientOptions, StatusCode, Transport, Url,
    },
    Bytes,
};
use azure_core_test::http::MockHttpClient;
use azure_storage_blob::{
    models::{BlobContainerClientListBlobsOptions, BlobItem, StorageResponseFormat},
    BlobContainerClient, BlobContainerClientOptions,
};
use futures::{FutureExt as _, TryStreamExt as _};
use std::{collections::HashMap, error::Error, sync::Arc};

#[tokio::test]
async fn test_list_blobs_mock_xml() -> Result<(), Box<dyn Error>> {
    let client = container_client_with(xml_mock_client());

    let all_blobs: Vec<BlobItem> = client.list_blobs(None)?.into_stream().try_collect().await?;
    let names: Vec<_> = all_blobs.iter().filter_map(|b| b.name.as_deref()).collect();

    assert_eq!(
        names,
        ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
    );
    Ok(())
}

#[tokio::test]
async fn test_list_blobs_mock_explicit_xml() -> Result<(), Box<dyn Error>> {
    let client = container_client_with(xml_mock_client());

    let options = BlobContainerClientListBlobsOptions {
        response_format: Some(StorageResponseFormat::Xml),
        ..Default::default()
    };
    let all_blobs: Vec<BlobItem> = client
        .list_blobs(Some(options))?
        .into_stream()
        .try_collect()
        .await?;
    let names: Vec<_> = all_blobs.iter().filter_map(|b| b.name.as_deref()).collect();

    assert_eq!(
        names,
        ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
    );
    Ok(())
}

#[tokio::test]
async fn test_list_blobs_mock_arrow() -> Result<(), Box<dyn Error>> {
    let client = container_client_with(arrow_mock_client());

    let options = BlobContainerClientListBlobsOptions {
        response_format: Some(StorageResponseFormat::Arrow),
        ..Default::default()
    };
    let all_blobs: Vec<BlobItem> = client
        .list_blobs(Some(options))?
        .into_stream()
        .try_collect()
        .await?;
    let names: Vec<_> = all_blobs.iter().filter_map(|b| b.name.as_deref()).collect();

    assert_eq!(
        names,
        ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
    );
    Ok(())
}

#[tokio::test]
async fn test_list_blobs_mock_arrow_from_continuation() -> Result<(), Box<dyn Error>> {
    let client = container_client_with(arrow_mock_client());

    let options = BlobContainerClientListBlobsOptions {
        method_options: PagerOptions {
            continuation: Some(PagerContinuation::Token("page2".into())),
            ..Default::default()
        },
        response_format: Some(StorageResponseFormat::Arrow),
        ..Default::default()
    };
    let all_blobs: Vec<BlobItem> = client
        .list_blobs(Some(options))?
        .into_stream()
        .try_collect()
        .await?;
    let names: Vec<_> = all_blobs.iter().filter_map(|b| b.name.as_deref()).collect();

    assert_eq!(names, ["page2-a.txt", "page2-b.txt"]);
    Ok(())
}

#[tokio::test]
async fn test_list_blobs_mock_arrow_with_xml_fallback() -> Result<(), Box<dyn Error>> {
    let client = container_client_with(xml_mock_client_with_accept(
        "application/vnd.apache.arrow.stream,application/xml",
    ));

    let options = BlobContainerClientListBlobsOptions {
        response_format: Some(StorageResponseFormat::Arrow),
        ..Default::default()
    };
    let all_blobs: Vec<BlobItem> = client
        .list_blobs(Some(options))?
        .into_stream()
        .try_collect()
        .await?;
    let names: Vec<_> = all_blobs.iter().filter_map(|b| b.name.as_deref()).collect();

    assert_eq!(
        names,
        ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
    );
    Ok(())
}

// --- Setup helpers ---

fn container_client_with(mock: Arc<dyn azure_core::http::HttpClient>) -> BlobContainerClient {
    BlobContainerClient::new(
        Url::parse("https://account.blob.core.windows.net/container").unwrap(),
        None,
        Some(BlobContainerClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(mock)),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
    .unwrap()
}

fn xml_mock_client() -> Arc<dyn azure_core::http::HttpClient> {
    xml_mock_client_with_accept("application/xml")
}

fn xml_mock_client_with_accept(accept: &'static str) -> Arc<dyn azure_core::http::HttpClient> {
    Arc::new(MockHttpClient::new(move |req| {
        assert_eq!(req.headers().get_str(&ACCEPT).unwrap(), accept);
        let is_page2 = req
            .url()
            .query_pairs()
            .any(|(k, v)| k == "marker" && v == "page2");
        async move {
            let mut headers = Headers::new();
            headers.insert(CONTENT_TYPE, "application/xml");
            let body = if is_page2 { XML_PAGE_2 } else { XML_PAGE_1 };
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::Ok,
                headers,
                Bytes::from_static(body),
            ))
        }
        .boxed()
    }))
}

fn arrow_mock_client() -> Arc<dyn azure_core::http::HttpClient> {
    let page1 = build_arrow_list_blobs(&["page1-a.txt", "page1-b.txt"], Some("page2"));
    let page2 = build_arrow_list_blobs(&["page2-a.txt", "page2-b.txt"], None);
    Arc::new(MockHttpClient::new(move |req| {
        assert_eq!(
            req.headers().get_str(&ACCEPT).unwrap(),
            "application/vnd.apache.arrow.stream,application/xml"
        );
        let is_page2 = req
            .url()
            .query_pairs()
            .any(|(k, v)| k == "marker" && v == "page2");
        let body = if is_page2 {
            page2.clone()
        } else {
            page1.clone()
        };
        async move {
            let mut headers = Headers::new();
            headers.insert(CONTENT_TYPE, "application/vnd.apache.arrow.stream");
            Ok(AsyncRawResponse::from_bytes(StatusCode::Ok, headers, body))
        }
        .boxed()
    }))
}

fn build_arrow_list_blobs(names: &[&str], next_marker: Option<&str>) -> Bytes {
    let metadata: HashMap<String, String> = next_marker
        .map(|m| HashMap::from([("NextMarker".to_string(), m.to_string())]))
        .unwrap_or_default();
    let schema = Arc::new(Schema::new_with_metadata(
        vec![Field::new("Name", DataType::Utf8, true)],
        metadata,
    ));
    let mut builder = StringBuilder::new();
    for name in names {
        builder.append_value(name);
    }
    let batch =
        arrow::array::RecordBatch::try_new(schema.clone(), vec![Arc::new(builder.finish())])
            .expect("valid batch");

    let mut buf = Vec::new();
    let mut writer = StreamWriter::try_new(&mut buf, &schema).expect("valid writer");
    writer.write(&batch).expect("write batch");
    writer.finish().expect("finish");
    Bytes::from(buf)
}

const XML_PAGE_1: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://account.blob.core.windows.net/" ContainerName="container">
  <Blobs>
    <Blob>
      <Name>page1-a.txt</Name>
      <Properties><BlobType>BlockBlob</BlobType></Properties>
    </Blob>
    <Blob>
      <Name>page1-b.txt</Name>
      <Properties><BlobType>BlockBlob</BlobType></Properties>
    </Blob>
  </Blobs>
  <NextMarker>page2</NextMarker>
</EnumerationResults>"#;

const XML_PAGE_2: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://account.blob.core.windows.net/" ContainerName="container">
  <Blobs>
    <Blob>
      <Name>page2-a.txt</Name>
      <Properties><BlobType>BlockBlob</BlobType></Properties>
    </Blob>
    <Blob>
      <Name>page2-b.txt</Name>
      <Properties><BlobType>BlockBlob</BlobType></Properties>
    </Blob>
  </Blobs>
</EnumerationResults>"#;
