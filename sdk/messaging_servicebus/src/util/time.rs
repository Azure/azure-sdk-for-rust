use std::time::Duration;

cfg_not_wasm32! {
    type Delay = tokio::time::Sleep;
    type Instant = tokio::time::Instant;
}

cfg_wasm32! {
    type Delay = fluvio_wasm_timer::Delay;
    type Instant = fluvio_wasm_timer::Instant;
}

type DelayQueue<T> = timer_kit::DelayQueue<Delay, T>;

pub(crate) async fn sleep(duration: Duration) {
    timer_kit::sleep::<Delay>(duration).await
}

pub(crate) async fn timeout<Fut>(duration: Duration, future: Fut) -> Result<Fut::Output, timer_kit::error::Elapsed>
where
    Fut: std::future::Future,
{
    timer_kit::timeout::<Delay, Fut>(duration, future).await
}
