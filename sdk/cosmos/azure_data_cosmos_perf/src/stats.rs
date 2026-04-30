// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Latency tracking and periodic summary reporting.
//!
//! Uses [`hdrhistogram`] for percentile estimation with fixed memory and
//! per-operation sharded mutexes to keep lock contention low regardless of
//! throughput.

use std::collections::HashMap;
use std::sync::Mutex;
use std::time::Duration;

use hdrhistogram::Histogram;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System};

/// Collects per-operation latency measurements and error counts.
///
/// Each operation name gets its own [`Mutex`]-protected [`OperationStats`],
/// so workers recording different operations never contend with each other.
pub struct Stats {
    shards: HashMap<String, Mutex<OperationStats>>,
}

/// Latency data for a single operation type within a reporting window.
///
/// Uses an [`hdrhistogram::Histogram`] for percentile estimation with constant
/// memory. Exact count, min, max, and sum are tracked separately so the mean
/// is always precise. Two parallel histograms are tracked: one for the
/// client-observed wall-clock latency, and one for the server-reported
/// processing duration parsed from `x-ms-request-duration-ms`.
struct OperationStats {
    /// HdrHistogram for percentile estimation (values in microseconds).
    histogram: Histogram<u64>,
    /// Total number of successful operations observed.
    count: u64,
    /// Running minimum latency.
    min: Duration,
    /// Running maximum latency.
    max: Duration,
    /// Running sum for exact mean calculation.
    sum: Duration,
    /// Number of failed operations.
    errors: u64,
    /// Histogram for backend-reported request duration.
    backend_histogram: Histogram<u64>,
    /// Number of operations that contributed a backend duration sample.
    backend_count: u64,
    backend_min: Duration,
    backend_max: Duration,
    backend_sum: Duration,
}

/// Upper bound for the histogram: 1 hour in microseconds.
const MAX_LATENCY_US: u64 = 3_600_000_000;

impl Default for OperationStats {
    fn default() -> Self {
        Self {
            // 1µs–1h range, 3 significant figures of precision.
            histogram: Histogram::new_with_bounds(1, MAX_LATENCY_US, 3)
                .expect("valid histogram bounds"),
            count: 0,
            min: Duration::MAX,
            max: Duration::ZERO,
            sum: Duration::ZERO,
            errors: 0,
            backend_histogram: Histogram::new_with_bounds(1, MAX_LATENCY_US, 3)
                .expect("valid histogram bounds"),
            backend_count: 0,
            backend_min: Duration::MAX,
            backend_max: Duration::ZERO,
            backend_sum: Duration::ZERO,
        }
    }
}

impl OperationStats {
    /// Records a client-observed latency sample, plus an optional
    /// server-reported backend duration when available.
    fn record(&mut self, latency: Duration, backend: Option<Duration>) {
        self.count += 1;
        self.sum += latency;
        if latency < self.min {
            self.min = latency;
        }
        if latency > self.max {
            self.max = latency;
        }

        let micros = latency.as_micros().min(u64::MAX as u128) as u64;
        // Clamp to histogram bounds; values above MAX_LATENCY_US are recorded
        // at the max and still counted.
        let _ = self.histogram.record(micros.clamp(1, MAX_LATENCY_US));

        if let Some(b) = backend {
            self.backend_count += 1;
            self.backend_sum += b;
            if b < self.backend_min {
                self.backend_min = b;
            }
            if b > self.backend_max {
                self.backend_max = b;
            }
            let bm = b.as_micros().min(u64::MAX as u128) as u64;
            let _ = self.backend_histogram.record(bm.clamp(1, MAX_LATENCY_US));
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
    /// Server-reported processing latency (`x-ms-request-duration-ms`).
    /// `None` when the interval contained zero samples carrying the header.
    pub backend_min: Option<Duration>,
    pub backend_max: Option<Duration>,
    pub backend_mean: Option<Duration>,
    pub backend_p50: Option<Duration>,
    pub backend_p90: Option<Duration>,
    pub backend_p99: Option<Duration>,
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
    /// contention. Pass `backend = Some(d)` when the underlying response
    /// included an `x-ms-request-duration-ms` header.
    pub fn record_latency(&self, operation: &str, latency: Duration, backend: Option<Duration>) {
        if let Some(m) = self.shards.get(operation) {
            m.lock().unwrap().record(latency, backend);
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

fn compute_summary(name: String, stats: OperationStats) -> Summary {
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
            backend_min: None,
            backend_max: None,
            backend_mean: None,
            backend_p50: None,
            backend_p90: None,
            backend_p99: None,
        };
    }

    let mean = Duration::from_secs_f64(stats.sum.as_secs_f64() / stats.count as f64);
    let p50 = Duration::from_micros(stats.histogram.value_at_quantile(0.50));
    let p90 = Duration::from_micros(stats.histogram.value_at_quantile(0.90));
    let p99 = Duration::from_micros(stats.histogram.value_at_quantile(0.99));

    let (backend_min, backend_max, backend_mean, backend_p50, backend_p90, backend_p99) =
        if stats.backend_count > 0 {
            let backend_mean_dur = Duration::from_secs_f64(
                stats.backend_sum.as_secs_f64() / stats.backend_count as f64,
            );
            let bp50 = Duration::from_micros(stats.backend_histogram.value_at_quantile(0.50));
            let bp90 = Duration::from_micros(stats.backend_histogram.value_at_quantile(0.90));
            let bp99 = Duration::from_micros(stats.backend_histogram.value_at_quantile(0.99));
            (
                Some(stats.backend_min),
                Some(stats.backend_max),
                Some(backend_mean_dur),
                Some(bp50),
                Some(bp90),
                Some(bp99),
            )
        } else {
            (None, None, None, None, None, None)
        };

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
        backend_min,
        backend_max,
        backend_mean,
        backend_p50,
        backend_p90,
        backend_p99,
    }
}

/// Prints a formatted table of operation summaries to stdout.
pub fn print_report(summaries: &[Summary]) {
    if summaries.is_empty() {
        println!("  (no operations recorded)");
        return;
    }

    println!(
        "  {:<15} {:>8} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
        "Operation", "Count", "Errors", "Min", "Max", "Mean", "P50", "P90", "P99", "BackendP99"
    );
    println!("  {}", "-".repeat(114));

    for s in summaries {
        let backend_p99 = s
            .backend_p99
            .map(format_duration)
            .unwrap_or_else(|| "—".to_string());
        println!(
            "  {:<15} {:>8} {:>8} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10} {:>10}",
            s.name,
            s.count,
            s.errors,
            format_duration(s.min),
            format_duration(s.max),
            format_duration(s.mean),
            format_duration(s.p50),
            format_duration(s.p90),
            format_duration(s.p99),
            backend_p99,
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
    /// Cgroup CPU usage as a percentage of the cgroup's CPU quota.
    /// This matches what `kubectl top` reports. Only available on cgroupv2.
    pub cgroup_cpu_percent: Option<f32>,
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
        cgroup_cpu_percent: read_cgroup_cpu_percent(),
    })
}

/// Previous cgroup usage snapshot for delta computation.
static PREV_CGROUP_USAGE: std::sync::Mutex<Option<(u64, std::time::Instant)>> =
    std::sync::Mutex::new(None);

/// Reads cgroupv2 CPU usage and computes utilization as a percentage of the
/// cgroup's CPU quota. This matches what `kubectl top pods` reports.
fn read_cgroup_cpu_percent() -> Option<f32> {
    use std::fs;
    use std::time::Instant;

    // Read current usage_usec from cgroup
    let stat = fs::read_to_string("/sys/fs/cgroup/cpu.stat").ok()?;
    let usage_usec: u64 = stat
        .lines()
        .find(|l| l.starts_with("usage_usec"))
        .and_then(|l| l.split_whitespace().nth(1))
        .and_then(|v| v.parse().ok())?;

    // Read quota: "max 100000" means unlimited, "400000 100000" means 4 cores
    let max_content = fs::read_to_string("/sys/fs/cgroup/cpu.max").ok()?;
    let mut parts = max_content.split_whitespace();
    let quota_str = parts.next()?;
    if quota_str == "max" {
        return None; // unlimited, can't compute percentage
    }
    let quota_usec: u64 = quota_str.parse().ok()?;
    let period_usec: u64 = parts.next().and_then(|v| v.parse().ok()).unwrap_or(100_000);

    if period_usec == 0 {
        return None;
    }
    // cores_allocated = quota / period (e.g., 400000/100000 = 4.0 cores)
    let cores = quota_usec as f64 / period_usec as f64;
    if cores <= 0.0 {
        return None;
    }

    let now = Instant::now();
    let mut prev = PREV_CGROUP_USAGE.lock().ok()?;
    let result = if let Some((prev_usage, prev_time)) = *prev {
        let delta_usec = usage_usec.saturating_sub(prev_usage);
        let delta_time = now.duration_since(prev_time);
        let wall_usec = delta_time.as_micros() as f64;
        if wall_usec > 0.0 {
            // (cpu_usec_used / wall_usec) / cores * 100
            Some((delta_usec as f64 / wall_usec / cores * 100.0) as f32)
        } else {
            None
        }
    } else {
        None // first call, no delta available yet
    };
    *prev = Some((usage_usec, now));
    result
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
    if let Some(cgroup) = metrics.cgroup_cpu_percent {
        println!("  Cgroup:  CPU {:.1}% (kubectl-equivalent)", cgroup);
    }
}

#[cfg(test)]
mod tests {
    use super::Stats;
    use std::time::Duration;

    #[test]
    fn record_and_drain() {
        let stats = Stats::new(&["read"]);
        stats.record_latency("read", Duration::from_millis(10), None);
        stats.record_latency("read", Duration::from_millis(20), None);
        stats.record_latency("read", Duration::from_millis(30), None);
        stats.record_error("read");

        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        assert_eq!(summaries[0].count, 3);
        assert_eq!(summaries[0].errors, 1);
        assert_eq!(summaries[0].min, Duration::from_millis(10));
        assert_eq!(summaries[0].max, Duration::from_millis(30));
        assert!(summaries[0].backend_p99.is_none());

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
    fn percentile_accuracy() {
        let stats = Stats::new(&["write"]);
        // Record 1ms through 100ms — p50 ≈ 50ms, p99 ≈ 99ms
        for i in 1..=100u64 {
            stats.record_latency("write", Duration::from_millis(i), None);
        }
        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        let s = &summaries[0];
        assert_eq!(s.count, 100);
        assert_eq!(s.min, Duration::from_millis(1));
        assert_eq!(s.max, Duration::from_millis(100));

        // hdrhistogram values are within ±0.1% of the true value at 3
        // significant figures, so allow a small tolerance.
        let p50_ms = s.p50.as_millis();
        assert!((49..=51).contains(&p50_ms), "p50 was {p50_ms}ms");
        let p99_ms = s.p99.as_millis();
        assert!((98..=100).contains(&p99_ms), "p99 was {p99_ms}ms");
    }

    #[test]
    fn high_volume_recording() {
        let stats = Stats::new(&["write"]);
        for i in 0..20_000u64 {
            stats.record_latency("write", Duration::from_micros(i), None);
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
        stats.record_latency("unknown", Duration::from_millis(10), None);
        stats.record_error("unknown");
        let summaries = stats.drain_summaries();
        assert!(summaries.is_empty());
    }

    #[test]
    fn backend_durations_aggregate_separately_from_client() {
        let stats = Stats::new(&["read"]);
        // Mix of samples: some carry a backend duration, some don't.
        // Client latencies span 10..=30ms, backend latencies span 5..=15ms
        // for the subset that have one — verifies the two histograms are
        // independent.
        stats.record_latency(
            "read",
            Duration::from_millis(10),
            Some(Duration::from_millis(5)),
        );
        stats.record_latency(
            "read",
            Duration::from_millis(20),
            Some(Duration::from_millis(10)),
        );
        stats.record_latency(
            "read",
            Duration::from_millis(30),
            Some(Duration::from_millis(15)),
        );
        stats.record_latency("read", Duration::from_millis(25), None);

        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        let s = &summaries[0];

        assert_eq!(s.count, 4, "all 4 client samples count");
        assert_eq!(s.min, Duration::from_millis(10));
        assert_eq!(s.max, Duration::from_millis(30));

        let back_min = s.backend_min.expect("3 backend samples were recorded");
        let back_max = s.backend_max.expect("3 backend samples were recorded");
        let bp99 = s.backend_p99.expect("3 backend samples were recorded");
        assert_eq!(back_min, Duration::from_millis(5));
        assert_eq!(back_max, Duration::from_millis(15));
        // p99 of {5,10,15} is the max (15ms), within hdrhistogram tolerance.
        let bp99_ms = bp99.as_millis();
        assert!((14..=16).contains(&bp99_ms), "backend p99 was {bp99_ms}ms");
    }

    #[test]
    fn backend_summary_is_none_when_no_samples() {
        let stats = Stats::new(&["read"]);
        for i in 1..=10u64 {
            stats.record_latency("read", Duration::from_millis(i), None);
        }
        let summaries = stats.drain_summaries();
        assert_eq!(summaries.len(), 1);
        let s = &summaries[0];
        assert!(s.backend_min.is_none());
        assert!(s.backend_max.is_none());
        assert!(s.backend_mean.is_none());
        assert!(s.backend_p50.is_none());
        assert!(s.backend_p90.is_none());
        assert!(s.backend_p99.is_none());
    }
}
