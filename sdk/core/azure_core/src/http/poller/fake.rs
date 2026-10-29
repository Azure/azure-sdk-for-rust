// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Helpers for fake servers handling long-running operations.

use crate::http::{
    poller::{PollerOptions, PollerResultFuture, PollerState, StatusMonitor},
    Format,
};

/// Wraps a polling callback to include its state in the request context.
///
/// [`Poller::new()`](crate::http::Poller::new) applies this automatically when the
/// `test` and `unstable_fake` features are enabled. A fake server can read the current [`PollerState`]
/// from the context and return a raw response for each polling request.
pub fn with_state<M, F, C>(
    make_request: C,
) -> impl Fn(PollerState, PollerOptions<'static>) -> PollerResultFuture<M, F> + Send + 'static
where
    M: StatusMonitor,
    F: Format,
    C: Fn(PollerState, PollerOptions<'static>) -> PollerResultFuture<M, F> + Send + 'static,
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
            fake::ResponseExt as _,
            headers::Headers,
            poller::{
                PollerContinuation, PollerOptions, PollerResult, PollerState, PollerStatus,
                StatusMonitor,
            },
            Context, JsonFormat, Poller, Response, StatusCode,
        },
        time::Duration,
        Result,
    };
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct Monitor {
        status: String,
    }

    impl StatusMonitor for Monitor {
        type Output = Self;
        type Format = JsonFormat;

        fn status(&self) -> PollerStatus {
            self.status.as_str().into()
        }
    }

    async fn response(status: &str) -> Result<Response<Monitor>> {
        Response::fake_with_model(
            StatusCode::Ok,
            Headers::new(),
            Monitor {
                status: status.into(),
            },
        )
    }

    #[tokio::test]
    async fn forwards_polling_state_and_final_response() -> Result<()> {
        let poller = Poller::<Monitor>::new(
            |state, options| {
                assert_eq!(options.context.value::<PollerState>(), Some(&state));
                assert_eq!(options.context.value::<u32>(), Some(&42));
                Box::pin(async move {
                    Ok(match state {
                        PollerState::Initial => PollerResult::InProgress {
                            response: response("InProgress").await?,
                            retry_after: Duration::ZERO,
                            continuation: PollerContinuation::Links {
                                next_link: "https://example.com/operations/1".parse()?,
                                final_link: None,
                            },
                        },
                        PollerState::More(continuation) => {
                            assert!(matches!(continuation, PollerContinuation::Links { .. }));
                            PollerResult::Succeeded {
                                response: response("Succeeded").await?,
                                target: Box::new(|| Box::pin(response("Succeeded"))),
                            }
                        }
                    })
                })
            },
            Some(PollerOptions {
                context: Context::new().with_value(42u32),
                ..Default::default()
            }),
        );
        assert_eq!(
            poller.await?.into_model()?.status(),
            PollerStatus::Succeeded
        );
        Ok(())
    }

    #[tokio::test]
    async fn callback_preserves_original_context() -> Result<()> {
        let context = Context::new().with_value(PollerState::Initial);
        let callback = with_state::<Monitor, JsonFormat, _>(|state, options| {
            assert_eq!(options.context.value::<PollerState>(), Some(&state));
            Box::pin(async {
                Ok(PollerResult::Done {
                    response: response("Failed").await?,
                })
            })
        });
        callback(
            PollerState::More(PollerContinuation::Links {
                next_link: "https://example.com/operations/1".parse()?,
                final_link: None,
            }),
            PollerOptions {
                context: context.clone(),
                ..Default::default()
            },
        )
        .await?;
        assert_eq!(context.value::<PollerState>(), Some(&PollerState::Initial));
        Ok(())
    }
}
