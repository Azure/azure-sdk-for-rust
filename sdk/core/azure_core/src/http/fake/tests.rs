// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{get_api_name, AsyncResponseExt as _, ResponseExt as _};
use crate as azure_core;
use crate::{
    error::{check_success, ErrorKind, ErrorResponse},
    http::{
        headers::{HeaderName, Headers, CONTENT_TYPE, ERROR_CODE},
        pager::{PagerContinuation, PagerOptions, PagerResult, PagerState},
        policies::{Policy, PolicyResult},
        AsyncRawResponse, AsyncResponse, Body, ClientMethodOptions, ClientOptions, Context, Method,
        NoFormat, PageIterator, Pipeline, RawResponse, Request, RequestContent, Response,
        RetryOptions, StatusCode, Transport, Url, UrlExt as _,
    },
    json,
    tracing::{self, PublicApiInstrumentationInformation, Span, Tracer},
    Bytes, Error, Result,
};
use async_trait::async_trait;
use futures::{
    io::{AsyncReadExt as _, AsyncSeekExt as _},
    stream, TryStreamExt as _,
};
use serde::Deserialize;
use std::{
    borrow::Cow,
    collections::HashMap,
    io::SeekFrom,
    sync::{Arc, Mutex},
};

#[derive(Debug, Deserialize)]
struct Model {
    value: String,
}

#[derive(Default)]
struct Options<'a> {
    method_options: ClientMethodOptions<'a>,
    query_name: Option<Vec<String>>,
    header_name: Option<String>,
}

struct GetArguments {
    path_name: String,
    version: Option<String>,
    query_name: Option<Vec<String>>,
    header_name: Option<String>,
}

struct OpaqueBody;

#[derive(Default)]
struct ListOptions<'a> {
    method_options: PagerOptions<'a>,
    count: Option<i32>,
}

#[derive(Clone)]
struct ListArguments {
    name: String,
    count: Option<i32>,
}

#[tracing::client]
pub struct TestClient {
    endpoint: Url,
    pipeline: Pipeline,
}

impl TestClient {
    fn with_fake(
        endpoint: &str,
        server: Arc<dyn Server>,
        mut options: ClientOptions,
    ) -> Result<Self> {
        if options.transport.is_some() {
            return Err(Error::with_message(
                ErrorKind::Other,
                "transport already configured",
            ));
        }
        let endpoint = Url::parse(endpoint)?;
        options.transport = Some(Transport::with_policy(Arc::new(ServerTransport { server })));
        Ok(Self {
            endpoint,
            pipeline: Pipeline::new(None, None, options, Vec::new(), Vec::new(), None),
            tracer: None,
        })
    }

    #[tracing::function("Test.get")]
    pub async fn get(
        &self,
        name: &str,
        version: Option<&str>,
        options: Option<Options<'_>>,
    ) -> Result<Response<Model>> {
        let mut options = options.unwrap_or_default();
        options.method_options.context.insert(GetArguments {
            path_name: name.to_owned(),
            version: version.map(str::to_owned),
            query_name: options.query_name.clone(),
            header_name: options.header_name.clone(),
        });
        let mut url = self.endpoint.clone();
        url.append_path(&format!("/items/{name}"));
        if let Some(version) = version {
            url.query_pairs_mut().append_pair("version", version);
        }
        if let Some(names) = &options.query_name {
            for name in names {
                url.query_pairs_mut().append_pair("name", name);
            }
        }
        let mut request = Request::new(url, Method::Get);
        if let Some(name) = options.header_name {
            request.insert_header("name", name);
        }
        Ok(self
            .pipeline
            .send(&options.method_options.context, &mut request, None)
            .await?
            .into())
    }

    #[tracing::function("Test.put")]
    pub async fn put(
        &self,
        content: RequestContent<OpaqueBody, NoFormat>,
        options: Option<Options<'_>>,
    ) -> Result<Response<Model>> {
        let options = options.unwrap_or_default();
        let mut request = Request::new(self.endpoint.clone(), Method::Put);
        request.set_body(content);
        Ok(self
            .pipeline
            .send(&options.method_options.context, &mut request, None)
            .await?
            .into())
    }

    #[tracing::function("Test.list")]
    pub fn list(
        &self,
        name: &str,
        options: Option<ListOptions<'_>>,
    ) -> Result<PageIterator<RawResponse>> {
        let options = options.unwrap_or_default();
        let mut method_options = PagerOptions {
            context: options.method_options.context.into_owned(),
            continuation: options.method_options.continuation,
        };
        method_options.context.insert(ListArguments {
            name: name.to_owned(),
            count: options.count,
        });
        let first = self.endpoint.join("items")?;
        let pipeline = self.pipeline.clone();
        Ok(PageIterator::new(
            move |state, options| {
                let url = match state {
                    PagerState::Initial => first.clone(),
                    PagerState::More(PagerContinuation::Link(url)) => url,
                    _ => panic!("expected link continuation"),
                };
                let pipeline = pipeline.clone();
                Box::pin(async move {
                    let mut request = Request::new(url.clone(), Method::Get);
                    let response = pipeline.send(&options.context, &mut request, None).await?;
                    let body: serde_json::Value = json::from_json(response.body())?;
                    Ok(match body["nextLink"].as_str() {
                        Some(link) => PagerResult::More {
                            response,
                            continuation: PagerContinuation::Link(url.join(link)?),
                        },
                        None => PagerResult::Done { response },
                    })
                })
            },
            Some(method_options),
        ))
    }
}

#[async_trait]
trait Server: Send + Sync {
    async fn get(
        &self,
        _name: &str,
        _version: Option<&str>,
        _context: &Context<'_>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!()
    }

    async fn put(
        &self,
        _content: RequestContent<OpaqueBody, NoFormat>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!()
    }

    async fn list(
        &self,
        _arguments: &ListArguments,
        _state: PagerState,
        _context: &Context<'_>,
    ) -> Result<AsyncRawResponse> {
        unimplemented!()
    }
}

struct ServerTransport {
    server: Arc<dyn Server>,
}

impl std::fmt::Debug for ServerTransport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ServerTransport")
    }
}

#[async_trait]
impl Policy for ServerTransport {
    async fn send(
        &self,
        context: &Context,
        request: &mut Request,
        _next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        match get_api_name(context) {
            Some("Test.get") => {
                let arguments = context.value::<GetArguments>().ok_or_else(|| {
                    Error::with_message(ErrorKind::Other, "Test.get: missing operation arguments")
                })?;
                self.server
                    .get(&arguments.path_name, arguments.version.as_deref(), context)
                    .await
            }
            Some("Test.put") => self.server.put(request.body().clone().into()).await,
            Some("Test.list") => {
                let arguments = context.value::<ListArguments>().ok_or_else(|| {
                    Error::with_message(ErrorKind::Other, "Test.list: missing operation arguments")
                })?;
                let state = context
                    .value::<PagerState>()
                    .ok_or_else(|| {
                        Error::with_message(ErrorKind::Other, "Test.list: missing pager state")
                    })?
                    .clone();
                self.server.list(arguments, state, context).await
            }
            _ => Err(Error::with_message(
                ErrorKind::Other,
                "missing or unknown API name",
            )),
        }
    }
}

#[derive(Debug)]
struct PageCall {
    name: String,
    count: Option<i32>,
    state: PagerState,
}

#[derive(Default)]
struct TestServer {
    contexts: Mutex<Vec<Context<'static>>>,
    pages: Mutex<Vec<PageCall>>,
}

#[async_trait]
impl Server for TestServer {
    async fn get(
        &self,
        name: &str,
        version: Option<&str>,
        context: &Context<'_>,
    ) -> Result<AsyncRawResponse> {
        assert_eq!(get_api_name(context), Some("Test.get"));
        assert!(context.value::<Arc<dyn Tracer>>().is_none());
        assert!(context.value::<Arc<dyn Span>>().is_none());
        self.contexts
            .lock()
            .expect("context lock")
            .push(context.to_owned());
        match name {
            "connection-error" => Err(Error::with_message(
                ErrorKind::Io,
                "simulated connection failure",
            )),
            "missing" => AsyncRawResponse::fake_with_error(
                StatusCode::NotFound,
                Headers::new(),
                "MissingItem",
                "No such item",
            ),
            _ => Ok(AsyncRawResponse::fake_with_bytes(
                StatusCode::Ok,
                Headers::new(),
                format!(
                    r#"{{"value":{}}}"#,
                    serde_json::to_string(&format!("{name}:{}", version.unwrap_or_default()))?
                ),
            )),
        }
    }

    async fn put(&self, content: RequestContent<OpaqueBody, NoFormat>) -> Result<AsyncRawResponse> {
        let bytes = match content.body() {
            Body::Bytes(bytes) => bytes.to_vec(),
            Body::SeekableStream(stream) => {
                let mut stream = stream.clone();
                let mut bytes = Vec::new();
                stream.read_to_end(&mut bytes).await?;
                bytes
            }
        };
        let model: Model = json::from_json(bytes)?;
        Ok(AsyncRawResponse::fake_with_bytes(
            StatusCode::Ok,
            Headers::new(),
            format!(r#"{{"value":{}}}"#, serde_json::to_string(&model.value)?),
        ))
    }

    async fn list(
        &self,
        arguments: &ListArguments,
        state: PagerState,
        context: &Context<'_>,
    ) -> Result<AsyncRawResponse> {
        assert_eq!(get_api_name(context), Some("Test.list"));
        assert_eq!(context.value::<PagerState>(), Some(&state));
        assert_eq!(context.value::<u32>(), Some(&42));
        self.pages.lock().expect("page lock").push(PageCall {
            name: arguments.name.clone(),
            count: arguments.count,
            state: state.clone(),
        });
        Ok(AsyncRawResponse::fake_with_bytes(
            StatusCode::Ok,
            Headers::new(),
            match state {
                PagerState::Initial => r#"{"nextLink":"/opaque?cursor=next"}"#,
                PagerState::More(_) => "{}",
            },
        ))
    }
}

#[derive(Debug)]
struct AddContext;

#[async_trait]
impl Policy for AddContext {
    async fn send(
        &self,
        context: &Context,
        request: &mut Request,
        next: &[Arc<dyn Policy>],
    ) -> PolicyResult {
        let context = context.to_owned().with_value(42u32);
        next[0].send(&context, request, &next[1..]).await
    }
}

#[tokio::test]
async fn client_routes_original_arguments_and_preserves_policies_without_tracing() -> Result<()> {
    let server = Arc::new(TestServer::default());
    let client = TestClient::with_fake(
        "ftp://example.com/prefix",
        server.clone(),
        ClientOptions {
            per_call_policies: vec![Arc::new(AddContext)],
            ..Default::default()
        },
    )?;
    let response = client
        .get("folder%2Fname%20%2B", Some("version/1 +"), None)
        .await?
        .into_model()?;
    assert_eq!(response.value, "folder%2Fname%20%2B:version/1 +");
    let contexts = server.contexts.lock().expect("context lock");
    assert_eq!(contexts[0].value::<u32>(), Some(&42));
    Ok(())
}

#[tokio::test]
async fn server_can_simulate_http_and_connection_errors() -> Result<()> {
    let client = TestClient::with_fake(
        "custom://example.com",
        Arc::new(TestServer::default()),
        ClientOptions {
            retry: RetryOptions::none(),
            ..Default::default()
        },
    )?;
    let error = client
        .get("missing", None, None)
        .await
        .expect_err("HTTP error");
    assert_eq!(error.http_status(), Some(StatusCode::NotFound));
    let details = ErrorResponse::try_from(error)?
        .error
        .expect("error details");
    assert_eq!(details.code.as_deref(), Some("MissingItem"));
    let error = client
        .get("connection-error", None, None)
        .await
        .expect_err("connection error");
    assert!(matches!(error.kind(), ErrorKind::Io));
    Ok(())
}

#[tokio::test]
async fn client_accepts_bytes_and_stream_bodies_without_resetting() -> Result<()> {
    let client = TestClient::with_fake(
        "https://example.com",
        Arc::new(TestServer::default()),
        ClientOptions::default(),
    )?;
    let bytes = br#"{"value":"body"}"#.to_vec();
    let mut positioned = futures::io::Cursor::new([b"skip".as_slice(), &bytes].concat());
    positioned.seek(SeekFrom::Start(4)).await?;
    for content in [
        RequestContent::from(bytes.clone()),
        RequestContent::from_reader(
            futures::io::Cursor::new(bytes.clone()),
            Some(bytes.len() as u64),
        ),
        RequestContent::from_seekable_reader(
            futures::io::Cursor::new(bytes.clone()),
            Some(bytes.len() as u64),
        ),
        RequestContent::from_reader(positioned, Some(bytes.len() as u64)),
    ] {
        assert_eq!(client.put(content, None).await?.into_model()?.value, "body");
    }
    Ok(())
}

#[tokio::test]
async fn client_pagers_preserve_arguments_when_interleaved_and_resumed() -> Result<()> {
    let server = Arc::new(TestServer::default());
    let client = TestClient::with_fake(
        "https://example.com/prefix",
        server.clone(),
        ClientOptions::default(),
    )?;
    let options = |count, continuation| ListOptions {
        count: Some(count),
        method_options: PagerOptions {
            context: Context::new().with_value(42u32),
            continuation,
        },
    };
    let mut first = client.list("first", Some(options(3, None)))?;
    let mut second = client.list("second", Some(options(7, None)))?;
    assert!(first.try_next().await?.is_some());
    let continuation = first.continuation().cloned();
    assert!(second.try_next().await?.is_some());
    assert!(first.try_next().await?.is_some());
    assert!(second.try_next().await?.is_some());
    assert!(first.try_next().await?.is_none());
    assert!(second.try_next().await?.is_none());
    {
        let calls = server.pages.lock().expect("page lock");
        assert_eq!(calls.len(), 4);
        for (index, call) in calls.iter().enumerate() {
            assert_eq!(call.name, if index % 2 == 0 { "first" } else { "second" });
            assert_eq!(call.count, Some(if index % 2 == 0 { 3 } else { 7 }));
            assert_eq!(matches!(call.state, PagerState::Initial), index < 2);
        }
    }
    let server = Arc::new(TestServer::default());
    let client = TestClient::with_fake(
        "https://example.com",
        server.clone(),
        ClientOptions::default(),
    )?;
    let mut resumed = client.list("resumed", Some(options(11, continuation)))?;
    assert!(resumed.try_next().await?.is_some());
    assert!(resumed.try_next().await?.is_none());
    let calls = server.pages.lock().expect("page lock");
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].name, "resumed");
    assert_eq!(calls[0].count, Some(11));
    assert!(matches!(calls[0].state, PagerState::More(_)));
    Ok(())
}

#[tokio::test]
async fn router_uses_api_name_instead_of_validating_the_request() -> Result<()> {
    let server = Arc::new(TestServer::default());
    let transport = ServerTransport { server };
    let context = Context::new()
        .with_value(PublicApiInstrumentationInformation::new(
            "Test.get",
            Vec::new(),
        ))
        .with_value(GetArguments {
            path_name: "name".to_owned(),
            version: None,
            query_name: None,
            header_name: None,
        });
    let mut request = Request::new(
        "https://example.com/anything/name?extra=1&extra=2".parse()?,
        Method::Post,
    );
    assert_eq!(
        transport.send(&context, &mut request, &[]).await?.status(),
        StatusCode::Ok
    );
    for name in [None, Some("Test.unknown")] {
        let context = name.map_or_else(Context::new, |name| {
            Context::new().with_value(PublicApiInstrumentationInformation::new(name, Vec::new()))
        });
        let error = transport
            .send(&context, &mut request, &[])
            .await
            .expect_err("API name error");
        assert!(matches!(error.kind(), ErrorKind::Other));
    }
    Ok(())
}

#[tokio::test]
async fn missing_required_arguments_return_errors() -> Result<()> {
    let transport = ServerTransport {
        server: Arc::new(TestServer::default()),
    };
    let context = Context::new().with_value(PublicApiInstrumentationInformation::new(
        "Test.get",
        Vec::new(),
    ));
    let mut request = Request::new(
        "https://example.com/items".parse().expect("URL"),
        Method::Get,
    );
    let error = transport
        .send(&context, &mut request, &[])
        .await
        .expect_err("missing arguments");
    assert!(matches!(error.kind(), ErrorKind::Other));
    assert!(error
        .to_string()
        .contains("Test.get: missing operation arguments"));
    Ok(())
}

#[tokio::test]
async fn preserves_arguments_independently_of_endpoint_segments() -> Result<()> {
    for path in ["/base", "/base/", "/base//", "/base///"] {
        let client = TestClient::with_fake(
            &format!("https://example.com{path}"),
            Arc::new(TestServer::default()),
            ClientOptions::default(),
        )?;
        for name in ["100%25", "%FF", "a/b", "a b", "a+b", "\u{00e9}", ""] {
            let response = client.get(name, Some(""), None).await?.into_model()?;
            assert_eq!(response.value, format!("{name}:"));
        }
    }
    Ok(())
}

#[tokio::test]
async fn preserves_query_arrays_and_header_arguments_without_name_collisions() -> Result<()> {
    let server = Arc::new(TestServer::default());
    let client = TestClient::with_fake(
        "https://example.com",
        server.clone(),
        ClientOptions::default(),
    )?;
    for query_name in [
        None,
        Some(Vec::new()),
        Some(vec![
            "".to_owned(),
            "100%25".to_owned(),
            "100%25".to_owned(),
        ]),
    ] {
        client
            .get(
                "path",
                None,
                Some(Options {
                    query_name: query_name.clone(),
                    header_name: Some("header".to_owned()),
                    ..Default::default()
                }),
            )
            .await?;
        let contexts = server.contexts.lock().expect("context lock");
        let arguments = contexts
            .last()
            .expect("request context")
            .value::<GetArguments>()
            .expect("arguments");
        assert_eq!(arguments.path_name, "path");
        assert_eq!(arguments.query_name, query_name);
        assert_eq!(arguments.header_name.as_deref(), Some("header"));
    }
    Ok(())
}

#[test]
fn wraps_request_content_without_serde_or_validation() -> Result<()> {
    let mut request = Request::new("https://example.com".parse()?, Method::Post);
    request.set_body(Bytes::from_static(&[0, 255, b'x']));
    let content: RequestContent<OpaqueBody, NoFormat> = request.body().clone().into();
    match content.body() {
        Body::Bytes(bytes) => assert_eq!(bytes.as_ref(), &[0, 255, b'x']),
        _ => panic!("expected buffered content"),
    }
    Ok(())
}

#[tokio::test]
async fn forwards_stream_handles_without_buffering_or_resetting() -> Result<()> {
    for len in [None, Some(3)] {
        let mut positioned =
            futures::io::Cursor::new([b"skip".as_slice(), &[0, 255, b'x']].concat());
        positioned.seek(SeekFrom::Start(4)).await?;
        for content in [
            RequestContent::<OpaqueBody, NoFormat>::from_reader(positioned.clone(), len),
            RequestContent::from_seekable_reader(positioned.clone(), len),
        ] {
            let mut request = Request::new("https://example.com".parse()?, Method::Put);
            request.set_body(content);
            let content: RequestContent<OpaqueBody, NoFormat> = request.body().clone().into();
            assert_eq!(content.body().len(), len);
            let Body::SeekableStream(mut reader) = Body::from(content) else {
                panic!("stream must not be buffered")
            };
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            assert_eq!(bytes, [0, 255, b'x']);
        }
    }
    let failing = stream::iter([Err::<Bytes, _>(std::io::Error::other("body read failed"))])
        .into_async_read();
    let mut request = Request::new("https://example.com".parse()?, Method::Put);
    request.set_body(RequestContent::<OpaqueBody, NoFormat>::from_reader(
        failing, None,
    ));
    let content: RequestContent<OpaqueBody, NoFormat> = request.body().clone().into();
    let Body::SeekableStream(mut reader) = Body::from(content) else {
        panic!("stream must not be read eagerly")
    };
    let error = reader.read_to_end(&mut Vec::new()).await.unwrap_err();
    assert_eq!(error.to_string(), "body read failed");
    Ok(())
}

#[test]
fn borrows_api_name_from_context() {
    let context = Context::new().with_value(42u32);
    assert_eq!(get_api_name(&context), None);
    for name in [
        Cow::Borrowed("Test.get"),
        Cow::Owned(String::from("Test.list")),
    ] {
        let pointer = name.as_ptr();
        let expected = name.to_string();
        let context = context
            .clone()
            .with_value(PublicApiInstrumentationInformation::new(name, Vec::new()));
        let actual = get_api_name(&context).expect("expected operation name");
        assert_eq!(actual, expected);
        assert_eq!(actual.as_ptr(), pointer);
        assert_eq!(context.value::<u32>(), Some(&42));
    }
}

#[tokio::test]
async fn json_response() -> Result<()> {
    let mut headers = Headers::new();
    headers.insert("x-test", "example");
    let response = AsyncRawResponse::fake_with_bytes(
        StatusCode::Created,
        headers,
        r#"{"value":"fake secret"}"#,
    )
    .try_into_raw_response()
    .await?;
    assert_eq!(response.status(), StatusCode::Created);
    assert!(response.headers().get_optional_str(&CONTENT_TYPE).is_none());
    assert_eq!(
        response
            .headers()
            .get_str(&HeaderName::from_static("x-test"))?,
        "example"
    );
    let actual: HashMap<String, String> = json::from_json(response.body())?;
    assert_eq!(actual.get("value").map(String::as_str), Some("fake secret"));
    Ok(())
}

#[test]
fn preserves_content_type() -> Result<()> {
    let mut headers = Headers::new();
    headers.insert(CONTENT_TYPE, "application/problem+json");
    let response = RawResponse::fake_with_bytes(StatusCode::BadRequest, headers, "example");
    assert_eq!(
        response.headers().get_str(&CONTENT_TYPE)?,
        "application/problem+json"
    );
    assert_eq!(response.body().as_ref(), b"example");
    Ok(())
}

#[tokio::test]
async fn raw_and_streamed_responses() -> Result<()> {
    let response = AsyncRawResponse::fake_with_bytes(
        StatusCode::NotFound,
        Headers::new(),
        Bytes::from_static(b"not JSON"),
    );
    assert_eq!(response.status(), StatusCode::NotFound);
    assert_eq!(response.into_body().collect().await?, "not JSON");

    let response = AsyncResponse::<()>::fake_with_stream(
        StatusCode::Ok,
        Headers::new(),
        Box::pin(stream::iter([
            Ok(Bytes::from_static(b"first ")),
            Ok(Bytes::from_static(b"second")),
        ])),
    );
    assert_eq!(response.into_body().collect().await?, "first second");
    Ok(())
}

#[tokio::test]
async fn typed_responses() -> Result<()> {
    let model = HashMap::from([("value".to_owned(), "fake secret".to_owned())]);
    let response: Response<HashMap<String, String>> =
        Response::fake_with_bytes(StatusCode::Ok, Headers::new(), r#"{"value":"fake secret"}"#);
    assert_eq!(response.into_model()?, model);

    let response: AsyncResponse = AsyncResponse::fake_with_bytes(
        StatusCode::Ok,
        Headers::new(),
        r#"{"value":"fake secret"}"#,
    );
    let actual: HashMap<String, String> = json::from_json(response.into_body().collect().await?)?;
    assert_eq!(actual, model);
    Ok(())
}

#[tokio::test]
async fn azure_error_response() -> Result<()> {
    let message = "A \"secret\" was not found.\nTry another name.";
    let mut headers = Headers::new();
    headers.insert("x-test", "example");
    headers.insert(ERROR_CODE, "SecretNotFound");
    let response = AsyncRawResponse::fake_with_error(
        StatusCode::NotFound,
        headers,
        "SecretNotFound",
        message,
    )?;
    assert_eq!(response.headers().get_str(&ERROR_CODE)?, "SecretNotFound");
    assert_eq!(
        response
            .headers()
            .get_str(&HeaderName::from_static("x-test"))?,
        "example"
    );
    let err = check_success(response, None).await.unwrap_err();
    assert_eq!(err.http_status(), Some(StatusCode::NotFound));
    assert!(matches!(
        err.kind(),
        ErrorKind::HttpResponse { error_code: Some(code), .. } if code == "SecretNotFound"
    ));
    let detail = ErrorResponse::try_from(err)?.error.unwrap();
    assert_eq!(detail.message.as_deref(), Some(message));
    Ok(())
}

#[test]
fn error_response_preserves_headers_and_escapes_strings() -> Result<()> {
    let mut headers = Headers::new();
    headers.insert(CONTENT_TYPE, "application/problem+json");
    headers.insert("x-test", "example");
    let response: Response<()> = Response::fake_with_error(
        StatusCode::BadRequest,
        headers,
        "Invalid\"Value",
        "Invalid input.\nTry again.",
    )?;
    let (status, headers, body) = response.deconstruct();
    assert_eq!(status, StatusCode::BadRequest);
    assert_eq!(headers.get_str(&CONTENT_TYPE)?, "application/problem+json");
    assert_eq!(
        headers.get_str(&HeaderName::from_static("x-test"))?,
        "example"
    );
    let body: ErrorResponse = json::from_json(&body)?;
    let details = body.error.expect("error details");
    assert_eq!(details.code.as_deref(), Some("Invalid\"Value"));
    assert_eq!(
        details.message.as_deref(),
        Some("Invalid input.\nTry again.")
    );
    Ok(())
}

#[tokio::test]
async fn stream_errors() {
    let response = AsyncRawResponse::fake_with_stream(
        StatusCode::Ok,
        Headers::new(),
        Box::pin(stream::once(async { Err(Error::from(ErrorKind::Io)) })),
    );
    let err = response.into_body().collect().await.unwrap_err();
    assert!(matches!(err.kind(), ErrorKind::Io));
}
