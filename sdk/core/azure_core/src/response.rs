use crate::{
    error::{ErrorKind, ResultExt},
    headers::Headers,
    json::from_json,
    BytesStream, StatusCode,
};
use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use std::{fmt, pin::Pin};

#[cfg(not(target_arch = "wasm32"))]
pub(crate) type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>> + Send + Sync>>;
#[cfg(target_arch = "wasm32")]
pub(crate) type PinnedStream = Pin<Box<dyn Stream<Item = crate::Result<Bytes>>>>;

/// The response to an HTTP request, including the status code, headers, and body.
///
/// The HTTP pipeline produces responses that contain a body of type [`ResponseBody`], representing the raw bytes.
/// This "raw" response can then be parsed using a method such as [`Response<ResponseBody>::json`] to deserialize the body into a structured type.
/// Parsing the body will transform the instance into a new [`Response`] instance containing the parsed body object, but retaining the original status code and headers.
pub struct Response<T> {
    status: StatusCode,
    headers: Headers,
    body: T,
}

impl<T> Response<T> {
    /// Create an HTTP response.
    pub fn new(status: StatusCode, headers: Headers, body: T) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        &self.headers
    }

    /// Gets a reference to the body of the response.
    pub fn body(&self) -> &T {
        &self.body
    }

    /// Consume the HTTP response and return the body.
    pub fn into_body(self) -> T {
        self.body
    }

    /// Deconstruct the HTTP response into its components.
    pub fn deconstruct(self) -> (StatusCode, Headers, T) {
        (self.status, self.headers, self.body)
    }
}

impl Response<ResponseBody> {
    /// Creates a [`Response<ResponseBody>`] from a raw collection of bytes.
    pub fn from_bytes(status: StatusCode, headers: Headers, body: impl Into<Bytes>) -> Self {
        let stream: BytesStream = body.into().into();
        let stream = Box::pin(stream);
        Self::new(status, headers, ResponseBody::new(stream))
    }

    /// Consumes this instance, parses the body as JSON, and returns a new [`Response<T>`] containing the parsed value.
    pub async fn json<T: DeserializeOwned>(self) -> crate::Result<Response<T>> {
        let (status, headers, body) = self.deconstruct();
        let body = body.json().await?;
        Ok(Response::new(status, headers, body))
    }

    /// Consumes this instance, parses the body as XML, and returns a new [`Response<T>`] containing the parsed value.
    #[cfg(feature = "xml")]
    pub async fn xml<T: DeserializeOwned>(self) -> crate::Result<Response<T>> {
        let (status, headers, body) = self.deconstruct();
        let body = body.xml().await?;
        Ok(Response::new(status, headers, body))
    }

    /// Collect the stream into a [Response] containing the complete body.
    ///
    /// This method will wait until the entire stream has been read before returning.
    pub async fn collect(self) -> crate::Result<Response<Bytes>> {
        Response::from_response(self).await
    }
}

impl<T: fmt::Debug> fmt::Debug for Response<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Response")
            .field("status", &self.status)
            .field("headers", &self.headers)
            .field("body", &self.body)
            .finish()
    }
}

impl Response<Bytes> {
    /// Create a collected HTTP response from a [`Response`].
    pub async fn from_response(response: Response<ResponseBody>) -> crate::Result<Self> {
        let (status, headers, body) = response.deconstruct();
        let body = body.collect().await?;
        Ok(Self::new(status, headers, body))
    }
}

impl AsRef<[u8]> for Response<Bytes> {
    fn as_ref(&self) -> &[u8] {
        self.body.as_ref()
    }
}

/// A response body stream.
///
/// This body can either be streamed or collected into [`Bytes`].
#[pin_project::pin_project]
pub struct ResponseBody(#[pin] PinnedStream);

impl ResponseBody {
    pub fn new(stream: PinnedStream) -> Self {
        Self(stream)
    }

    /// Collect the stream into a [`Bytes`] collection.
    pub async fn collect(mut self) -> crate::Result<Bytes> {
        let mut final_result = Vec::new();

        while let Some(res) = self.0.next().await {
            final_result.extend(&res?);
        }

        Ok(final_result.into())
    }

    /// Collect the stream into a [`String`].
    pub async fn collect_string(self) -> crate::Result<String> {
        std::str::from_utf8(&self.collect().await?)
            .context(
                ErrorKind::DataConversion,
                "response body was not utf-8 like expected",
            )
            .map(ToOwned::to_owned)
    }

    /// Deserialize the JSON stream into type `T`.
    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        from_json(body)
    }

    /// Deserialize the XML stream into type `T`.
    #[cfg(feature = "xml")]
    pub async fn xml<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        crate::xml::read_xml(&body)
    }
}

impl Stream for ResponseBody {
    type Item = crate::Result<Bytes>;
    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.0.poll_next(cx)
    }
}

impl fmt::Debug for ResponseBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResponseBody")
    }
}

#[cfg(test)]
mod tests {
    use crate::headers::{HeaderName, HeaderValue, Headers};
    use crate::Response;
    use http_types::StatusCode;
    use serde::Deserialize;

    #[derive(Deserialize)]
    pub struct TestObj {
        string_prop: String,
        int_prop: i32,
        float_prop: f32,
        bool_prop: bool,
        null_prop: Option<String>,
        obj_prop: NestedObj,
        arr_prop: Vec<String>,
    }

    #[derive(Deserialize)]
    pub struct NestedObj {
        nested_prop: String,
    }

    #[tokio::test]
    pub async fn deserialize_json_response() {
        // It's not really that necessary to test all kinds of deserialization here.
        // We're using serde which should handle deserialization for us.
        // So just a quick smattering of property types as a basic test.
        let body = r#"{
            "string_prop": "foo",
            "int_prop": 42,
            "float_prop": 4.2,
            "bool_prop": true,
            "null_prop": null,
            "obj_prop": {
                "nested_prop": "bar"
            },
            "arr_prop": [ "a", "b", "c" ]
        }"#;
        let mut headers = Headers::new();
        headers.insert("content-type", "application/json");
        headers.insert("foo", "bar");

        let resp = Response::from_bytes(StatusCode::Accepted, headers, body);
        let json_resp = resp.json::<TestObj>().await.unwrap();

        // Header order is inconsistent, so order the list
        let mut header_list = json_resp.headers().iter().collect::<Vec<_>>();
        header_list.sort_by(|(nl, _), (nr, _)| nl.cmp(nr));

        assert_eq!(StatusCode::Accepted, json_resp.status());
        assert_eq!(
            vec![
                (
                    &HeaderName::from("content-type"),
                    &HeaderValue::from("application/json")
                ),
                (&HeaderName::from("foo"), &HeaderValue::from("bar")),
            ],
            header_list
        );
        assert_eq!("foo", json_resp.body().string_prop);
        assert_eq!(42, json_resp.body().int_prop);
        assert_eq!(4.2, json_resp.body().float_prop);
        assert_eq!(true, json_resp.body().bool_prop);
        assert_eq!(None, json_resp.body().null_prop);
        assert_eq!("bar", json_resp.body().obj_prop.nested_prop);
        assert_eq!(vec!["a", "b", "c"], json_resp.body().arr_prop);
    }

    #[tokio::test]
    #[cfg(feature = "xml")]
    pub async fn deserialize_xml_response() {
        // It's not really that necessary to test all kinds of deserialization here.
        // We're using serde which should handle deserialization for us.
        // So just a quick smattering of property types as a basic test.
        let body = r#"
            <TestObj>
                <string_prop>foo</string_prop>
                <int_prop>42</int_prop>
                <float_prop>4.2</float_prop>
                <bool_prop>true</bool_prop>
                <obj_prop>
                    <nested_prop>bar</nested_prop>
                </obj_prop>
                <arr_prop>a</arr_prop>
                <arr_prop>b</arr_prop>
                <arr_prop>c</arr_prop>
            </TestObj>
        "#;
        let mut headers = Headers::new();
        headers.insert("content-type", "application/json");
        headers.insert("foo", "bar");

        let resp = Response::from_bytes(StatusCode::Accepted, headers, body);
        let xml_resp = resp.xml::<TestObj>().await.unwrap();

        // Header order is inconsistent, so order the list
        let mut header_list = xml_resp.headers().iter().collect::<Vec<_>>();
        header_list.sort_by(|(nl, _), (nr, _)| nl.cmp(nr));

        assert_eq!(StatusCode::Accepted, xml_resp.status());
        assert_eq!(
            vec![
                (
                    &HeaderName::from("content-type"),
                    &HeaderValue::from("application/json")
                ),
                (&HeaderName::from("foo"), &HeaderValue::from("bar")),
            ],
            header_list
        );
        assert_eq!("foo", xml_resp.body().string_prop);
        assert_eq!(42, xml_resp.body().int_prop);
        assert_eq!(4.2, xml_resp.body().float_prop);
        assert_eq!(true, xml_resp.body().bool_prop);
        assert_eq!(None, xml_resp.body().null_prop);
        assert_eq!("bar", xml_resp.body().obj_prop.nested_prop);
        assert_eq!(vec!["a", "b", "c"], xml_resp.body().arr_prop);
    }
}
