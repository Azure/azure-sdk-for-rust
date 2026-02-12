// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Latency tracking and periodic summary reporting.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

/// Collects per-operation latency measurements and error counts.
#[derive(Debug)]
pub struct Stats {
    inner: Mutex<HashMap<String, OperationStats>>,
}

/// Latency data for a single operation type within a reporting window.
#[derive(Debug, Default)]
struct OperationStats {
    latencies: Vec<Duration>,
    errors: u64,
}

/// Summary statistics for a single operation type.
pub struct Summary {
    pub name: String,
    pub count: u64,
    pub errors: u64,
    pub min: Duration,
    pub max: Duration,
    pub mean: Duration,
    pub p50: Duration,
    pub p90: Duration,
    pub p99: Duration,
}

impl Stats {
    /// Creates a new empty stats collector.
    pub fn new() -> Self {
        Self {
            inner: Mutex::new(HashMap::new()),
        }
    }

    /// Records a successful operation latency.
    pub fn record_latency(&self, operation: &str, latency: Duration) {
        let mut map = self.inner.lock().unwrap();
        map.entry(operation.to_string())
            .or_default()
            .latencies
            .push(latency);
    }

    /// Records an operation error.
    pub fn record_error(&self, operation: &str) {
        let mut map = self.inner.lock().unwrap();
        map.entry(operation.to_string()).or_default().errors += 1;
    }

    /// Drains all collected data and returns per-operation summaries.
    ///
    /// This resets the internal state so each report covers only the
    /// interval since the last drain.
    pub fn drain_summaries(&self) -> Vec<Summary> {
        let mut map = self.inner.lock().unwrap();
        let mut summaries = Vec::new();

        for (name, stats) in map.drain() {
            if stats.latencies.is_empty() && stats.errors == 0 {
                continue;
            }
            if let Some(summary) = compute_summary(name, stats) {
                summaries.push(summary);
            }
        }

        summaries.sort_by(|a, b| a.name.cmp(&b.name));
        summaries
    }
}

fn compute_summary(name: String, mut stats: OperationStats) -> Option<Summary> {
    let errors = stats.errors;

    if stats.latencies.is_empty() {
        return Some(Summary {
            name,
            count: 0,
            errors,
            min: Duration::ZERO,
            max: Duration::ZERO,
            mean: Duration::ZERO,
            p50: Duration::ZERO,
            p90: Duration::ZERO,
            p99: Duration::ZERO,
        });
    }

    stats.latencies.sort();
    let count = stats.latencies.len() as u64;
    let min = stats.latencies[0];
    let max = *stats.latencies.last().unwrap();
    let sum: Duration = stats.latencies.iter().sum();
    let mean = sum / count as u32;
    let p50 = percentile(&stats.latencies, 50.0);
    let p90 = percentile(&stats.latencies, 90.0);
    let p99 = percentile(&stats.latencies, 99.0);

    Some(Summary {
        name,
        count,
        errors,
        min,
        max,
        mean,
        p50,
        p90,
        p99,
    })
}

fn percentile(sorted: &[Duration], pct: f64) -> Duration {
    if sorted.is_empty() {
        return Duration::ZERO;
    }
    let idx = ((pct / 100.0) * (sorted.len() as f64 - 1.0)).round() as usize;
    sorted[idx.min(sorted.len() - 1)]
}

/// Prints a formatted table of operation summaries to stdout.
pub fn print_report(summaries: &[Summary]) {
    if summaries.is_empty() {
        println!("  (no operations recorded)");
        return;
    }

    println!(
        "  {:<15} {:>8} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "Operation", "Count", "Errors", "Min", "Max", "Mean", "P50", "P90", "P99"
    );
    println!("  {}", "-".repeat(103));

    for s in summaries {
        println!(
            "  {:<15} {:>8} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
            s.name,
            s.count,
            s.errors,
            format_duration(s.min),
            format_duration(s.max),
            format_duration(s.mean),
            format_duration(s.p50),
            format_duration(s.p90),
            format_duration(s.p99),
        );
    }
}

fn format_duration(d: Duration) -> String {
    let ms = d.as_secs_f64() * 1000.0;
    if ms < 1000.0 {
        format!("{:.1}ms", ms)
    } else {
        format!("{:.2}s", d.as_secs_f64())
    }
}

/// Process-level CPU and memory metrics.
pub struct ProcessMetrics {
    /// CPU usage as a percentage (may exceed 100% on multi-core).
    pub cpu_percent: f32,
    /// Resident memory in bytes.
    pub memory_bytes: u64,
}

/// Captures process-level CPU and memory metrics for the current process.
///
/// The `System` instance must be kept alive between calls for CPU usage
/// to be computed correctly (it's based on the delta between refreshes).
pub fn refresh_process_metrics(sys: &mut System) -> Option<ProcessMetrics> {
    let pid = sysinfo::get_current_pid().ok()?;
    let refresh = ProcessRefreshKind::nothing().with_cpu().with_memory();
    sys.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), true, refresh);
    let proc = sys.process(pid)?;
    Some(ProcessMetrics {
        cpu_percent: proc.cpu_usage(),
        memory_bytes: proc.memory(),
    })
}

fn format_bytes(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;
    if bytes >= GB {
        format!("{:.1} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.1} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.1} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Prints process-level CPU and memory metrics.
pub fn print_process_metrics(metrics: &ProcessMetrics) {
    println!(
        "  Process: CPU {:.1}%, Memory {}",
        metrics.cpu_percent,
        format_bytes(metrics.memory_bytes),
    );
}

#[cfg(test)]
mod tests {
    use super::Stats;
    use std::time::Duration;

    #[test]
    fn record_and_drain() {
        let stats = Stats::new();
        stats.record_latency("read", Duration::from_millis(10));
        stats.record_latency("read", Duration::from_millis(20));
        stats.record_latency("read", Duration::from_millis(30));
        stats.record_error("read");

        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].count, 3);
        assert_eq!(summaries[0].errors, 1);
        assert_eq!(summaries[0].min, Duration::from_millis(10));
        assert_eq!(summaries[0].max, Duration::from_millis(30));

        // After drain, should be empty
        let summaries = stats.drain_summaries();
        assert!(summaries.is_empty());
    }

    #[test]
    fn errors_only() {
        let stats = Stats::new();
        stats.record_error("upsert");
        stats.record_error("upsert");

        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].count, 0);
        assert_eq!(summaries[0].errors, 2);
    }
}
