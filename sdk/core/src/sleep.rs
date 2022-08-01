use futures::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

pub fn sleep(duration: Duration) -> Sleep {
    Sleep {
        thread: None,
        duration,
    }
}

#[derive(Debug)]
pub struct Sleep {
    thread: Option<thread::JoinHandle<()>>,
    duration: Duration,
}

impl Future for Sleep {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.thread.is_none() {
            let waker = cx.waker().clone();
            let duration = self.duration;
            self.get_mut().thread = Some(thread::spawn(move || {
                thread::sleep(duration);
                waker.wake();
            }));
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}
