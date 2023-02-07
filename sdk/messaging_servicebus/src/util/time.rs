use std::time::Duration;

use crate::primitives::error::TimeoutElapsed;

cfg_not_wasm32! {
    pub(crate) type Delay = tokio::time::Sleep;
    pub(crate) type Instant = tokio::time::Instant;
}

cfg_wasm32! {
    pub(crate) type Delay = fluvio_wasm_timer::Delay;
    pub(crate) type Instant = fluvio_wasm_timer::Instant;
}

pub(crate) use timer_kit::Key;
pub(crate) type DelayQueue<T> = timer_kit::DelayQueue<Delay, T>;

pub(crate) async fn sleep(duration: Duration) -> <Delay as timer_kit::Delay>::Value {
    timer_kit::sleep::<Delay>(duration).await
}

pub(crate) async fn timeout<Fut>(duration: Duration, future: Fut) -> Result<Fut::Output, TimeoutElapsed>
where
    Fut: std::future::Future,
{
    timer_kit::timeout::<Delay, Fut>(duration, future).await
        .map_err(Into::into)
}
