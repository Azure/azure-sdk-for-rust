// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The run manifest: `manifest.json`, an index of one run's trials.
//!
//! The manifest lists every planned trial, including skipped and ignored ones
//! that have no result file, with its backend, service profile, status, and a
//! link to its result file. Tools read the manifest instead of parsing result
//! file names.
//!
//! The runner writes the manifest when the run starts, rewrites it as each
//! trial finishes, and marks it `completed` at the end. Every write replaces
//! the file atomically, so readers never see a partial manifest.
//!
//! The format is versioned by [`MANIFEST_VERSION`], separately from result
//! files. Adding fields is not a breaking change; renaming, removing, or
//! changing the meaning of a field is, and bumps the version.

use crate::{
    backend::Backend, results::RESULT_VERSION, runner::SpecDescriptor, ServiceProfile, SpecResult,
};
use azure_core::time::{to_rfc3339, OffsetDateTime};
use serde::Serialize;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex, OnceLock},
};

/// The version of the manifest format.
pub const MANIFEST_VERSION: u32 = 1;

/// The manifest's file name inside a run directory.
pub const MANIFEST_FILE: &str = "manifest.json";

/// The manifest's `kind`, which marks a directory as a cosspec run.
pub const MANIFEST_KIND: &str = "cosspec-run";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum RunStatus {
    Running,
    Completed,
}

/// A trial's or check's status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Status {
    /// Not finished yet.
    Pending,
    Passed,
    Failed,
    /// Not run because the backend lacks a required capability.
    Skipped,
    /// Not run because of a known issue.
    Ignored,
    /// Not run, for example because `--ignored` selected only ignored trials.
    NotRun,
}

/// Trial counts by status. Checks are not counted.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub(crate) struct Totals {
    passed: usize,
    failed: usize,
    skipped: usize,
    ignored: usize,
    not_run: usize,
    pending: usize,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct BackendEntry {
    name: String,
    kind: &'static str,
    profiles: Vec<&'static str>,
}

/// Why a planned trial won't run unless ignored trials are selected.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Planned {
    Run,
    Skip,
    KnownIssue,
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct TrialEntry {
    test_name: String,
    spec_id: String,
    summary: &'static str,
    source: String,
    backend: String,
    backend_kind: &'static str,
    service_profile: &'static str,
    requires: Vec<&'static str>,
    status: Status,
    /// The skip reason or known-issue text.
    reason: Option<String>,
    duration_ms: Option<f64>,
    error: Option<String>,
    /// The result file, relative to the run directory.
    result_file: Option<String>,
    #[serde(skip)]
    planned: Planned,
    #[serde(skip)]
    file_name: String,
}

impl TrialEntry {
    pub(crate) fn new(
        spec: &'static SpecDescriptor,
        backend: &dyn Backend,
        profile: ServiceProfile,
        planned: Planned,
        reason: Option<String>,
    ) -> Self {
        Self {
            test_name: spec.test_name(backend.name(), profile),
            spec_id: spec.id(),
            summary: spec.summary,
            source: format!("{}:{}", spec.file, spec.line),
            backend: backend.name().to_owned(),
            backend_kind: backend.kind(),
            service_profile: profile.name(),
            requires: spec.requires.iter().map(|c| c.name()).collect(),
            status: Status::Pending,
            reason,
            duration_ms: None,
            error: None,
            result_file: None,
            planned,
            file_name: crate::results::file_name(spec, backend.name(), profile),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub(crate) struct Manifest {
    version: u32,
    kind: &'static str,
    run_id: String,
    started_at: String,
    finished_at: Option<String>,
    status: RunStatus,
    args: Vec<String>,
    cleanup: bool,
    result_version: u32,
    backends: Vec<BackendEntry>,
    totals: Totals,
    /// Checks that aren't trials. None exist today; the field stays so the
    /// manifest format doesn't change.
    checks: [(); 0],
    trials: Vec<TrialEntry>,
}

impl Manifest {
    pub(crate) fn new(run_id: &str, args: Vec<String>, cleanup: bool) -> Self {
        Self {
            version: MANIFEST_VERSION,
            kind: MANIFEST_KIND,
            run_id: run_id.to_owned(),
            started_at: to_rfc3339(&OffsetDateTime::now_utc()),
            finished_at: None,
            status: RunStatus::Running,
            args,
            cleanup,
            result_version: RESULT_VERSION,
            backends: Vec::new(),
            totals: Totals::default(),
            checks: [],
            trials: Vec::new(),
        }
    }

    pub(crate) fn add_backend(&mut self, backend: &dyn Backend, profiles: &[ServiceProfile]) {
        self.backends.push(BackendEntry {
            name: backend.name().to_owned(),
            kind: backend.kind(),
            profiles: profiles.iter().map(|p| p.name()).collect(),
        });
    }

    /// Adds a trial. Returns its index.
    pub(crate) fn add_trial(&mut self, trial: TrialEntry) -> usize {
        self.trials.push(trial);
        self.trials.len() - 1
    }

    /// Records the outcome of the trial at `index`.
    fn record(&mut self, index: usize, result: &Result<(), String>, duration_ms: f64, dir: &Path) {
        let (status, error) = match result {
            Ok(()) => (Status::Passed, None),
            Err(e) => (Status::Failed, Some(e.clone())),
        };
        let trial = &mut self.trials[index];
        trial.status = status;
        trial.duration_ms = Some(duration_ms);
        trial.error = error;
        trial.result_file = dir
            .join(&trial.file_name)
            .exists()
            .then(|| trial.file_name.clone());
    }

    /// Marks the run finished; jobs that never ran get their final status.
    fn complete(&mut self) {
        self.finished_at = Some(to_rfc3339(&OffsetDateTime::now_utc()));
        self.status = RunStatus::Completed;
        for trial in &mut self.trials {
            if trial.status == Status::Pending {
                trial.status = match trial.planned {
                    Planned::Skip => Status::Skipped,
                    Planned::KnownIssue => Status::Ignored,
                    Planned::Run => Status::NotRun,
                };
            }
        }
    }

    fn update_totals(&mut self) {
        let mut totals = Totals::default();
        for status in self.trials.iter().map(|t| t.status) {
            *match status {
                Status::Pending => &mut totals.pending,
                Status::Passed => &mut totals.passed,
                Status::Failed => &mut totals.failed,
                Status::Skipped => &mut totals.skipped,
                Status::Ignored => &mut totals.ignored,
                Status::NotRun => &mut totals.not_run,
            } += 1;
        }
        self.totals = totals;
    }

    /// Writes the manifest into `dir`, replacing any earlier copy atomically.
    fn write(&mut self, dir: &Path) -> SpecResult {
        self.update_totals();
        std::fs::create_dir_all(dir)?;
        let json = serde_json::to_vec_pretty(self)?;
        let tmp = dir.join(format!("{MANIFEST_FILE}.tmp"));
        std::fs::write(&tmp, json)?;
        std::fs::rename(&tmp, dir.join(MANIFEST_FILE))?;
        Ok(())
    }
}

struct Live {
    dir: PathBuf,
    manifest: Mutex<Manifest>,
}

static LIVE: OnceLock<Arc<Live>> = OnceLock::new();

/// Writes the run's first manifest into `dir` and keeps it for updates.
pub(crate) fn begin(dir: PathBuf, mut manifest: Manifest) {
    if let Err(e) = manifest.write(&dir) {
        eprintln!("warning: failed to write the run manifest: {e}");
    }
    let live = Live {
        dir,
        manifest: Mutex::new(manifest),
    };
    if LIVE.set(Arc::new(live)).is_err() {
        panic!("the run manifest is started once");
    }
}

/// Records the outcome of the trial at `index` and rewrites the manifest.
pub(crate) fn record(index: usize, result: &Result<(), String>, duration_ms: f64) {
    let Some(live) = LIVE.get() else {
        return;
    };
    let mut manifest = live.manifest.lock().unwrap_or_else(|e| e.into_inner());
    manifest.record(index, result, duration_ms, &live.dir);
    if let Err(e) = manifest.write(&live.dir) {
        eprintln!("warning: failed to update the run manifest: {e}");
    }
}

/// Marks the run completed and rewrites the manifest.
pub(crate) fn finish() {
    let Some(live) = LIVE.get() else {
        return;
    };
    let mut manifest = live.manifest.lock().unwrap_or_else(|e| e.into_inner());
    manifest.complete();
    if let Err(e) = manifest.write(&live.dir) {
        eprintln!("warning: failed to finish the run manifest: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::{Manifest, Planned, Status, TrialEntry, MANIFEST_FILE};
    use crate::{backend::InMemoryBackend, runner::SpecDescriptor, ServiceProfile, SpecContext};
    use std::sync::Arc;
    use uuid::Uuid;

    static SPEC: SpecDescriptor = SpecDescriptor {
        function: "split",
        module: "spec::query",
        id: None,
        summary: "A query survives a split.",
        file: "tests/spec/query.rs",
        line: 1,
        service_profiles: &[ServiceProfile::SingleRegion],
        requires: &[],
        known_issue: None,
        params: &[],
        run: run_nothing,
    };

    fn run_nothing(_: Arc<SpecContext>) -> crate::runner::SpecFuture {
        Box::pin(async { Ok(()) })
    }

    #[test]
    fn records_outcomes_and_completes() {
        let dir = std::env::temp_dir().join(format!("cosspec-manifest-{}", Uuid::new_v4()));
        let profile = ServiceProfile::SingleRegion;
        let backend = InMemoryBackend::new(vec![profile]);
        let spec = &SPEC;
        let mut manifest = Manifest::new("run", vec!["--no-cleanup".to_owned()], false);
        manifest.add_backend(&backend, &[profile]);
        let ran = manifest.add_trial(TrialEntry::new(spec, &backend, profile, Planned::Run, None));
        manifest.add_trial(TrialEntry::new(
            spec,
            &backend,
            profile,
            Planned::Skip,
            Some("lacks it".to_owned()),
        ));
        manifest.add_trial(TrialEntry::new(spec, &backend, profile, Planned::Run, None));
        manifest.write(&dir).unwrap();

        let file = dir.join(&manifest.trials[ran].file_name);
        std::fs::write(&file, "{}").unwrap();
        manifest.record(ran, &Err("boom".to_owned()), 2.0, &dir);
        manifest.complete();
        manifest.write(&dir).unwrap();

        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(dir.join(MANIFEST_FILE)).unwrap()).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();
        assert_eq!(json["version"], 1);
        assert_eq!(json["kind"], "cosspec-run");
        assert_eq!(json["status"], "completed");
        assert_eq!(json["checks"], serde_json::json!([]));
        let trials = json["trials"].as_array().unwrap();
        assert_eq!(trials[0]["status"], "failed");
        assert_eq!(trials[0]["error"], "boom");
        assert_eq!(
            trials[0]["result_file"].as_str(),
            file.file_name().unwrap().to_str()
        );
        assert_eq!(trials[1]["status"], "skipped");
        assert_eq!(trials[1]["reason"], "lacks it");
        assert!(trials[1]["result_file"].is_null());
        assert_eq!(trials[2]["status"], "not_run");
        assert_eq!(
            json["totals"],
            serde_json::json!({ "passed": 0, "failed": 1, "skipped": 1, "ignored": 0, "not_run": 1, "pending": 0 })
        );
        assert_eq!(manifest.trials[1].status, Status::Skipped);
    }
}
