// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
// cspell:ignore logfile

//! Console output for spec runs: a tree grouped by backend, then spec.
//!
//! ```text
//! inmemory
//!   query.continuation_survives_split
//!     SingleRegion       ok
//!     MultiRead          ok
//! emulator
//!   query.continuation_survives_split
//!     SingleRegion       skipped (emulator lacks SimpleSplitMerge)
//! ```
//!
//! Statuses line up in one column, sized from every label before trials start.
//!
//! Trials run in parallel, but each spec's group prints once all of its trials
//! and every group before it have finished, so the output is in a stable order.
//! Machine-readable modes (`--list`, `--format json` or `terse`, and `--quiet`)
//! use `libtest-mimic`'s output unchanged, so IDEs and test runners keep working.

use libtest_mimic::{Arguments, ColorSetting, Failed, FormatSetting, Trial};
use std::{
    collections::VecDeque,
    io::IsTerminal,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{mpsc, Mutex},
    thread,
    time::{Duration, Instant},
};

/// Where a job's result appears in the tree.
/// One service profile of one spec on one backend.
pub(crate) struct Place {
    pub backend: String,
    pub spec: String,
    pub profile: String,
}

/// Why a job is ignored unless `--ignored` or `--include-ignored` is passed.
pub(crate) enum Ignore {
    /// The backend lacks a capability; the note says which.
    Skip(String),
    /// The spec documents a known SDK issue.
    KnownIssue,
}

/// One trial to run.
pub(crate) struct Job {
    pub name: String,
    pub place: Place,
    pub ignore: Option<Ignore>,
    pub run: Box<dyn FnOnce() -> Result<(), Failed> + Send>,
}

impl Job {
    fn kind(&self) -> &'static str {
        match self.ignore {
            None => "",
            Some(Ignore::Skip(_)) => "skip",
            Some(Ignore::KnownIssue) => "known-issue",
        }
    }

    fn into_trial(self) -> Trial {
        let kind = self.kind();
        let ignored = self.ignore.is_some();
        Trial::test(self.name, self.run)
            .with_kind(kind)
            .with_ignored_flag(ignored)
    }

    fn probe(&self) -> Trial {
        Trial::test(self.name.clone(), || Ok(()))
            .with_kind(self.kind())
            .with_ignored_flag(self.ignore.is_some())
    }
}

/// Whether `args` select `job`, ignoring whether it is ignored.
pub(crate) fn is_selected(args: &Arguments, job: &Job) -> bool {
    !args.is_filtered_out(&job.probe())
}

enum Outcome {
    Passed(Duration),
    Failed(String, Duration),
    Ignored,
}

/// Runs `jobs` and prints their results. Returns `true` if none failed.
///
/// Jobs for one backend and spec must be adjacent; they form one group.
pub(crate) fn run(args: &Arguments, jobs: Vec<Job>) -> bool {
    let pretty = matches!(args.format, None | Some(FormatSetting::Pretty));
    if args.list || args.quiet || args.logfile.is_some() || !pretty {
        let trials = jobs.into_iter().map(Job::into_trial).collect();
        return !libtest_mimic::run(args, trials).has_failed();
    }

    let started = Instant::now();
    let before = jobs.len();
    let jobs: Vec<Job> = jobs
        .into_iter()
        .filter(|job| !args.is_filtered_out(&job.probe()))
        .collect();
    let filtered_out = before - jobs.len();
    let run_ignored = args.ignored || args.include_ignored;
    let threads = args
        .test_threads
        .or_else(|| thread::available_parallelism().ok().map(Into::into))
        .unwrap_or(1)
        .max(1);

    crate::results::quiet_panics();
    println!("\nrunning {} tests", jobs.len());
    let mut printer = Printer::new(args);
    let (tx, rx) = mpsc::channel();
    let mut queue = VecDeque::new();
    for (index, job) in jobs.into_iter().enumerate() {
        let ignored = job.ignore.is_some() && !run_ignored;
        printer.add(job.name, job.place, job.ignore);
        if ignored {
            tx.send((index, Outcome::Ignored))
                .expect("receiver is alive");
        } else {
            queue.push_back((index, job.run));
        }
    }
    let workers = threads.min(queue.len());
    let queue = Mutex::new(queue);
    thread::scope(|scope| {
        for _ in 0..workers {
            let tx = tx.clone();
            let queue = &queue;
            scope.spawn(move || loop {
                let Some((index, run)) = queue.lock().expect("queue lock").pop_front() else {
                    break;
                };
                let start = Instant::now();
                let outcome = match catch_unwind(AssertUnwindSafe(run)) {
                    Ok(Ok(())) => Outcome::Passed(start.elapsed()),
                    Ok(Err(failed)) => Outcome::Failed(
                        failed.message().unwrap_or_default().to_owned(),
                        start.elapsed(),
                    ),
                    Err(payload) => Outcome::Failed(
                        crate::runner::panic_message(payload.as_ref()),
                        start.elapsed(),
                    ),
                };
                if tx.send((index, outcome)).is_err() {
                    break;
                }
            });
        }
        drop(tx);
        for (index, outcome) in rx {
            printer.record(index, outcome);
        }
    });
    printer.finish(filtered_out, started.elapsed())
}

const LEAF_INDENT: &str = "    ";
const COLUMN_GAP: &str = "  ";

struct Entry {
    name: String,
    place: Place,
    ignore: Option<Ignore>,
    outcome: Option<Outcome>,
}

struct Printer {
    color: bool,
    entries: Vec<Entry>,
    /// Index of the first entry not yet printed.
    cursor: usize,
    /// The width of the widest label that has a status, including its indent.
    width: usize,
    backend: Option<String>,
}

impl Printer {
    fn new(args: &Arguments) -> Self {
        let color = match args.color {
            Some(ColorSetting::Always) => true,
            Some(ColorSetting::Never) => false,
            Some(ColorSetting::Auto) | None => std::io::stdout().is_terminal(),
        };
        Self {
            color,
            entries: Vec::new(),
            cursor: 0,
            width: 0,
            backend: None,
        }
    }

    fn add(&mut self, name: String, place: Place, ignore: Option<Ignore>) {
        self.width = self
            .width
            .max(LEAF_INDENT.len() + place.profile.chars().count());
        self.entries.push(Entry {
            name,
            place,
            ignore,
            outcome: None,
        });
    }

    fn record(&mut self, index: usize, outcome: Outcome) {
        self.entries[index].outcome = Some(outcome);
        while self.cursor < self.entries.len() {
            let end = self.group_end(self.cursor);
            if self.entries[self.cursor..end]
                .iter()
                .any(|e| e.outcome.is_none())
            {
                break;
            }
            self.print_group(self.cursor, end);
            self.cursor = end;
        }
    }

    /// The end of the group that starts at `start`: every adjacent entry of
    /// the same backend and spec.
    fn group_end(&self, start: usize) -> usize {
        let Place { backend, spec, .. } = &self.entries[start].place;
        start
            + self.entries[start..]
                .iter()
                .take_while(|e| &e.place.backend == backend && &e.place.spec == spec)
                .count()
    }

    fn print_group(&mut self, start: usize, end: usize) {
        let mut lines = String::new();
        let Place { backend, spec, .. } = &self.entries[start].place;
        if self.backend.as_ref() != Some(backend) {
            lines.push_str(&format!("{}\n", self.paint("1", backend)));
            self.backend = Some(backend.clone());
        }
        lines.push_str(&format!("  {spec}\n"));
        for entry in &self.entries[start..end] {
            lines.push_str(&self.row(LEAF_INDENT, &entry.place.profile, entry));
        }
        print!("{lines}");
    }

    /// One status row, with the status in the aligned column.
    fn row(&self, indent: &str, label: &str, entry: &Entry) -> String {
        let pad = self.width - indent.len() - label.chars().count();
        format!(
            "{indent}{label}{:pad$}{COLUMN_GAP}{}\n",
            "",
            self.status(entry)
        )
    }

    fn status(&self, entry: &Entry) -> String {
        let timing = |elapsed: &Duration| {
            if *elapsed >= Duration::from_secs(1) {
                format!(" ({:.1}s)", elapsed.as_secs_f64())
            } else {
                String::new()
            }
        };
        match (&entry.outcome, &entry.ignore) {
            (Some(Outcome::Passed(elapsed)), _) => {
                format!("{}{}", self.paint("32", "ok"), timing(elapsed))
            }
            (Some(Outcome::Failed(_, elapsed)), _) => {
                format!("{}{}", self.paint("31", "FAILED"), timing(elapsed))
            }
            (_, Some(Ignore::Skip(note))) => format!("{} ({note})", self.paint("33", "skipped")),
            (_, Some(Ignore::KnownIssue)) => {
                format!("{} (known issue)", self.paint("33", "ignored"))
            }
            (_, None) => self.paint("33", "ignored"),
        }
    }

    fn paint(&self, code: &str, text: &str) -> String {
        if self.color {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_owned()
        }
    }

    fn finish(&self, filtered_out: usize, elapsed: Duration) -> bool {
        let (mut passed, mut ignored) = (0, 0);
        let mut failures = Vec::new();
        for entry in &self.entries {
            match &entry.outcome {
                Some(Outcome::Passed(_)) => passed += 1,
                Some(Outcome::Failed(message, _)) => failures.push((&entry.name, message)),
                Some(Outcome::Ignored) | None => ignored += 1,
            }
        }
        if !failures.is_empty() {
            println!("\nfailures:\n");
            for (name, message) in &failures {
                println!("---- {name} ----\n{message}\n");
            }
            println!("\nfailures:");
            for (name, _) in &failures {
                println!("    {name}");
            }
        }
        let result = if failures.is_empty() {
            self.paint("32", "ok")
        } else {
            self.paint("31", "FAILED")
        };
        println!(
            "\ntest result: {result}. {passed} passed; {} failed; {ignored} ignored; 0 measured; {filtered_out} filtered out; finished in {:.2}s\n",
            failures.len(),
            elapsed.as_secs_f64(),
        );
        failures.is_empty()
    }
}
