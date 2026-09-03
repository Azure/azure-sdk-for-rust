// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

pub use crate::generated::clients::{BlobContainerClient, BlobContainerClientOptions};

use crate::{
    models::{
        decode_next_marker, AutoFormat, BlobContainerClientListBlobsHierarchicalOptions,
        BlobContainerClientListBlobsOptions, ListBlobsHierarchicalResponse, ListBlobsResponse,
        StorageErrorCode,
    },
    BlobClient,
};
use azure_core::{
    credentials::TokenCredential,
    error::ErrorKind,
    http::{
        pager::{PagerContinuation, PagerResult, PagerState},
        policies::{auth::BearerTokenAuthorizationPolicy, Policy},
        ClientMethodOptions, Pager, Pipeline, RawResponse, StatusCode, Url,
    },
    tracing, Result,
};
use std::sync::Arc;

#[cfg(feature = "arrow")]
const LIST_BLOBS_ACCEPT: &str = "application/vnd.apache.arrow.stream,application/xml";
#[cfg(not(feature = "arrow"))]
const LIST_BLOBS_ACCEPT: &str = "application/xml";

impl BlobContainerClient {
    /// Creates a new BlobContainerClient from a container URL.
    ///
    /// # Arguments
    ///
    /// * `container_url` - The full URL of the container, for example `https://myaccount.blob.core.windows.net/mycontainer`.
    ///   The caller is responsible for percent-encoding the URL correctly; it will be used as-is.
    /// * `credential` - An optional implementation of [`TokenCredential`] that can provide an Entra ID token to use when authenticating.
    /// * `options` - Optional configuration for the client.
    #[tracing::new("Storage.Blob.Container")]
    pub fn new(
        container_url: Url,
        credential: Option<Arc<dyn TokenCredential>>,
        options: Option<BlobContainerClientOptions>,
    ) -> Result<Self> {
        // Storage endpoints must be base URLs.
        if container_url.cannot_be_a_base() {
            return Err(azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                format!("{container_url} is not a valid base URL"),
            ));
        }

        let mut options = options.unwrap_or_default();
        super::apply_client_defaults(&mut options.client_options);

        let mut per_retry_policies: Vec<Arc<dyn Policy>> = Vec::default();
        if let Some(token_credential) = credential {
            if !container_url.scheme().starts_with("https") {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!("{container_url} must use https"),
                ));
            }
            per_retry_policies.push(Arc::new(BearerTokenAuthorizationPolicy::new(
                token_credential,
                vec!["https://storage.azure.com/.default"],
            )));
        }

        let pipeline = Pipeline::new(
            option_env!("CARGO_PKG_NAME"),
            option_env!("CARGO_PKG_VERSION"),
            options.client_options.clone(),
            Vec::default(),
            per_retry_policies,
            None,
        );

        Ok(Self {
            endpoint: container_url,
            version: options.version,
            pipeline,
        })
    }

    /// Returns a new instance of BlobClient.
    ///
    /// # Arguments
    ///
    /// * `blob_name` - The name of the blob.
    pub fn blob_client(&self, blob_name: &str) -> BlobClient {
        let mut blob_url = self.url().clone();
        blob_url
            .path_segments_mut()
            // This should not fail as container URL has already been validated on client construction.
            .expect("Invalid endpoint URL: Cannot append blob_name to the blob endpoint.")
            .extend([blob_name]);

        BlobClient {
            endpoint: blob_url,
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        }
    }

    /// Gets the URL of the container.
    pub fn url(&self) -> &Url {
        &self.endpoint
    }

    /// Checks if the container exists.
    ///
    /// Returns `true` if the container exists, `false` if the container does not exist, and propagates all other errors.
    pub async fn exists(&self) -> Result<bool> {
        match self.get_properties(None).await {
            Ok(_) => Ok(true),
            Err(e) if e.http_status() == Some(StatusCode::NotFound) => match e.kind() {
                ErrorKind::HttpResponse {
                    error_code: Some(error_code),
                    ..
                } if error_code == StorageErrorCode::ContainerNotFound.as_ref() => Ok(false),
                // Propagate all other error types.
                _ => Err(e),
            },
            Err(e) => Err(e),
        }
    }

    /// Returns a list of the blobs in the specified container.
    ///
    /// Requests Apache Arrow with XML fallback when the `arrow` feature is enabled and XML only
    /// otherwise.
    ///
    /// # Arguments
    ///
    /// * `options` - Optional parameters for the request.
    ///
    pub fn list_blobs(
        &self,
        options: Option<BlobContainerClientListBlobsOptions<'_>>,
    ) -> Result<Pager<ListBlobsResponse, AutoFormat>> {
        let options = options.unwrap_or_default().into_owned();
        #[cfg(not(feature = "arrow"))]
        if options.end_before.is_some() {
            return Err(azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "end_before requires the `arrow` feature",
            ));
        }
        let pager_options = options.method_options.clone();
        let client = Arc::new(BlobContainerClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        });

        Ok(Pager::new(
            move |state: PagerState, pager_options| {
                let client = client.clone();
                let mut options = options.to_internal(ClientMethodOptions {
                    context: pager_options.context,
                });
                if let PagerState::More(continuation) = state {
                    options.marker = Some(continuation.into());
                }
                Box::pin(async move {
                    let response = client
                        .list_blobs_internal(LIST_BLOBS_ACCEPT.to_string(), Some(options))
                        .await?;
                    let (status, headers, body) = response.deconstruct();
                    let body = body.collect().await?;
                    let next_marker = decode_next_marker(&headers, &body)?;
                    let response = RawResponse::from_bytes(status, headers, body).into();
                    Ok(match next_marker {
                        Some(next_marker) => PagerResult::More {
                            response,
                            continuation: PagerContinuation::Token(next_marker),
                        },
                        None => PagerResult::Done { response },
                    })
                })
            },
            Some(pager_options),
        ))
    }

    /// Returns a list of the blobs in the specified container, grouping blobs under virtual
    /// directories using `delimiter`.
    ///
    /// Virtual directories are returned as [`BlobPrefix`](crate::models::BlobPrefix) entries on the
    /// page's [`hierarchical_list`](crate::models::BlobHierarchyList). Requests Apache Arrow with
    /// XML fallback when the `arrow` feature is enabled and XML only otherwise.
    ///
    /// # Arguments
    ///
    /// * `delimiter` - Groups blobs whose names share a common substring up to this separator into
    ///   a single `BlobPrefix` placeholder.
    /// * `options` - Optional parameters for the request.
    pub fn list_blobs_hierarchical(
        &self,
        delimiter: &str,
        options: Option<BlobContainerClientListBlobsHierarchicalOptions<'_>>,
    ) -> Result<Pager<ListBlobsHierarchicalResponse, AutoFormat>> {
        let options = options.unwrap_or_default().into_owned();
        #[cfg(not(feature = "arrow"))]
        if options.end_before.is_some() {
            return Err(azure_core::Error::with_message(
                ErrorKind::DataConversion,
                "end_before requires the `arrow` feature",
            ));
        }
        let delimiter = delimiter.to_string();
        let pager_options = options.method_options.clone();
        let client = Arc::new(BlobContainerClient {
            endpoint: self.endpoint.clone(),
            pipeline: self.pipeline.clone(),
            version: self.version.clone(),
            tracer: self.tracer.clone(),
        });

        Ok(Pager::new(
            move |state: PagerState, pager_options| {
                let client = client.clone();
                let delimiter = delimiter.clone();
                let mut options = options.to_internal(ClientMethodOptions {
                    context: pager_options.context,
                });
                if let PagerState::More(continuation) = state {
                    options.marker = Some(continuation.into());
                }
                Box::pin(async move {
                    let response = client
                        .list_blobs_hierarchical_internal(
                            LIST_BLOBS_ACCEPT.to_string(),
                            &delimiter,
                            Some(options),
                        )
                        .await?;
                    let (status, headers, body) = response.deconstruct();
                    let body = body.collect().await?;
                    let next_marker = decode_next_marker(&headers, &body)?;
                    let response = RawResponse::from_bytes(status, headers, body).into();
                    Ok(match next_marker {
                        Some(next_marker) => PagerResult::More {
                            response,
                            continuation: PagerContinuation::Token(next_marker),
                        },
                        None => PagerResult::Done { response },
                    })
                })
            },
            Some(pager_options),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(feature = "arrow")]
    use arrow_array::{builder::StringBuilder, RecordBatch};
    #[cfg(feature = "arrow")]
    use arrow_ipc::writer::StreamWriter;
    #[cfg(feature = "arrow")]
    use arrow_schema::{DataType, Field, Schema};
    #[cfg(feature = "arrow")]
    use azure_core::http::pager::PagerOptions;
    use azure_core::{
        http::{
            headers::{Headers, ACCEPT, CONTENT_TYPE},
            pager::PagerContinuation,
            AsyncRawResponse, ClientOptions, StatusCode, Transport,
        },
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use futures::{FutureExt as _, TryStreamExt as _};
    #[cfg(feature = "arrow")]
    use std::collections::HashMap;
    use std::sync::Arc;

    const LIST_BLOBS_PAGE: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://example.blob.core.windows.net/" ContainerName="container">
  <Blobs>
    <Blob>
      <Name>blob1</Name>
      <Properties>
        <BlobType>BlockBlob</BlobType>
      </Properties>
    </Blob>
  </Blobs>
  <NextMarker>page-2</NextMarker>
</EnumerationResults>"#;

    const XML_PAGE_1: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://example.blob.core.windows.net/" ContainerName="container">
  <Blobs>
    <Blob><Name>page1-a.txt</Name><Properties><BlobType>BlockBlob</BlobType></Properties></Blob>
    <Blob><Name>page1-b.txt</Name><Properties><BlobType>BlockBlob</BlobType></Properties></Blob>
  </Blobs>
  <NextMarker>page2</NextMarker>
</EnumerationResults>"#;

    const XML_PAGE_2: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://example.blob.core.windows.net/" ContainerName="container">
  <Blobs>
    <Blob><Name>page2-a.txt</Name><Properties><BlobType>BlockBlob</BlobType></Properties></Blob>
    <Blob><Name>page2-b.txt</Name><Properties><BlobType>BlockBlob</BlobType></Properties></Blob>
  </Blobs>
</EnumerationResults>"#;

    #[test]
    fn from_url_rejects_cannot_be_a_base_url() {
        let url = Url::parse("data:text/plain,hello").unwrap();
        assert!(BlobContainerClient::new(url, None, None).is_err());
    }

    #[test]
    fn from_url_accepts_http_without_credential() {
        let url = Url::parse("http://127.0.0.1:10000/devstoreaccount1/container").unwrap();
        let container = BlobContainerClient::new(url, None, None).unwrap();
        assert_eq!(
            container.blob_client("blob").url().path(),
            "/devstoreaccount1/container/blob"
        );
    }

    #[test]
    fn from_url_accepts_https_custom_hostname() {
        // CDN / Front Door / private endpoint hostnames are still https URLs.
        let url = Url::parse("https://cdn.contoso.com/container").unwrap();
        assert!(BlobContainerClient::new(url, None, None).is_ok());
    }

    #[tokio::test]
    async fn list_blobs_page_keeps_body_for_into_model() -> Result<()> {
        let mock_client = Arc::new(MockHttpClient::new(|req| {
            assert_eq!(req.url().path(), "/container");
            assert!(req
                .url()
                .query()
                .is_some_and(|query| query.contains("comp=list")));
            async move {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    Bytes::from_static(LIST_BLOBS_PAGE),
                ))
            }
            .boxed()
        }));
        let client = container_client_with(mock_client);

        let mut pages = client.list_blobs(None)?.into_pages();
        let page = pages.try_next().await?.expect("expected a page");

        assert!(matches!(
            pages.continuation(),
            Some(PagerContinuation::Token(token)) if token == "page-2"
        ));

        let page = page.into_model()?;
        assert_eq!(page.next_marker.as_deref(), Some("page-2"));
        assert_eq!(page.blob_items.len(), 1);
        assert_eq!(page.blob_items[0].name.as_deref(), Some("blob1"));

        Ok(())
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_mock_arrow_all_pages() -> Result<()> {
        let client = container_client_with(arrow_mock_client());
        let names = collect_blob_names(client.list_blobs(None)?).await?;
        assert_eq!(
            names,
            ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
        );
        Ok(())
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_mock_arrow_xml_fallback() -> Result<()> {
        // Arrow is requested (the default), but the service replies with XML; the pager must
        // still decode every blob across both pages via the XML fallback path.
        let client = container_client_with(xml_mock_client_with_accept(
            "application/vnd.apache.arrow.stream,application/xml",
        ));
        let names = collect_blob_names(client.list_blobs(None)?).await?;
        assert_eq!(
            names,
            ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
        );
        Ok(())
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_mock_arrow_from_continuation() -> Result<()> {
        let client = container_client_with(arrow_mock_client());
        let options = BlobContainerClientListBlobsOptions {
            method_options: PagerOptions {
                continuation: Some(PagerContinuation::Token("page2".into())),
                ..Default::default()
            },
            ..Default::default()
        };
        let names = collect_blob_names(client.list_blobs(Some(options))?).await?;
        assert_eq!(names, ["page2-a.txt", "page2-b.txt"]);
        Ok(())
    }

    #[cfg(not(feature = "arrow"))]
    #[tokio::test]
    async fn list_blobs_mock_defaults_to_xml_without_arrow() -> Result<()> {
        let client = container_client_with(xml_mock_client_with_accept("application/xml"));
        let names = collect_blob_names(client.list_blobs(None)?).await?;
        assert_eq!(
            names,
            ["page1-a.txt", "page1-b.txt", "page2-a.txt", "page2-b.txt"]
        );
        Ok(())
    }

    #[cfg(not(feature = "arrow"))]
    #[test]
    fn list_blobs_rejects_end_before_without_arrow() {
        let client = container_client_with(xml_mock_client_with_accept("application/xml"));
        let options = BlobContainerClientListBlobsOptions {
            end_before: Some("cc.txt".to_string()),
            ..Default::default()
        };
        assert!(client.list_blobs(Some(options)).is_err());
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_mock_arrow_sends_end_before() -> Result<()> {
        // The end_before option flows out as the `endBefore` query parameter.
        let client = container_client_with(Arc::new(MockHttpClient::new(|req| {
            assert!(req
                .url()
                .query_pairs()
                .any(|(k, v)| k == "endBefore" && v == "cc.txt"));
            let body = build_arrow_list_blobs(&["aa.txt"], None);
            async move {
                let mut headers = Headers::new();
                headers.insert(CONTENT_TYPE, "application/vnd.apache.arrow.stream");
                Ok(AsyncRawResponse::from_bytes(StatusCode::Ok, headers, body))
            }
            .boxed()
        })));
        let options = BlobContainerClientListBlobsOptions {
            end_before: Some("cc.txt".to_string()),
            ..Default::default()
        };
        let names = collect_blob_names(client.list_blobs(Some(options))?).await?;
        assert_eq!(names, ["aa.txt"]);
        Ok(())
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_mock_arrow_drops_envelope() -> Result<()> {
        // Arrow carries only blob rows and the next marker; envelope fields stay None.
        let client = container_client_with(arrow_mock_client());
        let page = client
            .list_blobs(None)?
            .into_pages()
            .try_next()
            .await?
            .expect("expected a page")
            .into_model()?;
        assert!(page.container_name.is_none());
        assert!(page.prefix.is_none());
        assert!(page.max_results.is_none());
        assert!(page.service_endpoint.is_none());
        Ok(())
    }

    fn container_client_with(mock: Arc<dyn azure_core::http::HttpClient>) -> BlobContainerClient {
        BlobContainerClient::new(
            Url::parse("https://example.blob.core.windows.net/container").unwrap(),
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

    async fn collect_blob_names(
        pager: Pager<ListBlobsResponse, AutoFormat>,
    ) -> Result<Vec<String>> {
        let mut pages = pager.into_pages();
        let mut names = Vec::new();
        while let Some(page) = pages.try_next().await? {
            let model = page.into_model()?;
            names.extend(model.blob_items.into_iter().filter_map(|b| b.name));
        }
        Ok(names)
    }

    #[cfg(feature = "arrow")]
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
        let batch = RecordBatch::try_new(schema.clone(), vec![Arc::new(builder.finish())])
            .expect("valid batch");
        let mut buf = Vec::new();
        let mut writer = StreamWriter::try_new(&mut buf, &schema).expect("valid writer");
        writer.write(&batch).expect("write batch");
        writer.finish().expect("finish");
        Bytes::from(buf)
    }

    #[cfg(feature = "arrow")]
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

    const XML_HIERARCHY_PAGE: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="https://example.blob.core.windows.net/" ContainerName="container">
  <Delimiter>/</Delimiter>
  <Blobs>
    <BlobPrefix><Name>dir1/</Name></BlobPrefix>
    <Blob><Name>top.txt</Name><Properties><BlobType>BlockBlob</BlobType></Properties></Blob>
  </Blobs>
</EnumerationResults>"#;

    #[cfg(feature = "arrow")]
    fn build_arrow_hierarchy(
        blobs: &[&str],
        prefixes: &[&str],
        next_marker: Option<&str>,
    ) -> Bytes {
        let metadata: HashMap<String, String> = next_marker
            .map(|m| HashMap::from([("NextMarker".to_string(), m.to_string())]))
            .unwrap_or_default();
        let schema = Arc::new(Schema::new_with_metadata(
            vec![
                Field::new("Name", DataType::Utf8, true),
                Field::new("ResourceType", DataType::Utf8, true),
            ],
            metadata,
        ));
        let mut names = StringBuilder::new();
        let mut resource_types = StringBuilder::new();
        for prefix in prefixes {
            names.append_value(prefix);
            resource_types.append_value("blobprefix");
        }
        for blob in blobs {
            names.append_value(blob);
            resource_types.append_value("blob");
        }
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![Arc::new(names.finish()), Arc::new(resource_types.finish())],
        )
        .expect("valid batch");
        let mut buf = Vec::new();
        let mut writer = StreamWriter::try_new(&mut buf, &schema).expect("valid writer");
        writer.write(&batch).expect("write batch");
        writer.finish().expect("finish");
        Bytes::from(buf)
    }

    #[cfg(feature = "arrow")]
    fn arrow_hierarchy_mock_client() -> Arc<dyn azure_core::http::HttpClient> {
        let page = build_arrow_hierarchy(&["top.txt"], &["dir1/", "dir2/"], None);
        Arc::new(MockHttpClient::new(move |req| {
            assert_eq!(
                req.headers().get_str(&ACCEPT).unwrap(),
                "application/vnd.apache.arrow.stream,application/xml"
            );
            assert!(req
                .url()
                .query_pairs()
                .any(|(k, v)| k == "delimiter" && v == "/"));
            let page = page.clone();
            async move {
                let mut headers = Headers::new();
                headers.insert(CONTENT_TYPE, "application/vnd.apache.arrow.stream");
                Ok(AsyncRawResponse::from_bytes(StatusCode::Ok, headers, page))
            }
            .boxed()
        }))
    }

    fn xml_hierarchy_mock_client(accept: &'static str) -> Arc<dyn azure_core::http::HttpClient> {
        Arc::new(MockHttpClient::new(move |req| {
            assert_eq!(req.headers().get_str(&ACCEPT).unwrap(), accept);
            assert!(req
                .url()
                .query_pairs()
                .any(|(key, value)| key == "delimiter" && value == "/"));
            async move {
                let mut headers = Headers::new();
                headers.insert(CONTENT_TYPE, "application/xml");
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    headers,
                    Bytes::from_static(XML_HIERARCHY_PAGE),
                ))
            }
            .boxed()
        }))
    }

    #[cfg(not(feature = "arrow"))]
    #[tokio::test]
    async fn list_blobs_hierarchical_mock_defaults_to_xml_without_arrow() -> Result<()> {
        let client = container_client_with(xml_hierarchy_mock_client("application/xml"));
        let page = client
            .list_blobs_hierarchical("/", None)?
            .into_pages()
            .try_next()
            .await?
            .expect("expected a page")
            .into_model()?;

        assert_eq!(page.delimiter.as_deref(), Some("/"));
        assert_eq!(page.container_name.as_deref(), Some("container"));
        let prefixes = page
            .hierarchical_list
            .blob_prefixes
            .as_deref()
            .expect("expected blob prefixes");
        assert_eq!(prefixes.len(), 1);
        assert_eq!(prefixes[0].name.as_deref(), Some("dir1/"));
        assert_eq!(
            page.hierarchical_list.blob_items[0].name.as_deref(),
            Some("top.txt")
        );
        Ok(())
    }

    #[cfg(feature = "arrow")]
    fn arrow_hierarchy_mock_client_paged() -> Arc<dyn azure_core::http::HttpClient> {
        let page1 = build_arrow_hierarchy(&["a.txt"], &["dir1/"], Some("page2"));
        let page2 = build_arrow_hierarchy(&["b.txt"], &["dir2/"], None);
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

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_hierarchical_mock_arrow() -> Result<()> {
        let client = container_client_with(arrow_hierarchy_mock_client());
        let page = client
            .list_blobs_hierarchical("/", None)?
            .into_pages()
            .try_next()
            .await?
            .expect("expected a page")
            .into_model()?;

        let blobs: Vec<_> = page
            .hierarchical_list
            .blob_items
            .iter()
            .filter_map(|b| b.name.as_deref())
            .collect();
        assert_eq!(blobs, ["top.txt"]);

        let prefixes = page
            .hierarchical_list
            .blob_prefixes
            .expect("prefixes should be present");
        let prefix_names: Vec<_> = prefixes.iter().filter_map(|p| p.name.as_deref()).collect();
        assert_eq!(prefix_names, ["dir1/", "dir2/"]);

        // Arrow omits the response envelope fields, including the delimiter.
        assert!(page.delimiter.is_none());
        assert!(page.container_name.is_none());
        assert!(page.prefix.is_none());
        Ok(())
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_hierarchical_mock_arrow_xml_fallback() -> Result<()> {
        // Arrow requested (default) but the service replies with XML; prefixes, blobs, and the
        // delimiter (which Arrow omits) all decode via the XML fallback.
        let client = container_client_with(xml_hierarchy_mock_client(
            "application/vnd.apache.arrow.stream,application/xml",
        ));
        let page = client
            .list_blobs_hierarchical("/", None)?
            .into_pages()
            .try_next()
            .await?
            .expect("expected a page")
            .into_model()?;

        assert_eq!(page.delimiter.as_deref(), Some("/"));
        let prefixes = page
            .hierarchical_list
            .blob_prefixes
            .expect("prefixes should be present");
        assert_eq!(prefixes[0].name.as_deref(), Some("dir1/"));
        assert_eq!(
            page.hierarchical_list.blob_items[0].name.as_deref(),
            Some("top.txt")
        );
        Ok(())
    }

    #[cfg(feature = "arrow")]
    #[tokio::test]
    async fn list_blobs_hierarchical_mock_arrow_all_pages() -> Result<()> {
        let client = container_client_with(arrow_hierarchy_mock_client_paged());
        let mut pages = client.list_blobs_hierarchical("/", None)?.into_pages();

        let mut blobs = Vec::new();
        let mut prefixes = Vec::new();
        while let Some(page) = pages.try_next().await? {
            let page = page.into_model()?;
            blobs.extend(
                page.hierarchical_list
                    .blob_items
                    .into_iter()
                    .filter_map(|b| b.name),
            );
            if let Some(page_prefixes) = page.hierarchical_list.blob_prefixes {
                prefixes.extend(page_prefixes.into_iter().filter_map(|p| p.name));
            }
        }

        // Blobs and prefixes from both pages aggregate across the NextMarker boundary.
        assert_eq!(blobs, ["a.txt", "b.txt"]);
        assert_eq!(prefixes, ["dir1/", "dir2/"]);
        Ok(())
    }
}
