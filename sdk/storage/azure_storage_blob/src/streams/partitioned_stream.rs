use pin_project::pin_project;
use std::{
    mem::{self, MaybeUninit},
    num::NonZero,
    pin::{pin, Pin},
    task::Poll,
};

use azure_core::stream::SeekableStream;
use bytes::{Bytes, BytesMut};
use futures::{ready, stream::FusedStream, AsyncRead, Stream};

type AzureResult<T> = azure_core::Result<T>;

#[pin_project]
pub(crate) struct PartitionedStream {
    #[pin]
    inner: Box<dyn SeekableStream>,
    buf: BytesMut,
    partition_len: usize,
    total_read: usize,
    inner_complete: bool,
}

impl PartitionedStream {
    pub(crate) fn new(inner: Box<dyn SeekableStream>, partition_len: NonZero<usize>) -> Self {
        let partition_len = partition_len.get();
        Self {
            buf: BytesMut::with_capacity(std::cmp::min(partition_len, inner.len())),
            inner,
            partition_len,
            total_read: 0,
            inner_complete: false,
        }
    }
}

impl Stream for PartitionedStream {
    type Item = AzureResult<Bytes>;

    fn poll_next(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            if *this.inner_complete || this.buf.len() >= *this.partition_len {
                let ret = mem::replace(
                    this.buf,
                    BytesMut::with_capacity(std::cmp::min(
                        *this.partition_len,
                        this.inner.len() - *this.total_read,
                    )),
                );
                return if ret.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(ret.freeze())))
                };
            }
            match ready!(this.inner.as_mut().poll_read(cx, unsafe {
                // spare_capacity_mut() gives us the known remaining capacity of BytesMut.
                // Those bytes are valid reserved memory but have had no values written
                // to them. Those are the exact bytes we want to write into.
                // This transmuted data is not saved to a variable, leaving it inaccessible
                // to anything but poll_read().
                mem::transmute::<&mut [MaybeUninit<u8>], &mut [u8]>(this.buf.spare_capacity_mut())
            })) {
                Ok(bytes_read) => {
                    // poll_read() wrote bytes_read-many bytes into the spare capacity.
                    // those values are therefore initialized and we can add them to
                    // the existing buffer length
                    unsafe { this.buf.set_len(this.buf.len() + bytes_read) };
                    *this.total_read += bytes_read;
                    *this.inner_complete = bytes_read == 0;
                }
                Err(e) => {
                    return Poll::Ready(Some(Err(e.into())));
                }
            }
        }
    }
}

impl FusedStream for PartitionedStream {
    fn is_terminated(&self) -> bool {
        self.inner_complete && self.buf.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use azure_core::stream::BytesStream;
    use futures::TryStreamExt;

    use super::*;

    fn get_random_data(len: usize) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; len];
        rand::fill(&mut data[..]);
        data
    }

    #[tokio::test]
    async fn partitions_exact_len() -> AzureResult<()> {
        for part_count in [2usize, 3, 11, 16] {
            for part_len in [1024usize, 1000, 9999, 1] {
                let data = get_random_data(part_len * part_count);
                let stream = PartitionedStream::new(
                    Box::new(BytesStream::new(data.clone())),
                    NonZero::new(part_len).unwrap(),
                );

                let parts: Vec<_> = stream.try_collect().await?;

                assert_eq!(parts.len(), part_count);
                for (i, bytes) in parts.iter().enumerate() {
                    assert_eq!(bytes.len(), part_len);
                    assert_eq!(bytes[..], data[i * part_len..i * part_len + part_len]);
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn partitions_with_remainder() -> AzureResult<()> {
        for part_count in [2usize, 3, 11, 16] {
            for part_len in [1024usize, 1000, 9999] {
                for dangling_len in [part_len / 2, 100, 128, 99] {
                    let data = get_random_data(part_len * (part_count - 1) + dangling_len);
                    let stream = PartitionedStream::new(
                        Box::new(BytesStream::new(data.clone())),
                        NonZero::new(part_len).unwrap(),
                    );

                    let parts: Vec<_> = stream.try_collect().await?;

                    assert_eq!(parts.len(), part_count);
                    for (i, bytes) in parts[..parts.len()].iter().enumerate() {
                        if i == parts.len() - 1 {
                            assert_eq!(bytes.len(), dangling_len);
                            assert_eq!(bytes[..], data[i * part_len..]);
                        } else {
                            assert_eq!(bytes.len(), part_len);
                            assert_eq!(bytes[..], data[i * part_len..i * part_len + part_len]);
                        }
                    }
                }
            }
        }
        Ok(())
    }

    #[tokio::test]
    async fn exactly_one_partition() -> AzureResult<()> {
        for len in [1024usize, 1000, 9999, 1] {
            let data = get_random_data(len);
            let mut stream = PartitionedStream::new(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(len).unwrap(),
            );

            let single_partition = stream.try_next().await?.unwrap();

            assert_eq!(stream.try_next().await?, None);
            assert_eq!(single_partition[..], data[..]);
        }
        Ok(())
    }

    #[tokio::test]
    async fn less_than_one_partition() -> AzureResult<()> {
        let part_len = 99999usize;
        for len in [1024usize, 1000, 9999, 1] {
            let data = get_random_data(len);
            let mut stream = PartitionedStream::new(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(part_len).unwrap(),
            );

            let single_partition = stream.try_next().await?.unwrap();

            assert!(stream.try_next().await?.is_none());
            assert_eq!(single_partition[..], data[..]);
        }
        Ok(())
    }

    #[tokio::test]
    async fn successful_empty_stream_when_empty_source_stream() -> AzureResult<()> {
        for part_len in [1024usize, 1000, 9999, 1] {
            let data = get_random_data(0);
            let mut stream = PartitionedStream::new(
                Box::new(BytesStream::new(data.clone())),
                NonZero::new(part_len).unwrap(),
            );

            assert!(stream.try_next().await?.is_none());
        }
        Ok(())
    }
}
