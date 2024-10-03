use std::{fmt, pin::Pin};

use bytes::Bytes;
use futures::{Stream, StreamExt};
use serde::de::DeserializeOwned;
use typespec::error::{ErrorKind, ResultExt};

use crate::http::PinnedStream;

/// A response body stream.
///
/// This body can either be streamed or collected into [`Bytes`].
#[pin_project::pin_project]
pub struct ResponseBody(#[pin] PinnedStream);

impl ResponseBody {
    /// Create a new [`ResponseBody`] from an async stream of bytes.
    pub(crate) fn new(stream: PinnedStream) -> Self {
        Self(stream)
    }

    /// Create a new [`ResponseBody`] from a byte slice.
    pub(crate) fn from_bytes(bytes: impl Into<Bytes>) -> Self {
        let bytes = bytes.into();
        Self::new(Box::pin(futures::stream::once(async move { Ok(bytes) })))
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
    #[cfg(feature = "json")]
    pub async fn json<T>(self) -> crate::Result<T>
    where
        T: DeserializeOwned,
    {
        let body = self.collect().await?;
        crate::json::from_json(body)
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
