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
//! without surprising callers who expect idiomatic Rust behavior.
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
        atomic::{AtomicBool, AtomicU32, AtomicU64, AtomicUsize, Ordering},
        Arc, OnceLock, RwLock,
    },
    time::Instant,
};

// =================================================================
// Public configuration API
// =================================================================

/// Process-wide backtrace tuning knobs. Programmatic counterpart to the
/// `AZURE_COSMOS_BACKTRACE_*` environment variables, applied via
/// [`set_backtrace_options`].
///
/// Both fields are per-second caps on a rolling 1-second window:
///
/// * `max_captures_per_second` bounds stack-walk + IP-vector allocation
///   work. `0` disables capture entirely — `Backtrace::capture` returns
///   `None` before allocating.
/// * `max_resolutions_per_second` bounds *fresh* symbol-resolution work.
///   Cache hits do not consume budget; only render attempts that hit at
///   least one unseen instruction pointer charge it. `0` disables fresh
///   resolution — already-captured backtraces still render to
///   `<unresolved> @ 0xIP` placeholders for cache-missed frames.
///
/// Construct via [`BacktraceOptions::default`], which consults the
/// stdlib `RUST_LIB_BACKTRACE` / `RUST_BACKTRACE` environment variables
/// to pick between fully-off (both fields `0`) and the safe per-second
/// defaults (`1_000` captures, `5` resolutions). Then mutate the
/// individual fields as needed before passing to
/// [`set_backtrace_options`]. The struct is `#[non_exhaustive]` to
/// reserve room for future knobs without breaking external construction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct BacktraceOptions {
    /// Per-second cap on stack-walk captures. `0` disables capture.
    pub max_captures_per_second: u32,
    /// Per-second cap on fresh symbol resolution. `0` disables resolution.
    pub max_resolutions_per_second: u32,
}

impl BacktraceOptions {
    /// Safe default capture cap applied when `RUST_LIB_BACKTRACE` /
    /// `RUST_BACKTRACE` enables backtraces.
    const SAFE_CAPTURES_PER_SECOND: u32 = 1_000;
    /// Safe default fresh-resolution cap applied when `RUST_LIB_BACKTRACE`
    /// / `RUST_BACKTRACE` enables backtraces.
    const SAFE_RESOLUTIONS_PER_SECOND: u32 = 5;
}

impl Default for BacktraceOptions {
    /// Returns the env-derived default options.
    ///
    /// Consults the stdlib `RUST_LIB_BACKTRACE` (library-scoped) and
    /// `RUST_BACKTRACE` (process-wide) environment variables, matching
    /// stdlib precedence (library-scoped wins). When either asks for
    /// backtraces, returns the safe per-second defaults (`1_000`
    /// captures, `5` fresh resolutions); otherwise returns both fields
    /// set to `0` (fully disabled).
    fn default() -> Self {
        if rust_backtrace_enabled() {
            Self {
                max_captures_per_second: Self::SAFE_CAPTURES_PER_SECOND,
                max_resolutions_per_second: Self::SAFE_RESOLUTIONS_PER_SECOND,
            }
        } else {
            Self {
                max_captures_per_second: 0,
                max_resolutions_per_second: 0,
            }
        }
    }
}

/// Sets the process-wide backtrace options programmatically, **overriding**
/// the `AZURE_COSMOS_BACKTRACE_*` environment variables and the
/// `RUST_BACKTRACE` / `RUST_LIB_BACKTRACE`-keyed default.
///
/// In particular this overrides **both directions**:
///
/// * If `RUST_LIB_BACKTRACE` / `RUST_BACKTRACE` is set to `0` (off) and
///   the operator wants backtraces on, supply non-zero capacities — the
///   programmatic call wins.
/// * If the env vars ask for backtraces but the operator wants them off
///   in production, call with both fields `0` — the programmatic call
///   still wins.
///
/// Backtrace tuning is process-scoped (the underlying limiters are
/// process-global atomics — see the module docs for why per-runtime state
/// isn't viable on the error-construction path). Repeated programmatic
/// calls follow last-writer-wins semantics: the most recent call's
/// options become the active configuration.
///
/// After this function returns, the env-var-derived lazy init is
/// **permanently suppressed** — any in-flight or future
/// `ensure_initialized()` call observes `PROGRAMMATIC_OVERRIDE = true`
/// and refuses to apply env defaults that would clobber the operator's
/// setting. This closes the race where a concurrent first
/// `Backtrace::capture` could otherwise have overwritten the
/// just-applied programmatic capacities with `0` (env-default when
/// `RUST_BACKTRACE` is unset).
///
/// Typical use is once at process / runtime startup. Concurrent
/// programmatic calls race in the standard last-writer-wins way.
pub fn set_backtrace_options(options: BacktraceOptions) {
    // Mark first to block any concurrent `ensure_initialized()` from
    // overwriting our about-to-be-applied capacities with env defaults.
    // `Release` pairs with the `Acquire` load in `ensure_initialized`.
    PROGRAMMATIC_OVERRIDE.store(true, Ordering::Release);
    global_capture_throttle().set_capacity(options.max_captures_per_second);
    global_resolution_limiter().set_capacity(options.max_resolutions_per_second);
}

/// Idempotent lazy initializer that applies the env-var-derived defaults
/// the first time backtrace machinery is exercised, **unless** a
/// programmatic call to [`set_backtrace_options`] has already run (or
/// races with this one). Cheap fast-path: a relaxed-load of a `OnceLock`
/// after the first call.
///
/// Implementation note: env-derived init runs at most once per process
/// via [`OnceLock`], and the init closure first checks
/// `PROGRAMMATIC_OVERRIDE` so a programmatic call that races with a
/// first `Backtrace::capture` cannot be clobbered. The previous
/// `AtomicBool`-gated implementation had a window where a thread that
/// observed `INITIALIZED == false`, computed env defaults, and was then
/// preempted could overwrite a concurrently-applied programmatic
/// setting with `0` (env default when `RUST_BACKTRACE` is unset). See
/// finding #4 in the review thread for the timeline.
pub(crate) fn ensure_initialized() {
    ENV_INIT_DONE.get_or_init(|| {
        // If a programmatic override has already been applied (or is
        // being applied concurrently and won the `Release` store
        // sequencing against our `Acquire` load), do NOT touch the
        // capacities — the operator's setting is authoritative.
        if !PROGRAMMATIC_OVERRIDE.load(Ordering::Acquire) {
            let options = resolve_from_env();
            global_capture_throttle().set_capacity(options.max_captures_per_second);
            global_resolution_limiter().set_capacity(options.max_resolutions_per_second);
        }
    });
}

fn resolve_from_env() -> BacktraceOptions {
    // Start from the `RUST_LIB_BACKTRACE` / `RUST_BACKTRACE`-keyed
    // default, then let the Cosmos-specific env vars override either
    // knob individually.
    let defaults = BacktraceOptions::default();
    BacktraceOptions {
        max_captures_per_second: env_u32(
            "AZURE_COSMOS_BACKTRACE_CAPTURES_PER_SECOND",
            defaults.max_captures_per_second,
        ),
        max_resolutions_per_second: env_u32(
            "AZURE_COSMOS_BACKTRACE_RESOLUTIONS_PER_SECOND",
            defaults.max_resolutions_per_second,
        ),
    }
}

fn env_u32(name: &str, default: u32) -> u32 {
    // Thin wrapper: the parsing/precedence logic lives in `parse_env_u32`
    // so unit tests can exercise it without touching real env vars
    // (`std::env::set_var` is not safe in a multi-threaded test
    // harness on non-Windows platforms).
    parse_env_u32(std::env::var(name).ok().as_deref(), default)
}

/// Pure parsing helper: returns `default` when `raw` is `None`, the raw
/// string fails to parse as a `u32`, or contains only whitespace.
/// Returns the parsed value otherwise (including `0`, which is a valid
/// explicit "disable" override).
fn parse_env_u32(raw: Option<&str>, default: u32) -> u32 {
    raw.and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(default)
}

/// Set to `true` (with `Release` ordering) by [`set_backtrace_options`]
/// before it writes any capacity. [`ensure_initialized`] checks this with
/// `Acquire` ordering inside its `OnceLock` init closure and skips the
/// env-derived capacity writes when set — preventing a concurrent first
/// capture from overwriting a just-applied programmatic configuration
/// with env defaults.
static PROGRAMMATIC_OVERRIDE: AtomicBool = AtomicBool::new(false);

/// Runs the env-derived init at most once per process. Hit on every
/// `Backtrace::capture` / `Backtrace::rendered` call as the fast-path
/// gate; after the first init the closure is never re-executed and
/// `get_or_init` reduces to a relaxed load.
static ENV_INIT_DONE: OnceLock<()> = OnceLock::new();

/// Returns `true` when the stdlib backtrace environment variables ask
/// for library-generated backtraces, matching stdlib precedence:
/// [`RUST_LIB_BACKTRACE`] takes priority over [`RUST_BACKTRACE`] (it's
/// the library-scoped knob — `RUST_BACKTRACE` also controls panic-handler
/// backtraces, so an operator may want library backtraces off while
/// still keeping panic stacks). For each variable, anything other than
/// unset / empty / `"0"` enables.
///
/// Read **once** per process via [`OnceLock`] (matching stdlib);
/// mid-process mutations of either environment variable have no
/// effect.
///
/// [`RUST_LIB_BACKTRACE`]: https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
/// [`RUST_BACKTRACE`]: https://doc.rust-lang.org/std/backtrace/index.html#environment-variables
pub(crate) fn rust_backtrace_enabled() -> bool {
    static ENABLED: OnceLock<bool> = OnceLock::new();
    *ENABLED.get_or_init(|| {
        // Mirror std's resolution order (library/std/src/backtrace.rs):
        // RUST_LIB_BACKTRACE wins if set; otherwise fall back to
        // RUST_BACKTRACE; otherwise off.
        fn var_is_on(name: &str) -> Option<bool> {
            match std::env::var(name) {
                Ok(value) => Some(!value.is_empty() && value != "0"),
                Err(_) => None,
            }
        }
        var_is_on("RUST_LIB_BACKTRACE")
            .or_else(|| var_is_on("RUST_BACKTRACE"))
            .unwrap_or(false)
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
        // Lazy env-var read on first capture (no-op once any prior
        // capture or programmatic `set_backtrace_options` ran).
        ensure_initialized();
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
    // Defensive: a `Backtrace` value may have been captured under a prior
    // (programmatic) configuration but rendered before any env-var read
    // happened. Idempotent on the hot path.
    ensure_initialized();
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
        // Bound the cache to keep long-lived hosts that load/unload
        // modules (JNI / P/Invoke / dlopen) from accumulating frames
        // indefinitely. Swap the full map out for a fresh empty one and
        // hand the old map to a separate binding so its Drop — atomic
        // refcount decrements on every `Arc<ResolvedFrame>` plus String
        // frees — runs *off* the calling thread (see below). Keeps the
        // critical section `O(1)` even at the cap.
        //
        // Scope the write guard explicitly so it drops at the end of the
        // block (before we spawn the eviction-drop thread).
        let evicted = {
            let mut cache = frame_cache().write().unwrap();
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
            evicted
        };
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
/// [`BacktraceCaptureLimiter`] is robust against clock skew.
fn now_monotonic_secs() -> u64 {
    static ANCHOR: OnceLock<Instant> = OnceLock::new();
    let anchor = ANCHOR.get_or_init(Instant::now);
    Instant::now().saturating_duration_since(*anchor).as_secs()
}

/// Returns a reference to the process-global symbol-resolution limiter.
///
/// The runtime builder uses this to apply caller-supplied configuration; most
/// other callers should not need direct access.
pub(crate) fn global_resolution_limiter() -> &'static BacktraceCaptureLimiter {
    static LIMITER: BacktraceCaptureLimiter = BacktraceCaptureLimiter::new_disabled();
    &LIMITER
}

/// Returns a reference to the process-global per-second cap on stack
/// captures (a second, independent limiter from the resolution one).
///
/// Each successful `Backtrace::capture` consumes one token; when the
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
pub(crate) mod tests {
    use super::*;
    use std::sync::Mutex;

    /// Returns a pointer-identity handle (as `usize`) to the inner Arc,
    /// for tests that need to assert two `Backtrace` values refer to the
    /// same captured stack (e.g. backtrace-inheritance from a wrapped
    /// source). Lives here rather than as an inherent `Backtrace` method
    /// so the production type stays free of test-only surface; child
    /// modules can still see the private `inner` field through `super`.
    pub(crate) fn backtrace_inner_arc_identity(bt: &Backtrace) -> usize {
        Arc::as_ptr(&bt.inner) as usize
    }

    // Serializes backtrace tests across the whole crate that mutate the
    // per-second limiter or capture throttle (process-global). Made
    // `pub(crate)` so the sibling test module in `error::mod` (which
    // also touches `global_capture_throttle()` /
    // `global_resolution_limiter()`) serializes against these tests via
    // the **same** mutex. Two separate locks would not serialize and
    // would race.
    pub(crate) static TEST_LOCK: Mutex<()> = Mutex::new(());

    fn with_limiter_capacity<R>(capacity: u32, f: impl FnOnce() -> R) -> R {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Prime the OnceLock-gated env-derived init BEFORE we set capacities,
        // so the first `Backtrace::capture()` inside `f` doesn't fire the
        // lazy init and clobber our test-set values back to env defaults
        // (which are 0 when RUST_BACKTRACE is unset).
        ensure_initialized();
        let prev = global_resolution_limiter().capacity();
        global_resolution_limiter().set_capacity(capacity);
        global_resolution_limiter().reset_for_tests();
        // Ensure the capture throttle starts with a fresh window and a
        // generous capacity so it never accidentally gates these tests —
        // we are exercising the resolution limiter, not capture throttling.
        let prev_throttle = global_capture_throttle().capacity();
        global_capture_throttle().set_capacity(10_000);
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
            // Exhaust the limiter completely, then resolve the same captured
            // IPs again. Every frame should be a cache hit, so no fresh budget
            // is required. Do not assert on rendered text: some platforms can
            // legitimately symbolize Rust frames while leaving native tail
            // frames as `<unknown> @ ...`.
            global_resolution_limiter().set_capacity(0);
            global_resolution_limiter().reset_for_tests();
            let frames = try_resolve_frames(&bt1.inner.ips)
                .expect("cached frames must resolve even with zero fresh-resolution budget");
            assert_eq!(frames.len(), bt1.inner.ips.len());
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
            global_resolution_limiter().set_capacity(1_000);
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
        // Prime the OnceLock-gated env init so it can't clobber our
        // capacity below.
        ensure_initialized();
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
        ensure_initialized();
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

    /// End-to-end: the public `set_backtrace_options` API writes both
    /// limiter capacities and the next `Backtrace::capture` observes the
    /// applied values. This is the lowest-level "the public API actually
    /// works" guarantee.
    #[test]
    fn set_backtrace_options_writes_both_limiter_capacities() {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev_cap = global_capture_throttle().capacity();
        let prev_res = global_resolution_limiter().capacity();

        set_backtrace_options(BacktraceOptions {
            max_captures_per_second: 42,
            max_resolutions_per_second: 7,
        });
        assert_eq!(global_capture_throttle().capacity(), 42);
        assert_eq!(global_resolution_limiter().capacity(), 7);

        // Restore so this test does not leak state into sibling tests.
        global_capture_throttle().set_capacity(prev_cap);
        global_resolution_limiter().set_capacity(prev_res);
        global_capture_throttle().reset_for_tests();
        global_resolution_limiter().reset_for_tests();
    }

    /// Pin the override-after-disabled property: even when the
    /// limiters are at capacity `0` (the "disabled" state that
    /// `RUST_LIB_BACKTRACE=0` / `RUST_BACKTRACE=0` produces via
    /// `BacktraceOptions::default()`), a subsequent
    /// `set_backtrace_options` call with non-zero values raises the cap
    /// and capture starts working again. This is the property that
    /// matters for "set_backtrace_options trumps env-var-disabled".
    #[test]
    fn set_backtrace_options_overrides_disabled_baseline() {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev_cap = global_capture_throttle().capacity();
        let prev_res = global_resolution_limiter().capacity();

        // Disabled baseline — matches what `BacktraceOptions::default()`
        // produces when `rust_backtrace_enabled()` is `false`.
        set_backtrace_options(BacktraceOptions {
            max_captures_per_second: 0,
            max_resolutions_per_second: 0,
        });
        global_capture_throttle().reset_for_tests();
        assert!(
            Backtrace::capture().is_none(),
            "with both caps at 0 capture must be disabled"
        );

        // Programmatic override flips it back on regardless of prior state.
        set_backtrace_options(BacktraceOptions {
            max_captures_per_second: 100,
            max_resolutions_per_second: 0,
        });
        global_capture_throttle().reset_for_tests();
        assert!(
            Backtrace::capture().is_some(),
            "programmatic override of a disabled baseline must re-enable capture"
        );

        global_capture_throttle().set_capacity(prev_cap);
        global_resolution_limiter().set_capacity(prev_res);
        global_capture_throttle().reset_for_tests();
        global_resolution_limiter().reset_for_tests();
    }

    /// Companion of the above: programmatic override **off** wins even
    /// when the limiters were previously enabled (covers the "operator
    /// wants backtraces off in production despite `RUST_BACKTRACE`
    /// asking for them" case). Last-writer-wins semantics also implicitly
    /// covered by this pair.
    #[test]
    fn set_backtrace_options_overrides_enabled_baseline() {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        let prev_cap = global_capture_throttle().capacity();
        let prev_res = global_resolution_limiter().capacity();

        set_backtrace_options(BacktraceOptions {
            max_captures_per_second: 1_000,
            max_resolutions_per_second: 5,
        });
        global_capture_throttle().reset_for_tests();
        assert!(Backtrace::capture().is_some());

        // Programmatic "off" override.
        set_backtrace_options(BacktraceOptions {
            max_captures_per_second: 0,
            max_resolutions_per_second: 0,
        });
        global_capture_throttle().reset_for_tests();
        assert!(
            Backtrace::capture().is_none(),
            "programmatic override to 0 must disable capture regardless of prior state"
        );

        global_capture_throttle().set_capacity(prev_cap);
        global_resolution_limiter().set_capacity(prev_res);
        global_capture_throttle().reset_for_tests();
        global_resolution_limiter().reset_for_tests();
    }

    /// Pins the env-var parsing precedence via the pure
    /// [`parse_env_u32`] helper. Exercises the helper directly rather
    /// than mutating real env vars — `std::env::set_var` /
    /// `std::env::remove_var` are not safe in a multi-threaded test
    /// harness on non-Windows platforms, and the production code path
    /// (`env_u32`) is a thin wrapper that only delegates `std::env::var`
    /// + this helper.
    #[test]
    fn parse_env_u32_precedence() {
        // Missing -> default wins.
        assert_eq!(parse_env_u32(None, 99), 99);

        // Valid integer -> override wins.
        assert_eq!(parse_env_u32(Some("7"), 99), 7);

        // Surrounding whitespace is tolerated (operator config noise).
        assert_eq!(parse_env_u32(Some("  7  "), 99), 7);

        // Malformed value -> default wins (best-effort robustness; a
        // typo in operator config doesn't accidentally enable capture).
        assert_eq!(parse_env_u32(Some("not-a-number"), 99), 99);

        // Empty string -> default wins.
        assert_eq!(parse_env_u32(Some(""), 99), 99);

        // Zero is a valid override (operator explicitly disables) and
        // beats the non-zero default.
        assert_eq!(parse_env_u32(Some("0"), 99), 0);
    }

    /// Regression guard for the `set_backtrace_options` ↔
    /// `ensure_initialized` race (review finding #4).
    ///
    /// Operator timeline that must succeed:
    ///
    /// 1. `set_backtrace_options({captures: 12345, resolutions: 67})`
    ///    runs (typically at startup).
    /// 2. Some thread later calls `Backtrace::capture` for the first
    ///    time, which triggers `ensure_initialized`.
    /// 3. The operator's capacities must **survive** the lazy env-init
    ///    \u2014 a previous implementation would clobber them with
    ///    `(0, 0)` if `RUST_BACKTRACE` was unset.
    ///
    /// We can't fully exercise the *concurrent* race deterministically
    /// in a single-process unit test, but we can prove the contract:
    /// once `set_backtrace_options` has run, a subsequent
    /// `ensure_initialized` is a structural no-op for the capacities.
    /// Combined with the `PROGRAMMATIC_OVERRIDE` flag's
    /// `Release`-before-write / `Acquire`-before-check ordering, this
    /// proves the concurrent case too: any `ensure_initialized` that
    /// happens-after `set_backtrace_options`'s `Release` store sees
    /// the override and refuses to write.
    #[test]
    fn set_backtrace_options_wins_against_subsequent_ensure_initialized() {
        let _guard = TEST_LOCK.lock().unwrap_or_else(|e| e.into_inner());

        // Snapshot existing state so we don't leak into sibling tests.
        let throttle = global_capture_throttle();
        let resolution = global_resolution_limiter();
        let prev_cap = throttle.capacity();
        let prev_res = resolution.capacity();
        let prev_override = PROGRAMMATIC_OVERRIDE.swap(false, Ordering::AcqRel);

        // Apply operator configuration via the public API.
        set_backtrace_options(BacktraceOptions {
            max_captures_per_second: 12_345,
            max_resolutions_per_second: 67,
        });
        assert_eq!(throttle.capacity(), 12_345);
        assert_eq!(resolution.capacity(), 67);

        // Now drive `ensure_initialized` — even though `ENV_INIT_DONE`
        // may not yet have been populated by another test in this run,
        // the `PROGRAMMATIC_OVERRIDE` guard must keep the env-derived
        // init from clobbering the operator's values.
        ensure_initialized();
        assert_eq!(
            throttle.capacity(),
            12_345,
            "ensure_initialized() must not clobber a prior set_backtrace_options() capture capacity",
        );
        assert_eq!(
            resolution.capacity(),
            67,
            "ensure_initialized() must not clobber a prior set_backtrace_options() resolution capacity",
        );

        // Restore.
        throttle.set_capacity(prev_cap);
        resolution.set_capacity(prev_res);
        PROGRAMMATIC_OVERRIDE.store(prev_override, Ordering::Release);
    }
}
