use std::{
    future::{Future, IntoFuture},
    pin::Pin,
};

use serde::de::DeserializeOwned;
use typespec::Error;

use crate::http::{headers::Headers, Response, ResponseBody, StatusCode};

// Below are a series of types that serve to just keep the amount of repetition down.
// Wasm32 targets don't have Send or Sync, because they are single-threaded, so we need to exclude those from our trait objects on those platforms.

/// Utility shorthand for a Boxed future that outputs T and can capture values with lifetime 'a.
#[cfg(not(target_arch = "wasm32"))]
type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

/// Utility shorthand for a Boxed future that outputs T and can capture values with lifetime 'a.
#[cfg(target_arch = "wasm32")]
type BoxedFuture<'a, T> = Pin<Box<dyn Future<Output = T> + 'a>>;

/// Utility shorthand for a Boxed Future that takes a ResponseBody and "deserializes" the body, returning a BoxedFuture with the result.
#[cfg(not(target_arch = "wasm32"))]
pub type Deserializer<'a, T> =
    Box<dyn FnOnce(ResponseBody) -> BoxedFuture<'a, Result<T, Error>> + Send + 'a>;

/// Utility shorthand for a Boxed Future that takes a ResponseBody and "deserializes" the body, returning a BoxedFuture with the result.
#[cfg(target_arch = "wasm32")]
pub type Deserializer<'a, T> =
    Box<dyn FnOnce(ResponseBody) -> BoxedFuture<'a, Result<T, Error>> + 'a>;

/// Represents a response where the body has not been downloaded yet, but can be downloaded and deserialized into a value of type `T`.
///
/// You get a [`LazyResponse`] by calling [`ResponseFuture::lazy()`] on a [`ResponseFuture`] you received from a service API.
/// To read the response and deserialize it, call [`LazyResponse::into_body()`].
///
/// See [`ResponseFuture::lazy()`] for more information.
pub struct LazyResponse<'a, T> {
    response: Response,
    deserializer: Deserializer<'a, T>,
}

impl<'a, T> LazyResponse<'a, T> {
    /// Get the status code from the response.
    pub fn status(&self) -> StatusCode {
        self.response.status()
    }

    /// Get the headers from the response.
    pub fn headers(&self) -> &Headers {
        self.response.headers()
    }

    /// Consumes the response and returns the body.
    ///
    /// This method is asynchronous because the body of a [`LazyResponse`] has not been yet been read from the transport.
    /// Calling this method will force the body to be read, wait for it to complete, and deserialize it into the target type `T`.
    pub async fn into_body(self) -> Result<T, Error> {
        let body = self.response.into_body();
        (self.deserializer)(body).await
    }

    /// Deconstruct the HTTP response into its components.
    ///
    /// This method is asynchronous because the body of a [`LazyResponse`] has not been yet been read from the transport.
    /// Calling this method will force the body to be read, wait for it to complete, and deserialize it into the target type `T`.
    pub async fn deconstruct(self) -> Result<(StatusCode, Headers, T), Error> {
        let (status, headers, body) = self.response.deconstruct();
        let body = (self.deserializer)(body).await?;
        Ok((status, headers, body))
    }
}

/// Represents a future that, when awaited, will produce a [`Response<T>`].
///
/// In most scenarios, a [`ResponseFuture`] that you get from a client API should just be awaited.
/// Awaiting that future will execute the request, download the entire HTTP body,
/// deserialize it into the target type `T`, and return a [`Response<T>`] with that value, along with the status code and response headers.
///
/// However, you can also use [`ResponseFuture::lazy`] and [`ResponseFuture::raw`] to change how the response is processed.
/// These methods allow you to defer downloading the entire body until later, or forgo deserializing the body entirely.
///
/// # Examples
///
/// ```rust
/// # use serde::Deserialize;
/// # use typespec_client_core::http::{Response, ResponseFuture, StatusCode, headers::Headers};
/// #
/// # struct SecretClient;
/// #
/// # #[derive(Debug, Deserialize)]
/// # struct Secret {
/// #    pub name: String,
/// #    pub value: String,
/// # }
/// #
/// # impl SecretClient {
/// #     pub fn new() -> Self { Self }
/// #     pub fn get_secret(&self, name: &str) -> ResponseFuture<Secret> {
/// #         ResponseFuture::new(async {
/// #             Ok(Response::from_bytes(StatusCode::Ok, Headers::new(), r#"{"name":"secret_password", "value": "hunter2"}"#))
/// #         }).json()
/// #     }
/// # }
/// # #[tokio::main]
/// # async fn main() {
/// let secret_client = SecretClient::new();
/// let response = secret_client.get_secret("secret_password").await.unwrap();
/// let secret = response.into_body();
/// assert_eq!("secret_password", secret.name);
/// assert_eq!("hunter2", secret.value);
/// # }
/// ```
pub struct ResponseFuture<'a, T = ResponseBody> {
    future: BoxedFuture<'a, Result<Response, Error>>,
    deserializer: Deserializer<'a, T>,
}

// This impl is constrained on `T: DeserializeOwned` but that's not technically necessary.
// We do that to prevent these methods from appearing on `ResponseFuture<ResponseBody>`, because they are not relevant to that scenario.
// Having said that, these methods would be **safe to call** on `ResponseFuture<ResponseBody>`, since all they do is strip the deserializer, or convert the future to a [`LazyResponse`].
// Neither of those operations are invalid on a `ResponseFuture<ResponseBody>`, they're just meaningless no-ops.
// Rust doesn't have a way to provide negative type bounds, so this is the best we can do.
impl<'a, T: DeserializeOwned + 'a> ResponseFuture<'a, T> {
    /// Executes the request associated with this future, but skips any deserialization and returns a [`Response<ResponseBody>`]
    /// which can be used to read the raw body of the HTTP response.
    ///
    /// This is intended for scenarios where you want to perform some custom deserialization on the response body.
    /// For example, if you want to deserialize the response in to your own custom type.
    /// Once the request has completed, you can use [`Response::into_body`] to extract the [`ResponseBody`] and then call
    /// a deserialization method like [`ResponseBody::json`] to download the body and deserialize it.
    ///
    /// NOTE: Calling [`ResponseFuture::raw`] implies [`ResponseFuture::lazy`] as well, the body will not be read until you choose to do so by calling a method on [`ResponseBody`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde::Deserialize;
    /// # use typespec_client_core::http::{Response, ResponseFuture, StatusCode, headers::Headers};
    /// #
    /// # struct SecretClient;
    /// #
    /// # #[derive(Debug, Deserialize)]
    /// # struct Secret {
    /// #    name: String,
    /// #    value: String,
    /// # }
    /// #[derive(Debug, Deserialize)]
    /// struct MyCustomSecret {
    ///     #[serde(rename = "name")]
    ///     pub my_custom_name: String,
    ///     #[serde(rename = "value")]
    ///     pub my_custom_value: String,
    /// }
    /// #
    /// # impl SecretClient {
    /// #     pub fn new() -> Self { Self }
    /// #     pub fn get_secret(&self, name: &str) -> ResponseFuture<Secret> {
    /// #         ResponseFuture::new(async {
    /// #             Ok(Response::from_bytes(StatusCode::Ok, Headers::new(), r#"{"name":"secret_password", "value": "hunter2"}"#))
    /// #         }).json()
    /// #     }
    /// # }
    /// # #[tokio::main]
    /// # async fn main() {
    /// let secret_client = SecretClient::new();
    /// let response = secret_client.get_secret("secret_password").raw().await.unwrap();
    /// let raw_body = response.into_body();
    /// let secret: MyCustomSecret = raw_body.json().await.unwrap();
    /// assert_eq!("secret_password", secret.my_custom_name);
    /// assert_eq!("hunter2", secret.my_custom_value);
    /// # }
    /// ```
    pub async fn raw(self) -> Result<Response, Error> {
        self.future.await
    }

    /// Executes the request associated with this future, but does not read the full response body from the transport,
    /// returns a [`LazyResponse`] that you can use to read the body at a later point, if at all.
    ///
    /// This is intended for scenarios where either you don't care about the response body (only the status code and headers)
    /// **or** you want to defer reading the body and make a decision later based on the status code and headers.
    /// Once the request has completed and the status code and headers are available, this returns and you can use [`LazyResponse::into_body`] to deserialize the value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use serde::Deserialize;
    /// # use typespec_client_core::http::{Response, ResponseFuture, StatusCode, headers::Headers};
    /// #
    /// # struct BlobClient;
    /// #
    /// # #[derive(Debug, Deserialize)]
    /// # struct Blob {
    /// #     pub content: String,
    /// # }
    /// #
    /// # impl BlobClient {
    /// #     pub fn new() -> Self { Self }
    /// #     pub fn get_blob(&self, name: &str) -> ResponseFuture<Blob> {
    /// #         ResponseFuture::new(async {
    /// #             Ok(Response::from_bytes(StatusCode::Ok, Headers::new(), r#"{"content": "the large blob content"}"#))
    /// #         }).json()
    /// #     }
    /// # }
    /// # #[tokio::main]
    /// # async fn main() {
    /// let blob_client = BlobClient::new();
    /// let response = blob_client.get_blob("really_big_file.zip").lazy().await.unwrap();
    ///
    /// // You now have the option to read the body IF you want by calling 'into_body'.
    /// // This is an async call because it needs to wait for the body to be downloaded and deserialized.
    /// let blob = response.into_body().await.unwrap();
    /// assert_eq!("the large blob content", blob.content);
    /// # }
    /// ```
    pub async fn lazy(self) -> Result<LazyResponse<'a, T>, Error> {
        let response = self.future.await?;
        Ok(LazyResponse {
            response,
            deserializer: self.deserializer,
        })
    }
}

impl<'a, T: 'a> IntoFuture for ResponseFuture<'a, T> {
    type Output = Result<Response<T>, Error>;
    type IntoFuture = BoxedFuture<'a, Self::Output>;

    /// Executes the request and deserializes the response into the target type `T`.
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let resp = self.future.await?;
            let (status, headers, body) = resp.deconstruct();
            let body = (self.deserializer)(body).await?;
            Ok(Response::new(status, headers, body))
        })
    }
}

// The order of impls actually matters to the docs.
// Putting this one at the bottom keeps it below the other impl(s) which provide methods that the user is more likely to want to use.
impl<'a> ResponseFuture<'a, ResponseBody> {
    /// Create a new [`ResponseFuture`] by wrapping the provided future, which returns the raw HTTP body.
    ///
    /// This function returns a [`ResponseFuture`] that returns a [`Response<ResponseBody>`] when awaited.
    /// [`ResponseBody`] represents a raw, unparsed, HTTP body.
    /// If you want to transform the future into one that parses the HTTP body, use a function like [`ResponseFuture::json`] to attach a deserializer to the future.
    #[cfg(not(target_arch = "wasm32"))]
    pub fn new(future: impl Future<Output = Result<Response, Error>> + Send + 'a) -> Self {
        ResponseFuture {
            future: Box::pin(future),
            deserializer: Box::new(|body| Box::pin(std::future::ready(Ok(body)))),
        }
    }

    /// Create a new [`ResponseFuture`] by wrapping the provided future, which returns the raw HTTP body.
    ///
    /// This function returns a [`ResponseFuture`] that returns a [`Response<ResponseBody>`] when awaited.
    /// [`ResponseBody`] represents a raw, unparsed, HTTP body.
    /// If you want to transform the future into one that parses the HTTP body, use a function like [`ResponseFuture::json`] to attach a deserializer to the future.
    #[cfg(target_arch = "wasm32")]
    pub fn new(future: impl Future<Output = Result<Response, Error>> + 'a) -> Self {
        ResponseFuture {
            future: Box::pin(future),
            deserializer: Box::new(|body| Box::pin(std::future::ready(Ok(body)))),
        }
    }

    /// Converts a [`ResponseFuture<ResponseBody>`] into a [`ResponseFuture<T>`], by deserializing the body as JSON into a `T`.
    #[cfg(feature = "json")]
    pub fn json<T>(self) -> ResponseFuture<'a, T>
    where
        T: DeserializeOwned,
    {
        ResponseFuture {
            future: self.future,
            deserializer: Box::new(|body| Box::pin(async { body.json().await })),
        }
    }

    /// Converts a [`ResponseFuture<ResponseBody>`] into a [`ResponseFuture<T>`], by deserializing the body as XML into a `T`.
    #[cfg(feature = "xml")]
    pub fn xml<T>(self) -> ResponseFuture<'a, T>
    where
        T: DeserializeOwned,
    {
        ResponseFuture {
            future: self.future,
            deserializer: Box::new(|body| Box::pin(async { body.xml().await })),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::{
            atomic::{AtomicBool, Ordering},
            Arc,
        },
        task::Poll,
    };

    use bytes::Bytes;
    use futures::Stream;
    use serde::Deserialize;
    use typespec::Error;

    use crate::http::{headers::Headers, Response, StatusCode};

    use super::ResponseFuture;

    #[derive(Deserialize)]
    struct Secret {
        pub name: String,
        pub value: String,
    }

    #[derive(Clone)]
    struct BodyReadTracker(Arc<AtomicBool>);

    impl BodyReadTracker {
        pub fn new() -> Self {
            Self(Arc::new(AtomicBool::new(false)))
        }

        pub fn was_body_read(&self) -> bool {
            self.0.load(Ordering::SeqCst)
        }

        pub fn mark_body_read(&self) {
            self.0.store(true, Ordering::SeqCst)
        }
    }

    #[derive(Clone)]
    struct FakeBodyStream {
        tracker: BodyReadTracker,
        bytes: Vec<u8>,
        complete: bool,
    }

    impl FakeBodyStream {
        pub fn new(body: impl Into<Vec<u8>>, tracker: BodyReadTracker) -> Self {
            Self {
                tracker,
                bytes: body.into(),
                complete: false,
            }
        }
    }

    impl Stream for FakeBodyStream {
        type Item = Result<Bytes, Error>;

        fn poll_next(
            mut self: std::pin::Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
        ) -> std::task::Poll<Option<Self::Item>> {
            if self.complete {
                Poll::Ready(None)
            } else {
                self.tracker.mark_body_read();
                self.complete = true;
                Poll::Ready(Some(Ok(self.bytes.clone().into())))
            }
        }
    }

    // A fake pipeline that just returns the provided value
    struct FakePipeline {
        pub body: FakeBodyStream,
        pub tracker: BodyReadTracker,
    }

    impl FakePipeline {
        pub fn new(body: impl Into<Vec<u8>>) -> Self {
            let tracker = BodyReadTracker::new();
            Self {
                body: FakeBodyStream::new(body, tracker.clone()),
                tracker,
            }
        }

        pub fn send(&self) -> ResponseFuture {
            ResponseFuture::new(async {
                Ok(Response::from_stream(
                    StatusCode::Ok,
                    Headers::new(),
                    Box::pin(self.body.clone()),
                ))
            })
        }
    }

    struct FakeSecretClient(pub FakePipeline);

    impl FakeSecretClient {
        pub fn new() -> Self {
            FakeSecretClient(FakePipeline::new(
                r#"{"name":"secret_password","value":"hunter2"}"#,
            ))
        }

        pub fn get_secret(&self) -> ResponseFuture<Secret> {
            self.0.send().json()
        }
    }

    #[cfg(feature = "xml")]
    struct FakeSecretClientXml(pub FakePipeline);

    #[cfg(feature = "xml")]
    impl FakeSecretClientXml {
        pub fn new() -> Self {
            FakeSecretClientXml(FakePipeline::new(
                r"
                <GetSecretResponse>
                    <name>secret_password</name>
                    <value>hunter2</value>
                </GetSecretResponse>",
            ))
        }

        pub fn get_secret(&self) -> ResponseFuture<Secret> {
            self.0.send().xml()
        }
    }

    #[tokio::test]
    pub async fn response_future_returns_json_model_when_awaited() -> Result<(), Error> {
        let client = FakeSecretClient::new();
        assert!(!client.0.tracker.was_body_read());
        let response = client.get_secret().await?;
        assert!(client.0.tracker.was_body_read());
        let secret = response.into_body();
        assert_eq!("secret_password", secret.name);
        assert_eq!("hunter2", secret.value);
        Ok(())
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    pub async fn response_future_returns_xml_model_when_awaited() -> Result<(), Error> {
        let client = FakeSecretClientXml::new();
        assert!(!client.0.tracker.was_body_read());
        let response = client.get_secret().await?;
        assert!(client.0.tracker.was_body_read());
        let secret = response.into_body();
        assert_eq!("secret_password", secret.name);
        assert_eq!("hunter2", secret.value);
        Ok(())
    }

    #[tokio::test]
    pub async fn response_future_returns_json_model_lazily_when_lazy_called() -> Result<(), Error> {
        let client = FakeSecretClient::new();
        assert!(!client.0.tracker.was_body_read());
        let response = client.get_secret().lazy().await?;
        assert!(!client.0.tracker.was_body_read());
        let secret = response.into_body().await?;
        assert!(client.0.tracker.was_body_read());
        assert_eq!("secret_password", secret.name);
        assert_eq!("hunter2", secret.value);
        Ok(())
    }

    #[cfg(feature = "xml")]
    #[tokio::test]
    pub async fn response_future_returns_xml_model_lazily_when_lazy_called() -> Result<(), Error> {
        let client = FakeSecretClientXml::new();
        assert!(!client.0.tracker.was_body_read());
        let response = client.get_secret().lazy().await?;
        assert!(!client.0.tracker.was_body_read());
        let secret = response.into_body().await?;
        assert!(client.0.tracker.was_body_read());
        assert_eq!("secret_password", secret.name);
        assert_eq!("hunter2", secret.value);
        Ok(())
    }

    #[tokio::test]
    pub async fn response_future_returns_raw_bytes_when_raw_called() -> Result<(), Error> {
        let client = FakeSecretClient::new();
        assert!(!client.0.tracker.was_body_read());
        let response = client.get_secret().raw().await?;
        assert!(!client.0.tracker.was_body_read());
        let body = response.into_body();
        let bytes = body.collect().await?;
        assert!(client.0.tracker.was_body_read());
        assert_eq!(
            br#"{"name":"secret_password","value":"hunter2"}"#,
            bytes.as_ref()
        );
        Ok(())
    }

    #[tokio::test]
    pub async fn can_parse_custom_model_from_raw_bytes() -> Result<(), Error> {
        #[derive(Deserialize)]
        struct MySecret {
            #[serde(rename = "name")]
            my_name: String,
            #[serde(rename = "value")]
            my_value: String,
        }

        let client = FakeSecretClient::new();
        assert!(!client.0.tracker.was_body_read());
        let response = client.get_secret().raw().await?;
        assert!(!client.0.tracker.was_body_read());
        let body = response.into_body();
        let secret: MySecret = body.json().await?;
        assert!(client.0.tracker.was_body_read());
        assert_eq!("secret_password", secret.my_name);
        assert_eq!("hunter2", secret.my_value);
        Ok(())
    }
}
