use futures::stream::unfold;
use futures::Stream;

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

pub struct Pageable<T> {
    stream: std::pin::Pin<Box<dyn Stream<Item = Result<T, crate::Error>>>>,
}

impl<T: Continuable> Pageable<T> {
    pub fn new<F>(make_request: std::sync::Arc<dyn Fn(Option<String>) -> F>) -> Self
    where
        F: std::future::Future<Output = Result<T, crate::Error>> + 'static,
    {
        let stream = unfold(State::Init, move |state: State| {
            let make_request = make_request.clone();
            async move {
                let response = match state {
                    State::Init => r#try!(make_request(None).await),
                    State::Continuation(token) => {
                        r#try!(make_request(Some(token)).await)
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

pub trait Continuable {
    fn continuation(&self) -> Option<String>;
}

impl<T> Stream for Pageable<T> {
    type Item = Result<T, crate::Error>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        std::pin::Pin::new(&mut self.stream).poll_next(cx)
    }
}

#[derive(Debug, Clone, PartialEq)]
enum State {
    Init,
    Continuation(String),
    Done,
}
