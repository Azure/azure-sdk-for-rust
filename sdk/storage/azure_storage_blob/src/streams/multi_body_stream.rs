// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{cmp::min, task::Poll};

use async_trait::async_trait;
use azure_core::{http::Body, stream::SeekableStream};
use futures::AsyncRead;

#[derive(Clone, Debug, Default)]
pub(crate) struct MultiBodyStream {
    bodies: Vec<Body>,
    len: Option<u64>,
    vec_cursor: usize,
    bytes_cursor: usize,
}

impl MultiBodyStream {
    pub(crate) fn new<Iter: IntoIterator<Item = Body>>(data: Iter) -> Self {
        let bodies: Vec<_> = data.into_iter().collect();
        // Sum<Option<U>> returns None if any element is None, otherwise the sum of all elements.
        let len = bodies.iter().map(|bytes| bytes.len()).sum();
        Self {
            bodies,
            len,
            ..Default::default()
        }
    }
}

impl AsyncRead for MultiBodyStream {
    fn poll_read(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
        mut buf: &mut [u8],
    ) -> std::task::Poll<std::io::Result<usize>> {
        let this = self.get_mut();
        // update cached len just in case
        this.len = this.bodies.iter().map(|bytes| bytes.len()).sum();

        // Avoid mixing Bytes and SeekableStream reads in the same poll_read call. Differing semantics
        // around polling and errors makes it super complex. We accomplish this through two means:
        // 1. Never read a Body::SeekableStream if data has been read from a Body::Bytes in this poll_read call.
        //    This allows us to read from multiple Body::Bytes variants consecutively and synchronously, and
        //    return them successfully before invoking the uncertainty of an inner poll_read().
        // 2. A non-Ready(Ok(0)) return from a SeekableStream returns immediately from this poll_read call.
        //    This provides the behavior of a simple forwarding wrapper, with the exception that when the inner
        //    SeekableStream reports finished, we move on to the next Body in the sequence before returning. (A
        //    necessary behavior. Otherwise, we might prematurely terminate the read sequence or return a
        //    misleading Pending). No bytes have yet been read to the buffer, so the next Body is evaluated as
        //    if this poll_read call is starting fresh.
        // Since Body::Bytes variants are the one that can be read multiple at a time, we track the the above
        // by counting the total number of bytes read explicitly from Body::Bytes variants in this poll.
        let mut total_read_from_bytes = 0;

        while !buf.is_empty() {
            match this.bodies.get_mut(this.vec_cursor) {
                Some(Body::Bytes(bytes)) => {
                    if bytes.len().saturating_sub(this.bytes_cursor) == 0 {
                        this.vec_cursor += 1;
                        this.bytes_cursor = 0;
                        continue;
                    }
                    let copy = min(buf.len(), bytes.len() - this.bytes_cursor);
                    buf[..copy]
                        .copy_from_slice(&bytes[this.bytes_cursor..this.bytes_cursor + copy]);

                    buf = &mut buf[copy..];
                    this.bytes_cursor += copy;
                    total_read_from_bytes += copy;
                }
                Some(Body::SeekableStream(stream)) => {
                    // already read from a previous Body::Bytes, return before starting to read from a SeekableStream
                    if total_read_from_bytes > 0 {
                        return Poll::Ready(Ok(total_read_from_bytes));
                    }
                    let pinned = std::pin::pin!(stream);
                    match pinned.poll_read(cx, buf) {
                        // can only get here if no Body::Bytes AND and no other Body::SeekableStream have been read.
                        // this stream is empty and we should move to the next body to ensure something is actually read
                        Poll::Ready(Ok(0)) => {
                            this.vec_cursor += 1;
                            this.bytes_cursor = 0;
                            continue;
                        }
                        // any other result should immediately return. this poll_read acts as a wrapper to the inner poll_read.
                        poll => return poll,
                    }
                }
                // no more bodies to read from. break in case we've already read something from a Body::Bytes
                None => break,
            };
        }

        // if we reach here, not only have we exhausted all available bodies, but we also guarantee we have not read
        // actual data from a Body::SeekableStream in this entire poll_read. We may have read some bytes from one or
        // more Body::Bytes variants, so we must reflect that.
        Poll::Ready(Ok(total_read_from_bytes))
    }
}

#[async_trait]
impl SeekableStream for MultiBodyStream {
    async fn reset(&mut self) -> azure_core::Result<()> {
        self.vec_cursor = 0;
        self.bytes_cursor = 0;
        for body in self.bodies.iter_mut() {
            if let Body::SeekableStream(stream) = body {
                stream.reset().await?;
            }
        }
        Ok(())
    }

    fn len(&self) -> Option<u64> {
        self.len
    }
}

#[cfg(test)]
mod tests {
    use azure_core::stream::BytesStream;
    use futures::AsyncReadExt;

    use super::*;

    #[tokio::test]
    async fn single_body_stream_small_reads_bytes() {
        const DATA_LEN: usize = 1024;
        const READ_LEN: usize = 100;
        let data = rand::random::<[u8; DATA_LEN]>();
        let mut stream = MultiBodyStream::new(vec![Body::Bytes(data.to_vec().into())]);

        let mut total_read = 0;
        let mut num_reads = 0;
        let mut dst = [0u8; DATA_LEN];
        loop {
            let mut buf = [0u8; READ_LEN];
            let read = stream.read(&mut buf).await.unwrap();
            if read == 0 {
                break;
            }
            dst[total_read..total_read + read].copy_from_slice(&buf[..read]);
            total_read += read;
            num_reads += 1;
        }

        assert_eq!(data, dst);
        assert_eq!(total_read, DATA_LEN);
        assert_eq!(num_reads, DATA_LEN.div_ceil(READ_LEN));
    }

    #[tokio::test]
    async fn single_body_stream_small_reads_seekable_stream() {
        // hangs!
        const DATA_LEN: usize = 1024;
        const READ_LEN: usize = 100;
        let data = rand::random::<[u8; DATA_LEN]>();
        let mut stream = MultiBodyStream::new(vec![Body::SeekableStream(Box::new(
            BytesStream::new(data.to_vec()),
        ))]);

        let mut total_read = 0;
        let mut num_reads = 0;
        let mut dst = [0u8; DATA_LEN];
        loop {
            let mut buf = [0u8; READ_LEN];
            let read = stream.read(&mut buf).await.unwrap();
            if read == 0 {
                break;
            }
            dst[total_read..total_read + read].copy_from_slice(&buf[..read]);
            total_read += read;
            num_reads += 1;
        }

        assert_eq!(data, dst);
        assert_eq!(total_read, DATA_LEN);
        assert_eq!(num_reads, DATA_LEN.div_ceil(READ_LEN));
    }

    #[tokio::test]
    async fn single_body_stream_large_read_bytes() {
        const DATA_LEN: usize = 1024;
        let data = rand::random::<[u8; DATA_LEN]>();
        let mut stream = MultiBodyStream::new(vec![Body::Bytes(data.to_vec().into())]);

        let mut dst = [0u8; DATA_LEN * 2];
        let read = stream.read(&mut dst).await.unwrap();

        assert_eq!(data[..], dst[..DATA_LEN]);
        assert_eq!(read, DATA_LEN);
    }

    #[tokio::test]
    async fn single_body_stream_large_read_seekable_stream() {
        const DATA_LEN: usize = 1024;
        let data = rand::random::<[u8; DATA_LEN]>();
        let mut stream = MultiBodyStream::new(vec![Body::SeekableStream(Box::new(
            BytesStream::new(data.to_vec()),
        ))]);

        let mut dst = [0u8; DATA_LEN * 2];
        let read = stream.read(&mut dst).await.unwrap();

        assert_eq!(data[..], dst[..DATA_LEN]);
        assert_eq!(read, DATA_LEN);
    }

    #[tokio::test]
    async fn multi_body_stream_large_read_bytes_reads_all_bodies() {
        const DATA_LEN: usize = 1024;
        const BYTES_1_LEN: usize = 500;
        const BYTES_2_LEN: usize = 123;
        const BYTES_3_LEN: usize = DATA_LEN - BYTES_1_LEN - BYTES_2_LEN;
        let data = [
            rand::random::<[u8; BYTES_1_LEN]>().to_vec(),
            rand::random::<[u8; BYTES_2_LEN]>().to_vec(),
            rand::random::<[u8; BYTES_3_LEN]>().to_vec(),
        ];
        let mut stream = MultiBodyStream::new(data.iter().map(|v| Body::Bytes(v.clone().into())));

        let mut dst = [0u8; DATA_LEN * 2];
        let read = stream.read(&mut dst).await.unwrap();

        assert_eq!(read, DATA_LEN);
        assert_eq!(data.concat()[..], dst[..DATA_LEN]);
    }

    #[tokio::test]
    async fn multi_body_stream_large_read_seekable_stream_only_reads_first_body() {
        const DATA_LEN: usize = 1024;
        const BYTES_1_LEN: usize = 500;
        const BYTES_2_LEN: usize = 123;
        const BYTES_3_LEN: usize = DATA_LEN - BYTES_1_LEN - BYTES_2_LEN;
        let data = [
            rand::random::<[u8; BYTES_1_LEN]>().to_vec(),
            rand::random::<[u8; BYTES_2_LEN]>().to_vec(),
            rand::random::<[u8; BYTES_3_LEN]>().to_vec(),
        ];
        let mut stream = MultiBodyStream::new(
            data.iter()
                .map(|v| Body::SeekableStream(Box::new(BytesStream::new(v.clone())))),
        );

        let mut dst = [0u8; DATA_LEN * 2];
        let read = stream.read(&mut dst).await.unwrap();

        assert_eq!(read, BYTES_1_LEN);
        assert_eq!(data[0][..], dst[..BYTES_1_LEN]);
    }

    #[tokio::test]
    async fn multi_body_stream_small_reads_mixed() {
        // hangs!
        const DATA_LEN: usize = 1024;
        const READ_LEN: usize = 500;
        let data = rand::random::<[u8; DATA_LEN]>();
        // The following should result in five separate reads, each using a READ_LEN-sized buffer.
        // 1. The entire first Body and the first 100 bytes of the second Body, total 500 bytes.
        // 2. The remaining 100 bytes of the second Body, total 100 bytes.
        // 3. The third Body, total 100 bytes.
        // 4. The fourth Body, total 100 bytes.
        // 5. The fifth Body, total 224 bytes.
        // While this layout does result in an equal number of bodies and reads, it does not *align* to those bodies.
        // This is demonstrated in the assertions.
        let mut stream = MultiBodyStream::new([
            Body::Bytes(data[0..400].to_vec().into()),
            Body::Bytes(data[400..600].to_vec().into()),
            Body::SeekableStream(Box::new(BytesStream::new(data[600..700].to_vec()))),
            Body::Bytes(data[700..800].to_vec().into()),
            Body::SeekableStream(Box::new(BytesStream::new(data[800..].to_vec()))),
        ]);

        let mut buf = [0u8; READ_LEN];

        assert_eq!(500, stream.read(&mut buf).await.unwrap());
        assert_eq!(data[0..500], buf[..]);
        buf.fill(0);
        assert_eq!(100, stream.read(&mut buf).await.unwrap());
        assert_eq!(data[500..600], buf[..100]);
        buf.fill(0);
        assert_eq!(100, stream.read(&mut buf).await.unwrap());
        assert_eq!(data[600..700], buf[..100]);
        buf.fill(0);
        assert_eq!(100, stream.read(&mut buf).await.unwrap());
        assert_eq!(data[700..800], buf[..100]);
        buf.fill(0);
        assert_eq!(224, stream.read(&mut buf).await.unwrap());
        assert_eq!(data[800..], buf[..224]);
    }
}
