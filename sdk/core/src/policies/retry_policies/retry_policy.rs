use crate::policies::{Policy, PolicyResult, Request, Response};
use crate::sleep::sleep;
use crate::PipelineContext;
use chrono::{DateTime, Local};
use std::sync::Arc;
use std::time::Duration;

pub trait RetryPolicy {
    fn is_expired(&self, first_retry_time: &mut Option<DateTime<Local>>, retry_count: u32) -> bool;
    fn sleep_duration(&self, retry_count: u32) -> Duration;
}

#[async_trait::async_trait]
impl<T, C> Policy<C> for T
where
    T: RetryPolicy + std::fmt::Debug + Send + Sync,
    C: Send + Sync,
{
    async fn send(
        &self,
        ctx: &mut PipelineContext<C>,
        request: &mut Request,
        next: &[Arc<dyn Policy<C>>],
    ) -> PolicyResult<Response> {
        let mut first_retry_time = None;
        let mut retry_count = 0;

        loop {
            match next[0].send(ctx, request, &next[1..]).await {
                Ok(response) => return Ok(response),
                Err(error) => {
                    log::error!("Error occurred when making request: {}", error);
                    if self.is_expired(&mut first_retry_time, retry_count) {
                        return Err(error);
                    } else {
                        retry_count += 1;

                        sleep(self.sleep_duration(retry_count)).await;
                    }
                }
            }
        }
    }
}
