use futures::stream::unfold;
use futures::Stream;

/// Helper macro for unwrapping `Result`s into the right types
/// that `futures::stream::unfold` expects.
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

/// Helper macro for declaring the `Pageable` and `Continuable types which easily allows
/// for conditionally compiling with a `Send` constraint or not.
macro_rules! declare {
    ($($extra:tt)*) => {
        // The use of a module here is a hack to get around the fact that `pin_project`
        // generates a method `project_ref` which is never used and generates a warning.
        // The module allows us to declare that `dead_code` is allowed but only for
        // the `Pageable` type.
        mod pageable {
            #![allow(dead_code)]
            use super::*;
            /// A pageable stream that yields items of type `T`
            ///
            /// Internally uses the Azure specific continuation header to
            /// make repeated requests to Azure yielding a new page each time.
            #[pin_project::pin_project]
            // This is to surpress the unused `project_ref` warning
            pub struct Pageable<T, E> {
                #[pin]
                pub(crate) stream: std::pin::Pin<Box<dyn Stream<Item = Result<T, E>> $($extra)*>>,
            }
        }
        pub use pageable::Pageable;

        impl<T, E> Pageable<T, E>
        where
            T: Continuable,
        {
            pub fn new<F>(
                make_request: impl Fn(Option<T::Continuation>) -> F + Clone $($extra)* + 'static,
            ) -> Self
            where
                F: std::future::Future<Output = Result<T, E>> $($extra)* + 'static,
            {
                let stream = unfold(State::Init, move |state: State<T::Continuation>| {
                    let make_request = make_request.clone();
                    async move {
                        let response = match state {
                            State::Init => {
                                let request = make_request(None);
                                r#try!(request.await)
                            }
                            State::Continuation(token) => {
                                let request = make_request(Some(token));
                                r#try!(request.await)
                            }
                            State::Done => {
                                return None;
                            }
                        };

                        let next_state = response
                            .continuation()
                            .map_or(State::Done, State::Continuation);

                        Some((Ok(response), next_state))
                    }
                });
                Self {
                    stream: Box::pin(stream),
                }
            }
        }

        /// A type that can yield an optional continuation token
        pub trait Continuable {
            type Continuation: 'static $($extra)*;
            fn continuation(&self) -> Option<Self::Continuation>;
        }
    };
}

#[cfg(not(target_arch = "wasm32"))]
declare!(+ Send);
#[cfg(target_arch = "wasm32")]
declare!();

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum State<T> {
    Init,
    Continuation(T),
    Done,
}
