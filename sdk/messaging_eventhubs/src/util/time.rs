use std::time::Duration;

use timer_kit::error::Elapsed;

use super::IntoAzureCoreError;

impl IntoAzureCoreError for Elapsed {
    fn into_azure_core_error(self) -> azure_core::Error {
        azure_core::Error::new(azure_core::error::ErrorKind::Other, self)
    }
}

cfg_not_wasm32! {
    pub(crate) type Delay = tokio::time::Sleep;
    pub(crate) type Instant = tokio::time::Instant;

    pub(crate) fn now_utc() -> time::OffsetDateTime {
        time::OffsetDateTime::now_utc()
    }
}

cfg_wasm32! {
    pub(crate) type Delay = fluvio_wasm_timer::Delay;
    pub(crate) type Instant = fluvio_wasm_timer::Instant;

    pub(crate) fn now_utc() -> time::OffsetDateTime {
        let js_now = js_sys::Date::new_0();
        let timestamp_nanos = (js_now.get_time() * 1_000_000.0) as i128;
        time::OffsetDateTime::from_unix_timestamp_nanos(timestamp_nanos)
            .expect("invalid timestamp: Timestamp cannot fit in range")
    }
}

pub(crate) use timer_kit::Key;

pub(crate) type DelayQueue<T> = timer_kit::DelayQueue<Delay, T>;

pub(crate) async fn sleep(duration: Duration) -> <Delay as timer_kit::Delay>::Value {
    timer_kit::sleep::<Delay>(duration).await
}

pub(crate) async fn timeout<Fut>(duration: Duration, future: Fut) -> Result<Fut::Output, Elapsed>
where
    Fut: std::future::Future,
{
    timer_kit::timeout::<Delay, Fut>(duration, future).await
}
