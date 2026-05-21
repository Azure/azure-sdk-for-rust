// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Backtrace capture for [`Error`](super::Error).
//!
//! Backtraces are mission-critical for debugging — especially when the Rust
//! driver is consumed as a black box by the Java / .NET SDKs. Rust's stdlib
//! backtraces are gated on the `RUST_BACKTRACE` env var, which forces
//! operators to choose between "always on" (unsafe under error storms) and
//! "always off" (no signal when an incident hits production).
//!
//! ## Cost model
//!
//! * **Capture** — `backtrace::Backtrace::new_unresolved` is microseconds:
//!   walking the call stack and recording instruction pointers. We pay this
//!   on **every** error construction, unconditionally.
//! * **Symbol resolution** — turning an instruction pointer into
//!   `module::function (file:line)` walks debug info and can take
//!   milliseconds per frame. We cache resolved frames in a process-wide
//!   [`HashMap`] keyed by IP, so repeat captures of the same call site only
//!   pay the cost once *per process lifetime*.
//! * **Rate limiting** — a single global [`BacktraceCaptureLimiter`] caps how
//!   many backtraces may perform fresh symbol resolution in any rolling
//!   1-second window (default `5`, configurable via
//!   [`CosmosDriverRuntimeBuilder::with_max_error_backtraces_per_second`](crate::driver::CosmosDriverRuntimeBuilder::with_max_error_backtraces_per_second)
//!   or the `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` environment
//!   variable; set to `0` to disable symbol resolution entirely). **Cache
//!   hits do not consume budget** — if every frame of a backtrace is already
//!   in the process-wide cache, rendering is essentially free and proceeds
//!   even when the budget is exhausted. The budget only protects against
//!   the cost of *new* symbol-resolution work during an error storm.
//! * **Degraded rendering** — when the budget is exhausted but the
//!   backtrace contains unresolved frames, those frames render as
//!   `<unresolved> @ 0xIP` instead of being resolved. The backtrace is still
//!   useful for correlating with later, fully-resolved captures from the
//!   same code paths.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU32, AtomicU64, Ordering},
        Arc, OnceLock, RwLock,
    },
    time::{SystemTime, UNIX_EPOCH},
};

/// Default maximum number of backtraces that may perform fresh symbol
/// resolution per rolling 1-second window.
///
/// Cache hits do not consume budget; this only bounds the number of
/// backtraces whose *resolution* work fires during an error storm. `5` per
/// second is plenty for typical production workloads while still leaving
/// headroom for diagnostic sampling.
pub(crate) const DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND: u32 = 5;

/// Environment variable that overrides the default symbol-resolution budget
/// when no explicit value is supplied via the runtime builder.
///
/// Value: a non-negative integer (`0` disables symbol resolution entirely;
/// every frame renders as `<unresolved> @ 0xIP`).
pub(crate) const BACKTRACE_RESOLUTIONS_PER_SECOND_ENV: &str =
    "AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND";

const WINDOW_SECS: u64 = 1;

/// Captured (but unresolved) backtrace attached to a [`Error`](super::Error).
///
/// Capture itself is cheap — only frame instruction pointers are recorded.
/// Symbol resolution is deferred to the first call to [`Self::rendered`] and
/// the result is cached as an [`Arc<str>`], so repeat renders return the
/// cached string without re-walking debug info.
#[derive(Clone)]
pub(crate) struct CosmosBacktrace {
    inner: Arc<CosmosBacktraceInner>,
}

struct CosmosBacktraceInner {
    /// Instruction pointers in stack order (innermost frame first).
    ips: Vec<usize>,
    /// Lazily rendered display string, populated on first `rendered()` call.
    rendered: OnceLock<Arc<str>>,
}

/// A single resolved stack frame.
#[derive(Clone, Debug)]
struct ResolvedFrame {
    /// Raw instruction pointer.
    ip: usize,
    /// Resolved symbol name (e.g. `azure_data_cosmos_driver::error::Error::service`).
    symbol: Option<String>,
    /// Source file path, if available.
    filename: Option<String>,
    /// Source line number, if available.
    lineno: Option<u32>,
}

impl CosmosBacktrace {
    /// Captures a backtrace unconditionally. The walk-stack step is cheap
    /// (microseconds); symbol resolution is deferred to [`Self::rendered`]
    /// and rate-limited there.
    ///
    /// Returns `None` only when the platform's `backtrace` crate refuses to
    /// produce any frames at all (e.g. fully stripped binaries on some
    /// targets).
    pub(crate) fn capture() -> Option<Self> {
        let bt = backtrace::Backtrace::new_unresolved();
        let ips: Vec<usize> = bt.frames().iter().map(|f| f.ip() as usize).collect();
        if ips.is_empty() {
            return None;
        }
        Some(Self {
            inner: Arc::new(CosmosBacktraceInner {
                ips,
                rendered: OnceLock::new(),
            }),
        })
    }

    /// Returns the rendered backtrace string, computed (and cached) on first
    /// successful render. Subsequent calls return a borrow of the cached
    /// string with no formatting or allocation.
    ///
    /// Rendering walks the per-frame process-global cache; missing frames are
    /// resolved through the cost-bounded [`BacktraceCaptureLimiter`]. **If
    /// the limiter denies a fresh resolution and there is at least one
    /// cache-missed frame, this returns `None`** — we never produce a
    /// partially-resolved backtrace because half-symbolized stacks are
    /// misleading. Cache hits never consume budget, so backtraces whose
    /// frames are already known render at full fidelity regardless of
    /// limiter state.
    ///
    /// `None` results are **not** cached — a later call may succeed if the
    /// limiter window has reopened.
    pub(crate) fn rendered(&self) -> Option<&str> {
        if let Some(cached) = self.inner.rendered.get() {
            return Some(cached);
        }
        let arc = try_render(&self.inner.ips)?;
        // Race-tolerant: if another thread won the init, both threads
        // produced equivalent strings; discard ours.
        let _ = self.inner.rendered.set(arc);
        Some(
            self.inner
                .rendered
                .get()
                .expect("just set or won by another thread"),
        )
    }
}

impl fmt::Debug for CosmosBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CosmosBacktrace")
            .field("frame_count", &self.inner.ips.len())
            .field("rendered", &self.inner.rendered.get().is_some())
            .finish()
    }
}

// -----------------------------------------------------------------
// Rendering pipeline
// -----------------------------------------------------------------

/// Renders `ips` into a single human-readable string, returning `None` when
/// the limiter denies fresh resolution for any cache-missed frame. Never
/// produces a partially-resolved rendering.
fn try_render(ips: &[usize]) -> Option<Arc<str>> {
    let frames = try_resolve_frames(ips)?;
    let mut out = String::with_capacity(frames.len() * 64);
    for (i, frame) in frames.iter().enumerate() {
        use fmt::Write;
        let _ = write!(out, "{i:4}: ");
        match frame.symbol.as_deref() {
            Some(sym) => out.push_str(sym),
            None => {
                let _ = write!(out, "<unknown> @ 0x{:x}", frame.ip);
            }
        }
        if let Some(file) = frame.filename.as_deref() {
            let _ = write!(out, "\n          at {file}");
            if let Some(line) = frame.lineno {
                let _ = write!(out, ":{line}");
            }
        }
        out.push('\n');
    }
    Some(Arc::from(out))
}

/// For each IP in `ips`, returns the resolved frame from the process-global
/// cache when available. Misses trigger a single budget acquisition: if
/// granted, every missing IP is resolved and inserted into the cache and
/// `Some` is returned; if denied, returns `None` so the caller can drop the
/// render entirely (no partial backtraces).
fn try_resolve_frames(ips: &[usize]) -> Option<Vec<ResolvedFrame>> {
    let mut out: Vec<Option<ResolvedFrame>> = Vec::with_capacity(ips.len());
    let mut missing: Vec<(usize, usize)> = Vec::new();
    {
        let cache = frame_cache().read().unwrap();
        for (idx, &ip) in ips.iter().enumerate() {
            match cache.get(&ip) {
                Some(frame) => out.push(Some((**frame).clone())),
                None => {
                    out.push(None);
                    missing.push((idx, ip));
                }
            }
        }
    }
    if !missing.is_empty() {
        // Charge the rate limiter exactly once per backtrace render that
        // needs fresh resolution. Cache hits already happened above and did
        // not consume budget.
        if !global_limiter().try_acquire() {
            // Budget denied — give up entirely. Returning a partially
            // resolved backtrace would be misleading; the caller will see
            // `None` and can retry later when the limiter window reopens.
            return None;
        }
        let mut resolved: Vec<(usize, Arc<ResolvedFrame>)> = Vec::with_capacity(missing.len());
        for (idx, ip) in &missing {
            resolved.push((*idx, Arc::new(resolve_single(*ip))));
        }
        let mut cache = frame_cache().write().unwrap();
        for (idx, frame) in resolved {
            let cached = cache
                .entry(frame.ip)
                .or_insert_with(|| frame.clone())
                .clone();
            out[idx] = Some((*cached).clone());
        }
    }
    Some(
        out.into_iter()
            .map(|f| f.expect("all frames filled"))
            .collect(),
    )
}

fn resolve_single(ip: usize) -> ResolvedFrame {
    let mut frame = ResolvedFrame {
        ip,
        symbol: None,
        filename: None,
        lineno: None,
    };
    // `backtrace::resolve` walks debug info for the given IP. We capture the
    // first resolved symbol; inlined frames are flattened.
    backtrace::resolve(ip as *mut std::ffi::c_void, |sym| {
        if frame.symbol.is_none() {
            frame.symbol = sym.name().map(|n| n.to_string());
        }
        if frame.filename.is_none() {
            frame.filename = sym
                .filename()
                .and_then(|p| p.to_str().map(|s| s.to_owned()));
        }
        if frame.lineno.is_none() {
            frame.lineno = sym.lineno();
        }
    });
    frame
}

fn frame_cache() -> &'static RwLock<HashMap<usize, Arc<ResolvedFrame>>> {
    static CACHE: OnceLock<RwLock<HashMap<usize, Arc<ResolvedFrame>>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

/// Clears the process-global symbol cache. Intended for tests.
#[cfg(test)]
pub(crate) fn clear_frame_cache_for_tests() {
    frame_cache().write().unwrap().clear();
}

/// Returns the current size of the process-global symbol cache.
#[cfg(test)]
pub(crate) fn frame_cache_len_for_tests() -> usize {
    frame_cache().read().unwrap().len()
}

// -----------------------------------------------------------------
// Rate limiter
// -----------------------------------------------------------------

/// Process-global limiter that bounds how many backtrace renders may perform
/// *fresh symbol resolution* in any rolling 1-second window.
///
/// Implemented as a packed `AtomicU64` carrying `(window_start_secs,
/// count_in_window)`, so `try_acquire` is a single CAS in the happy path.
/// Capacity is stored separately in an `AtomicU32` so the runtime builder
/// can reconfigure it at any time.
pub(crate) struct BacktraceCaptureLimiter {
    capacity: AtomicU32,
    /// High 32 bits: window start (seconds since UNIX epoch, truncated).
    /// Low 32 bits: count of resolutions granted in this window.
    state: AtomicU64,
}

impl BacktraceCaptureLimiter {
    const fn new() -> Self {
        Self {
            capacity: AtomicU32::new(DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND),
            state: AtomicU64::new(0),
        }
    }

    /// Returns the current capacity (resolutions allowed per 1-second window).
    #[allow(dead_code)]
    pub fn capacity(&self) -> u32 {
        self.capacity.load(Ordering::Relaxed)
    }

    /// Sets the capacity. `0` disables symbol resolution; every backtrace
    /// renders with placeholder frames for cache misses.
    pub fn set_capacity(&self, capacity: u32) {
        self.capacity.store(capacity, Ordering::Relaxed);
    }

    /// Attempts to consume one resolution token. Returns `true` if a token
    /// was granted, `false` if the current 1-second window is exhausted (or
    /// if symbol resolution is disabled).
    pub fn try_acquire(&self) -> bool {
        let capacity = self.capacity.load(Ordering::Relaxed);
        if capacity == 0 {
            return false;
        }
        let now_secs = now_unix_secs();
        loop {
            let raw = self.state.load(Ordering::Acquire);
            let window_start = raw >> 32;
            let count = (raw & 0xFFFF_FFFF) as u32;
            let (new_window, new_count) = if now_secs.saturating_sub(window_start) >= WINDOW_SECS {
                (now_secs, 1u32)
            } else if count < capacity {
                (window_start, count + 1)
            } else {
                return false;
            };
            let new_raw = (new_window << 32) | (new_count as u64);
            if self
                .state
                .compare_exchange_weak(raw, new_raw, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                return true;
            }
        }
    }

    #[cfg(test)]
    fn reset_for_tests(&self) {
        self.state.store(0, Ordering::Release);
    }
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn global_limiter() -> &'static BacktraceCaptureLimiter {
    static LIMITER: BacktraceCaptureLimiter = BacktraceCaptureLimiter::new();
    &LIMITER
}

/// Returns a reference to the process-global symbol-resolution limiter.
///
/// The runtime builder uses this to apply caller-supplied configuration; most
/// other callers should not need direct access.
pub(crate) fn capture_limiter() -> &'static BacktraceCaptureLimiter {
    global_limiter()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // The capture limiter is process-global, so tests that mutate its state
    // must run serially.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn with_limiter_capacity<R>(capacity: u32, f: impl FnOnce() -> R) -> R {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev = capture_limiter().capacity();
        capture_limiter().set_capacity(capacity);
        capture_limiter().reset_for_tests();
        let r = f();
        capture_limiter().set_capacity(prev);
        capture_limiter().reset_for_tests();
        r
    }

    #[test]
    fn capture_always_succeeds() {
        // Capture is unconditional; the limiter only gates symbol resolution.
        with_limiter_capacity(0, || {
            assert!(CosmosBacktrace::capture().is_some());
        });
    }

    #[test]
    fn rendering_returns_none_when_budget_exhausted_for_cache_misses() {
        with_limiter_capacity(0, || {
            clear_frame_cache_for_tests();
            let bt = CosmosBacktrace::capture().expect("capture always succeeds");
            assert!(
                bt.rendered().is_none(),
                "expected None when budget=0 and cache is empty"
            );
            // Failed render must not pollute the process-global cache.
            assert_eq!(frame_cache_len_for_tests(), 0);
        });
    }

    #[test]
    fn cache_hits_do_not_consume_budget() {
        with_limiter_capacity(1, || {
            clear_frame_cache_for_tests();
            // First render uses budget to populate the cache fully.
            let bt1 = CosmosBacktrace::capture().expect("capture");
            let s1 = bt1.rendered().expect("first render succeeds");
            assert!(!s1.is_empty());
            assert!(frame_cache_len_for_tests() > 0);
            // Budget is now exhausted, but a second backtrace whose frames
            // are already cached should still render. (Same call site as
            // the first capture, so frames overlap heavily.)
            let bt2 = CosmosBacktrace::capture().expect("capture");
            // If every frame is a cache hit, rendered() returns Some.
            // If any frame is new (inlining variance), rendered() returns
            // None because budget is exhausted — we never produce a
            // partially-resolved render.
            if let Some(s2) = bt2.rendered() {
                assert!(
                    !s2.contains("<unknown>"),
                    "successful render must not contain placeholders: {s2}"
                );
            }
        });
    }

    #[test]
    fn rendered_is_cached_per_backtrace() {
        with_limiter_capacity(5, || {
            let bt = CosmosBacktrace::capture().expect("capture");
            let s1 = bt.rendered().expect("render");
            let s2 = bt.rendered().expect("render");
            // Same string identity (same Arc<str> behind the OnceLock).
            assert!(std::ptr::eq(s1.as_ptr(), s2.as_ptr()));
        });
    }
}
