use futures::stream::unfold;
use futures::Stream;
use pin_project::pin_project;

use crate::prelude::Continuation;

macro_rules! r#try {
    ($expr:expr $(,)?) => {
        match $expr {
            Result::Ok(val) => val,
            Result::Err(err) => {
                return Some((Err(err.into()), State::Done));
            }
        }
    };
}

/// A pageable stream that yields items of type `T`
///
/// Internally uses the Azure specific continuation header to
/// make repeated requests to Azure yielding a new page each time.
#[pin_project]
pub struct Pageable<T, E> {
    #[pin]
    stream: std::pin::Pin<Box<dyn Stream<Item = Result<T, E>> + Send>>,
}

impl<T: Continuable, E> Pageable<T, E> {
    pub fn new<F>(make_request: impl Fn(Option<Continuation>) -> F + Clone + 'static + Send) -> Self
    where
        F: std::future::Future<Output = Result<T, E>> + Send + 'static,
    {
        let stream = unfold(State::Init, move |state: State| {
            let make_request = make_request.clone();
            async move {
                let response = match state {
                    State::Init => {
                        let request = make_request(None);
                        r#try!(request.await)
                    }
                    State::Continuation(token) => {
                        let request = make_request(Some(Continuation::new(token)));
                        r#try!(request.await)
                    }
                    State::Done => return None,
                };

                let next_state = response
                    .continuation()
                    .map(State::Continuation)
                    .unwrap_or(State::Done);

                Some((Ok(response), next_state))
            }
        });
        Self {
            stream: Box::pin(stream),
        }
    }
}

impl<T, E> Stream for Pageable<T, E> {
    type Item = Result<T, E>;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        let this = self.project();
        this.stream.poll_next(cx)
    }
}

impl<T, O> std::fmt::Debug for Pageable<T, O> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Pageable").finish_non_exhaustive()
    }
}

/// A type that can yield an optional continuation token
pub trait Continuable {
    fn continuation(&self) -> Option<String>;
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Init,
    Continuation(String),
    Done,
}
