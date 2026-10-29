// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for fake servers handling pageable operations.

use crate::http::pager::{PagerOptions, PagerResultFuture, PagerState};

/// Wraps a page callback to include its state in the request context.
///
/// [`PageIterator::new()`](crate::http::PageIterator::new) applies this automatically
/// when the `test` feature is enabled. A fake server can read the current
/// [`PagerState`] from the context without interpreting continuation URLs or tokens.
pub fn with_state<P, F>(
    make_request: F,
) -> impl Fn(PagerState, PagerOptions<'static>) -> PagerResultFuture<P> + Send + 'static
where
    F: Fn(PagerState, PagerOptions<'static>) -> PagerResultFuture<P> + Send + 'static,
{
    move |state, mut options| {
        options.context.insert(state.clone());
        make_request(state, options)
    }
}

#[cfg(test)]
mod tests {
    use super::with_state;
    use crate::{
        http::{
            pager::{PagerContinuation, PagerOptions, PagerResult, PagerState},
            Context, PageIterator,
        },
        Result,
    };
    use futures::TryStreamExt as _;

    #[tokio::test]
    async fn forwards_state_and_preserves_context() -> Result<()> {
        let options = PagerOptions {
            context: Context::new().with_value(42u32),
            ..Default::default()
        };
        let mut pages = PageIterator::new(
            |state, options| {
                assert_eq!(options.context.value::<PagerState>(), Some(&state));
                assert_eq!(options.context.value::<u32>(), Some(&42));
                Box::pin(async move {
                    Ok(match state {
                        PagerState::Initial => PagerResult::More {
                            response: 1,
                            continuation: PagerContinuation::Token("next".into()),
                        },
                        PagerState::More(token) => {
                            assert_eq!(token.as_ref(), "next");
                            PagerResult::Done { response: 2 }
                        }
                    })
                })
            },
            Some(options),
        );
        assert_eq!(pages.try_next().await?, Some(1));
        assert_eq!(pages.try_next().await?, Some(2));
        assert_eq!(pages.try_next().await?, None);
        Ok(())
    }

    #[tokio::test]
    async fn resumed_callback_replaces_stale_state() -> Result<()> {
        let state = PagerState::More(PagerContinuation::Token("resume".into()));
        let context = Context::new().with_value(PagerState::Initial);
        let callback = with_state(|state, options| {
            assert_eq!(options.context.value::<PagerState>(), Some(&state));
            Box::pin(async { Ok(PagerResult::Done { response: () }) })
        });
        callback(
            state,
            PagerOptions {
                context: context.clone(),
                ..Default::default()
            },
        )
        .await?;
        assert_eq!(context.value::<PagerState>(), Some(&PagerState::Initial));
        Ok(())
    }
}
