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
//! This module captures every error backtrace by default but bounds the cost
//! two ways:
//!
//! 1. **Rate limiting.** A global [`BacktraceCaptureLimiter`] enforces a
//!    sliding 60-second budget (default `1000` captures / minute, configurable
//!    via [`CosmosDriverRuntimeBuilder::with_max_error_backtraces_per_minute`](crate::driver::CosmosDriverRuntimeBuilder::with_max_error_backtraces_per_minute)
//!    or the `AZURE_COSMOS_BACKTRACE_CAPTURE_PER_MINUTE` environment
//!    variable; set to `0` to disable backtrace capture entirely).
//! 2. **Symbol-resolution caching.** The expensive part of a backtrace is
//!    resolving instruction pointers to symbol names + filenames + line
//!    numbers. Capture itself (just walking the stack) is cheap. We capture
//!    *unresolved* frame addresses on the hot path; resolution is deferred to
//!    the first call to [`CosmosBacktrace::frames`] or [`Display`], and every
//!    resolved frame is cached in a process-global table keyed by IP so
//!    repeat captures (the common case during an error storm) pay the
//!    resolution cost at most once per unique frame.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU32, AtomicU64, AtomicU8, Ordering},
        Arc, OnceLock, RwLock,
    },
    time::{SystemTime, UNIX_EPOCH},
};

use super::Kind;

/// Default maximum number of backtraces captured per rolling 60-second window.
///
/// Backtraces are now captured only for SDK-origin error kinds (see
/// [`DEFAULT_BACKTRACE_KIND_MASK`]); high-volume service errors (404 / 409 /
/// 412 / 429) and opaque transport failures do not consume budget. `100` per
/// minute is therefore plenty for typical production workloads while still
/// leaving headroom for diagnostic sampling.
pub(crate) const DEFAULT_BACKTRACE_CAPTURES_PER_MINUTE: u32 = 100;

/// Environment variable that overrides the default backtrace-capture budget
/// when no explicit value is supplied via the runtime builder.
///
/// Value: a non-negative integer (`0` disables backtrace capture entirely).
pub(crate) const BACKTRACE_CAPTURES_PER_MINUTE_ENV: &str =
    "AZURE_COSMOS_BACKTRACE_CAPTURE_PER_MINUTE";

const WINDOW_SECS: u64 = 60;

// Bit positions for the per-kind capture mask. Kept private — callers
// configure capture via the typed [`BacktraceCaptureLimiter`] API.
const BIT_SERVICE: u8 = 1 << 0;
const BIT_TRANSPORT: u8 = 1 << 1;
const BIT_CLIENT: u8 = 1 << 2;
const BIT_AUTHENTICATION: u8 = 1 << 3;
const BIT_SERIALIZATION: u8 = 1 << 4;
const BIT_CONFIGURATION: u8 = 1 << 5;

/// Default set of [`Kind`]s for which backtraces are captured.
///
/// Excludes `Service`, `Transport`, and `Authentication` — those failures are
/// either already self-describing via the wire response (status + sub-status +
/// activity-id + server diagnostics) or bottom out in third-party async-IO
/// stacks where a Rust backtrace adds little value.
pub(crate) const DEFAULT_BACKTRACE_KIND_MASK: u8 =
    BIT_CLIENT | BIT_SERIALIZATION | BIT_CONFIGURATION;

fn kind_bit(kind: Kind) -> u8 {
    match kind {
        Kind::Service => BIT_SERVICE,
        Kind::Transport => BIT_TRANSPORT,
        Kind::Client => BIT_CLIENT,
        Kind::Authentication => BIT_AUTHENTICATION,
        Kind::Serialization => BIT_SERIALIZATION,
        Kind::Configuration => BIT_CONFIGURATION,
    }
}

/// Captured (but unresolved) backtrace attached to a [`Error`](super::Error).
///
/// Capture itself is cheap — only frame instruction pointers are recorded.
/// Symbol resolution is deferred to the first call to [`Self::frames`] or
/// [`Display`] and cached in a process-global table keyed by IP, so repeat
/// captures of the same call site only pay the resolution cost once.
#[derive(Clone)]
pub(crate) struct CosmosBacktrace {
    inner: Arc<CosmosBacktraceInner>,
}

struct CosmosBacktraceInner {
    /// Instruction pointers in stack order (innermost frame first).
    ips: Vec<usize>,
    /// Lazily resolved frames, populated on first access.
    resolved: OnceLock<Vec<Arc<ResolvedFrame>>>,
    /// Lazily rendered display string, populated on first `rendered()` call.
    /// Stored as `Arc<str>` so callers can cheaply share ownership without
    /// re-copying the bytes.
    rendered: OnceLock<Arc<str>>,
}

/// A single resolved stack frame.
#[derive(Clone, Debug)]
pub(crate) struct ResolvedFrame {
    /// Raw instruction pointer.
    pub ip: usize,
    /// Resolved symbol name (e.g. `azure_data_cosmos_driver::error::Error::service`).
    pub symbol: Option<String>,
    /// Source file path, if available.
    pub filename: Option<String>,
    /// Source line number, if available.
    pub lineno: Option<u32>,
}

impl CosmosBacktrace {
    /// Attempts to capture a backtrace for the given error kind, honoring the
    /// global per-kind enable mask and per-minute budget.
    ///
    /// Returns `None` if backtraces are disabled for `kind`, if the limiter
    /// has already issued the maximum number of captures in the current
    /// 60-second window, or if capture is globally disabled (budget = `0`).
    /// Disabled kinds do **not** charge the limiter — the budget is reserved
    /// for the kinds where a stack actually pinpoints the fault.
    pub(crate) fn try_capture_for_kind(kind: Kind) -> Option<Self> {
        if !global_limiter().kind_enabled(kind) {
            return None;
        }
        Self::try_capture()
    }

    /// Attempts to capture a backtrace, honoring the global per-minute budget
    /// but **ignoring** the per-kind enable mask.
    ///
    /// Returns `None` if the limiter has already issued the maximum number of
    /// captures in the current 60-second window, or if backtrace capture is
    /// disabled (budget = `0`). Prefer [`Self::try_capture_for_kind`] when the
    /// error kind is known so that disabled kinds skip the budget entirely.
    pub(crate) fn try_capture() -> Option<Self> {
        if !global_limiter().try_acquire() {
            return None;
        }
        let bt = backtrace::Backtrace::new_unresolved();
        let ips: Vec<usize> = bt.frames().iter().map(|f| f.ip() as usize).collect();
        if ips.is_empty() {
            return None;
        }
        Some(Self {
            inner: Arc::new(CosmosBacktraceInner {
                ips,
                resolved: OnceLock::new(),
                rendered: OnceLock::new(),
            }),
        })
    }

    /// Returns the resolved frames, resolving (and caching) on first call.
    pub(crate) fn frames(&self) -> &[Arc<ResolvedFrame>] {
        self.inner
            .resolved
            .get_or_init(|| resolve_frames(&self.inner.ips))
            .as_slice()
    }

    /// Returns the rendered backtrace string, computed (and cached) on first
    /// call. Subsequent calls return the cached `&str` without re-formatting
    /// or copying — the string lives inside the `OnceLock` for the lifetime
    /// of the backtrace.
    pub(crate) fn rendered(&self) -> &str {
        self.inner
            .rendered
            .get_or_init(|| Arc::from(self.to_string()))
    }

    /// Returns the number of captured frames (cheap; never triggers resolution).
    #[allow(dead_code)]
    pub(crate) fn frame_count(&self) -> usize {
        self.inner.ips.len()
    }
}

impl fmt::Display for CosmosBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, frame) in self.frames().iter().enumerate() {
            write!(f, "{i:4}: ")?;
            match frame.symbol.as_deref() {
                Some(sym) => f.write_str(sym)?,
                None => write!(f, "<unknown> @ 0x{:x}", frame.ip)?,
            }
            if let Some(file) = frame.filename.as_deref() {
                write!(f, "\n          at {file}")?;
                if let Some(line) = frame.lineno {
                    write!(f, ":{line}")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl fmt::Debug for CosmosBacktrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CosmosBacktrace")
            .field("frame_count", &self.inner.ips.len())
            .field("resolved", &self.inner.resolved.get().is_some())
            .finish()
    }
}

// -----------------------------------------------------------------
// Symbol resolution cache
// -----------------------------------------------------------------

fn frame_cache() -> &'static RwLock<HashMap<usize, Arc<ResolvedFrame>>> {
    static CACHE: OnceLock<RwLock<HashMap<usize, Arc<ResolvedFrame>>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

fn resolve_frames(ips: &[usize]) -> Vec<Arc<ResolvedFrame>> {
    let mut out = Vec::with_capacity(ips.len());
    // First pass: try the read lock for cache hits.
    let mut missing: Vec<(usize, usize)> = Vec::new();
    {
        let cache = frame_cache().read().unwrap();
        for (idx, &ip) in ips.iter().enumerate() {
            match cache.get(&ip) {
                Some(frame) => out.push(Some(frame.clone())),
                None => {
                    out.push(None);
                    missing.push((idx, ip));
                }
            }
        }
    }
    if !missing.is_empty() {
        // Resolve missing frames outside any lock.
        let mut resolved: Vec<(usize, Arc<ResolvedFrame>)> = Vec::with_capacity(missing.len());
        for (idx, ip) in missing {
            resolved.push((idx, Arc::new(resolve_single(ip))));
        }
        // Insert into cache under write lock; another thread may have
        // populated the same IPs in between — last writer wins, both copies
        // are semantically equivalent.
        let mut cache = frame_cache().write().unwrap();
        for (idx, frame) in resolved {
            cache.entry(frame.ip).or_insert_with(|| frame.clone());
            out[idx] = Some(frame);
        }
    }
    out.into_iter()
        .map(|f| f.expect("all frames filled"))
        .collect()
}

fn resolve_single(ip: usize) -> ResolvedFrame {
    let mut frame = ResolvedFrame {
        ip,
        symbol: None,
        filename: None,
        lineno: None,
    };
    // SAFETY: `backtrace::resolve` walks debug info for the given IP. We
    // capture the first resolved symbol; inlined frames are flattened.
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

/// Process-global limiter that bounds how many backtraces may be captured in
/// any rolling 60-second window.
///
/// Implemented as a packed `AtomicU64` carrying `(window_start_secs,
/// count_in_window)`, so `try_acquire` is a single CAS in the happy path.
/// Capacity is stored separately in an `AtomicU32` so the runtime builder can
/// reconfigure it at any time.
pub(crate) struct BacktraceCaptureLimiter {
    capacity: AtomicU32,
    /// High 32 bits: window start (seconds since UNIX epoch, truncated).
    /// Low 32 bits: count of captures granted in this window.
    state: AtomicU64,
    /// Bitmask of [`Kind`]s for which capture is enabled.
    kind_mask: AtomicU8,
}

impl BacktraceCaptureLimiter {
    const fn new() -> Self {
        Self {
            capacity: AtomicU32::new(DEFAULT_BACKTRACE_CAPTURES_PER_MINUTE),
            state: AtomicU64::new(0),
            kind_mask: AtomicU8::new(DEFAULT_BACKTRACE_KIND_MASK),
        }
    }

    /// Returns the current capacity (captures allowed per 60-second window).
    #[allow(dead_code)]
    pub fn capacity(&self) -> u32 {
        self.capacity.load(Ordering::Relaxed)
    }

    /// Sets the capacity. `0` disables backtrace capture.
    pub fn set_capacity(&self, capacity: u32) {
        self.capacity.store(capacity, Ordering::Relaxed);
    }

    /// Returns `true` if backtrace capture is currently enabled for `kind`.
    pub fn kind_enabled(&self, kind: Kind) -> bool {
        self.kind_mask.load(Ordering::Relaxed) & kind_bit(kind) != 0
    }

    /// Enables or disables backtrace capture for a specific [`Kind`].
    pub fn set_kind_enabled(&self, kind: Kind, enabled: bool) {
        let bit = kind_bit(kind);
        if enabled {
            self.kind_mask.fetch_or(bit, Ordering::Relaxed);
        } else {
            self.kind_mask.fetch_and(!bit, Ordering::Relaxed);
        }
    }

    /// Attempts to consume one capture token. Returns `true` if a token was
    /// granted, `false` if the current 60-second window is exhausted (or if
    /// the limiter is disabled).
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
        self.kind_mask
            .store(DEFAULT_BACKTRACE_KIND_MASK, Ordering::Release);
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

/// Returns a reference to the process-global backtrace capture limiter.
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
    fn disabled_limiter_returns_none() {
        with_limiter_capacity(0, || {
            assert!(CosmosBacktrace::try_capture().is_none());
        });
    }

    #[test]
    fn captures_up_to_capacity_then_denies() {
        with_limiter_capacity(3, || {
            assert!(CosmosBacktrace::try_capture().is_some());
            assert!(CosmosBacktrace::try_capture().is_some());
            assert!(CosmosBacktrace::try_capture().is_some());
            assert!(CosmosBacktrace::try_capture().is_none());
        });
    }

    #[test]
    fn frames_resolve_and_cache() {
        with_limiter_capacity(2, || {
            clear_frame_cache_for_tests();
            let bt1 = CosmosBacktrace::try_capture().expect("capture allowed");
            let frames1 = bt1.frames();
            assert!(!frames1.is_empty());
            let cache_after_first = frame_cache_len_for_tests();
            assert!(cache_after_first > 0);
            // Second capture from the same site should hit the cache for
            // most frames — exact equality isn't guaranteed (a few frames may
            // differ between captures due to inlining variance) but the
            // cache size should not balloon.
            let bt2 = CosmosBacktrace::try_capture().expect("capture allowed");
            let _ = bt2.frames();
            let cache_after_second = frame_cache_len_for_tests();
            assert!(cache_after_second <= cache_after_first + bt2.frame_count());
        });
    }

    #[test]
    fn display_renders_resolved_frames() {
        with_limiter_capacity(1, || {
            let bt = CosmosBacktrace::try_capture().expect("capture allowed");
            let s = bt.to_string();
            assert!(s.contains("0:"), "expected frame index marker, got: {s}");
        });
    }

    #[test]
    fn try_capture_for_kind_honors_default_mask() {
        with_limiter_capacity(10, || {
            // SDK-origin kinds capture by default.
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Client).is_some());
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Serialization).is_some());
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Configuration).is_some());
            // Service / Transport / Authentication are skipped by default and
            // do not consume budget.
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Service).is_none());
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Transport).is_none());
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Authentication).is_none());
        });
    }

    #[test]
    fn set_kind_enabled_toggles_capture() {
        with_limiter_capacity(2, || {
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Service).is_none());
            capture_limiter().set_kind_enabled(Kind::Service, true);
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Service).is_some());
            capture_limiter().set_kind_enabled(Kind::Service, false);
            assert!(CosmosBacktrace::try_capture_for_kind(Kind::Service).is_none());
        });
    }
}
