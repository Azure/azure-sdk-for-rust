// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cspell:ignore dlopen

//! Backtrace capture for [`Error`](super::Error).
//!
//! Backtraces are invaluable for debugging — especially when the Rust
//! driver is consumed as a black box by the Java / .NET SDKs. Following
//! Rust's stdlib convention, capture is **opt-in**: it stays off until the
//! operator asks for it, either by setting the stdlib `RUST_BACKTRACE`
//! environment variable or by passing an explicit capacity to the runtime
//! builder. Defaults preserve cost predictability under error storms
//! without surprising callers who expect idiomatic Rust behaviour.
//!
//! ## Cost model
//!
//! * **Capture** — `backtrace::trace` is microseconds: walking the call
//!   stack and recording instruction pointers. When capture is enabled we
//!   pay this on every error construction up to the per-second cap.
//! * **Symbol resolution** — turning an instruction pointer into
//!   `module::function (file:line)` walks debug info and can take
//!   milliseconds per frame. We cache resolved frames in a process-wide
//!   [`HashMap`] keyed by IP, so repeat captures of the same call site only
//!   pay the cost once *per process lifetime*.
//! * **Rate limiting** — a single global [`BacktraceCaptureLimiter`] caps how
//!   many backtraces may perform fresh symbol resolution in any rolling
//!   1-second window, configurable via
//!   [`CosmosDriverRuntimeBuilder::with_max_error_backtrace_resolutions_per_second`](crate::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_resolutions_per_second)
//!   or the `AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND` environment
//!   variable. Setting either to `0` fully disables capture for that
//!   knob. **Cache hits do not consume budget** — if every frame of a
//!   backtrace is already in the process-wide cache, rendering is
//!   essentially free and proceeds even when the budget is exhausted. The
//!   budget only protects against the cost of *new* symbol-resolution
//!   work during an error storm.
//! * **Degraded rendering** — when the budget is exhausted but the
//!   backtrace contains unresolved frames, those frames render as
//!   `<unresolved> @ 0xIP` instead of being resolved. The backtrace is still
//!   useful for correlating with later, fully-resolved captures from the
//!   same code paths.

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering},
        Arc, OnceLock, RwLock,
    },
    time::Instant,
};

/// Safe per-second resolution budget used when capture is implicitly
/// enabled via `RUST_BACKTRACE`.
///
/// Cache hits do not consume budget; this only bounds the number of
/// backtraces whose *resolution* work fires during an error storm. `5` per
/// second is plenty for typical production workloads while still leaving
/// headroom for diagnostic sampling.
pub(crate) const DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND_WHEN_ENABLED: u32 = 5;

/// Default per-second resolution budget when capture is *not* explicitly
/// requested. `0` means "no fresh symbol resolution" — combined with the
/// disabled capture default below, this leaves backtraces fully off until
/// the operator opts in.
pub(crate) const DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND_DISABLED: u32 = 0;

/// Environment variable that overrides the default symbol-resolution budget
/// when no explicit value is supplied via the runtime builder.
///
/// Value: a non-negative integer (`>= 0`). Setting it to `0` disables
/// fresh symbol resolution entirely; captures still happen (subject to
/// the capture cap below) but unresolved frames render as
/// `<unresolved> @ 0xIP` placeholders. Set a low value like `1` to keep a
/// trickle of cold-cache resolution alive during an error storm; the
/// process-global symbol cache means recurring failures from the same
/// call sites still render at full fidelity for free.
pub(crate) const BACKTRACE_RESOLUTIONS_PER_SECOND_ENV: &str =
    "AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND";

/// Safe per-second capture cap used when capture is implicitly enabled
/// via `RUST_BACKTRACE`.
///
/// The resolution limiter
/// ([`DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND_WHEN_ENABLED`]) bounds the
/// *expensive* symbol-resolution work, but plain stack capture itself
/// (walking frames + allocating the IP vector) still costs a few
/// microseconds and a small allocation per error. Under a sustained
/// error storm where every failure originates from the same handful of
/// call sites — cache-hit-only territory where the resolution limiter is
/// never even asked — unbounded capture would still dominate CPU. This
/// throttle puts a hard ceiling on captures so the worst-case capture
/// cost is `O(cap)` microseconds per second regardless of error rate.
///
/// `10_000` is a generous default; tighten or relax via
/// [`CosmosDriverRuntimeBuilder::with_max_error_backtrace_captures_per_second`](crate::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_captures_per_second)
/// or the [`BACKTRACE_CAPTURES_PER_SECOND_ENV`] environment variable.
pub(crate) const DEFAULT_BACKTRACE_CAPTURES_PER_SECOND_WHEN_ENABLED: u32 = 10_000;

/// Default per-second capture cap when capture is *not* explicitly
/// requested. `0` means "no captures" — [`Backtrace::capture`] returns
/// `None` before allocating the IP vector, so the whole pipeline is off.
pub(crate) const DEFAULT_BACKTRACE_CAPTURES_PER_SECOND_DISABLED: u32 = 0;

/// Environment variable that overrides the default per-second cap on stack
/// captures when no explicit value is supplied via the runtime builder.
///
/// Value: a non-negative integer (`>= 0`). Setting it to `0` disables
/// backtrace capture entirely (capture returns `None` and no IP vector
/// is allocated).
pub(crate) const BACKTRACE_CAPTURES_PER_SECOND_ENV: &str =
    "AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND";

/// Returns `true` when the stdlib `RUST_BACKTRACE` environment variable
/// asks for backtraces, using stdlib semantics: anything other than unset
/// / empty / `"0"` enables. Read **once** per process via [`OnceLock`]
/// (matching stdlib); mid-process mutations of the environment variable
/// have no effect.
pub(crate) fn rust_backtrace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| match std::env::var("RUST_BACKTRACE") {
        Ok(value) => !value.is_empty() && value != "0",
        Err(_) => false,
    })
}

const WINDOW_SECS: u64 = 1;

/// Default soft ceiling on the number of resolved frames retained in the
/// process-global symbol cache before it is swapped out and re-warmed
/// from scratch.
///
/// At ~100 bytes per entry the steady-state memory ceiling is ~10 MB.
/// Hit on the write path (next cache-miss after the cap is reached);
/// when triggered, the old map is *swapped* with a fresh empty one and
/// the actual `drop` of the swapped-out map (~100k `Arc<ResolvedFrame>`
/// decrements + ~100k `String` frees) is offloaded to a detached OS
/// thread, so the unlucky thread that triggered the cap hit pays only
/// the swap cost (`O(1)`). After the swap, subsequent renders pay the
/// normal resolution cost (gated by the resolution limiter), so the
/// only visible effect is a few renders returning `None` while the hot
/// set re-warms — the same contract callers already get under
/// resolution pressure.
///
/// In Rust-only steady-state deployments the cache rarely approaches
/// this number; the cap exists to bound memory in long-lived hosts that
/// load/unload modules (JNI / P/Invoke / `dlopen`).
const DEFAULT_FRAME_CACHE_SOFT_CAP: usize = 100_000;

/// Currently-active soft cap, read by [`try_resolve_frames`] on the
/// write path. Stored as an atomic so tests can lower the cap without
/// recompiling, deterministically exercising the eviction path.
static FRAME_CACHE_SOFT_CAP: AtomicUsize = AtomicUsize::new(DEFAULT_FRAME_CACHE_SOFT_CAP);

/// Captured (but unresolved) backtrace attached to a [`Error`](super::Error).
///
/// Capture itself is cheap — only frame instruction pointers are recorded.
/// Symbol resolution is deferred to the first call to [`Self::rendered`] and
/// the result is cached as an [`Arc<str>`], so repeat renders return the
/// cached string without re-walking debug info.
#[derive(Clone)]
pub struct Backtrace {
    inner: Arc<BacktraceInner>,
}

struct BacktraceInner {
    /// Instruction pointers in stack order (innermost frame first).
    ips: Vec<usize>,
    /// Lazily rendered display string, populated on first `rendered()`
    /// call. Stored as `Arc<str>` so callers that need to retain the
    /// rendered backtrace beyond the borrow (tracing fields, telemetry
    /// exporters, owned struct fields) can `Arc::clone` it for a
    /// refcount bump instead of copying the entire formatted string.
    /// `Some(s)` = render succeeded; the `Option` inside the `OnceLock`
    /// is `None` when rendering was attempted but denied by the
    /// resolution limiter — the outcome is cached either way so
    /// subsequent calls are deterministic.
    rendered: OnceLock<Option<Arc<str>>>,
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

impl Backtrace {
    /// Captures a backtrace, subject to a single production-safety gate:
    /// the **per-second capture throttle** ([`global_capture_throttle`]).
    ///
    /// Capture is opt-in: by default the throttle starts at capacity `0`
    /// (disabled) and only becomes non-zero when the runtime builder
    /// applies an explicit value, the `AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND`
    /// env var sets one, or `RUST_BACKTRACE` enables the safe default.
    /// When enabled, each successful capture consumes one token from a
    /// process-global rolling 1-second budget (configurable via
    /// [`CosmosDriverRuntimeBuilder::with_max_error_backtrace_captures_per_second`](crate::driver::CosmosDriverRuntimeBuilder::with_max_error_backtrace_captures_per_second)
    /// or the [`BACKTRACE_CAPTURES_PER_SECOND_ENV`] environment variable).
    /// When the budget is exhausted (or capacity is `0`), capture returns
    /// `None` before walking the stack or allocating the IP vector,
    /// bounding the worst-case stack-walk cost during an error storm.
    ///
    /// Capture and symbol resolution are deliberately decoupled: the
    /// resolution limiter (charged later by [`Self::rendered`]) gates
    /// expensive symbol-resolution work, not capture itself. Resolution
    /// pressure on one error site has no effect on capture for unrelated
    /// sites — capture is cheap (microseconds + small allocation) and is
    /// bounded by this throttle alone.
    ///
    /// Returns `None` when the throttle denies, or when the platform's
    /// `backtrace` crate refuses to produce any frames.
    pub(crate) fn capture() -> Option<Self> {
        if !global_capture_throttle().try_acquire() {
            return None;
        }
        // Walk the stack directly into a single `Vec<usize>` via the
        // callback-based `backtrace::trace`, avoiding the intermediate
        // `Vec<Frame>` allocation that `backtrace::Backtrace::new_unresolved`
        // would produce. `trace` is the thread-safe variant — fine for
        // arbitrary concurrent capture across the driver. Pre-size to a
        // typical Cosmos async stack depth (tower-style middleware +
        // Cosmos pipeline + tokio runtime frames commonly land in the
        // 40–60 range) so the common case fits in one allocation;
        // deeper stacks still capture correctly via `Vec::push`'s
        // amortized doubling growth.
        let mut ips: Vec<usize> = Vec::with_capacity(64);
        backtrace::trace(|frame| {
            ips.push(frame.ip() as usize);
            true
        });
        if ips.is_empty() {
            return None;
        }
        Some(Self {
            inner: Arc::new(BacktraceInner {
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
    /// The first call's outcome (`Some(s)` or `None`) is **cached on
    /// this [`Backtrace`] instance** — every subsequent call returns the
    /// same answer for the lifetime of the [`Backtrace`] (and, because
    /// `Backtrace` is shared by `Arc`, for every cloned/inherited copy).
    /// This gives [`Error::backtrace`](super::Error::backtrace) a
    /// per-instance deterministic contract; callers can call it multiple
    /// times (e.g. once for logging, once for telemetry) without risk of
    /// seeing inconsistent results.
    pub(crate) fn rendered(&self) -> Option<&Arc<str>> {
        self.inner
            .rendered
            .get_or_init(|| try_render(&self.inner.ips).map(Arc::<str>::from))
            .as_ref()
    }
}

impl fmt::Debug for Backtrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Backtrace")
            .field("frame_count", &self.inner.ips.len())
            .field("rendered", &self.inner.rendered.get().map(Option::is_some))
            .finish()
    }
}

#[cfg(test)]
impl Backtrace {
    /// Returns a pointer-identity handle (as `usize`) to the inner Arc,
    /// for tests that need to assert two `Backtrace` values refer to the
    /// same captured stack (e.g. backtrace-inheritance from a wrapped
    /// source).
    pub(crate) fn inner_arc_identity_for_tests(&self) -> usize {
        Arc::as_ptr(&self.inner) as usize
    }
}

// -----------------------------------------------------------------
// Rendering pipeline
// -----------------------------------------------------------------

/// Renders `ips` into a single human-readable string, returning `None` when
/// the limiter denies fresh resolution for any cache-missed frame. Never
/// produces a partially-resolved rendering.
fn try_render(ips: &[usize]) -> Option<String> {
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
    Some(out)
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
        if !global_resolution_limiter().try_acquire() {
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
        // Bound the cache to keep long-lived hosts that load/unload
        // modules (JNI / P/Invoke / dlopen) from accumulating frames
        // indefinitely. Swap the full map out for a fresh empty one and
        // hand the old map to a separate binding so its Drop — atomic
        // refcount decrements on every `Arc<ResolvedFrame>` plus String
        // frees — runs *off* the calling thread (see below). Keeps the
        // critical section `O(1)` even at the cap.
        let evicted = if cache.len() >= FRAME_CACHE_SOFT_CAP.load(Ordering::Relaxed) {
            Some(std::mem::take(&mut *cache))
        } else {
            None
        };
        for (idx, frame) in resolved {
            let cached = cache
                .entry(frame.ip)
                .or_insert_with(|| frame.clone())
                .clone();
            out[idx] = Some((*cached).clone());
        }
        drop(cache);
        // Offload the eviction drop (~100k `Arc<ResolvedFrame>` decrements +
        // ~100k `String` frees, ~10 MB of memory work) to a detached OS
        // thread so the unlucky thread that triggered the cap hit returns
        // immediately. Thread creation is ~10–100 μs vs ~1–10 ms of drop
        // work, so the trade-off is net positive even on the worst case;
        // cap hits are also rare (steady-state Cosmos workloads stay well
        // below 100k unique frames), so the spawned thread is essentially
        // free in aggregate. We deliberately do NOT use
        // `BackgroundTaskManager` here: that runs on tokio (which may not
        // be present at this synchronous error-construction call site) and
        // is per-instance (not reachable from the process-global frame
        // cache) — both make `std::thread::spawn` the simpler primitive.
        if let Some(evicted) = evicted {
            std::thread::Builder::new()
                .name("cosmos-backtrace-cache-evict".into())
                .spawn(move || drop(evicted))
                .map(drop)
                .unwrap_or_else(|_| {
                    // Thread creation failed (extreme OS resource pressure).
                    // Fall back to dropping on the current thread so we
                    // never leak the evicted map.
                });
        }
    }
    Some(
        out.into_iter()
            .map(|f| {
                // The invariant — every `None` slot in `out` has a matching
                // entry in `missing` that the second pass refills — holds
                // structurally today. We still avoid `.expect()` here: this
                // module renders into `Display` / `Debug` / panic-message
                // formatters, and a panic on the error path would recurse
                // (panic-while-formatting-a-panic) and be effectively
                // undiagnosable. A future refactor regression instead
                // surfaces as a single `<unknown>` placeholder frame that
                // `try_render` already knows how to print.
                debug_assert!(f.is_some(), "all frame slots must be filled");
                f.unwrap_or(ResolvedFrame {
                    ip: 0,
                    symbol: None,
                    filename: None,
                    lineno: None,
                })
            })
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

/// Returns `true` if `ip` is currently in the process-global symbol
/// cache. Used by tests that need a race-free assertion against cache
/// state (e.g. "a failed render did not insert this IP"), since the
/// cache is shared with any other test that renders backtraces in
/// parallel and absolute-size assertions on it are inherently fragile.
#[cfg(test)]
pub(crate) fn frame_cache_contains_for_tests(ip: usize) -> bool {
    frame_cache().read().unwrap().contains_key(&ip)
}

/// Returns the current size of the process-global symbol cache.
#[cfg(test)]
pub(crate) fn frame_cache_len_for_tests() -> usize {
    frame_cache().read().unwrap().len()
}

/// Overrides the frame-cache soft cap so eviction can be exercised
/// deterministically without filling 100k entries. Tests must restore
/// the previous value before returning.
#[cfg(test)]
pub(crate) fn set_frame_cache_soft_cap_for_tests(cap: usize) -> usize {
    FRAME_CACHE_SOFT_CAP.swap(cap, Ordering::Relaxed)
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
pub struct BacktraceCaptureLimiter {
    capacity: AtomicU32,
    /// High 32 bits: window start (seconds since UNIX epoch, truncated).
    /// Low 32 bits: count of resolutions granted in this window.
    state: AtomicU64,
}

impl BacktraceCaptureLimiter {
    /// Constructs a disabled limiter. The runtime builder sets the
    /// capacity from the resolved configuration (explicit value > env
    /// var > opt-in default keyed on `RUST_BACKTRACE`) before any
    /// capture or render observes the new value.
    const fn new_disabled() -> Self {
        Self {
            capacity: AtomicU32::new(0),
            state: AtomicU64::new(0),
        }
    }

    /// Returns the current capacity (tokens allowed per 1-second window).
    #[cfg(any(test, feature = "__internal_backtrace_bench"))]
    pub fn capacity(&self) -> u32 {
        self.capacity.load(Ordering::Relaxed)
    }

    /// Sets the capacity (tokens allowed per 1-second window). A capacity
    /// of `0` disables this limiter — every [`Self::try_acquire`] call
    /// returns `false` for as long as the capacity stays `0`.
    pub fn set_capacity(&self, capacity: u32) {
        self.capacity.store(capacity, Ordering::Relaxed);
    }

    /// Attempts to consume one token. Returns `true` if a token was
    /// granted, `false` if the current 1-second window is exhausted or
    /// the limiter is disabled (capacity `0`).
    pub fn try_acquire(&self) -> bool {
        let capacity = self.capacity.load(Ordering::Relaxed);
        if capacity == 0 {
            return false;
        }
        let now_secs = now_monotonic_secs();
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

    #[cfg(any(test, feature = "__internal_backtrace_bench"))]
    fn reset_for_tests(&self) {
        self.state.store(0, Ordering::Release);
    }
}

/// Returns the number of whole seconds elapsed since the process-global
/// monotonic anchor. The anchor is initialized lazily on first use via
/// [`OnceLock`] and never moves backwards regardless of wall-clock changes
/// (NTP step, suspend/resume), so the rolling 1-second window in
/// [`BacktraceCaptureLimiter`] is robust against clock skew. `SystemTime`
/// was used previously and could trigger spurious window rollovers or
/// stalls when the wall clock jumped.
fn now_monotonic_secs() -> u64 {
    static ANCHOR: OnceLock<Instant> = OnceLock::new();
    let anchor = ANCHOR.get_or_init(Instant::now);
    Instant::now().saturating_duration_since(*anchor).as_secs()
}

fn global_limiter() -> &'static BacktraceCaptureLimiter {
    static LIMITER: BacktraceCaptureLimiter = BacktraceCaptureLimiter::new_disabled();
    &LIMITER
}

/// Returns a reference to the process-global symbol-resolution limiter.
///
/// The runtime builder uses this to apply caller-supplied configuration; most
/// other callers should not need direct access.
pub(crate) fn global_resolution_limiter() -> &'static BacktraceCaptureLimiter {
    global_limiter()
}

/// Returns a reference to the process-global per-second cap on stack
/// captures (a second, independent limiter from the resolution one).
///
/// Each successful [`Backtrace::capture`] consumes one token; when the
/// budget is exhausted, capture returns `None` for the rest of the 1-second
/// window. The runtime builder uses this to apply caller-supplied
/// configuration.
pub(crate) fn global_capture_throttle() -> &'static BacktraceCaptureLimiter {
    static LIMITER: BacktraceCaptureLimiter = BacktraceCaptureLimiter::new_disabled();
    &LIMITER
}

/// Internal bench-only surface (gated by the `__internal_backtrace_bench`
/// feature) used by `azure_data_cosmos_benchmarks` to drive the
/// rate-limited backtrace machinery deterministically. Not covered by
/// SemVer; production code MUST NOT enable the feature.
#[cfg(feature = "__internal_backtrace_bench")]
#[doc(hidden)]
pub mod __bench {
    use super::{
        global_capture_throttle as inner_capture_throttle,
        global_resolution_limiter as inner_resolution_limiter, Backtrace, BacktraceCaptureLimiter,
    };
    use std::sync::Arc;

    /// Captures a fresh backtrace through the production capture path
    /// (subject to the global capture throttle). Returns `None` when the
    /// throttle is exhausted.
    pub fn capture() -> Option<Backtrace> {
        Backtrace::capture()
    }

    /// Renders the captured backtrace through the production render path
    /// (subject to the global resolution limiter and the process-wide
    /// frame cache). First call resolves and caches on the `Backtrace`
    /// instance; subsequent calls are `OnceLock` hits.
    pub fn render(bt: &Backtrace) -> Option<Arc<str>> {
        bt.rendered().cloned()
    }

    /// Returns the process-global capture throttle so benches can set
    /// capacity to exercise the throttled / un-throttled cases.
    pub fn capture_throttle() -> &'static BacktraceCaptureLimiter {
        inner_capture_throttle()
    }

    /// Returns the process-global symbol-resolution limiter so benches
    /// can set capacity to exercise the cold-resolution case.
    pub fn resolution_limiter() -> &'static BacktraceCaptureLimiter {
        inner_resolution_limiter()
    }

    /// Forces the limiter's window state back to the initial value so a
    /// bench can re-prime per group.
    pub fn reset_limiter(limiter: &BacktraceCaptureLimiter) {
        limiter.reset_for_tests();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Serializes backtrace tests that mutate the per-second limiter
    // capacity (also process-global). Tests in *other* modules that
    // merely render backtraces don't need this lock — they assert on
    // per-IP properties, not absolute cache size, so concurrent renders
    // cannot break them.
    static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn with_limiter_capacity<R>(capacity: u32, f: impl FnOnce() -> R) -> R {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev = global_resolution_limiter().capacity();
        global_resolution_limiter().set_capacity(capacity);
        global_resolution_limiter().reset_for_tests();
        // Ensure the capture throttle starts with a fresh window and a
        // generous capacity so it never accidentally gates these tests —
        // we are exercising the resolution limiter, not capture throttling.
        let prev_throttle = global_capture_throttle().capacity();
        global_capture_throttle().set_capacity(DEFAULT_BACKTRACE_CAPTURES_PER_SECOND_WHEN_ENABLED);
        global_capture_throttle().reset_for_tests();
        let r = f();
        global_resolution_limiter().set_capacity(prev);
        global_resolution_limiter().reset_for_tests();
        global_capture_throttle().set_capacity(prev_throttle);
        global_capture_throttle().reset_for_tests();
        r
    }

    #[test]
    fn capture_succeeds_under_resolution_pressure() {
        // Capture is bounded only by the capture throttle, not by the
        // resolution limiter. Even with the resolution budget at zero
        // (i.e. rendering will fail) capture must still succeed, because
        // the captured IPs are useful for later renders once the
        // resolution window rolls over, and resolution pressure on one
        // error site must never blind capture for unrelated sites.
        with_limiter_capacity(0, || {
            assert!(Backtrace::capture().is_some());
        });
    }

    #[test]
    fn capture_throttle_caps_per_second_captures() {
        with_limiter_capacity(5, || {
            // Set a small capture-throttle capacity and drain *more than*
            // capacity in a tight loop. We do NOT assert that the first N
            // calls succeed — sibling tests in the same process may be
            // constructing `Error` values (which each consume one capture
            // token via `from_inner`), depleting our budget faster than we
            // expect. What IS race-free is the post-drain assertion: once
            // the limiter has counted at least `capacity` grants in the
            // current window (whether by us or by parallel tests), any
            // subsequent call within the same window MUST be denied.
            let capacity = 5;
            global_capture_throttle().set_capacity(capacity);
            global_capture_throttle().reset_for_tests();
            for _ in 0..(capacity * 2) {
                let _ = Backtrace::capture();
            }
            assert!(
                Backtrace::capture().is_none(),
                "after draining {capacity} tokens, captures in the same window must be throttled"
            );
        });
    }

    #[test]
    fn rendering_returns_none_when_budget_exhausted_for_cache_misses() {
        with_limiter_capacity(0, || {
            clear_frame_cache_for_tests();
            let bt = Backtrace::capture().expect("capture always succeeds");
            assert!(
                bt.rendered().is_none(),
                "expected None when budget=0 and cache is empty"
            );
            // We intentionally do NOT assert that the failed render left
            // the process-global cache untouched. Async test runtimes
            // share harness frames across threads, so a sibling test
            // rendering a successful backtrace in parallel can insert IPs
            // that overlap with ours — making any post-hoc cache-state
            // assertion racy in either direction (absolute size OR
            // per-IP). The no-pollution guarantee is enforced by code
            // structure in `try_resolve_frames`: the budget check returns
            // `None` before any write to the cache, so a failed render
            // cannot insert.
        });
    }

    #[test]
    fn cache_hits_do_not_consume_budget() {
        with_limiter_capacity(1, || {
            clear_frame_cache_for_tests();
            // First render uses budget to populate the cache fully.
            let bt1 = Backtrace::capture().expect("capture");
            let s1 = bt1.rendered().expect("first render succeeds");
            assert!(!s1.is_empty());
            assert!(frame_cache_len_for_tests() > 0);
            // Budget is now exhausted, but a second backtrace whose frames
            // are already cached should still render. (Same call site as
            // the first capture, so frames overlap heavily.)
            let bt2 = Backtrace::capture().expect("capture");
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
            let bt = Backtrace::capture().expect("capture");
            let s1 = bt.rendered().expect("render");
            let s2 = bt.rendered().expect("render");
            // Same string identity (same backing buffer behind the OnceLock).
            assert!(std::ptr::eq(s1.as_ptr(), s2.as_ptr()));
        });
    }

    #[test]
    fn none_render_is_also_cached_per_backtrace() {
        with_limiter_capacity(0, || {
            clear_frame_cache_for_tests();
            let bt = Backtrace::capture().expect("capture");
            // First call: budget=0 + cache empty -> None.
            assert!(bt.rendered().is_none());
            // Open the limiter wide so a subsequent render *would* succeed
            // if `None` were not cached. With per-instance caching the
            // first outcome wins and we still see None.
            global_resolution_limiter().set_capacity(
                crate::error::backtrace::DEFAULT_BACKTRACE_RESOLUTIONS_PER_SECOND_WHEN_ENABLED,
            );
            global_resolution_limiter().reset_for_tests();
            assert!(
                bt.rendered().is_none(),
                "rendered() must be deterministic per-Backtrace; None must stay None"
            );
        });
    }

    #[test]
    fn frame_cache_evicts_when_soft_cap_reached() {
        // Validates the soft-cap eviction path on `try_resolve_frames`:
        // when the cache size *before* an insert reaches the soft cap, the
        // existing map is swapped out (its drop is offloaded to a detached
        // OS thread) and only the new entries from the triggering call
        // survive. We deliberately set the cap low so the path fires
        // without filling 100k entries.
        //
        // Use synthetic low-address IPs that nothing else in the process
        // will ever insert, and assert per-IP membership instead of
        // absolute cache size — concurrent tests rendering real
        // backtraces in parallel may push other entries into the cache,
        // and an absolute-size assertion would be racy.
        with_limiter_capacity(100, || {
            clear_frame_cache_for_tests();
            let prev_cap = set_frame_cache_soft_cap_for_tests(10);

            // Use synthetic IPs that the platform symbol resolver almost
            // certainly cannot resolve (low addresses). `resolve_single`
            // tolerates an unresolved IP and still inserts a stub frame
            // into the cache.
            let first: Vec<usize> = (1..=12).collect();
            assert!(
                try_resolve_frames(&first).is_some(),
                "first resolve_frames call must succeed (budget acquired once)"
            );
            for ip in &first {
                assert!(
                    frame_cache_contains_for_tests(*ip),
                    "expected IP {ip} in cache before eviction trips"
                );
            }

            // Second call: cache len (>= 12) >= cap (10) before insert,
            // so the existing entries are swapped out and only the 3 new
            // ones land in the fresh map. The OLD 12 must be gone; the
            // NEW 3 must be present.
            let second: Vec<usize> = (13..=15).collect();
            assert!(try_resolve_frames(&second).is_some());
            for ip in &first {
                assert!(
                    !frame_cache_contains_for_tests(*ip),
                    "pre-eviction IP {ip} must be gone after swap"
                );
            }
            for ip in &second {
                assert!(
                    frame_cache_contains_for_tests(*ip),
                    "post-eviction IP {ip} must be present in fresh cache"
                );
            }

            // Restore the production cap so this test does not affect
            // others sharing the process-global static.
            set_frame_cache_soft_cap_for_tests(prev_cap);
        });
    }

    #[test]
    fn capacity_zero_disables_capture() {
        // Explicit `0` is the universal "off switch" and must fully
        // disable capture: `Backtrace::capture` returns `None` before
        // walking the stack or allocating the IP vector. Exercising the
        // production `set_capacity` path (no test-only escape hatch).
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev = global_capture_throttle().capacity();
        global_capture_throttle().set_capacity(0);
        global_capture_throttle().reset_for_tests();
        assert!(
            Backtrace::capture().is_none(),
            "capacity=0 must disable capture entirely"
        );
        global_capture_throttle().set_capacity(prev);
        global_capture_throttle().reset_for_tests();
    }

    #[test]
    fn capacity_nonzero_enables_capture() {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev = global_capture_throttle().capacity();
        global_capture_throttle().set_capacity(8);
        global_capture_throttle().reset_for_tests();
        assert!(
            Backtrace::capture().is_some(),
            "capacity>0 must allow capture within the fresh window"
        );
        global_capture_throttle().set_capacity(prev);
        global_capture_throttle().reset_for_tests();
    }

    #[test]
    fn rust_backtrace_enabled_is_stable() {
        // The helper caches its decision in a `OnceLock<bool>`; repeated
        // reads must return the same value regardless of mid-process
        // environment mutation, matching stdlib semantics.
        let first = rust_backtrace_enabled();
        // Flip the env var; the cached value should not change.
        let prev = std::env::var("RUST_BACKTRACE").ok();
        // SAFETY: mutating the process environment in tests is racy with
        // any test that reads other env vars in parallel, but this test
        // only inspects the cached `rust_backtrace_enabled()` decision —
        // it does not observe the live env var. We restore it before
        // returning.
        unsafe {
            std::env::set_var("RUST_BACKTRACE", if first { "0" } else { "1" });
        }
        assert_eq!(
            rust_backtrace_enabled(),
            first,
            "rust_backtrace_enabled must be cached (OnceLock) and ignore later env mutations"
        );
        unsafe {
            match prev {
                Some(v) => std::env::set_var("RUST_BACKTRACE", v),
                None => std::env::remove_var("RUST_BACKTRACE"),
            }
        }
    }
}
