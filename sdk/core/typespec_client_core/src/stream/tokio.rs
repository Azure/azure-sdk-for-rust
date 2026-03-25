// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Asynchronous stream support for `tokio`.
use crate::stream::SeekableStream;
use std::{
    fmt, io,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
};
use tokio::{
    fs::File,
    io::{AsyncRead, AsyncSeek, AsyncSeekExt as _, ReadBuf},
    sync::Mutex,
};

/// Wraps a [`File`] as a [`futures::io::AsyncRead`] and [`futures::io::AsyncSeek`] adapter.
///
/// # Warning
///
/// Clones share the underlying file position and should only be used with read-only files.
pub struct FileReader {
    file: Arc<Mutex<File>>,
    seeking: bool,
}

impl Clone for FileReader {
    fn clone(&self) -> Self {
        Self {
            file: self.file.clone(),
            seeking: false,
        }
    }
}

impl From<File> for FileReader {
    fn from(file: File) -> Self {
        Self {
            file: Arc::new(Mutex::new(file)),
            seeking: false,
        }
    }
}

impl From<FileReader> for super::FileStream<FileReader> {
    fn from(reader: FileReader) -> Self {
        Self::new(reader)
    }
}

impl fmt::Debug for FileReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileReader").finish_non_exhaustive()
    }
}

#[async_trait::async_trait]
impl SeekableStream for FileReader {
    async fn reset(&mut self) -> crate::Result<()> {
        let mut file = self.file.lock().await;
        file.seek(io::SeekFrom::Start(0)).await?;
        Ok(())
    }

    /// Get the length of the file.
    ///
    /// # Notes
    ///
    /// This may be inaccurate if the file is writable since it may be updated after getting the length.
    /// This is best used on files opened read-only.
    async fn len(&self) -> usize {
        let file = self.file.lock().await;
        file.metadata().await.map(|m| m.len() as usize).unwrap_or(0)
    }
}

impl futures::io::AsyncRead for FileReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let mut guard = match this.file.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };
        let pinned = Pin::new(&mut *guard);
        let mut read_buf = ReadBuf::new(buf);
        AsyncRead::poll_read(pinned, cx, &mut read_buf).map(|r| r.map(|()| read_buf.filled().len()))
    }
}

impl futures::io::AsyncSeek for FileReader {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        let mut guard = match this.file.try_lock() {
            Ok(guard) => guard,
            Err(_) => {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        };
        if !this.seeking {
            AsyncSeek::start_seek(Pin::new(&mut *guard), pos)?;
            this.seeking = true;
        }
        let result = AsyncSeek::poll_complete(Pin::new(&mut *guard), cx);
        if result.is_ready() {
            this.seeking = false;
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::FileReader;
    use futures::io::{AsyncReadExt, AsyncSeekExt};
    use std::{io::SeekFrom, path::Path};
    use tokio::fs::File;

    /// Returns the absolute path to this source file.
    ///
    /// In a workspace build, `file!()` expands to a workspace-relative path, so
    /// we anchor it against `CARGO_MANIFEST_DIR` (the crate root, three levels
    /// inside the workspace root).
    fn this_file() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join(file!())
    }

    #[tokio::test]
    async fn read() {
        let file = File::open(this_file()).await.unwrap();
        let mut adapter = FileReader::from(file);
        let mut buf = [0u8; 16];
        let n = adapter.read(&mut buf).await.unwrap();
        assert!(n > 0);
        // The file starts with the copyright comment "// Copyright"
        assert_eq!(&buf[..2], b"//");
    }

    #[tokio::test]
    async fn seek_to_start_rereads_same_bytes() {
        let file = File::open(this_file()).await.unwrap();
        let mut adapter = FileReader::from(file);

        let mut first = [0u8; 64];
        let n = adapter.read(&mut first).await.unwrap();
        assert!(n > 0);

        let pos = adapter.seek(SeekFrom::Start(0)).await.unwrap();
        assert_eq!(pos, 0);

        let mut second = [0u8; 64];
        let m = adapter.read(&mut second).await.unwrap();
        assert_eq!(n, m);
        assert_eq!(first[..n], second[..m]);
    }

    #[tokio::test]
    async fn seek_from_end_returns_nonzero() {
        let file = File::open(this_file()).await.unwrap();
        let mut adapter = FileReader::from(file);
        let pos = adapter.seek(SeekFrom::End(0)).await.unwrap();
        assert!(pos > 0);
    }
}
