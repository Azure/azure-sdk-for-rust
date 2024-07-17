use futures::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

/// Creates a future that resolves after a specified duration of time.
/// Uses a simple thread based implementation for sleep. A more efficient
/// implementation is available by using the `tokio-sleep` crate feature.
pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        signal: None,
        duration,
    }
}

#[derive(Debug)]
pub struct Sleep {
    signal: Option<Arc<AtomicBool>>,
    duration: Duration,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(signal) = &self.signal {
            if signal.load(Ordering::Acquire) {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        } else {
            let signal = Arc::new(AtomicBool::new(false));
            let waker = cx.waker().clone();
            let duration = self.duration;
            self.get_mut().signal = Some(signal.clone());
            thread::spawn(move || {
                thread::sleep(duration);
                signal.store(true, Ordering::Release);
                waker.wake();
            });
            Poll::Pending
        }
    }
}
