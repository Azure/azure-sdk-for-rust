# Logging Interface Specification

This document specifies the logging subsystem for `azure_data_cosmos_driver`.
Implementation is divided into three sequential PRs. Each PR section below
defines the technical design, new/modified files, public API surface, acceptance
criteria, and testing strategy.

## Table of Contents

1. [Overview](#overview)
2. [Design](#design)
   - [Log Level](#log-level)
   - [Log Entry](#log-entry)
   - [LogSink Trait](#logsink-trait)
   - [Built-in Sinks](#built-in-sinks)
   - [LoggingOptions](#loggingoptions)
   - [LoggingRuntime](#loggingruntime)
   - [`driver_log!` Macro](#driver_log-macro)
   - [Sampling Diagnostics Evaluator](#sampling-diagnostics-evaluator)
   - [Configuration Layering](#configuration-layering)
   - [Thread Safety & Performance](#thread-safety--performance)
   - [FFI Considerations](#ffi-considerations)
3. [Implementation Plan](#implementation-plan)
   - [PR 1 – Log Sink Callback Interface & Runtime Wiring](#pr-1--log-sink-callback-interface--runtime-wiring)
   - [PR 2 – Internal Driver Instrumentation](#pr-2--internal-driver-instrumentation)
   - [PR 3 – Sampling Diagnostics Evaluator](#pr-3--sampling-diagnostics-evaluator)
4. [Open Questions](#open-questions)
5. [Related Documents](#related-documents)

---

## Overview

The driver needs three distinct logging capabilities:

| Capability                   | Purpose                                                                   | PR  |
| ---------------------------- | ------------------------------------------------------------------------- | --- |
| **Log Sink**                 | Callback interface for consumers to receive driver-internal logs          | 1   |
| **Internal Instrumentation** | Structured log points inside the driver using `driver_log!`               | 2   |
| **Sampling Diagnostics**     | Rate-limited operation-diagnostic entries (errors + threshold violations) | 3   |

### Design Principles

- **Lazy formatting**: The `driver_log!` macro checks the enabled level *before* calling `format!()`. No formatting cost when the level is disabled.
- **Pre-formatted at sink boundary**: `LogSink::emit` receives `&[LogEntry]` with pre-formatted `String` messages. This simplifies FFI consumers that must copy data across language boundaries.
- **Batched delivery**: A `LogBuffer` accumulates entries and flushes them in batches (configurable size, configurable flush interval) to reduce per-entry overhead. Auto-flush on drop is enabled by default.
- **DOP alignment**: Data types are plain structs/enums; behavior lives in `LoggingRuntime` and the `LogSink` trait.

---

## Design

### Log Level

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Error = 0,
    Warn  = 1,
    Info  = 2,
    Debug = 3,
    Trace = 4,
}
```

Ordering: `Error < Warn < Info < Debug < Trace`. A sink configured at `Info` receives `Error`, `Warn`, and `Info` entries.

`LogLevel` must also implement `Display`, `FromStr`, and `Default` (defaulting to `Info`).

### Log Entry

```rust
pub struct LogEntry {
    pub level: LogLevel,
    pub timestamp: SystemTime,
    pub logger: &'static str,
    pub message: String,
    pub module: &'static str,
    pub file: &'static str,
    pub line: u32,
}
```

`LogEntry` must implement `Clone`, `Debug`. A `new()` constructor captures `SystemTime::now()` automatically.

The `logger` field identifies **which logical logger** produced the entry — see
[`driver_log!` Macro](#driver_log-macro) for how it is derived. Logger names are short,
stable identifiers like `"cosmos_driver"`, `"runtime"`, `"cache"`, `"transport"` that
consumers can use for filtering or routing in their sink implementation.

### LogSink Trait

```rust
pub trait LogSink: Send + Sync + 'static {
    /// Maximum level this sink will accept. The runtime skips entries above this level.
    fn max_level(&self) -> LogLevel;

    /// Deliver a batch of entries. Called from the buffer flush path.
    fn emit(&self, entries: &[LogEntry]);

    /// Flush any internal buffers. Called on runtime shutdown and on `LoggingRuntime::flush()`.
    fn flush(&self) {}
}
```

Because `dyn LogSink` doesn't implement `Debug`, any struct containing `Arc<dyn LogSink>` must
use a manual `Debug` impl (e.g., print `"LogSink(max_level=Info)"`).

### Built-in Sinks

| Sink             | Crate                      | Description                                               |
| ---------------- | -------------------------- | --------------------------------------------------------- |
| `TracingLogSink` | `azure_data_cosmos_driver` | Default sink. Forwards entries to the `tracing` crate.    |
| `FfiLogSink`     | `azure_data_cosmos_native` | C-FFI sink for cross-language hosts (Java, .NET, Python). |

#### `TracingLogSink`

The default sink used when no custom sink is provided via `LoggingOptions`.

- **Target**: All events use `target: "azure_data_cosmos_driver"` so that `tracing` subscribers
  can filter driver output independently from application logs.
- **Level mapping**: Each `LogLevel` maps 1:1 to a `tracing` macro:

  | `LogLevel` | `tracing` macro   |
  | ---------- | ----------------- |
  | `Error`    | `tracing::error!` |
  | `Warn`     | `tracing::warn!`  |
  | `Info`     | `tracing::info!`  |
  | `Debug`    | `tracing::debug!` |
  | `Trace`    | `tracing::trace!` |

- **Structured fields**: Each event carries `logger`, `module`, `file`, and `line` as structured
  fields in addition to the pre-formatted `message` string, so subscribers that support
  structured logging (e.g., JSON formatters) can extract them.
- **Derives / traits**: `Clone`, `Debug`, `Default` (defaults to `LogLevel::Info`).
- **No subscriber required**: If no `tracing` subscriber is installed, `emit()` is a no-op
  (the `tracing` crate silently discards events). This means the sink is always safe to use
  as a default — it adds zero overhead in applications that don't opt into `tracing`.
- **Thread safety**: `TracingLogSink` is stateless beyond the stored `max_level`; it is
  trivially `Send + Sync`.

#### `FfiLogSink`

Implemented in the `azure_data_cosmos_native` crate (not in `azure_data_cosmos_driver`).
Included here for completeness so the design handles its requirements.

- **Purpose**: Bridge log entries across the C-FFI boundary to host language runtimes
  (Java via JNI, .NET via P/Invoke, Python via ctypes/cffi).
- **Mechanism**: The host registers a function pointer (`extern "C" fn(entry: *const CosmosLogEntry)`)
  during native library initialization. `FfiLogSink::emit()` converts each `LogEntry` to a
  C-compatible `CosmosLogEntry` struct:

  ```c
  typedef struct {
      int32_t level;           // LogLevel discriminant (0=Error .. 4=Trace)
      int64_t timestamp_ms;    // milliseconds since Unix epoch
      const char* logger;      // UTF-8, null-terminated (e.g., "cosmos_driver")
      const char* message;     // UTF-8, null-terminated
      const char* module;      // UTF-8, null-terminated
      const char* file;        // UTF-8, null-terminated
      uint32_t line;
  } CosmosLogEntry;
  ```

- **Lifetime**: The `CosmosLogEntry` pointers are valid only for the duration of the callback
  invocation. The host must copy any data it needs to retain.
- **Pre-formatted design**: Because `message` is already a pre-formatted `String`, the FFI
  layer does **not** need to interpret Rust format arguments or handle `fmt::Arguments` across
  the boundary. This is a deliberate design choice.
- **Blocking constraint**: The function pointer is invoked synchronously on the flush path.
  The host callback **must not block** (no I/O, no locks held for extended periods), or it
  risks stalling the driver's buffer flush.
- **Registration API** (in `azure_data_cosmos_native`):

  ```rust
  // Conceptual — exact signature TBD in the native crate's own spec
  pub extern "C" fn cosmos_set_log_callback(
      callback: Option<extern "C" fn(*const CosmosLogEntry)>,
      max_level: i32,
  );
  ```

- **When not registered**: If no callback is registered, the native crate falls back to
  `TracingLogSink` (or a no-op sink), matching the driver-crate default behavior.

### LoggingOptions

```rust
#[non_exhaustive]
pub struct LoggingOptions {
    pub(crate) sink: Option<Arc<dyn LogSink>>,
    pub(crate) default_level: Option<LogLevel>,  // default: None (uses sink's max_level)
    pub(crate) level_overrides: Vec<(String, LogLevel)>, // per-logger overrides
    pub(crate) max_batch_size: usize,            // default: 64
    pub(crate) flush_interval: Duration,          // default: 100 ms
    pub(crate) auto_flush_on_drop: bool,          // default: true
}
```

Configurable via `LoggingOptionsBuilder` with `with_*` setters (following the same pattern as
`DiagnosticsOptionsBuilder`).

#### Per-Logger Level Overrides

The `default_level` sets the global log level for all loggers. Individual loggers can be
overridden to a different level via `level_overrides`:

```rust
let options = LoggingOptions::builder()
    .with_default_level(LogLevel::Info)       // most loggers at Info
    .with_level_override("cache", LogLevel::Trace)  // but caching at Trace
    .with_level_override("transport", LogLevel::Debug)
    .build();
```

The builder method `with_level_override(logger: &str, level: LogLevel)` appends to
`level_overrides`. At runtime initialization, overrides are compiled into a `HashMap<&'static str, LogLevel>`
inside `LoggingRuntime` for O(1) lookup.

#### Environment Variables

Builder reads defaults from environment where applicable:

| Environment Variable              | Overrides           | Example Values                       |
| --------------------------------- | ------------------- | ------------------------------------ |
| `AZURE_COSMOS_LOG_LEVEL`          | `default_level`     | `error`, `debug`                     |
| `AZURE_COSMOS_LOG_LEVEL_{LOGGER}` | Per-logger override | `AZURE_COSMOS_LOG_LEVEL_CACHE=trace` |
| `AZURE_COSMOS_LOG_BATCH_SIZE`     | `max_batch_size`    | `128`                                |
| `AZURE_COSMOS_LOG_FLUSH_MS`       | `flush_interval`    | `200`                                |

The `AZURE_COSMOS_LOG_LEVEL_{LOGGER}` pattern uses the **uppercased logger name** as the
suffix. For example, `AZURE_COSMOS_LOG_LEVEL_CACHE=trace` overrides the `"cache"` logger
to `Trace`. Programmatic overrides take priority over environment variables.

### LoggingRuntime

Owns the sink, buffer, and flush logic. Created once per `CosmosDriverRuntime`.

```rust
pub struct LoggingRuntime { /* fields below */ }
```

**Fields** (all private):

| Field                | Type                              | Description                                   |
| -------------------- | --------------------------------- | --------------------------------------------- |
| `sink`               | `Arc<dyn LogSink>`                | The active log sink                           |
| `buffer`             | `Mutex<Vec<LogEntry>>`            | Pending entries awaiting flush                |
| `max_batch_size`     | `usize`                           | Flush when buffer reaches this size           |
| `flush_interval`     | `Duration`                        | Flush when this time elapses since last flush |
| `last_flush`         | `Mutex<Instant>`                  | Timestamp of most recent flush                |
| `default_level`      | `LogLevel`                        | Global default level for all loggers          |
| `level_overrides`    | `HashMap<&'static str, LogLevel>` | Per-logger level overrides (compiled at init) |
| `auto_flush_on_drop` | `bool`                            | Whether `Drop` triggers a final flush         |

**Key methods**:

- `is_enabled(logger: &str, level: LogLevel) -> bool` – looks up `logger` in
  `level_overrides`; if found, compares against that level, otherwise compares against
  `default_level`. This is a `HashMap::get` (O(1) amortized) + integer comparison.
- `effective_level(logger: &str) -> LogLevel` – returns the override for `logger` if
  present, otherwise `default_level`. Useful for callers that want to inspect the level.
- `emit(entry: LogEntry)` – pushes to buffer; flushes if batch full or interval elapsed.
- `flush()` – drains buffer and calls `sink.emit()` + `sink.flush()`.

**Drop**: calls `flush()` when `auto_flush_on_drop` is `true`.

Manual `Debug` impl required (see `LogSink` note above).

### `driver_log!` Macro

The macro has two invocation forms — one with an explicit logger name and one that
derives the logger name automatically from the source file:

```rust
macro_rules! driver_log {
    // Form 1: explicit logger name
    ($logging:expr, $logger:expr, $level:expr, $($arg:tt)*) => {
        if $logging.is_enabled($logger, $level) {
            $logging.emit($crate::diagnostics::LogEntry::new(
                $level,
                $logger,
                format!($($arg)*),
                module_path!(),
                file!(),
                line!(),
            ));
        }
    };
    // Form 2: derive logger name from file!() stem
    ($logging:expr, $level:expr, $($arg:tt)*) => {
        driver_log!(
            $logging,
            $crate::diagnostics::file_stem(file!()),
            $level,
            $($arg)*
        )
    };
}
```

#### Logger Name Derivation

When the caller omits the logger name (Form 2), it is derived from `file!()` by stripping
the directory prefix and `.rs` extension at compile time via a `const fn`:

```rust
/// Extracts the file stem from a path at compile time.
///
/// `"src/driver/cosmos_driver.rs"` → `"cosmos_driver"`
pub(crate) const fn file_stem(path: &str) -> &str { /* ... */ }
```

This gives each source file a natural logger name that matches the module it represents:

| Source File                           | Logger Name                                    |
| ------------------------------------- | ---------------------------------------------- |
| `src/driver/cosmos_driver.rs`         | `"cosmos_driver"`                              |
| `src/driver/runtime.rs`               | `"runtime"`                                    |
| `src/driver/cache/container_cache.rs` | `"container_cache"`                            |
| `src/driver/transport/mod.rs`         | `"mod"` → override to `"transport"` via Form 1 |
| `src/system/cpu_memory.rs`            | `"cpu_memory"`                                 |
| `src/system/vm_metadata.rs`           | `"vm_metadata"`                                |

For `mod.rs` files (where the stem would be the unhelpful `"mod"`), callers should use
Form 1 with an explicit name:

```rust
driver_log!(self.runtime.logging(), "transport", LogLevel::Debug, "Pipeline created: ...");
```

#### Well-Known Logger Names

The following logger names are used throughout the driver and can be referenced in
`level_overrides` configuration:

| Logger Name                | Source Module                            | Typical Level |
| -------------------------- | ---------------------------------------- | ------------- |
| `"runtime"`                | `driver/runtime.rs`                      | `Info`        |
| `"cosmos_driver"`          | `driver/cosmos_driver.rs`                | `Debug`       |
| `"transport"`              | `driver/transport/`                      | `Debug`       |
| `"container_cache"`        | `driver/cache/container_cache.rs`        | `Trace`       |
| `"account_metadata_cache"` | `driver/cache/account_metadata_cache.rs` | `Trace`       |
| `"cpu_memory"`             | `system/cpu_memory.rs`                   | `Trace`       |
| `"vm_metadata"`            | `system/vm_metadata.rs`                  | `Info`        |
| `"sampling_evaluator"`     | `diagnostics/sampling_evaluator.rs`      | `Info`        |

These names are **stable** and form part of the configuration contract. Adding a new logger
name is not a breaking change; renaming or removing one is.

#### Visibility

- `pub(crate)` — the macro is for **internal driver use only**. External consumers never call
  it; they receive driver output through the `LogSink` callback.

#### Lazy Formatting Guarantee

The `if $logging.is_enabled($logger, $level)` guard is the critical performance feature.
When the effective level for the given logger is lower than `$level`, the entire body —
including `format!()`, `module_path!()`, `file!()`, and `line!()` — is skipped. This means:

- **Zero allocation** when the level is disabled (no `String` created).
- **Zero formatting cost** — the `format_args!` machinery inside `format!()` is never invoked.
- The guard is a `HashMap::get` + integer comparison — effectively O(1).

This is especially important for `Trace`-level calls in hot paths (cache lookups, CPU samples)
where production deployments typically run at `Info` or higher.

#### Source Location Capture

The macro uses three built-in Rust macros to capture the call site automatically:

| Built-in Macro   | Captured As        | Description                                                                           |
| ---------------- | ------------------ | ------------------------------------------------------------------------------------- |
| `module_path!()` | `LogEntry::module` | Fully qualified module path (e.g., `azure_data_cosmos_driver::driver::cosmos_driver`) |
| `file!()`        | `LogEntry::file`   | Source file path relative to the crate root (e.g., `src/driver/cosmos_driver.rs`)     |
| `line!()`        | `LogEntry::line`   | Line number of the `driver_log!` invocation                                           |

These are all `&'static str` / `u32` — no allocation, no runtime cost. Because they are
resolved at the **macro expansion site** (not inside `LoggingRuntime`), they correctly reflect
the caller's location.

#### Usage Pattern

The first argument is always a reference to the `LoggingRuntime`, obtained from
`self.runtime.logging()` (in `CosmosDriver` methods) or from the runtime directly:

```rust
// Form 2 (default): logger name derived from file stem → "cosmos_driver"
driver_log!(
    self.runtime.logging(),
    LogLevel::Debug,
    "execute_operation: op={:?}, activity_id={}",
    operation.operation_type(),
    activity_id,
);

// Form 1 (explicit): logger name for mod.rs files or custom names
driver_log!(
    self.runtime.logging(),
    "transport",
    LogLevel::Debug,
    "Pipeline created: type={:?}, endpoint={}",
    pipeline_type,
    endpoint,
);

// In CosmosDriverRuntimeBuilder::build():
driver_log!(
    logging_runtime,
    "runtime",
    LogLevel::Info,
    "CosmosDriverRuntime created (connection_pool={:?})",
    connection_pool,
);
```

#### Why a Macro (Not a Function)

A regular function cannot achieve the same semantics:

1. **`format!()`** would be evaluated at the call site before the function is entered,
   defeating lazy formatting. A closure-based API (`log(level, || format!(...))`) would
   work but is more verbose and less ergonomic.
2. **`module_path!()`, `file!()`, `line!()`** resolve to the location where they appear
   lexically. Inside a function, they would always report the function's own location, not
   the caller's. The `#[track_caller]` attribute helps for `line!()` but not for
   `module_path!()` or `file!()`.

#### Convention: No Direct `tracing` Calls

All internal logging must go through `driver_log!`. Direct calls to `tracing::info!`,
`tracing::debug!`, etc. are **not allowed** outside of `TracingLogSink::emit()`. This ensures:

- All log output is routed through the configurable `LogSink`.
- FFI consumers receive the same log entries as Rust consumers.
- Log level filtering is consistent (one `max_level` check, not two).

### Sampling Diagnostics Evaluator

#### Purpose

After each `execute_operation`, evaluate whether the completed operation's diagnostics should be
emitted to the log sink. Two categories trigger logging:

1. **Errors**: The operation completed with a non-success status (HTTP >= 400).
2. **Threshold violations**: Latency, request charge, or payload size exceeded configured
   `DiagnosticsThresholds`.

#### Reference Implementation

This is modeled after
[`CosmosSamplingDiagnosticsLogger`](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos-spark_3-4_2-12/src/main/scala/com/azure/cosmos/spark/CosmosSamplingDiagnosticsLogger.scala)
from the Spark connector. Key patterns borrowed:

- **Errors are always logged** (never rate-limited).
- **Threshold violations are rate-limited** via counter reset on interval expiry.
- **Log verbosity is configurable separately** for error vs. threshold entries.

#### Hierarchical Token-Bucket Sampler

To avoid flooding logs in hot loops, a **hierarchical token-bucket sampler** rate-limits
threshold-violation entries:

```text
┌─────────────────────────────────────────────────────────┐
│ Window: 10 min  – hard cap (e.g., 60 entries)           │
│  ┌─────────────────────────────────────────────────────┐│
│  │ Window: 1 min – soft cap (e.g., 10 entries)         ││
│  │  ┌─────────────────────────────────────────────────┐││
│  │  │ Window: 10 sec – burst cap (e.g., 3 entries)    │││
│  │  └─────────────────────────────────────────────────┘││
│  └─────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────┘
```

Longer windows are hard caps over shorter windows. If the 10-minute budget is exhausted, no
entries are emitted even if the 10-second bucket has tokens.

Each bucket uses an `AtomicU32` counter and an `AtomicU64` for the next-reset timestamp
(milliseconds since epoch). The `try_acquire()` method atomically increments the counter and
checks against the cap, resetting when the window expires. This is lock-free.

#### SamplingDiagnosticsOptions

```rust
#[non_exhaustive]
pub struct SamplingDiagnosticsOptions {
    pub(crate) enabled: bool,                       // default: true
    pub(crate) error_verbosity: DiagnosticsVerbosity,     // default: Summary
    pub(crate) threshold_verbosity: DiagnosticsVerbosity, // default: Summary
    pub(crate) burst_window: Duration,              // default: 10 s
    pub(crate) burst_cap: u32,                      // default: 3
    pub(crate) medium_window: Duration,             // default: 1 min
    pub(crate) medium_cap: u32,                     // default: 10
    pub(crate) long_window: Duration,               // default: 10 min
    pub(crate) long_cap: u32,                       // default: 60
}
```

Configurable via `SamplingDiagnosticsOptionsBuilder` with standard `with_*` setters and env var
support:

| Environment Variable                             | Overrides             | Example    |
| ------------------------------------------------ | --------------------- | ---------- |
| `AZURE_COSMOS_SAMPLING_DIAG_ENABLED`             | `enabled`             | `false`    |
| `AZURE_COSMOS_SAMPLING_DIAG_ERROR_VERBOSITY`     | `error_verbosity`     | `detailed` |
| `AZURE_COSMOS_SAMPLING_DIAG_THRESHOLD_VERBOSITY` | `threshold_verbosity` | `summary`  |
| `AZURE_COSMOS_SAMPLING_DIAG_BURST_CAP`           | `burst_cap`           | `5`        |

(Window durations are not expected to be env-configurable; only caps.)

#### SamplingDiagnosticsEvaluator

A struct that lives inside `LoggingRuntime` (or alongside it in `CosmosDriverRuntime`). It
holds the `SamplingDiagnosticsOptions`, the three `TokenBucket` instances, and a reference to
`DiagnosticsThresholds`.

```rust
pub(crate) struct SamplingDiagnosticsEvaluator {
    options: SamplingDiagnosticsOptions,
    thresholds: DiagnosticsThresholds,
    burst_bucket: TokenBucket,
    medium_bucket: TokenBucket,
    long_bucket: TokenBucket,
}
```

**Key methods**:

- `evaluate(&self, diagnostics: &DiagnosticsContext, sink: &dyn LogSink)` — called after each
  `execute_operation`. Checks completion, error/threshold status; emits to sink if allowed.
- `is_threshold_violated(&self, diagnostics: &DiagnosticsContext) -> bool` — compares the
  operation's latency/charge/payload against `DiagnosticsThresholds` fields.

#### Evaluation Point

Inside `CosmosDriver::execute_operation`, just before returning the `CosmosResponse`
(around line ~630 in the current codebase):

```rust
// Current code (simplified):
let diagnostics = std::sync::Arc::new(diagnostics_builder.complete());
// NEW: evaluate for sampling diagnostics
self.runtime.sampling_evaluator().evaluate(&diagnostics, /* effective options */);
return Ok(CosmosResponse::new(body, headers, status, diagnostics));
```

Also in the `Err` branch (around line ~660), a failed operation with a transport error
can be evaluated too (since errors are always logged).

#### Logging Format

When the evaluator decides to log an entry, it emits via the `LogSink` at the appropriate level:

- **Error entries**: `LogLevel::Error`, message includes account, db, container, status code,
  sub-status, and the diagnostics JSON at the configured `error_verbosity`.
- **Threshold-violation entries**: `LogLevel::Info`, message includes account, db, container,
  status code, sub-status, and the diagnostics JSON at the configured `threshold_verbosity`.
- **When rate-limited**: A single `LogLevel::Info` message noting "Sampling budget exhausted;
  suppressing threshold-violation diagnostics until window resets at {timestamp}."

### Configuration Layering

Logging options follow the standard layered configuration model:

| Priority    | Source               | Example                             |
| ----------- | -------------------- | ----------------------------------- |
| 1 (highest) | Runtime builder API  | `builder.with_logging_options(...)` |
| 2           | Environment variable | `AZURE_COSMOS_LOG_LEVEL=debug`      |
| 3 (lowest)  | Compiled default     | `TracingLogSink` at `Info`          |

Log levels are resolved once at runtime initialization and are **not** dynamically mutable.
This is a deliberate simplification; dynamic level changes can be added in a follow-up if needed.

`SamplingDiagnosticsOptions` follow the same pattern (builder API > env var > compiled default)
and are also resolved once at runtime initialization.

### Thread Safety & Performance

- `LogSink` requires `Send + Sync + 'static`.
- `LoggingRuntime` uses `Mutex<Vec<LogEntry>>` for the buffer — lock is held only to push/drain,
  never during `sink.emit()`.
- `is_enabled()` is a `HashMap::get` (O(1) amortized) + integer comparison — zero allocation, no lock.
- `emit()` allocations: one `String` for `format!()` per entry, amortized `Vec` growth in buffer.
- `TokenBucket` uses `AtomicU32`/`AtomicU64` — lock-free, no allocation.
- `evaluate()` on the sampling evaluator is called once per operation — the hot path
  (`is_enabled()` returning `false`, or sampling budget exhausted) is a few atomic loads/compares.

### FFI Considerations

`azure_data_cosmos_native` will implement an `FfiLogSink` that:

1. Converts each `LogEntry` to a C-compatible struct (`CosmosLogEntry` with `*const c_char` fields).
2. Invokes a function pointer registered by the host language (Java/C#/Python).
3. The function pointer is called on the flush thread — host must not block.

The pre-formatted `message: String` design avoids requiring the FFI layer to interpret Rust format arguments.

FFI integration is **not** in scope for PRs 1–3. It will be a separate follow-up PR in the
`azure_data_cosmos_native` crate.

---

## Implementation Plan

### PR 1 – Log Sink Callback Interface & Runtime Wiring

#### Goal

Deliver the foundational logging types, the `LogSink` trait, the default `TracingLogSink`,
`LoggingOptions`/`LoggingOptionsBuilder`, `LoggingRuntime`, the `driver_log!` macro, and
the runtime wiring so that `CosmosDriverRuntime` owns a `LoggingRuntime`.

After this PR, internal code *can* call `driver_log!` but no call sites exist yet — that is
PR 2. The sampling evaluator is also not included yet — that is PR 3.

#### New Files

| File                                  | Description                                                                              |
| ------------------------------------- | ---------------------------------------------------------------------------------------- |
| `src/diagnostics/log_sink.rs`         | `LogLevel` enum, `LogEntry` struct, `LogSink` trait                                      |
| `src/diagnostics/tracing_log_sink.rs` | `TracingLogSink`: default sink forwarding to `tracing`                                   |
| `src/diagnostics/logging_runtime.rs`  | `LoggingRuntime` struct, buffer/flush logic, `driver_log!` macro, `file_stem()` const fn |
| `src/options/logging_options.rs`      | `LoggingOptions`, `LoggingOptionsBuilder`                                                |

#### Modified Files

- `src/diagnostics/mod.rs`
  - Add `mod log_sink; mod tracing_log_sink; mod logging_runtime;` declarations.
  - Add public re-exports for `LogLevel`, `LogEntry`, `LogSink`, `TracingLogSink`.
  - Re-export `LoggingRuntime` as `pub(crate)`.
  - Re-export `driver_log!` as `pub(crate)`.
- `src/options/mod.rs`
  - Add `mod logging_options;` declaration.
  - Add `pub use logging_options::{LoggingOptions, LoggingOptionsBuilder};`.
- `src/lib.rs`
  - Add re-exports at crate root: `LogLevel`, `LogEntry`, `LogSink`, `TracingLogSink`, `LoggingOptions`.
- `src/driver/runtime.rs`
  - Add `logging: Arc<LoggingRuntime>` field to `CosmosDriverRuntime`.
  - Add `pub(crate) fn logging(&self) -> &Arc<LoggingRuntime>` accessor.
  - Add `with_logging_options(LoggingOptions)` to `CosmosDriverRuntimeBuilder`.
  - Construct `LoggingRuntime` in `build()`.

#### Public API Surface (New)

```rust
// diagnostics module
pub enum LogLevel { Error, Warn, Info, Debug, Trace }
pub struct LogEntry { pub level, pub timestamp, pub logger, pub message, pub module, pub file, pub line }
pub trait LogSink: Send + Sync + 'static { fn max_level() -> LogLevel; fn emit(&[LogEntry]); fn flush() {} }
pub struct TracingLogSink { /* ... */ }

// options module
pub struct LoggingOptions { /* ... */ }
pub struct LoggingOptionsBuilder { /* ... */ }
```

#### Internal API Surface (New)

```rust
// diagnostics module (pub(crate))
pub(crate) struct LoggingRuntime { /* ... */ }
macro_rules! driver_log { /* ... */ }
```

#### Acceptance Criteria

1. `cargo build -p azure_data_cosmos_driver` succeeds.
2. `cargo clippy -p azure_data_cosmos_driver --all-features --all-targets` is warning-free.
3. `cargo fmt -p azure_data_cosmos_driver -- --check` passes.
4. `cargo doc -p azure_data_cosmos_driver --no-deps --all-features` succeeds with no warnings.
5. `cargo test -p azure_data_cosmos_driver --all-features` passes with no failures.
6. Unit tests cover:
   - `LogLevel` ordering (`Error < Warn < Info < Debug < Trace`).
   - `LogLevel` `Display`/`FromStr` round-trip.
   - `LogEntry::new()` captures timestamp and logger name.
   - `TracingLogSink::default()` max level is `Info`.
   - `TracingLogSink::emit()` does not panic (no subscriber installed).
   - `LoggingRuntime::is_enabled()` returns correct boolean for various levels.
   - `LoggingRuntime::is_enabled()` with per-logger override returns a different result
     than the default level.
   - `LoggingRuntime::effective_level()` returns override when set, default otherwise.
   - `LoggingRuntime::emit()` buffers entries and `flush()` drains them.
   - `LoggingRuntime` auto-flush on drop delivers buffered entries.
   - `LoggingOptionsBuilder` reads env vars when programmatic values are absent.
   - `LoggingOptionsBuilder` programmatic values override env vars.
   - `LoggingOptionsBuilder::with_level_override()` is reflected in `LoggingRuntime`.
   - `file_stem()` correctly strips directory and `.rs` extension.
7. Doctest on `TracingLogSink` example compiles and runs.
8. The `driver_log!` macro compiles and correctly short-circuits when level is disabled.
9. The `driver_log!` macro Form 2 (no explicit logger) produces entries with the file stem as logger.

#### Testing Strategy

- All tests go in `#[cfg(test)] mod tests` at the bottom of each new file.
- Use a custom test sink (`TestSink`) that collects entries into a shared `Vec` to assert
  on batching, levels, message content, and flush behavior.
- No integration tests needed — this PR has no I/O.

#### Dependencies

- None — this is the first PR in the series.

#### Review Checklist

- [ ] `LogLevel` ordering is `Error < Warn < ... < Trace` (not reversed).
- [ ] `LogSink::emit()` receives `&[LogEntry]` (immutable slice), not `Vec`.
- [ ] `LoggingRuntime` does not hold the `Mutex` lock while calling `sink.emit()`.
- [ ] `driver_log!` checks `is_enabled(logger, level)` before `format!()`.
- [ ] `driver_log!` Form 2 derives logger from `file_stem(file!())`.
- [ ] `file_stem()` is `const fn` and handles edge cases (`mod.rs`, no extension, paths with `/` and `\`).
- [ ] `LoggingOptions` is `#[non_exhaustive]`.
- [ ] `level_overrides` are compiled into `HashMap` at `LoggingRuntime` construction, not on each `is_enabled` call.
- [ ] No `use` directives inside functions.
- [ ] Manual `Debug` impls for types containing `Arc<dyn LogSink>`.
- [ ] Copyright headers on all new `.rs` files.

---

### PR 2 – Internal Driver Instrumentation

#### Goal

Place `driver_log!` call sites throughout the driver codebase at appropriate levels.
This PR does not add new types — it only adds log statements using the infrastructure
from PR 1.

After this PR, users who configure a `LogSink` (or rely on the default `TracingLogSink`)
see structured log output for driver lifecycle events, per-operation telemetry, and
internal subsystem activity.

#### Depends On

PR 1 (Log Sink Callback Interface & Runtime Wiring).

#### New Files

None.

#### Modified Files

| File                                             | Changes                                                         | Log Points Added                                                                                                                                        |
| ------------------------------------------------ | --------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/driver/runtime.rs`                          | Import `driver_log!`, add log statements                        | `Info`: runtime created (in `build()`), runtime dropped (in `Drop` impl if appropriate)                                                                 |
| `src/driver/cosmos_driver.rs`                    | Import `driver_log!`, add log statements in `execute_operation` | `Debug`: operation entry (operation type, resource type, activity ID), retry decision (attempt #, reason), operation exit (status, duration, RU charge) |
| `src/driver/transport/mod.rs` *(or sub-modules)* | Import `driver_log!`                                            | `Debug`: pipeline creation, connection pool new/reuse events                                                                                            |
| `src/driver/cache/*.rs`                          | Import `driver_log!`                                            | `Trace`: cache hit/miss for containers and account metadata                                                                                             |
| `src/system/cpu_memory.rs`                       | Import `driver_log!`                                            | `Trace`: CPU/memory sample values                                                                                                                       |
| `src/system/vm_metadata.rs`                      | Import `driver_log!`                                            | `Info`: IMDS fetch success result; `Warn`: IMDS fallback (timeout or unavailable)                                                                       |

#### Instrumentation Points (Detailed)

| Module                                         | Logger Name                | Form             | Level   | When                      | Message Template                                                                          |
| ---------------------------------------------- | -------------------------- | ---------------- | ------- | ------------------------- | ----------------------------------------------------------------------------------------- |
| `runtime.rs` (`build()`)                       | `"runtime"`                | 2 (auto)         | `Info`  | Runtime created           | `"CosmosDriverRuntime created (connection_pool={:?}, cpu_interval={:?})"`                 |
| `cosmos_driver.rs` (`execute_operation` entry) | `"cosmos_driver"`          | 2 (auto)         | `Debug` | Before retry loop         | `"execute_operation: op={:?}, resource={:?}, activity_id={}"`                             |
| `cosmos_driver.rs` (retry branch)              | `"cosmos_driver"`          | 2 (auto)         | `Debug` | Retry decision made       | `"Retrying attempt {} for activity_id={} (reason: transport failure, sent_status={:?})"`  |
| `cosmos_driver.rs` (success return)            | `"cosmos_driver"`          | 2 (auto)         | `Debug` | After successful response | `"execute_operation complete: activity_id={}, status={}, charge={:.2} RU, duration={}ms"` |
| `cosmos_driver.rs` (error return)              | `"cosmos_driver"`          | 2 (auto)         | `Debug` | After final failure       | `"execute_operation failed: activity_id={}, attempts={}, error={}"`                       |
| `transport/mod.rs` (pipeline create)           | `"transport"`              | **1 (explicit)** | `Debug` | Pipeline created          | `"Pipeline created: type={:?}, endpoint={}"`                                              |
| `cache/container_cache.rs` (lookup)            | `"container_cache"`        | 2 (auto)         | `Trace` | Cache hit/miss            | `"Container cache {}: db={}, container={}"` (`"hit"` or `"miss"`)                         |
| `cache/account_metadata_cache.rs` (lookup)     | `"account_metadata_cache"` | 2 (auto)         | `Trace` | Cache hit/miss            | `"Account metadata cache {}: endpoint={}"`                                                |
| `cpu_memory.rs` (sample)                       | `"cpu_memory"`             | 2 (auto)         | `Trace` | Each sample               | `"CPU/memory sample: cpu={:.1}%, mem_available={}MB"`                                     |
| `vm_metadata.rs` (success)                     | `"vm_metadata"`            | 2 (auto)         | `Info`  | IMDS response             | `"VM metadata fetched: machine_id={}"`                                                    |
| `vm_metadata.rs` (fallback)                    | `"vm_metadata"`            | 2 (auto)         | `Warn`  | IMDS unavailable          | `"IMDS unavailable, using generated machine_id={}"`                                       |

> **Note**: `transport/mod.rs` uses Form 1 (explicit logger name) because `file_stem("…/mod.rs")` would
> return `"mod"`, which is not a useful identifier. All other call sites use Form 2 (auto-derived from file stem).

#### Noise Guidelines

- **Error**: Only for unexpected internal failures (not service-returned errors).
- **Warn**: Degraded paths (e.g., fallback from IMDS, retries exhausted before transport gives up).
- **Info**: Lifecycle events (runtime create/drop, first driver created).
- **Debug**: Per-operation events useful for troubleshooting.
- **Trace**: High-frequency telemetry (every cache lookup, every CPU sample).

Production deployments should run at `Info`; `Debug`/`Trace` are development aids.

#### Acceptance Criteria

1. `cargo build`, `cargo clippy`, `cargo fmt --check`, `cargo doc`, `cargo test` all pass (same standard as PR 1).
2. Each log statement uses `driver_log!` (not raw `tracing::info!` etc.) so output is
   routed through the `LogSink`.
3. No `Debug` or `Trace` log statements allocate or format strings when the level is disabled
   (the `driver_log!` macro ensures this).
4. All logging accesses `self.runtime.logging()` — there is no direct `tracing` use outside
   `TracingLogSink`.
5. No new public API types are introduced.
6. Existing tests continue to pass unchanged (log output is silent unless a sink is configured).

#### Testing Strategy

- No *new* test files. Existing tests should not break.
- Optionally: add one or two tests in `cosmos_driver.rs`'s test module that use a `TestSink`
  to verify that `execute_operation` emits the expected `Debug` entries (but this requires
  constructing a mock operation, which may be deferred to integration tests).
- The primary validation is manual: run the emulator test suite with `AZURE_COSMOS_LOG_LEVEL=trace`
  and a `tracing_subscriber` to see output.

#### Review Checklist

- [ ] Every `driver_log!` call receives `self.runtime.logging()` (or the equivalent `LoggingRuntime` ref).
- [ ] `mod.rs` files use Form 1 with an explicit logger name (not `"mod"`).
- [ ] All logger names match the well-known names table in the spec.
- [ ] No `format!()` happens outside the `driver_log!` macro (no eager formatting).
- [ ] Log levels assigned per the noise guidelines above.
- [ ] No `tracing::*!` macro calls added outside `tracing_log_sink.rs`.
- [ ] Message templates do not include sensitive data (keys, tokens, partition key values).
- [ ] Each modified file still compiles independently (`cargo check`).

---

### PR 3 – Sampling Diagnostics Evaluator

#### Goal

Implement the sampling diagnostics evaluator that, after each `execute_operation`,
inspects the completed `DiagnosticsContext` and decides whether to emit it to the `LogSink`
as a structured diagnostics log entry. Errors are always logged; threshold violations are
rate-limited by a hierarchical token-bucket sampler.

This is the Rust equivalent of Java/Scala's `CosmosDiagnosticsHandler` +
`CosmosSamplingDiagnosticsLogger`.

#### Depends On

PR 1 (types + runtime wiring) and PR 2 (instrumentation pattern established, though not
strictly required for compilation).

#### New Files

| File                                          | Description                                                                                                                                                                                                         |
| --------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/diagnostics/token_bucket.rs`             | `TokenBucket` struct: lock-free rate limiter with `AtomicU32` counter and `AtomicU64` next-reset timestamp. Methods: `try_acquire() -> bool`, `reset()`.                                                            |
| `src/diagnostics/sampling_evaluator.rs`       | `SamplingDiagnosticsEvaluator` struct: holds three `TokenBucket` instances, `SamplingDiagnosticsOptions`, `DiagnosticsThresholds`. Method: `evaluate(&self, diagnostics: &DiagnosticsContext, sink: &dyn LogSink)`. |
| `src/options/sampling_diagnostics_options.rs` | `SamplingDiagnosticsOptions`, `SamplingDiagnosticsOptionsBuilder` with defaults and env-var support.                                                                                                                |

#### Modified Files

| File                          | Changes                                                                                                                                                                                                                                                               |
| ----------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `src/diagnostics/mod.rs`      | Add `mod token_bucket; mod sampling_evaluator;` declarations. Re-export `SamplingDiagnosticsEvaluator` as `pub(crate)`.                                                                                                                                               |
| `src/options/mod.rs`          | Add `mod sampling_diagnostics_options;` declaration. Add `pub use sampling_diagnostics_options::{SamplingDiagnosticsOptions, SamplingDiagnosticsOptionsBuilder};`.                                                                                                    |
| `src/lib.rs`                  | Add re-export for `SamplingDiagnosticsOptions` at crate root.                                                                                                                                                                                                         |
| `src/driver/runtime.rs`       | Add `sampling_evaluator: Arc<SamplingDiagnosticsEvaluator>` field to `CosmosDriverRuntime`. Add `pub(crate) fn sampling_evaluator(&self)` accessor. Add `with_sampling_diagnostics_options(SamplingDiagnosticsOptions)` to builder. Construct evaluator in `build()`. |
| `src/driver/cosmos_driver.rs` | In `execute_operation`, after `diagnostics_builder.complete()` and before `return Ok(CosmosResponse::new(...))`, call `self.runtime.sampling_evaluator().evaluate(...)`. Also call in the final `Err` branch if we have a partially-built diagnostics context.        |

#### Public API Surface (New)

```rust
// options module
pub struct SamplingDiagnosticsOptions { /* ... */ }
pub struct SamplingDiagnosticsOptionsBuilder { /* ... */ }
```

#### Internal API Surface (New)

```rust
// diagnostics module (pub(crate))
pub(crate) struct TokenBucket { /* ... */ }
pub(crate) struct SamplingDiagnosticsEvaluator { /* ... */ }
```

#### Key Design Decisions

1. **Errors always logged**: `evaluate()` always emits error diagnostics regardless of
   sampling budget. This matches the Scala reference.

2. **Threshold check**: `is_threshold_violated()` compares `DiagnosticsContext::duration()`
   against `DiagnosticsThresholds::point_operation_latency_threshold()` /
   `non_point_operation_latency_threshold()` (depending on operation type), and
   `DiagnosticsContext::total_request_charge()` against `request_charge_threshold()`.
   Payload size checking uses response body length if available.

3. **Verbosity for log message**: The diagnostics JSON string is obtained via
   `DiagnosticsContext::to_json_string(verbosity)` where `verbosity` is
   `error_verbosity` or `threshold_verbosity` from `SamplingDiagnosticsOptions`.
   Both default to `Summary`.

4. **Rate-limit suppression message**: When a bucket first exhausts, a single
   `LogLevel::Info` entry is emitted noting the suppression. No further
   suppression messages until the bucket resets.

5. **Where `DiagnosticsThresholds` comes from**: The evaluator receives thresholds
   from the effective operation options (merged runtime → driver → operation level).
   The `evaluate()` method signature must accept the thresholds or the merged options.

#### TokenBucket Details

```rust
pub(crate) struct TokenBucket {
    window: Duration,
    cap: u32,
    counter: AtomicU32,
    next_reset_ms: AtomicU64,  // millis since epoch
    suppression_logged: AtomicBool,
}

impl TokenBucket {
    pub(crate) fn new(window: Duration, cap: u32) -> Self;

    /// Returns true if the caller is allowed to proceed.
    /// Atomically increments counter; resets if window has expired.
    pub(crate) fn try_acquire(&self) -> bool;
}
```

The `try_acquire` logic:

1. Load `next_reset_ms`. If `now >= next_reset_ms`, CAS to `now + window_ms`, reset counter to 0.
2. `counter.fetch_add(1, Relaxed)`. If old value < cap, return `true`. Else `false`.

This is intentionally approximate (racy) for performance — off-by-one under contention is
acceptable for rate limiting.

#### Evaluation Flow (Pseudocode)

```rs
evaluate(diagnostics, thresholds, sink):
    if not diagnostics.is_completed():
        return

    is_error = diagnostics.status() is Some and status.is_error()
    is_threshold = is_threshold_violated(diagnostics, thresholds)

    if is_error:
        emit_to_sink(sink, LogLevel::Error, diagnostics, error_verbosity)
        return

    if is_threshold:
        if burst_bucket.try_acquire() and medium_bucket.try_acquire() and long_bucket.try_acquire():
            emit_to_sink(sink, LogLevel::Info, diagnostics, threshold_verbosity)
        else:
            // Rate-limited. Log suppression notice once per bucket reset.
```

#### Acceptance Criteria

1. All standard checks pass (`build`, `clippy`, `fmt`, `doc`, `test`).
2. Unit tests cover:
   - `TokenBucket::try_acquire()` returns `true` up to `cap` times, then `false`.
   - `TokenBucket` resets after window expires (use `std::thread::sleep` or mock time).
   - Hierarchical bucket: inner bucket exhausted but outer still has capacity → allowed.
   - Hierarchical bucket: outer bucket exhausted → denied even if inner has capacity.
   - `SamplingDiagnosticsEvaluator::evaluate()` with error diagnostics → always emits.
   - `SamplingDiagnosticsEvaluator::evaluate()` with threshold violation → emits until budget.
   - `SamplingDiagnosticsEvaluator::evaluate()` with successful, non-threshold operation → no emit.
   - `SamplingDiagnosticsOptionsBuilder` default values and env-var override.
3. The `evaluate()` call in `execute_operation` does not change the operation's return value
   or error behavior — it is purely side-effectful (logging).
4. No new allocations on the fast path (successful operation, not threshold-violated,
   sampling budget not checked because `is_threshold_violated` returned false early).

#### Testing Strategy

- `token_bucket.rs`: Pure unit tests with `#[cfg(test)] mod tests`. Test window reset
  with `std::thread::sleep(Duration::from_millis(50))` and a 50ms window, or test
  deterministically by constructing a bucket with `next_reset_ms` in the past.
- `sampling_evaluator.rs`: Unit tests using a `TestSink` that records emitted entries.
  Construct `DiagnosticsContext` values via `DiagnosticsContextBuilder` (which is `pub(crate)`
  and available in tests within the crate).
- `sampling_diagnostics_options.rs`: Builder/default tests, same pattern as `diagnostics_options.rs`.
- No integration tests required in this PR, but the emulator test suite should continue to
  pass unchanged (the evaluator is a no-op when `SamplingDiagnosticsOptions::enabled` is true
  but no thresholds are exceeded and no errors occur).

#### Review Checklist

- [ ] `TokenBucket` uses `Relaxed` ordering (sufficient for rate limiting).
- [ ] `evaluate()` never panics — all `Option`/`Result` paths handled gracefully.
- [ ] Error diagnostics are *never* rate-limited.
- [ ] The `CosmosResponse` return path in `execute_operation` is not altered (evaluate is
      called before `return`, not after).
- [ ] `SamplingDiagnosticsOptions` is `#[non_exhaustive]`.
- [ ] No sensitive data in emitted log messages (partition key values, auth tokens).
- [ ] `TokenBucket` tests don't use wall-clock sleeps longer than 100ms (to keep tests fast).
- [ ] Copyright headers on all new `.rs` files.

---

## Open Questions

| #   | Question                                                                                     | Status                                                                   |
| --- | -------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| 1   | Should log levels be dynamically mutable after runtime init?                                 | Deferred — static for now                                                |
| 2   | Should `FfiLogSink` live in `azure_data_cosmos_native` or `azure_data_cosmos_driver`?        | TBD — likely native crate                                                |
| 3   | Should sampling diagnostics support per-operation-type budgets?                              | Deferred to post-PR 3 follow-up                                          |
| 4   | Should the evaluator accept `DiagnosticsThresholds` per call or use a runtime-level default? | TBD in PR 3 design — likely per-call from effective options              |
| 5   | Should a background flush thread be used instead of inline flushing in `emit()`?             | Deferred — inline flush is simpler, background thread can be added later |

---

## Related Documents

- [TRANSPORT_PIPELINE_SPEC.md](TRANSPORT_PIPELINE_SPEC.md) — Transport pipeline architecture
- [ARCHITECTURE.md](../ARCHITECTURE.md) — Driver API overview
- [CosmosSamplingDiagnosticsLogger.scala](https://github.com/Azure/azure-sdk-for-java/blob/main/sdk/cosmos/azure-cosmos-spark_3-4_2-12/src/main/scala/com/azure/cosmos/spark/CosmosSamplingDiagnosticsLogger.scala) — Scala reference implementation
