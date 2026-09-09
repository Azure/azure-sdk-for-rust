// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! An auto-refreshing cache with single-flight foreground acquisition.
//!
//! One cache instance holds one value (a session for a single container).
//! It acquires the value on first use, reuses it until a refresh window opens,
//! then proactively refreshes it in the background while still serving the
//! current value. Concurrent foreground callers share a single acquisition, so
//! a cold or expired cache issues one foreground acquire even when callers race.

use azure_core::{
    async_runtime::get_async_runtime,
    time::{Duration, OffsetDateTime},
    Result,
};
use futures::{
    future::{self, BoxFuture, Either},
    lock::Mutex,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

/// A cached value that knows when it should be refreshed and when it expires.
pub(crate) trait ExpiringValue: Clone + Send + Sync + 'static {
    /// The instant at which a proactive background refresh should begin. The
    /// value remains usable until [`expires_on`](ExpiringValue::expires_on).
    fn refresh_on(&self) -> OffsetDateTime;

    /// The instant at which the value is no longer usable and must be
    /// re-acquired in the foreground.
    fn expires_on(&self) -> OffsetDateTime;

    /// Whether a value returned by a background refresh may replace the
    /// still-valid value that triggered that refresh.
    fn replaces_current(&self) -> bool {
        true
    }

    /// Returns this value with a new proactive refresh time.
    fn with_refresh_on(&self, refresh_on: OffsetDateTime) -> Self;
}

/// A factory that asynchronously acquires a fresh value.
pub(crate) type AcquireFn<T> = Arc<dyn Fn() -> BoxFuture<'static, Result<T>> + Send + Sync>;

/// A clock, injected so tests can control the passage of time.
type ClockFn = Arc<dyn Fn() -> OffsetDateTime + Send + Sync>;

/// What to do with the currently cached value given the current time.
#[derive(Debug, PartialEq, Eq)]
enum Decision {
    /// Before the refresh window opens; use the value as-is.
    Fresh,
    /// The refresh window is open, but the value remains usable; serve it and
    /// refresh in the background.
    Stale,
    /// The value has expired; acquire a replacement in the foreground.
    Expired,
}

fn decide<T: ExpiringValue>(value: &T, now: OffsetDateTime) -> Decision {
    if now >= value.expires_on() {
        Decision::Expired
    } else if now < value.refresh_on() {
        Decision::Fresh
    } else {
        Decision::Stale
    }
}

struct Shared<T> {
    value: Mutex<Option<T>>,
    refreshing: AtomicBool,
    acquire: AcquireFn<T>,
    background_timeout: Duration,
    clock: ClockFn,
}

/// An auto-refreshing cache with single-flight foreground acquisition.
pub(crate) struct AutoRefreshingCache<T> {
    shared: Arc<Shared<T>>,
}

impl<T> Clone for AutoRefreshingCache<T> {
    fn clone(&self) -> Self {
        Self {
            shared: self.shared.clone(),
        }
    }
}

impl<T: ExpiringValue + PartialEq> AutoRefreshingCache<T> {
    /// Creates a cache that acquires values with `acquire`. Background refreshes
    /// run for at most `background_timeout`; failures and timeouts retain the
    /// current value.
    pub(crate) fn new(acquire: AcquireFn<T>, background_timeout: Duration) -> Self {
        Self::with_clock(
            acquire,
            background_timeout,
            Arc::new(OffsetDateTime::now_utc),
        )
    }

    fn with_clock(acquire: AcquireFn<T>, background_timeout: Duration, clock: ClockFn) -> Self {
        Self {
            shared: Arc::new(Shared {
                value: Mutex::new(None),
                refreshing: AtomicBool::new(false),
                acquire,
                background_timeout,
                clock,
            }),
        }
    }

    /// Returns a usable value, acquiring or refreshing as needed.
    pub(crate) async fn get(&self) -> Result<T> {
        let now = (self.shared.clock)();
        {
            let guard = self.shared.value.lock().await;
            if let Some(value) = guard.as_ref() {
                match decide(value, now) {
                    Decision::Fresh => return Ok(value.clone()),
                    Decision::Stale => {
                        let current = value.clone();
                        drop(guard);
                        self.trigger_background_refresh(current.clone());
                        return Ok(current);
                    }
                    Decision::Expired => {}
                }
            }
        }
        self.acquire_foreground().await
    }

    /// Clears the cached value, but only if it still equals `current`, so a
    /// concurrent refresh that already replaced it is not clobbered.
    pub(crate) async fn invalidate_if_current(&self, current: &T)
    where
        T: PartialEq,
    {
        let mut guard = self.shared.value.lock().await;
        if guard.as_ref() == Some(current) {
            *guard = None;
        }
    }

    /// Acquires under the value lock so concurrent callers coalesce onto a
    /// single acquisition (single-flight). A caller that loses the race sees the
    /// value the winner stored and returns it without acquiring again.
    async fn acquire_foreground(&self) -> Result<T> {
        let mut guard = self.shared.value.lock().await;
        if let Some(value) = guard.as_ref() {
            if decide(value, (self.shared.clock)()) != Decision::Expired {
                return Ok(value.clone());
            }
        }
        let value = (self.shared.acquire)().await?;
        *guard = Some(value.clone());
        Ok(value)
    }

    /// Spawns at most one background refresh. A successful replacing value is
    /// published only if the cache still contains `current`. A non-replacing
    /// result retains `current` and defers its next refresh. Failures and timeouts
    /// leave the cache unchanged.
    fn trigger_background_refresh(&self, current: T) {
        // Atomically claim the single background-refresh slot.
        if self.shared.refreshing.swap(true, Ordering::AcqRel) {
            return;
        }
        let shared = self.shared.clone();
        // Detached from the requesting caller; completion is bounded by the timeout.
        let _refresh = get_async_runtime().spawn(Box::pin(async move {
            let acquire = (shared.acquire)();
            let timeout = get_async_runtime().sleep(shared.background_timeout);
            if let Either::Left((Ok(value), _)) = future::select(acquire, timeout).await {
                let mut guard = shared.value.lock().await;
                // Publish only if no foreground acquisition or invalidation replaced `current`.
                if guard.as_ref() == Some(&current) {
                    if value.replaces_current() {
                        *guard = Some(value);
                    } else {
                        // Keep the usable value, but defer retries until the fallback cooldown
                        // ends or the current value expires, whichever comes first.
                        let refresh_on = value.expires_on().min(current.expires_on());
                        *guard = Some(current.with_refresh_on(refresh_on));
                    }
                }
            }
            shared.refreshing.store(false, Ordering::Release);
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::channel::oneshot;
    use std::{
        collections::VecDeque,
        sync::{
            atomic::{AtomicI64, AtomicUsize},
            Mutex as StdMutex,
        },
    };

    const BASE_UNIX: i64 = 1_700_000_000;
    const REFRESH_AFTER: i64 = 100;
    const EXPIRE_AFTER: i64 = 200;

    #[derive(Clone, Debug, PartialEq)]
    struct TestValue {
        id: usize,
        refresh_on: OffsetDateTime,
        expires_on: OffsetDateTime,
        replaces_current: bool,
    }

    impl ExpiringValue for TestValue {
        fn refresh_on(&self) -> OffsetDateTime {
            self.refresh_on
        }
        fn expires_on(&self) -> OffsetDateTime {
            self.expires_on
        }
        fn replaces_current(&self) -> bool {
            self.replaces_current
        }
        fn with_refresh_on(&self, refresh_on: OffsetDateTime) -> Self {
            Self {
                refresh_on,
                ..self.clone()
            }
        }
    }

    /// Test harness: a controllable clock plus an acquire that counts calls and
    /// stamps each value relative to the current (fake) time.
    struct Harness {
        offset: Arc<AtomicI64>,
        count: Arc<AtomicUsize>,
    }

    impl Harness {
        fn new() -> (Self, AutoRefreshingCache<TestValue>) {
            let offset = Arc::new(AtomicI64::new(0));
            let count = Arc::new(AtomicUsize::new(0));

            let clock_offset = offset.clone();
            let clock: ClockFn = Arc::new(move || {
                let secs = BASE_UNIX + clock_offset.load(Ordering::SeqCst);
                OffsetDateTime::from_unix_timestamp(secs).unwrap()
            });

            let acquire_offset = offset.clone();
            let acquire_count = count.clone();
            let acquire: AcquireFn<TestValue> = Arc::new(move || {
                let secs = BASE_UNIX + acquire_offset.load(Ordering::SeqCst);
                let now = OffsetDateTime::from_unix_timestamp(secs).unwrap();
                let id = acquire_count.fetch_add(1, Ordering::SeqCst) + 1;
                Box::pin(async move {
                    Ok(TestValue {
                        id,
                        refresh_on: now + Duration::seconds(REFRESH_AFTER),
                        expires_on: now + Duration::seconds(EXPIRE_AFTER),
                        replaces_current: true,
                    })
                })
            });

            let cache = AutoRefreshingCache::with_clock(acquire, Duration::seconds(30), clock);
            (Self { offset, count }, cache)
        }

        fn advance(&self, secs: i64) {
            self.offset.fetch_add(secs, Ordering::SeqCst);
        }

        fn acquire_count(&self) -> usize {
            self.count.load(Ordering::SeqCst)
        }
    }

    async fn await_count(harness: &Harness, expected: usize) {
        for _ in 0..200 {
            if harness.acquire_count() >= expected {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!(
            "acquire count reached {}, expected {}",
            harness.acquire_count(),
            expected
        );
    }

    fn value(id: usize, refresh_offset: i64, expire_offset: i64) -> TestValue {
        let base = OffsetDateTime::from_unix_timestamp(BASE_UNIX).unwrap();
        TestValue {
            id,
            refresh_on: base + Duration::seconds(refresh_offset),
            expires_on: base + Duration::seconds(expire_offset),
            replaces_current: true,
        }
    }

    fn controlled_cache(
        offset: Arc<AtomicI64>,
        count: Arc<AtomicUsize>,
        receivers: Vec<oneshot::Receiver<TestValue>>,
    ) -> AutoRefreshingCache<TestValue> {
        let clock: ClockFn = Arc::new(move || {
            OffsetDateTime::from_unix_timestamp(BASE_UNIX + offset.load(Ordering::SeqCst)).unwrap()
        });
        let receivers = Arc::new(StdMutex::new(VecDeque::from(receivers)));
        let acquire: AcquireFn<TestValue> = Arc::new(move || {
            count.fetch_add(1, Ordering::SeqCst);
            let receiver = receivers.lock().unwrap().pop_front().unwrap();
            Box::pin(async move { Ok(receiver.await.unwrap()) })
        });
        AutoRefreshingCache::with_clock(acquire, Duration::seconds(30), clock)
    }

    async fn await_refresh(cache: &AutoRefreshingCache<TestValue>) {
        for _ in 0..200 {
            if !cache.shared.refreshing.load(Ordering::Acquire) {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!("background refresh did not finish");
    }

    #[test]
    fn decide_classifies_by_time() {
        let base = OffsetDateTime::from_unix_timestamp(BASE_UNIX).unwrap();
        let v = value(1, 100, 200);
        assert_eq!(decide(&v, base + Duration::seconds(50)), Decision::Fresh);
        assert_eq!(decide(&v, base + Duration::seconds(150)), Decision::Stale);
        assert_eq!(decide(&v, base + Duration::seconds(250)), Decision::Expired);
    }

    #[tokio::test]
    async fn cold_get_acquires_once_then_reuses() {
        let (harness, cache) = Harness::new();

        let first = cache.get().await.unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(harness.acquire_count(), 1);

        // Before the refresh window opens: no new acquire.
        let second = cache.get().await.unwrap();
        assert_eq!(second.id, 1);
        assert_eq!(harness.acquire_count(), 1);
    }

    #[tokio::test]
    async fn concurrent_cold_gets_acquire_once() {
        let (harness, cache) = Harness::new();

        let gets = (0..8).map(|_| cache.get());
        let results = future::join_all(gets).await;

        for result in results {
            assert_eq!(result.unwrap().id, 1);
        }
        assert_eq!(harness.acquire_count(), 1);
    }

    #[tokio::test]
    async fn expired_value_reacquires_in_foreground() {
        let (harness, cache) = Harness::new();

        assert_eq!(cache.get().await.unwrap().id, 1);
        harness.advance(EXPIRE_AFTER + 1);

        assert_eq!(cache.get().await.unwrap().id, 2);
        assert_eq!(harness.acquire_count(), 2);
    }

    #[tokio::test]
    async fn stale_value_serves_current_and_refreshes_in_background() {
        let (harness, cache) = Harness::new();

        assert_eq!(cache.get().await.unwrap().id, 1);

        // Enter the refresh window (past refresh_on, before expires_on).
        harness.advance(REFRESH_AFTER + 1);

        // The stale value is served immediately.
        assert_eq!(cache.get().await.unwrap().id, 1);

        // The background refresh eventually acquires a new value.
        await_count(&harness, 2).await;
        assert_eq!(cache.get().await.unwrap().id, 2);
    }

    #[tokio::test]
    async fn late_background_result_does_not_replace_newer_foreground_value() {
        let offset = Arc::new(AtomicI64::new(150));
        let count = Arc::new(AtomicUsize::new(0));
        let (background_sender, background_receiver) = oneshot::channel();
        let (foreground_sender, foreground_receiver) = oneshot::channel();
        let cache = controlled_cache(
            offset.clone(),
            count.clone(),
            vec![background_receiver, foreground_receiver],
        );
        *cache.shared.value.lock().await = Some(value(1, 100, 200));

        assert_eq!(cache.get().await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 1 {
            tokio::task::yield_now().await;
        }

        offset.store(201, Ordering::SeqCst);
        let foreground = tokio::spawn({
            let cache = cache.clone();
            async move { cache.get().await.unwrap() }
        });
        while count.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
        foreground_sender.send(value(3, 300, 400)).unwrap();
        assert_eq!(foreground.await.unwrap().id, 3);

        background_sender.send(value(2, 250, 350)).unwrap();
        await_refresh(&cache).await;
        assert_eq!(cache.get().await.unwrap().id, 3);
    }

    #[tokio::test]
    async fn background_fallback_retains_current_and_defers_refresh() {
        let offset = Arc::new(AtomicI64::new(150));
        let count = Arc::new(AtomicUsize::new(0));
        let (fallback_sender, fallback_receiver) = oneshot::channel();
        let (_next_sender, next_receiver) = oneshot::channel();
        let cache = controlled_cache(
            offset.clone(),
            count.clone(),
            vec![fallback_receiver, next_receiver],
        );
        *cache.shared.value.lock().await = Some(value(1, 100, 200));

        assert_eq!(cache.get().await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 1 {
            tokio::task::yield_now().await;
        }
        let mut fallback = value(2, 180, 180);
        fallback.replaces_current = false;
        fallback_sender.send(fallback).unwrap();
        await_refresh(&cache).await;

        offset.store(179, Ordering::SeqCst);
        assert_eq!(cache.get().await.unwrap().id, 1);
        assert_eq!(count.load(Ordering::SeqCst), 1);

        offset.store(181, Ordering::SeqCst);
        assert_eq!(cache.get().await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
    }

    #[tokio::test]
    async fn invalidate_if_current_only_clears_matching_value() {
        let (harness, cache) = Harness::new();

        let first = cache.get().await.unwrap();
        assert_eq!(first.id, 1);

        // A stale, non-matching handle must not clear the cache.
        let stale = value(99, 100, 200);
        cache.invalidate_if_current(&stale).await;
        assert_eq!(cache.get().await.unwrap().id, 1);
        assert_eq!(harness.acquire_count(), 1);

        // The matching handle clears it, forcing a re-acquire.
        cache.invalidate_if_current(&first).await;
        assert_eq!(cache.get().await.unwrap().id, 2);
        assert_eq!(harness.acquire_count(), 2);
    }
}
