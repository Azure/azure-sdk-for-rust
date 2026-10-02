// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! An auto-refreshing cache with single-flight foreground acquisition.
//!
//! A cache holds independently refreshed values by key. It acquires a value on
//! first use, reuses it until a refresh window opens, then proactively refreshes
//! it in the background while still serving the current value. Concurrent
//! foreground callers for the same key share a single acquisition. A background
//! refresh that fails or times out keeps the current value and defers the next
//! attempt.

use azure_core::{
    async_runtime::get_async_runtime,
    time::{Duration, OffsetDateTime},
    Result,
};
use futures::{
    future::{self, BoxFuture, Either},
    lock::Mutex,
};
use std::{
    collections::HashMap,
    hash::Hash,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, RwLock,
    },
};

/// A cached value that knows when it should be refreshed and when it expires.
pub(crate) trait RefreshableValue: Clone + Send + Sync + 'static {
    /// The time at which a proactive background refresh should begin. The
    /// value remains usable until [`expires_on`](RefreshableValue::expires_on).
    fn refresh_on(&self) -> OffsetDateTime;

    /// The time at which the value is no longer usable and must be
    /// re-acquired in the foreground.
    fn expires_on(&self) -> OffsetDateTime;

    /// Whether a value returned by a background refresh may replace the
    /// still-valid value that triggered that refresh. When `false`, the cache
    /// keeps the current value and defers its next refresh instead.
    fn replaces_current(&self) -> bool {
        true
    }

    /// Returns this value with a new proactive refresh time, which the cache
    /// uses to defer the next attempt after a refresh fails or is declined.
    fn with_refresh_on(&self, refresh_on: OffsetDateTime) -> Self;
}

/// A factory that asynchronously acquires a fresh value.
pub(crate) type AcquireFn<K, V> = Arc<dyn Fn(K) -> BoxFuture<'static, Result<V>> + Send + Sync>;

/// A clock, injected so tests can control the passage of time.
type ClockFn = Arc<dyn Fn() -> OffsetDateTime + Send + Sync>;

/// What to do with the currently cached value given the current time.
#[derive(Debug, PartialEq, Eq)]
enum CacheItemState {
    /// Before the refresh window opens; use the value as-is.
    Fresh,
    /// The refresh window is open, but the value remains usable; serve it and
    /// refresh in the background.
    Stale,
    /// The value has expired; acquire a replacement in the foreground.
    Expired,
}

fn state<T: RefreshableValue>(value: &T, now: OffsetDateTime) -> CacheItemState {
    if now >= value.expires_on() {
        CacheItemState::Expired
    } else if now < value.refresh_on() {
        CacheItemState::Fresh
    } else {
        CacheItemState::Stale
    }
}

/// The outcome of one bounded background refresh attempt.
enum RefreshedState<T> {
    /// The acquire completed successfully.
    Value(T),
    /// The acquire returned an error.
    Failed,
    /// The acquire did not finish within the background timeout.
    TimedOut,
}

struct CacheItem<T> {
    value: Mutex<Option<T>>,
    refreshing: AtomicBool,
}

impl<T> CacheItem<T> {
    fn new() -> Self {
        Self {
            value: Mutex::new(None),
            refreshing: AtomicBool::new(false),
        }
    }
}

/// A keyed auto-refreshing cache with per-key single-flight acquisition.
pub(crate) struct AutoRefreshingCache<K, V> {
    items: Arc<RwLock<HashMap<K, Arc<CacheItem<V>>>>>,
    acquire: AcquireFn<K, V>,
    background_timeout: Duration,
    clock: ClockFn,
}

impl<K, V> Clone for AutoRefreshingCache<K, V> {
    fn clone(&self) -> Self {
        Self {
            items: self.items.clone(),
            acquire: self.acquire.clone(),
            background_timeout: self.background_timeout,
            clock: self.clock.clone(),
        }
    }
}

impl<K, V> AutoRefreshingCache<K, V>
where
    K: Clone + Eq + Hash + Send + Sync + 'static,
    V: RefreshableValue + PartialEq,
{
    /// Creates a cache that acquires values with `acquire`. Background refreshes
    /// run for at most `background_timeout`, which doubles as the backoff applied
    /// after one fails.
    pub(crate) fn new(acquire: AcquireFn<K, V>, background_timeout: Duration) -> Self {
        Self::with_clock(
            acquire,
            background_timeout,
            Arc::new(OffsetDateTime::now_utc),
        )
    }

    fn with_clock(acquire: AcquireFn<K, V>, background_timeout: Duration, clock: ClockFn) -> Self {
        Self {
            items: Arc::new(RwLock::new(HashMap::new())),
            acquire,
            background_timeout,
            clock,
        }
    }

    fn get_or_create_entry(&self, key: &K) -> Arc<CacheItem<V>> {
        if let Some(item) = self.find_entry(key) {
            return item;
        }

        // The initial lookup released its read lock. Another thread may have
        // inserted this key before we acquired the write lock, so check again.
        self.items
            .write()
            .unwrap_or_else(|error| error.into_inner())
            .entry(key.clone())
            .or_insert_with(|| Arc::new(CacheItem::new()))
            .clone()
    }

    fn find_entry(&self, key: &K) -> Option<Arc<CacheItem<V>>> {
        self.items
            .read()
            .unwrap_or_else(|error| error.into_inner())
            .get(key)
            .cloned()
    }

    /// Returns a usable value, acquiring or refreshing as needed. Blocks on a
    /// foreground acquire, and surfaces its error, only when no unexpired value
    /// is cached.
    pub(crate) async fn get(&self, key: &K) -> Result<V> {
        let item = self.get_or_create_entry(key);
        let current = item.value.lock().await.clone();
        if let Some(value) = current {
            match state(&value, (self.clock)()) {
                CacheItemState::Fresh => return Ok(value),
                CacheItemState::Stale => {
                    self.trigger_background_refresh(key.clone(), item, value.clone());
                    return Ok(value);
                }
                CacheItemState::Expired => {}
            }
        }
        self.acquire_foreground(key, item).await
    }

    /// Clears the cached value, but only if it still equals `current`, so a
    /// concurrent refresh that already replaced it is not clobbered.
    pub(crate) async fn invalidate_if_current(&self, key: &K, current: &V) {
        let Some(item) = self.find_entry(key) else {
            return;
        };
        let mut guard = item.value.lock().await;
        if guard.as_ref() == Some(current) {
            *guard = None;
        }
    }

    /// Acquires under the value lock so concurrent callers coalesce onto a
    /// single acquisition (single-flight). A caller that loses the race sees the
    /// value the winner stored and returns it without acquiring again.
    async fn acquire_foreground(&self, key: &K, item: Arc<CacheItem<V>>) -> Result<V> {
        let mut guard = item.value.lock().await;
        if let Some(value) = guard.as_ref() {
            if state(value, (self.clock)()) != CacheItemState::Expired {
                return Ok(value.clone());
            }
        }
        let value = (self.acquire)(key.clone()).await?;
        *guard = Some(value.clone());
        Ok(value)
    }

    /// Spawns at most one background refresh, publishing its result only if the
    /// cache still holds `current`. A non-replacing value, a failure, or a timeout
    /// all retain `current` and only move its next refresh time.
    fn trigger_background_refresh(&self, key: K, item: Arc<CacheItem<V>>, current: V) {
        // Atomically claim the single background-refresh slot.
        if item.refreshing.swap(true, Ordering::AcqRel) {
            return;
        }
        let cache = self.clone();
        // Detached from the requesting caller; completion is bounded by the timeout.
        let _refresh = get_async_runtime().spawn(Box::pin(async move {
            let acquire = (cache.acquire)(key);
            let timeout = get_async_runtime().sleep(cache.background_timeout);
            let refreshed = match future::select(acquire, timeout).await {
                Either::Left((Ok(value), _)) => RefreshedState::Value(value),
                Either::Left((Err(_), _)) => RefreshedState::Failed,
                Either::Right(_) => RefreshedState::TimedOut,
            };
            let now = (cache.clock)();
            let mut guard = item.value.lock().await;
            // Publish only if no foreground acquisition or invalidation replaced `current`.
            if guard.as_ref() == Some(&current) {
                *guard = Some(match refreshed {
                    RefreshedState::Value(value) if value.replaces_current() => value,
                    // Keep `current`, deferring the next attempt to whichever expires first.
                    RefreshedState::Value(value) => {
                        current.with_refresh_on(value.expires_on().min(current.expires_on()))
                    }
                    // The elapsed timeout already served as the backoff.
                    RefreshedState::TimedOut => current.with_refresh_on(now),
                    RefreshedState::Failed => {
                        current.with_refresh_on(now + cache.background_timeout)
                    }
                });
            }
            item.refreshing.store(false, Ordering::Release);
        }));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use azure_core::{error::ErrorKind, Error};
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
    const BACKGROUND_TIMEOUT: i64 = 30;
    const KEY: usize = 1;

    #[derive(Clone, Debug, PartialEq)]
    struct TestValue {
        id: usize,
        refresh_on: OffsetDateTime,
        expires_on: OffsetDateTime,
        replaces_current: bool,
    }

    impl RefreshableValue for TestValue {
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

    fn at(offset: i64) -> OffsetDateTime {
        OffsetDateTime::from_unix_timestamp(BASE_UNIX + offset).unwrap()
    }

    /// Test harness: a controllable clock plus an acquire that counts calls and
    /// stamps each value relative to the current (fake) time.
    struct Harness {
        offset: Arc<AtomicI64>,
        count: Arc<AtomicUsize>,
    }

    impl Harness {
        fn new() -> (Self, AutoRefreshingCache<usize, TestValue>) {
            let offset = Arc::new(AtomicI64::new(0));
            let count = Arc::new(AtomicUsize::new(0));

            let clock_offset = offset.clone();
            let clock: ClockFn = Arc::new(move || at(clock_offset.load(Ordering::SeqCst)));

            let acquire_offset = offset.clone();
            let acquire_count = count.clone();
            let acquire: AcquireFn<usize, TestValue> = Arc::new(move |_| {
                let now = at(acquire_offset.load(Ordering::SeqCst));
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

            let cache = AutoRefreshingCache::with_clock(
                acquire,
                Duration::seconds(BACKGROUND_TIMEOUT),
                clock,
            );
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
        TestValue {
            id,
            refresh_on: at(refresh_offset),
            expires_on: at(expire_offset),
            replaces_current: true,
        }
    }

    fn controlled_cache(
        offset: Arc<AtomicI64>,
        count: Arc<AtomicUsize>,
        receivers: Vec<oneshot::Receiver<TestValue>>,
    ) -> AutoRefreshingCache<usize, TestValue> {
        let clock: ClockFn = Arc::new(move || at(offset.load(Ordering::SeqCst)));
        let receivers = Arc::new(StdMutex::new(VecDeque::from(receivers)));
        let acquire: AcquireFn<usize, TestValue> = Arc::new(move |_| {
            count.fetch_add(1, Ordering::SeqCst);
            let receiver = receivers.lock().unwrap().pop_front().unwrap();
            Box::pin(async move { Ok(receiver.await.unwrap()) })
        });
        AutoRefreshingCache::with_clock(acquire, Duration::seconds(BACKGROUND_TIMEOUT), clock)
    }

    /// A cache whose acquire always fails, for exercising the backoff path.
    fn failing_cache(
        offset: Arc<AtomicI64>,
        count: Arc<AtomicUsize>,
    ) -> AutoRefreshingCache<usize, TestValue> {
        let clock: ClockFn = Arc::new(move || at(offset.load(Ordering::SeqCst)));
        let acquire: AcquireFn<usize, TestValue> = Arc::new(move |_| {
            count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Err(Error::with_message(ErrorKind::Other, "acquire failed")) })
        });
        AutoRefreshingCache::with_clock(acquire, Duration::seconds(BACKGROUND_TIMEOUT), clock)
    }

    async fn await_refresh(cache: &AutoRefreshingCache<usize, TestValue>, key: usize) {
        let item = cache.get_or_create_entry(&key);
        for _ in 0..200 {
            if !item.refreshing.load(Ordering::Acquire) {
                return;
            }
            tokio::task::yield_now().await;
        }
        panic!("background refresh did not finish");
    }

    #[test]
    fn state_classifies_by_time() {
        let v = value(1, 100, 200);
        assert_eq!(state(&v, at(50)), CacheItemState::Fresh);
        assert_eq!(state(&v, at(150)), CacheItemState::Stale);
        assert_eq!(state(&v, at(250)), CacheItemState::Expired);
    }

    #[tokio::test]
    async fn cold_get_acquires_once_then_reuses() {
        let (harness, cache) = Harness::new();

        let first = cache.get(&KEY).await.unwrap();
        assert_eq!(first.id, 1);
        assert_eq!(harness.acquire_count(), 1);

        // Before the refresh window opens: no new acquire.
        let second = cache.get(&KEY).await.unwrap();
        assert_eq!(second.id, 1);
        assert_eq!(harness.acquire_count(), 1);
    }

    #[tokio::test]
    async fn concurrent_cold_gets_acquire_once() {
        let (harness, cache) = Harness::new();

        let gets = (0..8).map(|_| cache.get(&KEY));
        let results = future::join_all(gets).await;

        for result in results {
            assert_eq!(result.unwrap().id, 1);
        }
        assert_eq!(harness.acquire_count(), 1);
    }

    #[tokio::test]
    async fn expired_value_reacquires_in_foreground() {
        let (harness, cache) = Harness::new();

        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        harness.advance(EXPIRE_AFTER + 1);

        assert_eq!(cache.get(&KEY).await.unwrap().id, 2);
        assert_eq!(harness.acquire_count(), 2);
    }

    #[tokio::test]
    async fn stale_value_serves_current_and_refreshes_in_background() {
        let (harness, cache) = Harness::new();

        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);

        // Enter the refresh window (past refresh_on, before expires_on).
        harness.advance(REFRESH_AFTER + 1);

        // The stale value is served immediately.
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);

        // The background refresh eventually acquires a new value.
        await_count(&harness, 2).await;
        assert_eq!(cache.get(&KEY).await.unwrap().id, 2);
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
        *cache.get_or_create_entry(&KEY).value.lock().await = Some(value(1, 100, 200));

        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 1 {
            tokio::task::yield_now().await;
        }

        offset.store(201, Ordering::SeqCst);
        let foreground = tokio::spawn({
            let cache = cache.clone();
            async move { cache.get(&KEY).await.unwrap() }
        });
        while count.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
        foreground_sender.send(value(3, 300, 400)).unwrap();
        assert_eq!(foreground.await.unwrap().id, 3);

        background_sender.send(value(2, 250, 350)).unwrap();
        await_refresh(&cache, KEY).await;
        assert_eq!(cache.get(&KEY).await.unwrap().id, 3);
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
        *cache.get_or_create_entry(&KEY).value.lock().await = Some(value(1, 100, 200));

        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 1 {
            tokio::task::yield_now().await;
        }
        let mut fallback = value(2, 180, 180);
        fallback.replaces_current = false;
        fallback_sender.send(fallback).unwrap();
        await_refresh(&cache, KEY).await;

        offset.store(179, Ordering::SeqCst);
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        assert_eq!(count.load(Ordering::SeqCst), 1);

        offset.store(181, Ordering::SeqCst);
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
    }

    #[tokio::test]
    async fn background_failure_retains_current_and_throttles_retries() {
        let offset = Arc::new(AtomicI64::new(150));
        let count = Arc::new(AtomicUsize::new(0));
        let cache = failing_cache(offset.clone(), count.clone());
        *cache.get_or_create_entry(&KEY).value.lock().await = Some(value(1, 100, 200));

        // Stale: the current value is served and a background refresh is spawned.
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        await_refresh(&cache, KEY).await;

        {
            let item = cache.get_or_create_entry(&KEY);
            let guard = item.value.lock().await;
            let held = guard
                .as_ref()
                .expect("a failed refresh must retain the current value");
            assert_eq!(held.id, 1);
            assert_eq!(held.expires_on, at(200));
            // The next attempt is deferred by one background timeout.
            assert_eq!(held.refresh_on, at(150 + BACKGROUND_TIMEOUT));
        }

        // Inside the backoff window: no new acquire.
        offset.store(150 + BACKGROUND_TIMEOUT - 1, Ordering::SeqCst);
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        assert_eq!(count.load(Ordering::SeqCst), 1);

        // Past it: one more attempt is allowed.
        offset.store(150 + BACKGROUND_TIMEOUT + 1, Ordering::SeqCst);
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        while count.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }
    }

    #[tokio::test]
    async fn invalidate_if_current_only_clears_matching_value() {
        let (harness, cache) = Harness::new();

        let first = cache.get(&KEY).await.unwrap();
        assert_eq!(first.id, 1);

        // A stale, non-matching handle must not clear the cache.
        let stale = value(99, 100, 200);
        cache.invalidate_if_current(&KEY, &stale).await;
        assert_eq!(cache.get(&KEY).await.unwrap().id, 1);
        assert_eq!(harness.acquire_count(), 1);

        // The matching handle clears it, forcing a re-acquire.
        cache.invalidate_if_current(&KEY, &first).await;
        assert_eq!(cache.get(&KEY).await.unwrap().id, 2);
        assert_eq!(harness.acquire_count(), 2);
    }

    #[tokio::test]
    async fn different_keys_cache_independent_values() {
        let count = Arc::new(AtomicUsize::new(0));
        let acquire_count = count.clone();
        let acquire: AcquireFn<usize, TestValue> = Arc::new(move |key| {
            acquire_count.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move { Ok(value(key, 100, 200)) })
        });
        let cache = AutoRefreshingCache::with_clock(
            acquire,
            Duration::seconds(BACKGROUND_TIMEOUT),
            Arc::new(|| at(0)),
        );

        assert_eq!(cache.get(&1).await.unwrap().id, 1);
        assert_eq!(cache.get(&2).await.unwrap().id, 2);
        assert_eq!(cache.get(&1).await.unwrap().id, 1);
        assert_eq!(count.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn different_keys_acquire_concurrently() {
        let (first_sender, first_receiver) = oneshot::channel();
        let (second_sender, second_receiver) = oneshot::channel();
        let count = Arc::new(AtomicUsize::new(0));
        let receivers = Arc::new(StdMutex::new(HashMap::from([
            (1, first_receiver),
            (2, second_receiver),
        ])));
        let acquire_count = count.clone();
        let acquire: AcquireFn<usize, TestValue> = Arc::new(move |key| {
            acquire_count.fetch_add(1, Ordering::SeqCst);
            let receiver = receivers.lock().unwrap().remove(&key).unwrap();
            Box::pin(async move { Ok(receiver.await.unwrap()) })
        });
        let cache = AutoRefreshingCache::with_clock(
            acquire,
            Duration::seconds(BACKGROUND_TIMEOUT),
            Arc::new(|| at(0)),
        );

        let first = tokio::spawn({
            let cache = cache.clone();
            async move { cache.get(&1).await.unwrap() }
        });
        let second = tokio::spawn({
            let cache = cache.clone();
            async move { cache.get(&2).await.unwrap() }
        });
        while count.load(Ordering::SeqCst) < 2 {
            tokio::task::yield_now().await;
        }

        first_sender.send(value(1, 100, 200)).unwrap();
        second_sender.send(value(2, 100, 200)).unwrap();
        assert_eq!(first.await.unwrap().id, 1);
        assert_eq!(second.await.unwrap().id, 2);
    }
}
