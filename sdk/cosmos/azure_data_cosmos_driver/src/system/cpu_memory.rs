// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! CPU and memory monitoring with historical snapshots.

use std::{
    cmp::Ordering,
    collections::VecDeque,
    fmt,
    hash::{Hash, Hasher},
    sync::{Arc, OnceLock, RwLock, Weak},
    thread,
    time::{Duration, Instant},
};

#[cfg(target_os = "linux")]
use std::fs;

/// Default interval between CPU/memory samples (5 seconds).
const DEFAULT_REFRESH_INTERVAL: Duration = Duration::from_secs(5);

/// Number of historical samples to retain.
const HISTORY_LENGTH: usize = 6;

/// CPU load threshold percentage for considering the system overloaded.
const CPU_OVERLOAD_THRESHOLD: CpuUsage = CpuUsage(90.0);

/// Global singleton for CPU/memory monitoring.
static CPU_MEMORY_MONITOR: OnceLock<Arc<CpuMemoryMonitorInner>> = OnceLock::new();

/// CPU usage percentage as a normalized `f64` newtype.
///
/// Uses the same normalization pattern as [`RequestCharge`](crate::models::RequestCharge):
/// NaN is normalized to `0.0` and negative zero becomes positive zero, which
/// allows this type to implement [`Eq`], [`Hash`], and [`Ord`].
///
/// Valid values range from `0.0` to `100.0`.
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::system::CpuUsage;
///
/// let usage = CpuUsage::new(42.5);
/// assert_eq!(usage.value(), 42.5);
///
/// // NaN normalises to 0.0
/// let nan = CpuUsage::new(f64::NAN);
/// assert_eq!(nan.value(), 0.0);
/// ```
#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct CpuUsage(f64);

impl CpuUsage {
    /// Creates a new `CpuUsage` from a raw `f64` percentage.
    ///
    /// NaN and negative zero are normalized to `0.0`. After normalization, the
    /// value must be between `0.0` and `100.0` (inclusive).
    ///
    /// # Panics
    ///
    /// Panics if the normalized value is not between 0.0 and 100.0.
    pub(crate) fn new(value: f64) -> Self {
        let normalized = Self::normalize(value);
        assert!(
            (0.0..=100.0).contains(&normalized),
            "CpuUsage must be between 0.0 and 100.0 after normalization, got {}",
            normalized
        );
        Self(normalized)
    }

    /// Returns the raw `f64` percentage.
    pub(crate) const fn value(self) -> f64 {
        self.0
    }

    /// Normalizes an `f64` value: NaN becomes `0.0`, and `-0.0` becomes `+0.0`.
    fn normalize(value: f64) -> f64 {
        if value.is_nan() || value == 0.0 {
            0.0
        } else {
            value
        }
    }

    /// Returns canonical bits for hashing.
    ///
    /// After normalization, NaN and -0.0 are impossible, so `to_bits()` is
    /// consistent with our [`PartialEq`] implementation.
    fn canonical_bits(self) -> u64 {
        self.0.to_bits()
    }
}

impl fmt::Display for CpuUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.1}%", self.0)
    }
}

impl PartialEq for CpuUsage {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for CpuUsage {}

impl PartialOrd for CpuUsage {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CpuUsage {
    fn cmp(&self, other: &Self) -> Ordering {
        // After normalization NaN is impossible, so total_cmp is safe.
        self.0.total_cmp(&other.0)
    }
}

impl Hash for CpuUsage {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical_bits().hash(state);
    }
}

impl From<f64> for CpuUsage {
    fn from(value: f64) -> Self {
        Self::new(value)
    }
}

impl From<CpuUsage> for f64 {
    fn from(usage: CpuUsage) -> Self {
        usage.0
    }
}

/// A single CPU load measurement at a point in time.
///
/// Pairs a [`CpuUsage`] percentage with the [`Instant`] when the sample was
/// taken.
///
/// # Panics
///
/// [`CpuLoad::new`] panics if the **normalized** value is outside the range `0.0..=100.0`
/// (see [`CpuUsage::new`]).
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct CpuLoad {
    /// When this measurement was taken.
    timestamp: Instant,
    /// CPU usage percentage.
    value: CpuUsage,
}

impl CpuLoad {
    /// Creates a new CPU load measurement.
    ///
    /// The `value` is wrapped in [`CpuUsage`], which normalizes NaN and
    /// negative zero to `0.0` and panics if the result is outside `0.0..=100.0`.
    pub(crate) fn new(timestamp: Instant, value: f64) -> Self {
        Self {
            timestamp,
            value: CpuUsage::new(value),
        }
    }

    /// Returns when this measurement was taken.
    pub(crate) fn timestamp(&self) -> Instant {
        self.timestamp
    }

    /// Returns the CPU usage percentage.
    pub(crate) fn value(&self) -> CpuUsage {
        self.value
    }
}

impl fmt::Display for CpuLoad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({})", self.value)
    }
}

/// A single memory measurement at a point in time.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct MemoryUsage {
    /// When this measurement was taken.
    pub(crate) timestamp: Instant,
    /// Available memory in megabytes.
    pub(crate) available_mb: u64,
}

/// Historical CPU and memory usage data.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) struct CpuMemoryHistory {
    /// Historical CPU load samples (oldest first).
    cpu_samples: Vec<CpuLoad>,
    /// Historical memory samples (oldest first).
    memory_samples: Vec<MemoryUsage>,
    /// The interval between samples.
    refresh_interval: Duration,
}

impl CpuMemoryHistory {
    /// Returns the CPU load samples.
    pub(crate) fn cpu_samples(&self) -> &[CpuLoad] {
        &self.cpu_samples
    }

    /// Returns the memory usage samples.
    pub(crate) fn memory_samples(&self) -> &[MemoryUsage] {
        &self.memory_samples
    }

    /// Returns the refresh interval between samples.
    pub(crate) fn refresh_interval(&self) -> Duration {
        self.refresh_interval
    }

    /// Returns `true` if the CPU appears to be overloaded.
    ///
    /// The CPU is considered overloaded if any recent sample exceeds 90%
    /// or if there are significant delays in thread scheduling.
    pub(crate) fn is_cpu_overloaded(&self) -> bool {
        self.is_cpu_over_threshold(CPU_OVERLOAD_THRESHOLD) || self.has_scheduling_delay()
    }

    /// Returns `true` if any CPU sample exceeds the given threshold.
    pub(crate) fn is_cpu_over_threshold(&self, threshold: CpuUsage) -> bool {
        self.cpu_samples.iter().any(|s| s.value > threshold)
    }

    /// Returns the most recent CPU load, if available.
    pub(crate) fn latest_cpu(&self) -> Option<CpuLoad> {
        self.cpu_samples.last().copied()
    }

    /// Returns the most recent memory usage, if available.
    pub(crate) fn latest_memory(&self) -> Option<MemoryUsage> {
        self.memory_samples.last().copied()
    }

    /// Returns `true` if there appears to be scheduling delays.
    fn has_scheduling_delay(&self) -> bool {
        // Check if there are gaps between consecutive samples larger than 1.5x the interval
        let threshold = self.refresh_interval.as_millis() * 3 / 2;
        for window in self.cpu_samples.windows(2) {
            let gap = window[1].timestamp.duration_since(window[0].timestamp);
            if gap.as_millis() > threshold {
                return true;
            }
        }
        false
    }
}

impl Default for CpuMemoryHistory {
    fn default() -> Self {
        Self {
            cpu_samples: Vec::new(),
            memory_samples: Vec::new(),
            refresh_interval: DEFAULT_REFRESH_INTERVAL,
        }
    }
}

impl std::fmt::Display for CpuMemoryHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.cpu_samples.is_empty() {
            write!(f, "empty")
        } else {
            let samples: Vec<String> = self.cpu_samples.iter().map(|s| s.to_string()).collect();
            write!(f, "{}", samples.join(", "))
        }
    }
}

/// Handle to the CPU/memory monitor singleton.
///
/// The background monitoring thread lives for the lifetime of the process
/// because the singleton is held in a global `OnceLock<Arc<...>>`. When all
/// handles are dropped the thread continues to run but idles (skipping
/// sample collection) until a new handle is created via [`CpuMemoryMonitor::get_or_init`].
#[derive(Clone, Debug)]
pub(crate) struct CpuMemoryMonitor {
    inner: Arc<CpuMemoryMonitorInner>,
}

impl CpuMemoryMonitor {
    /// Gets or creates the global CPU/memory monitor singleton.
    ///
    /// On first call this starts a background thread that periodically
    /// samples CPU and memory usage. The thread persists for the lifetime
    /// of the process. When no `CpuMemoryMonitor` handles exist the thread
    /// idles without collecting samples; it resumes collection as soon as a
    /// new handle is created.
    ///
    /// The `refresh_interval` is only used on the first call to create the
    /// singleton; subsequent calls reuse the existing monitor regardless of
    /// the interval passed.
    pub(crate) fn get_or_init(refresh_interval: Duration) -> Self {
        let inner = CPU_MEMORY_MONITOR
            .get_or_init(|| {
                let inner = Arc::new(CpuMemoryMonitorInner::new(refresh_interval));
                inner.start();
                inner
            })
            .clone();

        // Register this as a listener to keep the monitor alive
        inner.register();

        Self { inner }
    }

    /// Returns a snapshot of the current CPU and memory history.
    pub(crate) fn snapshot(&self) -> CpuMemoryHistory {
        self.inner.snapshot()
    }

    /// Returns `true` if the CPU appears to be overloaded.
    pub(crate) fn is_cpu_overloaded(&self) -> bool {
        self.snapshot().is_cpu_overloaded()
    }
}

impl Drop for CpuMemoryMonitor {
    fn drop(&mut self) {
        self.inner.unregister();
    }
}

/// Internal state for the CPU/memory monitor.
#[derive(Debug)]
struct CpuMemoryMonitorInner {
    /// Circular buffer for CPU samples.
    cpu_buffer: RwLock<VecDeque<CpuLoad>>,
    /// Circular buffer for memory samples.
    memory_buffer: RwLock<VecDeque<MemoryUsage>>,
    /// Number of active listeners (handles).
    listener_count: RwLock<usize>,
    /// The refresh interval.
    refresh_interval: Duration,
}

impl CpuMemoryMonitorInner {
    fn new(refresh_interval: Duration) -> Self {
        Self {
            cpu_buffer: RwLock::new(VecDeque::with_capacity(HISTORY_LENGTH)),
            memory_buffer: RwLock::new(VecDeque::with_capacity(HISTORY_LENGTH)),
            listener_count: RwLock::new(0),
            refresh_interval,
        }
    }

    fn start(self: &Arc<Self>) {
        let weak = Arc::downgrade(self);
        let refresh_interval = self.refresh_interval;
        thread::Builder::new()
            .name("cosmos-cpu-monitor".into())
            .spawn(move || {
                Self::monitor_loop(weak, refresh_interval);
            })
            .expect("failed to spawn CPU monitor thread");
    }

    fn register(&self) {
        let mut count = self.listener_count.write().unwrap();
        *count += 1;
    }

    fn unregister(&self) {
        let mut count = self.listener_count.write().unwrap();
        *count = count.saturating_sub(1);
    }

    fn has_listeners(&self) -> bool {
        *self.listener_count.read().unwrap() > 0
    }

    fn snapshot(&self) -> CpuMemoryHistory {
        let cpu_samples: Vec<CpuLoad> = self.cpu_buffer.read().unwrap().iter().copied().collect();
        let memory_samples: Vec<MemoryUsage> =
            self.memory_buffer.read().unwrap().iter().copied().collect();

        CpuMemoryHistory {
            cpu_samples,
            memory_samples,
            refresh_interval: self.refresh_interval,
        }
    }

    fn monitor_loop(weak: Weak<CpuMemoryMonitorInner>, refresh_interval: Duration) {
        loop {
            thread::sleep(refresh_interval);

            let Some(inner) = weak.upgrade() else {
                // Monitor was dropped, exit the thread
                break;
            };

            if !inner.has_listeners() {
                // No listeners — idle until new handles are created.
                continue;
            }

            inner.refresh();
        }
    }

    fn refresh(&self) {
        let now = Instant::now();

        // Read CPU usage
        let cpu_value = read_cpu_usage();
        if let Some(cpu) = cpu_value {
            let mut cpu_buffer = self.cpu_buffer.write().unwrap();
            if cpu_buffer.len() >= HISTORY_LENGTH {
                cpu_buffer.pop_front();
            }
            cpu_buffer.push_back(CpuLoad::new(now, cpu));
        }

        // Read memory usage
        let memory_mb = read_available_memory_mb();
        {
            let mut memory_buffer = self.memory_buffer.write().unwrap();
            if memory_buffer.len() >= HISTORY_LENGTH {
                memory_buffer.pop_front();
            }
            memory_buffer.push_back(MemoryUsage {
                timestamp: now,
                available_mb: memory_mb,
            });
        }
    }
}

/// Reads the current system-wide CPU usage as a percentage (0.0 to 100.0).
fn read_cpu_usage() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        read_cpu_usage_linux()
    }

    #[cfg(target_os = "windows")]
    {
        read_cpu_usage_windows()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
fn read_cpu_usage_linux() -> Option<f64> {
    // Read /proc/stat for CPU statistics.
    // Tracks deltas between successive readings via static atomics.
    static PREV_IDLE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    static PREV_TOTAL: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

    let content = fs::read_to_string("/proc/stat").ok()?;
    let cpu_line = content.lines().find(|l| l.starts_with("cpu "))?;
    let values: Vec<u64> = cpu_line
        .split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect();

    if values.len() < 4 {
        return None;
    }

    let idle = values.get(3).copied().unwrap_or(0);
    let total: u64 = values.iter().sum();

    let prev_idle = PREV_IDLE.swap(idle, std::sync::atomic::Ordering::Relaxed);
    let prev_total = PREV_TOTAL.swap(total, std::sync::atomic::Ordering::Relaxed);

    if prev_total == 0 {
        return None; // First reading
    }

    let idle_delta = idle.saturating_sub(prev_idle);
    let total_delta = total.saturating_sub(prev_total);

    if total_delta == 0 {
        return Some(0.0);
    }

    let usage = 100.0 * (1.0 - (idle_delta as f64 / total_delta as f64));
    Some(usage.clamp(0.0, 100.0))
}

#[cfg(target_os = "windows")]
fn read_cpu_usage_windows() -> Option<f64> {
    use std::sync::atomic::{AtomicU64, Ordering};
    use windows::Win32::Foundation::FILETIME;
    use windows::Win32::System::Threading::GetSystemTimes;

    static PREV_IDLE: AtomicU64 = AtomicU64::new(0);
    static PREV_KERNEL: AtomicU64 = AtomicU64::new(0);
    static PREV_USER: AtomicU64 = AtomicU64::new(0);

    fn filetime_to_u64(file_time: FILETIME) -> u64 {
        (u64::from(file_time.dwHighDateTime) << 32) | u64::from(file_time.dwLowDateTime)
    }

    let mut idle_time = FILETIME::default();
    let mut kernel_time = FILETIME::default();
    let mut user_time = FILETIME::default();

    // SAFETY: GetSystemTimes writes into the provided pointers.
    // The pointers are valid stack-allocated i64 values (FILETIME-sized).
    let ok = unsafe {
        GetSystemTimes(
            Some(&mut idle_time as *mut FILETIME),
            Some(&mut kernel_time as *mut FILETIME),
            Some(&mut user_time as *mut FILETIME),
        )
    };

    if ok.is_err() {
        return None;
    }

    let idle = filetime_to_u64(idle_time);
    let kernel = filetime_to_u64(kernel_time);
    let user = filetime_to_u64(user_time);

    let prev_idle = PREV_IDLE.swap(idle, Ordering::Relaxed);
    let prev_kernel = PREV_KERNEL.swap(kernel, Ordering::Relaxed);
    let prev_user = PREV_USER.swap(user, Ordering::Relaxed);

    // First reading — no previous sample to compare against.
    if prev_kernel == 0 && prev_user == 0 {
        return None;
    }

    let idle_delta = idle.saturating_sub(prev_idle);
    let total_delta = kernel.saturating_sub(prev_kernel) + user.saturating_sub(prev_user);

    if total_delta == 0 {
        return Some(0.0);
    }

    // kernel_time includes idle_time, so active = total - idle.
    let usage = 100.0 * (1.0 - (idle_delta as f64 / total_delta as f64));
    Some(usage.clamp(0.0, 100.0))
}

/// Reads the available system memory in megabytes.
fn read_available_memory_mb() -> u64 {
    #[cfg(target_os = "linux")]
    {
        read_available_memory_linux()
    }

    #[cfg(target_os = "windows")]
    {
        read_available_memory_windows()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        0
    }
}

#[cfg(target_os = "linux")]
fn read_available_memory_linux() -> u64 {
    // Read /proc/meminfo for MemAvailable
    let content = match fs::read_to_string("/proc/meminfo") {
        Ok(c) => c,
        Err(_) => return 0,
    };

    for line in content.lines() {
        if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(kb_str) = parts.get(1) {
                if let Ok(kb) = kb_str.parse::<u64>() {
                    return kb / 1024; // Convert KB to MB
                }
            }
        }
    }

    0
}

#[cfg(target_os = "windows")]
fn read_available_memory_windows() -> u64 {
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};

    let mut mem_info = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        dwMemoryLoad: 0,
        ullTotalPhys: 0,
        ullAvailPhys: 0,
        ullTotalPageFile: 0,
        ullAvailPageFile: 0,
        ullTotalVirtual: 0,
        ullAvailVirtual: 0,
        ullAvailExtendedVirtual: 0,
    };

    // SAFETY: `mem_info` is a stack-allocated, correctly-sized MEMORYSTATUSEX
    // with `dwLength` set. The function writes into it.
    let ok = unsafe { GlobalMemoryStatusEx(&mut mem_info) };

    if ok.is_err() {
        return 0;
    }

    mem_info.ullAvailPhys / (1024 * 1024)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cpu_usage_valid_range() {
        let usage = CpuUsage::new(50.0);
        assert_eq!(usage.value(), 50.0);
    }

    #[test]
    fn cpu_usage_nan_normalizes_to_zero() {
        let usage = CpuUsage::new(f64::NAN);
        assert_eq!(usage.value(), 0.0);
    }

    #[test]
    fn cpu_usage_negative_zero_normalizes_to_positive_zero() {
        let usage = CpuUsage::new(-0.0);
        assert_eq!(usage.value().to_bits(), 0.0_f64.to_bits());
    }

    #[test]
    fn cpu_usage_eq() {
        assert_eq!(CpuUsage::new(42.5), CpuUsage::new(42.5));
    }

    #[test]
    fn cpu_usage_ord() {
        assert!(CpuUsage::new(10.0) < CpuUsage::new(20.0));
    }

    #[test]
    fn cpu_usage_from_f64() {
        let usage: CpuUsage = 75.0_f64.into();
        assert_eq!(usage.value(), 75.0);
        let back: f64 = usage.into();
        assert_eq!(back, 75.0);
    }

    #[test]
    fn cpu_usage_display() {
        let usage = CpuUsage::new(42.5);
        assert_eq!(format!("{usage}"), "42.5%");
    }

    #[test]
    #[should_panic(expected = "CpuUsage must be between 0.0 and 100.0 after normalization")]
    fn cpu_usage_invalid_negative() {
        CpuUsage::new(-1.0);
    }

    #[test]
    #[should_panic(expected = "CpuUsage must be between 0.0 and 100.0 after normalization")]
    fn cpu_usage_invalid_over_100() {
        CpuUsage::new(101.0);
    }

    #[test]
    fn cpu_load_wraps_cpu_usage() {
        let load = CpuLoad::new(Instant::now(), 50.0);
        assert_eq!(load.value(), CpuUsage::new(50.0));
    }

    #[test]
    fn cpu_load_eq() {
        let t = Instant::now();
        let a = CpuLoad::new(t, 42.5);
        let b = CpuLoad::new(t, 42.5);
        assert_eq!(a, b);
    }

    #[test]
    fn memory_usage_eq() {
        let t = Instant::now();
        let a = MemoryUsage {
            timestamp: t,
            available_mb: 1024,
        };
        let b = MemoryUsage {
            timestamp: t,
            available_mb: 1024,
        };
        assert_eq!(a, b);
    }

    #[test]
    fn cpu_memory_history_empty() {
        let history = CpuMemoryHistory::default();
        assert!(history.cpu_samples().is_empty());
        assert!(history.memory_samples().is_empty());
        assert!(!history.is_cpu_overloaded());
    }

    #[test]
    fn cpu_memory_history_overload_detection() {
        let history = CpuMemoryHistory {
            cpu_samples: vec![CpuLoad::new(Instant::now(), 95.0)],
            memory_samples: vec![],
            refresh_interval: DEFAULT_REFRESH_INTERVAL,
        };
        assert!(history.is_cpu_overloaded());
        assert!(history.is_cpu_over_threshold(CpuUsage::new(90.0)));
        assert!(!history.is_cpu_over_threshold(CpuUsage::new(96.0)));
    }

    #[test]
    fn cpu_memory_monitor_singleton() {
        let monitor1 = CpuMemoryMonitor::get_or_init(DEFAULT_REFRESH_INTERVAL);
        let monitor2 = CpuMemoryMonitor::get_or_init(DEFAULT_REFRESH_INTERVAL);

        // Both should point to the same inner
        assert!(Arc::ptr_eq(&monitor1.inner, &monitor2.inner));
    }

}
