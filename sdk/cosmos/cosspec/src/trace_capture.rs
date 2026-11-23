// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-trial capture of [`tracing`] events and spans.
//!
//! Trials run concurrently on one runtime, so a plain subscriber would
//! interleave their output. [`TraceCapture`] is a [`Layer`] that keeps each
//! trial's output separate by span ancestry: [`TraceCapture::begin`] returns a
//! root span, and every span and event under it, on any thread, is recorded
//! into that trial's buffer. Work outside every trial span, such as a
//! long-lived background task, is recorded into a run-level buffer instead.
//! So is anything that arrives after its trial finished, tagged with the
//! trial's name.
//!
//! Tasks spawned with `tokio::spawn` don't inherit the current span unless
//! they're instrumented, for example with
//! [`Instrument::in_current_span`](tracing::Instrument::in_current_span).
//!
//! This module depends only on `tracing`, `tracing-subscriber`, and `std`.
//!
//! [`Layer`]: tracing_subscriber::Layer

use std::{
    collections::HashMap,
    fmt,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex, MutexGuard,
    },
    time::Instant,
};
use tracing::{
    field::{Field, Visit},
    span::{Attributes, Id, Record},
    Event, Level, Span, Subscriber,
};
use tracing_subscriber::{layer::Context, registry::LookupSpan, Layer};

/// The target of every trial's root span.
pub const TRIAL_TARGET: &str = "cosspec::trial";

const TRIAL_SPAN: &str = "cosspec.trial";
const TRIAL_FIELD: &str = "trial";
const ACTIVITY_ID_FIELD: &str = "activity_id";
const MESSAGE_FIELD: &str = "message";

/// A recorded field value.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    Bool(bool),
    I64(i64),
    U64(u64),
    F64(f64),
    /// A string, or any other value's `Debug` form.
    Str(String),
}

impl fmt::Display for FieldValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FieldValue::Bool(v) => v.fmt(f),
            FieldValue::I64(v) => v.fmt(f),
            FieldValue::U64(v) => v.fmt(f),
            FieldValue::F64(v) => v.fmt(f),
            FieldValue::Str(v) => v.fmt(f),
        }
    }
}

/// Named field values, in recording order.
pub type Fields = Vec<(&'static str, FieldValue)>;

/// One captured event.
#[derive(Clone, Debug)]
pub struct LogRecord {
    /// Milliseconds from the trial start, or from the capture's creation for
    /// run-level events.
    pub t_ms: f64,
    pub level: Level,
    pub target: String,
    pub message: String,
    /// Every field except `message`.
    pub fields: Fields,
    /// The index into the trial's spans of the event's innermost span.
    pub span: Option<usize>,
    /// The `activity_id` field of the event or its nearest span that has one.
    pub activity_id: Option<String>,
    pub file: Option<&'static str>,
    pub line: Option<u32>,
    /// For run-level events: the trial that emitted the event after it
    /// finished.
    pub trial: Option<String>,
}

/// One captured span.
#[derive(Clone, Debug)]
pub struct SpanRecord {
    /// The span's index in its trial's spans.
    pub id: usize,
    /// The index of the parent span; `None` directly under the trial span.
    pub parent: Option<usize>,
    pub name: &'static str,
    pub target: &'static str,
    pub level: Level,
    pub fields: Fields,
    /// Milliseconds from the trial start.
    pub start_ms: f64,
    /// Milliseconds from the trial start; `None` if the span was still open
    /// when the trial finished.
    pub end_ms: Option<f64>,
}

/// Everything captured for one trial.
#[derive(Debug, Default)]
pub struct TrialCapture {
    pub logs: Vec<LogRecord>,
    pub spans: Vec<SpanRecord>,
}

/// A [`Layer`] that captures events and spans per trial.
///
/// Clones share their buffers. Install one clone as a layer and keep another
/// to call [`begin`](Self::begin) and [`take_run_logs`](Self::take_run_logs).
#[derive(Clone)]
pub struct TraceCapture {
    inner: Arc<Inner>,
}

struct Inner {
    created: Instant,
    next_trial: AtomicU64,
    /// Buffers whose root span hasn't been created yet.
    pending: Mutex<HashMap<u64, Arc<Buffer>>>,
    run: Mutex<Vec<LogRecord>>,
}

struct Buffer {
    name: String,
    start: Instant,
    state: Mutex<BufferState>,
}

#[derive(Default)]
struct BufferState {
    closed: bool,
    capture: TrialCapture,
}

impl Buffer {
    fn state(&self) -> MutexGuard<'_, BufferState> {
        self.state.lock().unwrap_or_else(|e| e.into_inner())
    }

    fn millis(&self) -> f64 {
        self.start.elapsed().as_secs_f64() * 1000.0
    }
}

/// A trial's handle on its buffer.
pub struct TrialTrace {
    id: u64,
    buffer: Arc<Buffer>,
    inner: Arc<Inner>,
}

impl TrialTrace {
    /// Stops capturing and returns everything captured. Later events of the
    /// trial's spans go to the run-level buffer.
    pub fn finish(self) -> TrialCapture {
        lock(&self.inner.pending).remove(&self.id);
        let mut state = self.buffer.state();
        state.closed = true;
        std::mem::take(&mut state.capture)
    }
}

/// What the layer stores in each span's extensions.
struct SpanData {
    buffer: Option<Arc<Buffer>>,
    /// The span's index in its buffer; `None` for trial spans and spans
    /// recorded nowhere.
    index: Option<usize>,
    activity_id: Option<String>,
}

impl Default for TraceCapture {
    fn default() -> Self {
        Self::new()
    }
}

impl TraceCapture {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Inner {
                created: Instant::now(),
                next_trial: AtomicU64::new(0),
                pending: Mutex::new(HashMap::new()),
                run: Mutex::new(Vec::new()),
            }),
        }
    }

    /// Starts capturing a trial named `name` whose times are measured from
    /// `start`. Run the trial inside the returned span.
    pub fn begin(&self, name: impl Into<String>, start: Instant) -> (TrialTrace, Span) {
        let id = self.inner.next_trial.fetch_add(1, Ordering::Relaxed);
        let buffer = Arc::new(Buffer {
            name: name.into(),
            start,
            state: Mutex::new(BufferState::default()),
        });
        lock(&self.inner.pending).insert(id, buffer.clone());
        let span = tracing::info_span!(target: TRIAL_TARGET, parent: None, TRIAL_SPAN, trial = id);
        let trace = TrialTrace {
            id,
            buffer,
            inner: self.inner.clone(),
        };
        (trace, span)
    }

    /// Takes the events captured outside every trial so far.
    pub fn take_run_logs(&self) -> Vec<LogRecord> {
        std::mem::take(&mut *lock(&self.inner.run))
    }

    fn run_millis(&self) -> f64 {
        self.inner.created.elapsed().as_secs_f64() * 1000.0
    }
}

impl<S> Layer<S> for TraceCapture
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else {
            return;
        };
        let meta = attrs.metadata();
        let mut visitor = FieldVisitor::default();
        attrs.record(&mut visitor);

        let data = if meta.target() == TRIAL_TARGET && meta.name() == TRIAL_SPAN {
            let buffer = visitor
                .trial
                .and_then(|trial| lock(&self.inner.pending).remove(&trial));
            SpanData {
                buffer,
                index: None,
                activity_id: None,
            }
        } else {
            let (buffer, parent, inherited) = span
                .parent()
                .and_then(|parent| {
                    let ext = parent.extensions();
                    ext.get::<SpanData>()
                        .map(|data| (data.buffer.clone(), data.index, data.activity_id.clone()))
                })
                .unwrap_or_default();
            let activity_id = visitor.activity_id.take().or(inherited);
            let index = buffer.as_ref().and_then(|buffer| {
                let mut state = buffer.state();
                if state.closed {
                    return None;
                }
                let spans = &mut state.capture.spans;
                let index = spans.len();
                spans.push(SpanRecord {
                    id: index,
                    parent,
                    name: meta.name(),
                    target: meta.target(),
                    level: *meta.level(),
                    fields: std::mem::take(&mut visitor.fields),
                    start_ms: buffer.millis(),
                    end_ms: None,
                });
                Some(index)
            });
            SpanData {
                buffer,
                index,
                activity_id,
            }
        };
        span.extensions_mut().insert(data);
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else {
            return;
        };
        let mut visitor = FieldVisitor::default();
        values.record(&mut visitor);
        let mut ext = span.extensions_mut();
        let Some(data) = ext.get_mut::<SpanData>() else {
            return;
        };
        if let Some(activity_id) = visitor.activity_id.take() {
            data.activity_id = Some(activity_id);
        }
        if let (Some(buffer), Some(index)) = (&data.buffer, data.index) {
            let mut state = buffer.state();
            if let Some(record) = state.capture.spans.get_mut(index) {
                for (name, value) in visitor.fields {
                    match record.fields.iter_mut().find(|(n, _)| *n == name) {
                        Some(field) => field.1 = value,
                        None => record.fields.push((name, value)),
                    }
                }
            }
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let meta = event.metadata();
        let mut visitor = FieldVisitor::default();
        event.record(&mut visitor);

        let mut buffer = None;
        let mut span = None;
        let mut activity_id = visitor.activity_id.take();
        if let Some(scope) = ctx.event_scope(event) {
            for parent in scope {
                let ext = parent.extensions();
                let Some(data) = ext.get::<SpanData>() else {
                    continue;
                };
                if activity_id.is_none() {
                    activity_id.clone_from(&data.activity_id);
                }
                span = data.index;
                buffer = data.buffer.clone();
                break;
            }
        }

        let mut record = LogRecord {
            t_ms: 0.0,
            level: *meta.level(),
            target: meta.target().to_owned(),
            message: visitor.message.unwrap_or_default(),
            fields: visitor.fields,
            span,
            activity_id,
            file: meta.file(),
            line: meta.line(),
            trial: None,
        };
        if let Some(buffer) = buffer {
            let mut state = buffer.state();
            if !state.closed {
                record.t_ms = buffer.millis();
                state.capture.logs.push(record);
                return;
            }
            record.span = None;
            record.trial = Some(buffer.name.clone());
        }
        record.t_ms = self.run_millis();
        lock(&self.inner.run).push(record);
    }

    fn on_close(&self, id: Id, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(&id) else {
            return;
        };
        let ext = span.extensions();
        let Some(SpanData {
            buffer: Some(buffer),
            index: Some(index),
            ..
        }) = ext.get::<SpanData>()
        else {
            return;
        };
        let end = buffer.millis();
        let mut state = buffer.state();
        if let Some(record) = state.capture.spans.get_mut(*index) {
            record.end_ms = Some(end);
        }
    }
}

#[derive(Default)]
struct FieldVisitor {
    message: Option<String>,
    activity_id: Option<String>,
    trial: Option<u64>,
    fields: Fields,
}

impl FieldVisitor {
    fn value(&mut self, field: &Field, value: FieldValue) {
        match field.name() {
            MESSAGE_FIELD => self.message = Some(value.to_string()),
            ACTIVITY_ID_FIELD => {
                self.activity_id = Some(value.to_string());
                self.fields.push((field.name(), value));
            }
            name => self.fields.push((name, value)),
        }
    }
}

impl Visit for FieldVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        self.value(field, FieldValue::Bool(value));
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.value(field, FieldValue::I64(value));
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        if field.name() == TRIAL_FIELD {
            self.trial = Some(value);
        }
        self.value(field, FieldValue::U64(value));
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.value(field, FieldValue::F64(value));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.value(field, FieldValue::Str(value.to_owned()));
    }

    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.value(field, FieldValue::Str(value.to_string()));
    }

    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.value(field, FieldValue::Str(format!("{value:?}")));
    }
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::{FieldValue, TraceCapture};
    use std::time::Instant;
    use tracing::{Dispatch, Instrument, Level};
    use tracing_subscriber::{layer::SubscriberExt, Registry};

    fn dispatch(capture: &TraceCapture) -> Dispatch {
        Dispatch::new(Registry::default().with(capture.clone()))
    }

    /// A runtime whose worker threads use `dispatch`, like a global default.
    fn runtime(dispatch: &Dispatch) -> tokio::runtime::Runtime {
        let dispatch = dispatch.clone();
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(4)
            .on_thread_start(move || {
                std::mem::forget(tracing::dispatcher::set_default(&dispatch));
            })
            .enable_all()
            .build()
            .unwrap()
    }

    fn messages(logs: &[super::LogRecord]) -> Vec<&str> {
        logs.iter().map(|log| log.message.as_str()).collect()
    }

    #[test]
    fn concurrent_trials_capture_only_their_own_events() {
        let capture = TraceCapture::new();
        let dispatch = dispatch(&capture);
        let _guard = tracing::dispatcher::set_default(&dispatch);
        let rt = runtime(&dispatch);
        let trial = |name: &'static str| {
            let (trace, span) = capture.begin(name, Instant::now());
            let task = async move {
                for i in 0..20 {
                    tracing::info!("{name} {i}");
                    tokio::task::yield_now().await;
                }
            };
            (trace, rt.spawn(task.instrument(span)))
        };
        let (a, a_task) = trial("a");
        let (b, b_task) = trial("b");
        rt.block_on(async {
            a_task.await.unwrap();
            b_task.await.unwrap();
        });
        let (a, b) = (a.finish(), b.finish());
        assert_eq!(a.logs.len(), 20);
        assert_eq!(b.logs.len(), 20);
        assert!(a.logs.iter().all(|log| log.message.starts_with("a ")));
        assert!(b.logs.iter().all(|log| log.message.starts_with("b ")));
        assert!(capture.take_run_logs().is_empty());
    }

    #[test]
    fn events_outside_trials_go_to_the_run() {
        let capture = TraceCapture::new();
        let _guard = tracing::dispatcher::set_default(&dispatch(&capture));
        let (trace, _span) = capture.begin("t", Instant::now());
        tracing::warn!(n = 1, "outside");
        assert!(trace.finish().logs.is_empty());
        let run = capture.take_run_logs();
        assert_eq!(messages(&run), ["outside"]);
        assert_eq!(run[0].level, Level::WARN);
        assert_eq!(run[0].trial, None);
        assert_eq!(run[0].fields, [("n", FieldValue::I64(1))]);
    }

    #[test]
    fn spawned_tasks_need_the_current_span() {
        let capture = TraceCapture::new();
        let dispatch = dispatch(&capture);
        let _guard = tracing::dispatcher::set_default(&dispatch);
        let rt = runtime(&dispatch);
        let (trace, span) = capture.begin("t", Instant::now());
        rt.block_on(
            async {
                tokio::spawn(async { tracing::info!("kept") }.in_current_span())
                    .await
                    .unwrap();
                tokio::spawn(async { tracing::info!("lost") })
                    .await
                    .unwrap();
            }
            .instrument(span),
        );
        assert_eq!(messages(&trace.finish().logs), ["kept"]);
        assert_eq!(messages(&capture.take_run_logs()), ["lost"]);
    }

    #[test]
    fn late_events_go_to_the_run_tagged_with_the_trial() {
        let capture = TraceCapture::new();
        let _guard = tracing::dispatcher::set_default(&dispatch(&capture));
        let (trace, span) = capture.begin("my trial", Instant::now());
        let inner = span.in_scope(|| tracing::debug_span!("background"));
        trace.finish();
        inner.in_scope(|| tracing::info!("late"));
        let run = capture.take_run_logs();
        assert_eq!(messages(&run), ["late"]);
        assert_eq!(run[0].trial.as_deref(), Some("my trial"));
        assert_eq!(run[0].span, None);
    }

    #[test]
    fn spans_record_parents_fields_and_activity_ids() {
        let capture = TraceCapture::new();
        let _guard = tracing::dispatcher::set_default(&dispatch(&capture));
        let (trace, span) = capture.begin("t", Instant::now());
        span.in_scope(|| {
            let outer = tracing::info_span!("outer", activity_id = "abc", ok = true);
            outer.in_scope(|| {
                let inner = tracing::debug_span!("inner", n = tracing::field::Empty);
                inner.record("n", 7u64);
                inner.in_scope(|| tracing::info!(x = 1.5, "hello {}", "world"));
            });
            tracing::info!("bare");
        });
        let capture = trace.finish();

        let [outer, inner] = &capture.spans[..] else {
            panic!("expected two spans: {:?}", capture.spans);
        };
        assert_eq!((outer.id, outer.parent, outer.name), (0, None, "outer"));
        assert_eq!((inner.id, inner.parent, inner.name), (1, Some(0), "inner"));
        assert_eq!(
            outer.fields,
            [
                ("activity_id", FieldValue::Str("abc".to_owned())),
                ("ok", FieldValue::Bool(true)),
            ]
        );
        assert_eq!(inner.fields, [("n", FieldValue::U64(7))]);
        assert!(outer.end_ms.is_some() && inner.end_ms.is_some());
        assert!(outer.start_ms <= inner.start_ms);

        let [hello, bare] = &capture.logs[..] else {
            panic!("expected two logs: {:?}", capture.logs);
        };
        assert_eq!(hello.message, "hello world");
        assert_eq!(hello.span, Some(1));
        assert_eq!(hello.activity_id.as_deref(), Some("abc"));
        assert_eq!(hello.fields, [("x", FieldValue::F64(1.5))]);
        assert_eq!((bare.span, bare.activity_id.as_deref()), (None, None));
    }
}
