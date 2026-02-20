// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Latency tracking and periodic summary reporting.
//!
//! Uses reservoir sampling (Algorithm R) and per-operation sharded mutexes to
//! keep memory bounded and lock contention low regardless of throughput.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use rand::RngExt;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

/// Maximum number of latency samples retained per operation per interval.
///
/// Reservoir sampling guarantees a uniform random subset of all observations,
/// giving accurate percentile estimates with O(1) memory.
const RESERVOIR_SIZE: usize = 10_000;

/// Collects per-operation latency measurements and error counts.
///
/// Each operation name gets its own [`Mutex`]-protected [`OperationStats`],
/// so workers recording different operations never contend with each other.
pub struct Stats {
    shards: HashMap<String, Mutex<OperationStats>>,
}

/// Latency data for a single operation type within a reporting window.
///
/// Instead of storing every latency, we use reservoir sampling (Algorithm R)
/// to keep at most [`RESERVOIR_SIZE`] samples. Exact count, min, max, and sum
/// are tracked separately so mean is always precise.
struct OperationStats {
    /// Fixed-size sample reservoir for percentile estimation.
    reservoir: Vec<Duration>,
    /// Total number of successful operations observed (may exceed reservoir size).
    count: u64,
    /// Running minimum latency.
    min: Duration,
    /// Running maximum latency.
    max: Duration,
    /// Running sum for exact mean calculation.
    sum: Duration,
    /// Number of failed operations.
    errors: u64,
}

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            reservoir: Vec::with_capacity(RESERVOIR_SIZE),
            count: 0,
            min: Duration::MAX,
            max: Duration::ZERO,
            sum: Duration::ZERO,
            errors: 0,
        }
    }
}

impl OperationStats {
    /// Records a latency sample using reservoir sampling (Algorithm R).
    fn record(&mut self, latency: Duration) {
        self.count += 1;
        self.sum += latency;
        if latency < self.min {
            self.min = latency;
        }
        if latency > self.max {
            self.max = latency;
        }

        if self.reservoir.len() < RESERVOIR_SIZE {
            self.reservoir.push(latency);
        } else {
            // Algorithm R: replace a random element with probability RESERVOIR_SIZE / count
            let j = rand::rng().random_range(0..self.count as usize);
            if j < RESERVOIR_SIZE {
                self.reservoir[j] = latency;
            }
        }
    }

    /// Resets all counters and returns a fresh default instance.
    fn drain(&mut self) -> Self {
        std::mem::take(self)
    }
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
    /// Creates a new stats collector pre-sharded for the given operation names.
    pub fn new(operation_names: &[&str]) -> Self {
        let mut shards = HashMap::with_capacity(operation_names.len());
        for name in operation_names {
            shards.insert(name.to_string(), Mutex::new(OperationStats::default()));
        }
        Self { shards }
    }

    /// Records a successful operation latency.
    ///
    /// Only locks the mutex for this specific operation — no cross-operation
    /// contention.
    pub fn record_latency(&self, operation: &str, latency: Duration) {
        if let Some(m) = self.shards.get(operation) {
            m.lock().unwrap().record(latency);
        }
    }

    /// Records an operation error.
    pub fn record_error(&self, operation: &str) {
        if let Some(m) = self.shards.get(operation) {
            m.lock().unwrap().errors += 1;
        }
    }

    /// Drains all collected data and returns per-operation summaries.
    ///
    /// Each shard is locked, drained, and released independently. This resets
    /// the internal state so each report covers only the interval since the
    /// last drain.
    pub fn drain_summaries(&self) -> Vec<Summary> {
        let mut summaries = Vec::with_capacity(self.shards.len());

        for (name, shard) in &self.shards {
            let drained = shard.lock().unwrap().drain();
            if drained.count == 0 && drained.errors == 0 {
                continue;
            }
            summaries.push(compute_summary(name.clone(), drained));
        }

        summaries.sort_by(|a, b| a.name.cmp(&b.name));
        summaries
    }
}

fn compute_summary(name: String, mut stats: OperationStats) -> Summary {
    let errors = stats.errors;

    if stats.count == 0 {
        return Summary {
            name,
            count: 0,
            errors,
            min: Duration::ZERO,
            max: Duration::ZERO,
            mean: Duration::ZERO,
            p50: Duration::ZERO,
            p90: Duration::ZERO,
            p99: Duration::ZERO,
        };
    }

    stats.reservoir.sort();
    let mean = stats.sum / stats.count as u32;
    let p50 = percentile(&stats.reservoir, 50.0);
    let p90 = percentile(&stats.reservoir, 90.0);
    let p99 = percentile(&stats.reservoir, 99.0);

    Summary {
        name,
        count: stats.count,
        errors,
        min: stats.min,
        max: stats.max,
        mean,
        p50,
        p90,
        p99,
    }
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

/// Process-level and system-level CPU and memory metrics.
pub struct ProcessMetrics {
    /// Process CPU usage as a percentage (may exceed 100% on multi-core).
    pub cpu_percent: f32,
    /// Process resident memory in bytes.
    pub memory_bytes: u64,
    /// System-wide CPU usage as a percentage.
    pub system_cpu_percent: f32,
    /// Total physical memory in bytes.
    pub system_total_memory_bytes: u64,
    /// Used physical memory in bytes.
    pub system_used_memory_bytes: u64,
}

/// Captures process-level and system-level CPU and memory metrics.
///
/// The `System` instance must be kept alive between calls for CPU usage
/// to be computed correctly (it's based on the delta between refreshes).
pub fn refresh_process_metrics(sys: &mut System) -> Option<ProcessMetrics> {
    let pid = sysinfo::get_current_pid().ok()?;
    let refresh = ProcessRefreshKind::nothing().with_cpu().with_memory();
    sys.refresh_processes_specifics(ProcessesToUpdate::Some(&[pid]), true, refresh);
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    let proc = sys.process(pid)?;
    Some(ProcessMetrics {
        cpu_percent: proc.cpu_usage(),
        memory_bytes: proc.memory(),
        system_cpu_percent: sys.global_cpu_usage(),
        system_total_memory_bytes: sys.total_memory(),
        system_used_memory_bytes: sys.used_memory(),
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

/// Prints process-level and system-level CPU and memory metrics.
pub fn print_process_metrics(metrics: &ProcessMetrics) {
    println!(
        "  Process: CPU {:.1}%, Memory {}",
        metrics.cpu_percent,
        format_bytes(metrics.memory_bytes),
    );
    println!(
        "  System:  CPU {:.1}%, Memory {}/{}",
        metrics.system_cpu_percent,
        format_bytes(metrics.system_used_memory_bytes),
        format_bytes(metrics.system_total_memory_bytes),
    );
}

#[cfg(test)]
mod tests {
    use super::Stats;
    use std::time::Duration;

    #[test]
    fn record_and_drain() {
        let stats = Stats::new(&["read"]);
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
        let stats = Stats::new(&["upsert"]);
        stats.record_error("upsert");
        stats.record_error("upsert");

        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].count, 0);
        assert_eq!(summaries[0].errors, 2);
    }

    #[test]
    fn reservoir_caps_memory() {
        let stats = Stats::new(&["write"]);
        // Record more samples than RESERVOIR_SIZE
        for i in 0..20_000u64 {
            stats.record_latency("write", Duration::from_micros(i));
        }
        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].count, 20_000);
        assert_eq!(summaries[0].min, Duration::from_micros(0));
        assert_eq!(summaries[0].max, Duration::from_micros(19_999));
    }

    #[test]
    fn unknown_operation_ignored() {
        let stats = Stats::new(&["read"]);
        stats.record_latency("unknown", Duration::from_millis(10));
        stats.record_error("unknown");
        let summaries = stats.drain_summaries();
        assert!(summaries.is_empty());
    }
}
