// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use crate::async_runtime::get_async_runtime;
use crate::stream::SeekableStream;
use futures::io::{AsyncRead, AsyncSeek, AsyncSeekExt as _};
use std::{
    fmt,
    fs::File,
    io,
    pin::Pin,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    task::{Context, Poll},
};

/// Wraps a [`File`] as a [`AsyncRead`] and [`AsyncSeek`] adapter.
///
/// Blocking file I/O is offloaded to the configured
/// [`AsyncRuntime`](crate::async_runtime::AsyncRuntime) via
/// [`get_async_runtime`].
pub struct FileReader {
    file: Arc<Mutex<File>>,
    pending_read: Option<PendingOp<Vec<u8>>>,
    pending_seek: Option<PendingOp<u64>>,
}

struct PendingOp<T> {
    done: Arc<AtomicBool>,
    result: Arc<Mutex<Option<io::Result<T>>>>,
}

impl<T: Send + 'static> PendingOp<T> {
    /// Spawns `op` on the async runtime, locking `file` to run synchronously.
    fn spawn(
        file: &Arc<Mutex<File>>,
        cx: &mut Context<'_>,
        op: impl FnOnce(&mut File) -> io::Result<T> + Send + 'static,
    ) -> Self {
        let file = file.clone();
        let done = Arc::new(AtomicBool::new(false));
        let result: Arc<Mutex<Option<io::Result<T>>>> = Arc::new(Mutex::new(None));
        let done_clone = done.clone();
        let result_clone = result.clone();
        let waker = cx.waker().clone();

        drop(get_async_runtime().spawn(Box::pin(async move {
            let op_result = match file.lock() {
                Ok(mut f) => op(&mut f),
                Err(_) => Err(io::Error::other("lock poisoned")),
            };
            if let Ok(mut r) = result_clone.lock() {
                *r = Some(op_result);
            }
            done_clone.store(true, Ordering::Release);
            waker.wake();
        })));

        Self { done, result }
    }

    /// Returns the result if the spawned operation has completed.
    fn try_take(&self) -> Poll<io::Result<T>> {
        if self.done.load(Ordering::Acquire) {
            let data = self
                .result
                .lock()
                .map_err(|_| io::Error::other("lock poisoned"))?
                .take()
                .ok_or_else(|| io::Error::other("missing result"))?;
            Poll::Ready(data)
        } else {
            Poll::Pending
        }
    }
}

/// Polls a previously spawned operation, or spawns a new one.
fn poll_spawn<T: Send + 'static>(
    pending: &mut Option<PendingOp<T>>,
    cx: &mut Context<'_>,
    file: &Arc<Mutex<File>>,
    op: impl FnOnce(&mut File) -> io::Result<T> + Send + 'static,
) -> Poll<io::Result<T>> {
    if let Some(p) = pending {
        let poll = p.try_take();
        if poll.is_ready() {
            *pending = None;
        }
        poll
    } else {
        *pending = Some(PendingOp::spawn(file, cx, op));
        Poll::Pending
    }
}

/// Spawns a blocking file operation on the async runtime and awaits the result.
async fn spawn_blocking<T: Send + 'static>(
    file: &Arc<Mutex<File>>,
    op: impl FnOnce(&mut File) -> T + Send + 'static,
) -> Option<T> {
    let file = file.clone();
    let result: Arc<Mutex<Option<T>>> = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    let task = get_async_runtime().spawn(Box::pin(async move {
        if let Ok(mut f) = file.lock() {
            let value = op(&mut f);
            if let Ok(mut r) = result_clone.lock() {
                *r = Some(value);
            }
        }
    }));
    let _ = task.await;
    let value = result.lock().ok()?.take();
    value
}

impl From<File> for FileReader {
    fn from(file: File) -> Self {
        Self {
            file: Arc::new(Mutex::new(file)),
            pending_read: None,
            pending_seek: None,
        }
    }
}

impl fmt::Debug for FileReader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FileReader").finish_non_exhaustive()
    }
}

impl AsyncRead for FileReader {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<usize>> {
        let this = self.get_mut();
        let read_len = buf.len();
        let poll = poll_spawn(&mut this.pending_read, cx, &this.file, move |file| {
            let mut tmp = vec![0u8; read_len];
            let n = io::Read::read(file, &mut tmp)?;
            tmp.truncate(n);
            Ok(tmp)
        });
        match poll {
            Poll::Ready(Ok(bytes)) => {
                let n = std::cmp::min(bytes.len(), buf.len());
                buf[..n].copy_from_slice(&bytes[..n]);
                Poll::Ready(Ok(n))
            }
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl AsyncSeek for FileReader {
    fn poll_seek(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        pos: io::SeekFrom,
    ) -> Poll<io::Result<u64>> {
        let this = self.get_mut();
        poll_spawn(&mut this.pending_seek, cx, &this.file, move |file| {
            io::Seek::seek(file, pos)
        })
    }
}

#[async_trait::async_trait]
impl SeekableStream for FileReader {
    async fn reset(&mut self) -> crate::Result<()> {
        self.seek(io::SeekFrom::Start(0)).await?;
        Ok(())
    }

    async fn len(&self) -> usize {
        spawn_blocking(&self.file, |file| {
            file.metadata().map(|m| m.len() as usize).unwrap_or(0)
        })
        .await
        .unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::io::AsyncReadExt as _;
    use std::{fs, path::Path};

    fn this_file() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../..")
            .join(file!())
    }

    #[tokio::test]
    async fn read() {
        let file = File::open(this_file()).unwrap();
        let mut reader = FileReader::from(file);
        let mut buf = [0u8; 16];
        let n = reader.read(&mut buf).await.unwrap();
        assert!(n > 0);
        assert_eq!(&buf[..2], b"//");
    }

    #[tokio::test]
    async fn seek_to_start_rereads_same_bytes() {
        let file = File::open(this_file()).unwrap();
        let mut reader = FileReader::from(file);

        let mut first = [0u8; 64];
        let n = reader.read(&mut first).await.unwrap();
        assert!(n > 0);

        let pos = reader.seek(io::SeekFrom::Start(0)).await.unwrap();
        assert_eq!(pos, 0);

        let mut second = [0u8; 64];
        let m = reader.read(&mut second).await.unwrap();
        assert_eq!(n, m);
        assert_eq!(first[..n], second[..m]);
    }

    #[tokio::test]
    async fn seek_from_end_returns_nonzero() {
        let file = File::open(this_file()).unwrap();
        let mut reader = FileReader::from(file);
        let pos = reader.seek(io::SeekFrom::End(0)).await.unwrap();
        assert!(pos > 0);
    }

    #[tokio::test]
    async fn len_returns_file_size() {
        let path = this_file();
        let expected = fs::metadata(&path).unwrap().len() as usize;
        let file = File::open(path).unwrap();
        let reader = FileReader::from(file);
        assert_eq!(reader.len().await, expected);
    }

    #[tokio::test]
    async fn reset_seeks_to_start() {
        let file = File::open(this_file()).unwrap();
        let mut reader = FileReader::from(file);

        let mut first = [0u8; 16];
        let n = reader.read(&mut first).await.unwrap();
        assert!(n > 0);

        reader.reset().await.unwrap();

        let mut second = [0u8; 16];
        let m = reader.read(&mut second).await.unwrap();
        assert_eq!(n, m);
        assert_eq!(first[..n], second[..m]);
    }
}
