use crate::policies::{Policy, PolicyResult, Request, Response};
use crate::sleep::sleep;
use crate::PipelineContext;
use chrono::{DateTime, Local};
use http::StatusCode;
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
            let error = match next[0].send(ctx, request, &next[1..]).await {
                Ok(response) => {
                    let status = response.status();
                    if status.as_u16() < 400 {
                        // Successful status code
                        return Ok(response);
                    } else if status.as_u16() < 500 {
                        // Server returned a client caused error
                        return Ok(response.validate(StatusCode::OK).await?);
                    }
                    // Server returned an internal error, try again
                    log::error!("server returned error 500 status: {}", status);
                    Box::new(response.validate(StatusCode::OK).await.unwrap_err())
                }
                Err(error) => {
                    log::error!("error occurred when making request: {}", error);
                    error
                }
            };

            if self.is_expired(&mut first_retry_time, retry_count) {
                return Err(error);
            }
            retry_count += 1;

            sleep(self.sleep_duration(retry_count)).await;
        }
    }
}
