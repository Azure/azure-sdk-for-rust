// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Spec results: one JSON file per executed trial, describing what happened.
//!
//! The format is versioned by [`RESULT_VERSION`]. Adding fields is not a
//! breaking change; renaming, removing, or changing the meaning of a field is,
//! and bumps the version.

use crate::{
    backend::Backend, http_tap::HttpRecord, runner::SpecDescriptor, ServiceProfile, SpecError,
    SpecResult,
};
use azure_core::time::{to_rfc3339, OffsetDateTime};
use azure_data_cosmos_driver::transport_observer::HttpOutcomeKind;
use serde::Serialize;
use std::{
    backtrace::Backtrace,
    cell::RefCell,
    path::{Path, PathBuf},
    sync::{
        atomic::{AtomicBool, Ordering},
        OnceLock,
    },
    time::Instant,
};
use uuid::Uuid;

/// The version of the result file format.
pub const RESULT_VERSION: u32 = 1;

/// The most bytes of each HTTP request or response body a result file keeps.
pub const BODY_LIMIT: usize = 4096;

/// The command-line option that sets the results directory.
pub const RESULTS_DIR_ARG: &str = "--results-dir";

/// What kind of resource a spec used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ResourceKind {
    /// A Cosmos DB account.
    Account,
    /// A database.
    Database,
    /// A container.
    Container,
}

/// Where a resource came from.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Origin {
    /// Created from scratch for this spec.
    Created,
    /// Borrowed from the run's resource pool.
    Pooled,
}

/// One resource a spec created or borrowed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ResourceRecord {
    /// The resource kind.
    pub kind: ResourceKind,
    /// Whether the resource was created for the spec or borrowed from the pool.
    pub origin: Origin,
    /// The resource name. For an account, its endpoint.
    pub name: String,
    /// The parent database of a container.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    /// The container profile the container was shaped and seeded by.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<&'static str>,
    /// The spec parameter the resource was injected into.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameter: Option<&'static str>,
}

/// A panic caught while running a spec.
#[derive(Clone, Debug, Serialize)]
pub struct PanicRecord {
    /// The panic message.
    pub message: String,
    /// Where the panic happened, as `file:line:column`.
    pub location: Option<String>,
    /// The captured backtrace.
    pub backtrace: String,
}

thread_local! {
    static LAST_PANIC: RefCell<Option<PanicRecord>> = const { RefCell::new(None) };
}

/// Installs a panic hook that remembers the current thread's last panic, then
/// runs the previous hook so the usual output still appears.
static QUIET_PANICS: AtomicBool = AtomicBool::new(false);

/// Stops panics from printing to stderr; the console prints them with the failures.
pub(crate) fn quiet_panics() {
    QUIET_PANICS.store(true, Ordering::Relaxed);
}

pub(crate) fn install_panic_hook() {
    static INSTALLED: OnceLock<()> = OnceLock::new();
    INSTALLED.get_or_init(|| {
        let previous = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |info| {
            let payload = info.payload();
            let message = payload
                .downcast_ref::<String>()
                .cloned()
                .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
                .unwrap_or_else(|| "spec panicked".to_owned());
            let record = PanicRecord {
                message,
                location: info.location().map(ToString::to_string),
                backtrace: Backtrace::force_capture().to_string(),
            };
            LAST_PANIC.with(|last| *last.borrow_mut() = Some(record));
            if !QUIET_PANICS.load(Ordering::Relaxed) {
                previous(info);
            }
        }));
    });
}

/// Takes the current thread's last panic, if any.
pub(crate) fn take_panic() -> Option<PanicRecord> {
    LAST_PANIC.with(|last| last.borrow_mut().take())
}

static RESULTS_DIR: OnceLock<PathBuf> = OnceLock::new();

/// The default results directory: `<target>/cosspec-results/<run-id>`.
pub(crate) fn default_dir(run_id: &str) -> PathBuf {
    default_root().join(run_id)
}

/// The directory holding one `<run-id>` directory per run: `<target>/cosspec-results`.
pub(crate) fn default_root() -> PathBuf {
    let target = std::env::current_exe()
        .ok()
        .and_then(|exe| exe.ancestors().nth(3).map(Path::to_path_buf))
        .unwrap_or_else(std::env::temp_dir);
    target.join("cosspec-results")
}

/// The command-line flag that keeps every earlier run's results.
pub const NO_PRUNE_ARG: &str = "--no-prune";

/// How many runs [`prune`] keeps, including the current one.
pub(crate) const KEEP_RUNS: usize = 10;

/// Deletes all but the `keep` newest run directories in `root`.
///
/// Only directories named by a UUIDv7 run ID are considered; they sort by
/// creation time. Anything else in `root` is left alone. Returns how many
/// directories were deleted.
pub(crate) fn prune(root: &Path, keep: usize) -> std::io::Result<usize> {
    let mut runs: Vec<(Uuid, PathBuf)> = std::fs::read_dir(root)?
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|t| t.is_dir()))
        .filter_map(|entry| {
            let id = Uuid::parse_str(entry.file_name().to_str()?).ok()?;
            (id.get_version_num() == 7).then(|| (id, entry.path()))
        })
        .collect();
    runs.sort_by_key(|run| std::cmp::Reverse(run.0));
    let mut deleted = 0;
    for (_, path) in runs.into_iter().skip(keep) {
        std::fs::remove_dir_all(path)?;
        deleted += 1;
    }
    Ok(deleted)
}

/// Sets the directory result files are written to.
pub(crate) fn set_dir(dir: PathBuf) {
    RESULTS_DIR
        .set(dir)
        .expect("the results directory is set once");
}

/// The directory result files are written to.
pub(crate) fn dir() -> Option<&'static Path> {
    RESULTS_DIR.get().map(PathBuf::as_path)
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum Status {
    Passed,
    Failed,
}

#[derive(Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum OutcomeJson {
    Response {
        status: u16,
        sub_status: Option<u32>,
    },
    TransportError,
    Cancelled,
    Unknown,
}

#[derive(Debug, Serialize)]
struct HttpJson {
    label: Option<String>,
    sequence: u64,
    method: String,
    url: String,
    operation: Option<String>,
    region: Option<String>,
    /// The region's alias, such as `PrimaryReadWrite`.
    region_alias: Option<String>,
    activity_id: Option<String>,
    /// Milliseconds from the spec's start to the request.
    start_ms: f64,
    /// `None` if the exchange never finished.
    duration_ms: Option<f64>,
    /// `None` if the exchange never finished.
    outcome: Option<OutcomeJson>,
    /// `None` if the request had no body.
    request_body: Option<BodyJson>,
    /// `None` if no response arrived.
    response_body: Option<BodyJson>,
}

/// An HTTP body, cut to at most [`BODY_LIMIT`] bytes.
///
/// Exactly one of `text` and `base64` is set.
#[derive(Debug, PartialEq, Serialize)]
struct BodyJson {
    /// The body's full length in bytes.
    length: usize,
    /// Whether only the first part of the body was kept.
    truncated: bool,
    /// The kept bytes, when they are UTF-8 text such as JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    /// The kept bytes, base64-encoded, when they are binary, such as Cosmos DB
    /// binary encoding.
    #[serde(skip_serializing_if = "Option::is_none")]
    base64: Option<String>,
}

impl BodyJson {
    fn new(body: &[u8]) -> Self {
        let kept = &body[..body.len().min(BODY_LIMIT)];
        let truncated = kept.len() < body.len();
        let text = match std::str::from_utf8(kept) {
            Ok(text) => Some(text),
            // The cut split a multi-byte character; drop its first bytes.
            Err(e) if truncated && e.error_len().is_none() => {
                std::str::from_utf8(&kept[..e.valid_up_to()]).ok()
            }
            Err(_) => None,
        };
        Self {
            length: body.len(),
            truncated,
            text: text.map(str::to_owned),
            base64: text.is_none().then(|| azure_core::base64::encode(kept)),
        }
    }
}

fn millis(d: std::time::Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

impl HttpJson {
    fn new(record: HttpRecord, base: Instant) -> Self {
        Self {
            start_ms: millis(record.started_at.saturating_duration_since(base)),
            duration_ms: record.duration().map(millis),
            outcome: record.outcome.map(|kind| match kind {
                HttpOutcomeKind::Response { status, sub_status } => {
                    OutcomeJson::Response { status, sub_status }
                }
                HttpOutcomeKind::TransportError => OutcomeJson::TransportError,
                HttpOutcomeKind::Cancelled => OutcomeJson::Cancelled,
                _ => OutcomeJson::Unknown,
            }),
            request_body: record.request_body.as_deref().map(BodyJson::new),
            response_body: record.response_body.as_deref().map(BodyJson::new),
            label: record.label,
            sequence: record.sequence,
            method: record.method,
            url: record.url,
            operation: record.operation,
            region: record.region,
            region_alias: record.region_alias,
            activity_id: record.activity_id,
        }
    }
}

/// A named stretch of a trial, such as acquiring pooled resources or running
/// the spec body.
#[derive(Debug, Serialize)]
struct PhaseJson {
    name: &'static str,
    /// Milliseconds from the trial start.
    start_ms: f64,
    /// Milliseconds from the trial start.
    end_ms: f64,
}

#[derive(Debug, Serialize)]
struct KnownIssueJson {
    reason: &'static str,
    profiles: Vec<&'static str>,
}

#[derive(Debug, Serialize)]
struct ResultFile {
    version: u32,
    id: String,
    test_name: String,
    summary: &'static str,
    source: String,
    service_profile: &'static str,
    backend: String,
    backend_kind: &'static str,
    cleanup: bool,
    requires: Vec<&'static str>,
    known_issue: Option<KnownIssueJson>,
    started_at: String,
    duration_ms: f64,
    status: Status,
    error: Option<String>,
    teardown_error: Option<String>,
    panic: Option<PanicRecord>,
    resources: Vec<ResourceRecord>,
    phases: Vec<PhaseJson>,
    http: Vec<HttpJson>,
}

/// The result file name of one trial, for example
/// `query.split[inmemory.SingleRegion].json`.
pub(crate) fn file_name(spec: &SpecDescriptor, backend: &str, profile: ServiceProfile) -> String {
    format!("{}[{}.{}].json", spec.id(), backend, profile)
}

/// Everything collected while one trial ran.
#[derive(Debug)]
pub(crate) struct TrialRecord {
    pub spec: &'static SpecDescriptor,
    pub profile: ServiceProfile,
    pub backend: String,
    pub backend_kind: &'static str,
    pub started_at: OffsetDateTime,
    pub started: Instant,
    pub error: Option<SpecError>,
    pub teardown_error: Option<SpecError>,
    pub panic: Option<PanicRecord>,
    pub resources: Vec<ResourceRecord>,
    pub phases: Vec<(&'static str, Instant, Instant)>,
    pub http: Vec<HttpRecord>,
}

impl TrialRecord {
    /// Starts recording a trial.
    pub fn start(
        spec: &'static SpecDescriptor,
        profile: ServiceProfile,
        backend: &dyn Backend,
    ) -> Self {
        Self {
            spec,
            profile,
            backend: backend.name().to_owned(),
            backend_kind: backend.kind(),
            started_at: OffsetDateTime::now_utc(),
            started: Instant::now(),
            error: None,
            teardown_error: None,
            panic: None,
            resources: Vec::new(),
            phases: Vec::new(),
            http: Vec::new(),
        }
    }

    /// Records that the phase `name` ran from `start` until now.
    pub fn phase(&mut self, name: &'static str, start: Instant) {
        self.phases.push((name, start, Instant::now()));
    }

    /// The trial's result: the body's error, else the teardown's.
    pub fn result(&self) -> Result<(), String> {
        match (&self.panic, &self.error, &self.teardown_error) {
            (Some(panic), _, _) => Err(match &panic.location {
                Some(location) => format!("panicked at {location}:\n{}", panic.message),
                None => panic.message.clone(),
            }),
            (_, Some(e), _) | (_, _, Some(e)) => Err(e.to_string()),
            _ => Ok(()),
        }
    }

    fn file(self) -> ResultFile {
        let spec = self.spec;
        let status = if self.result().is_ok() {
            Status::Passed
        } else {
            Status::Failed
        };
        ResultFile {
            version: RESULT_VERSION,
            id: spec.id(),
            test_name: spec.test_name(&self.backend, self.profile),
            summary: spec.summary,
            source: format!("{}:{}", spec.file, spec.line),
            service_profile: self.profile.name(),
            backend_kind: self.backend_kind,
            backend: self.backend,
            cleanup: crate::pool::cleanup(),
            requires: spec.requires.iter().map(|c| c.name()).collect(),
            known_issue: spec
                .known_issue
                .as_ref()
                .filter(|issue| issue.affects(self.profile))
                .map(|issue| KnownIssueJson {
                    reason: issue.reason,
                    profiles: issue.profiles.iter().map(|p| p.name()).collect(),
                }),
            started_at: to_rfc3339(&self.started_at),
            duration_ms: millis(self.started.elapsed()),
            status,
            error: self.error.map(|e| e.to_string()),
            teardown_error: self.teardown_error.map(|e| e.to_string()),
            panic: self.panic,
            resources: self.resources,
            phases: self
                .phases
                .into_iter()
                .map(|(name, start, end)| PhaseJson {
                    name,
                    start_ms: millis(start.saturating_duration_since(self.started)),
                    end_ms: millis(end.saturating_duration_since(self.started)),
                })
                .collect(),
            http: self
                .http
                .into_iter()
                .map(|record| HttpJson::new(record, self.started))
                .collect(),
        }
    }

    /// The result file name, for example `query.split[inmemory.SingleRegion].json`.
    pub fn file_name(&self) -> String {
        file_name(self.spec, &self.backend, self.profile)
    }

    /// Writes the result file into `dir`.
    pub fn write(self, dir: &Path) -> SpecResult {
        std::fs::create_dir_all(dir)?;
        let path = dir.join(self.file_name());
        let json = serde_json::to_vec_pretty(&self.file())?;
        std::fs::write(&path, json)
            .map_err(|e| format!("failed to write spec result {}: {e}", path.display()).into())
    }
}

#[cfg(test)]
mod tests {
    use super::{prune, BodyJson, Uuid, BODY_LIMIT};

    #[test]
    fn body_keeps_short_bodies_whole() {
        assert_eq!(
            BodyJson::new(br#"{"id":"1"}"#),
            BodyJson {
                length: 10,
                truncated: false,
                text: Some(r#"{"id":"1"}"#.to_owned()),
                base64: None,
            }
        );
    }

    #[test]
    fn body_encodes_binary_as_base64() {
        let body = vec![0x80u8; BODY_LIMIT + 1];
        let json = BodyJson::new(&body);
        assert!(json.truncated);
        assert_eq!(json.text, None);
        assert_eq!(
            json.base64,
            Some(azure_core::base64::encode(&body[..BODY_LIMIT]))
        );
    }

    #[test]
    fn body_truncates_on_a_char_boundary() {
        // 'é' is two bytes, so the cut lands inside one at BODY_LIMIT.
        let body = format!("a{}", "é".repeat(BODY_LIMIT));
        let json = BodyJson::new(body.as_bytes());
        assert_eq!(json.length, body.len());
        assert!(json.truncated);
        let text = json.text.unwrap();
        assert_eq!(text.len(), BODY_LIMIT - 1);
        assert!(body.starts_with(&text));
    }

    #[test]
    fn prune_keeps_newest_runs_and_ignores_other_entries() {
        let root = std::env::temp_dir().join(format!("cosspec-prune-{}", Uuid::new_v4()));
        let runs: Vec<String> = (0..4).map(|_| Uuid::now_v7().to_string()).collect();
        for run in &runs {
            std::fs::create_dir_all(root.join(run)).unwrap();
        }
        let v4 = Uuid::new_v4().to_string();
        for other in ["notes", v4.as_str()] {
            std::fs::create_dir_all(root.join(other)).unwrap();
        }
        std::fs::write(root.join("README.txt"), "keep me").unwrap();

        assert_eq!(prune(&root, 2).unwrap(), 2);
        let mut left: Vec<String> = std::fs::read_dir(&root)
            .unwrap()
            .map(|e| e.unwrap().file_name().into_string().unwrap())
            .collect();
        left.sort();
        let mut expected = vec![
            runs[2].clone(),
            runs[3].clone(),
            "README.txt".to_owned(),
            "notes".to_owned(),
            v4.clone(),
        ];
        expected.sort();
        assert_eq!(left, expected);
        std::fs::remove_dir_all(root).unwrap();
    }
}
