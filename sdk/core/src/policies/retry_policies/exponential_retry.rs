use crate::policies::{Policy, PolicyResult, Request, Response};
use crate::sleep::sleep;
use crate::PipelineContext;
use chrono::{DateTime, Local};
use std::sync::Arc;
use std::time::Duration;

/// Retry policy with exponential back-off.
///
/// Retry policy with exponential back-off (with an added random delay up to 256 ms). Each retry
/// will happen at least after an exponential wait time. So if x is the first retry wait, the
/// second will be x*2, the third x*4 and so on. The policy will retry until the maximum number of
/// retries have been reached or the maximum allowed delay has passed (whichever comes first). The
/// wait time is not precise.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExponentialRetryPolicy {
    delay: Duration,
    max_retries: u32,
    max_delay: Duration,
}

impl ExponentialRetryPolicy {
    pub(crate) fn new(delay: Duration, max_retries: u32, max_delay: Duration) -> Self {
        ExponentialRetryPolicy {
            delay,
            max_retries,
            max_delay,
        }
    }

    fn is_expired(
        &self,
        first_retry_time: &mut Option<DateTime<Local>>,
        current_retries: &u32,
    ) -> bool {
        if *current_retries > self.max_retries {
            return true;
        }

        if first_retry_time.is_none() {
            *first_retry_time = Some(Local::now());
        }

        Local::now()
            > first_retry_time.unwrap() + chrono::Duration::from_std(self.max_delay).unwrap()
    }
}

#[async_trait::async_trait]
impl<R> Policy<R> for ExponentialRetryPolicy
where
    R: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<R>,
        request: &mut Request,
        next: &[Arc<dyn Policy<R>>],
    ) -> PolicyResult<Response> {
        let mut first_retry_time = None;
        let mut current_retries = 0;

        loop {
            match next[0].send(ctx, request, &next[1..]).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    if self.is_expired(&mut first_retry_time, &mut current_retries) {
                        return Err(error);
                    } else {
                        current_retries += 1;

                        let sleep_ms = self.delay.as_millis() as u64
                            * u64::pow(2u64, current_retries - 1)
                            + rand::random::<u8>() as u64;
                        sleep(Duration::from_millis(sleep_ms)).await;
                    }
                }
            }
        }
    }
}
