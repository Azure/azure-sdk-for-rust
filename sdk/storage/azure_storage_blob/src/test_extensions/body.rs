use azure_core::http::Body;
use bytes::{Bytes, BytesMut};

use crate::test_extensions::streams::AsyncReadTestExt;

pub(crate) trait BodyTestExt {
    async fn collect_bytes(&mut self) -> azure_core::Result<Bytes>;
}

impl BodyTestExt for Body {
    async fn collect_bytes(&mut self) -> azure_core::Result<Bytes> {
        match self {
            Body::Bytes(bytes) => Ok(bytes.clone()),
            Body::SeekableStream(seekable_stream) => {
                seekable_stream.reset().await?;
                let mut bytes = BytesMut::with_capacity(seekable_stream.len());
                while seekable_stream.read_into_spare_capacity(&mut bytes).await? != 0 {}
                seekable_stream.reset().await?;
                Ok(bytes.freeze())
            }
        }
    }
}
