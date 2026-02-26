// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! CPU and memory monitoring with historical snapshots.
#![allow(dead_code)]

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

/// Raw Mach kernel FFI bindings for macOS CPU and memory monitoring.
///
/// These declarations mirror the subset of `<mach/mach.h>` needed to call
/// `host_statistics` (CPU load) and `host_statistics64` (VM stats). Using
/// inline `extern "C"` avoids adding a crate dependency (e.g., `mach2`).
#[cfg(target_os = "macos")]
mod mach_ffi {
    // Mach types used by the host_statistics family of calls.
    pub type MachPort = u32;
    pub type KernReturn = i32;
    pub type HostFlavor = i32;
    pub type MachMsgType = u32;
    pub type NaturalT = u32;
    pub type VmSize = usize;

    /// Successful Mach kernel return code.
    pub const KERN_SUCCESS: KernReturn = 0;

    // host_statistics flavors.
    pub const HOST_CPU_LOAD_INFO: HostFlavor = 3;
    pub const HOST_VM_INFO64: HostFlavor = 4;

    /// Number of `natural_t` values in [`HostCpuLoadInfo`].
    pub const HOST_CPU_LOAD_INFO_COUNT: MachMsgType =
        (std::mem::size_of::<HostCpuLoadInfo>() / std::mem::size_of::<NaturalT>()) as MachMsgType;

    /// Number of `natural_t` values in [`VmStatistics64`].
    pub const HOST_VM_INFO64_COUNT: MachMsgType =
        (std::mem::size_of::<VmStatistics64>() / std::mem::size_of::<NaturalT>()) as MachMsgType;

    /// Per-CPU-state tick counts returned by `HOST_CPU_LOAD_INFO`.
    /// Indices: 0 = user, 1 = system, 2 = idle, 3 = nice.
    #[repr(C)]
    #[derive(Debug, Default, Clone, Copy)]
    pub struct HostCpuLoadInfo {
        pub cpu_ticks: [NaturalT; 4],
    }

    /// 64-bit virtual-memory statistics returned by `HOST_VM_INFO64`.
    ///
    /// Only the fields we need (`free_count`, `inactive_count`,
    /// `purgeable_count`) are named; the rest are padding. The full
    /// struct is 172 bytes on arm64 / 168 on x86_64; we declare it at
    /// the maximum size to be safe.
    #[repr(C)]
    #[derive(Debug, Clone, Copy)]
    pub struct VmStatistics64 {
        pub free_count: NaturalT,
        pub active_count: NaturalT,
        pub inactive_count: NaturalT,
        pub wire_count: NaturalT,
        pub zero_fill_count: u64,
        pub reactivations: u64,
        pub pageins: u64,
        pub pageouts: u64,
        pub faults: u64,
        pub cow_faults: u64,
        pub lookups: u64,
        pub hits: u64,
        pub purges: u64,
        pub purgeable_count: NaturalT,
        // Remaining fields are unused; the struct is sized correctly by
        // HOST_VM_INFO64_COUNT so the kernel writes the right amount.
        _pad: [u8; 100],
    }

    impl Default for VmStatistics64 {
        fn default() -> Self {
            // SAFETY: All fields are integers or byte arrays; zeroed is valid.
            unsafe { std::mem::zeroed() }
        }
    }

    extern "C" {
        /// Returns the host port for the current task.
        pub fn mach_host_self() -> MachPort;

        /// Retrieves 32-bit host statistics (used for CPU load info).
        pub fn host_statistics(
            host: MachPort,
            flavor: HostFlavor,
            info: *mut NaturalT,
            count: *mut MachMsgType,
        ) -> KernReturn;

        /// Retrieves 64-bit host statistics (used for VM info).
        pub fn host_statistics64(
            host: MachPort,
            flavor: HostFlavor,
            info: *mut NaturalT,
            count: *mut MachMsgType,
        ) -> KernReturn;

        /// Returns the VM page size.
        pub fn vm_page_size() -> VmSize;
    }
}

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
/// ```rust,ignore
/// use azure_data_cosmos_driver::system::CpuUsage;
///
/// let usage = CpuUsage::new(42.5);
/// assert_eq!(usage.value(), 42.5);
///
/// // NaN normalises to 0.0
/// let nan = CpuUsage::new(f64::NAN);
/// assert_eq!(nan.value(), 0.0);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct CpuUsage(f64);

impl CpuUsage {
    /// Creates a new `CpuUsage` from a raw `f64` percentage.
    ///
    /// NaN and negative zero are normalized to `0.0`. Values below `0.0` are
    /// clamped to `0.0` and values above `100.0` are clamped to `100.0`.
    ///
    /// # Panics (debug builds only)
    ///
    /// Debug-asserts if the value (before clamping) is outside `0.0..=100.0`
    /// to catch callers passing unexpected data.
    pub(crate) fn new(value: f64) -> Self {
        let normalized = Self::normalize(value);
        debug_assert!(
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

    /// Normalizes an `f64` value.
    ///
    /// - NaN becomes `0.0`
    /// - `-0.0` becomes `+0.0`
    /// - Values below `0.0` are clamped to `0.0`
    /// - Values above `100.0` are clamped to `100.0`
    fn normalize(value: f64) -> f64 {
        if value.is_nan() || value <= 0.0 {
            0.0
        } else if value > 100.0 {
            100.0
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

/// A single combined CPU and memory measurement at a point in time.
///
/// Both readings are taken at the same [`Instant`] in each refresh tick.
/// CPU may be `None` on the first reading or if the platform API fails.
#[non_exhaustive]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct SystemSample {
    /// When this measurement was taken.
    pub(crate) timestamp: Instant,
    /// CPU usage percentage, if available.
    pub(crate) cpu: Option<CpuUsage>,
    /// Available memory in megabytes, if available.
    pub(crate) available_mb: Option<u64>,
}

/// Historical CPU and memory usage data.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub(crate) struct CpuMemoryHistory {
    /// Historical samples (oldest first).
    samples: Vec<SystemSample>,
    /// The interval between samples.
    refresh_interval: Duration,
}

impl CpuMemoryHistory {
    /// Returns all historical samples.
    pub(crate) fn samples(&self) -> &[SystemSample] {
        &self.samples
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
        self.samples
            .iter()
            .any(|s| s.cpu.is_some_and(|cpu| cpu > threshold))
    }

    /// Returns the most recent CPU usage, if available.
    pub(crate) fn latest_cpu(&self) -> Option<CpuUsage> {
        self.samples.last().and_then(|s| s.cpu)
    }

    /// Returns the most recent available memory in megabytes, if any sample exists.
    pub(crate) fn latest_memory_mb(&self) -> Option<u64> {
        self.samples.last().and_then(|s| s.available_mb)
    }

    /// Returns `true` if there appears to be scheduling delays.
    fn has_scheduling_delay(&self) -> bool {
        // Check if there are gaps between consecutive samples larger than 1.5x the interval
        let threshold = self.refresh_interval.as_millis() * 3 / 2;
        for window in self.samples.windows(2) {
            let gap = window[1].timestamp.duration_since(window[0].timestamp);
            if gap.as_millis() > threshold {
                return true;
            }
        }
        false
    }
}

impl std::fmt::Display for CpuMemoryHistory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cpu_entries: Vec<String> = self
            .samples
            .iter()
            .filter_map(|s| s.cpu.map(|cpu| format!("({cpu})")))
            .collect();
        if cpu_entries.is_empty() {
            write!(f, "empty")
        } else {
            write!(f, "{}", cpu_entries.join(", "))
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
    /// # Panics
    ///
    /// Debug-panics if called with a `refresh_interval` that differs from the
    /// one used to create the singleton on the first call.
    pub(crate) fn get_or_init(refresh_interval: Duration) -> Self {
        let inner = CPU_MEMORY_MONITOR
            .get_or_init(|| {
                let inner = Arc::new(CpuMemoryMonitorInner::new(refresh_interval));
                inner.start();
                inner
            })
            .clone();

        debug_assert_eq!(
            inner.refresh_interval, refresh_interval,
            "CpuMemoryMonitor singleton already created with {:?}, cannot change to {:?}",
            inner.refresh_interval, refresh_interval,
        );

        // Register as a listener so the background thread collects samples.
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
    /// Circular buffer for combined CPU+memory samples.
    buffer: RwLock<VecDeque<SystemSample>>,
    /// Number of active listeners (handles).
    listener_count: RwLock<usize>,
    /// The refresh interval.
    refresh_interval: Duration,
}

impl CpuMemoryMonitorInner {
    fn new(refresh_interval: Duration) -> Self {
        Self {
            buffer: RwLock::new(VecDeque::with_capacity(HISTORY_LENGTH)),
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
        // Poisoning cannot occur: the critical section only increments a counter.
        let mut count = self.listener_count.write().unwrap();
        *count += 1;
    }

    fn unregister(&self) {
        // Poisoning cannot occur: the critical section only decrements a counter.
        let mut count = self.listener_count.write().unwrap();
        *count = count.saturating_sub(1);
    }

    fn has_listeners(&self) -> bool {
        // Poisoning cannot occur: see register/unregister.
        *self.listener_count.read().unwrap() > 0
    }

    fn snapshot(&self) -> CpuMemoryHistory {
        // Poisoning cannot occur: the write side (refresh) only does
        // infallible VecDeque push/pop operations.
        let samples: Vec<SystemSample> = self.buffer.read().unwrap().iter().copied().collect();

        CpuMemoryHistory {
            samples,
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
        let cpu = read_cpu_usage().map(CpuUsage::new);
        let available_mb = read_available_memory_mb();

        let sample = SystemSample {
            timestamp: now,
            cpu,
            available_mb,
        };

        // Poisoning cannot occur: push_back/pop_front are infallible.
        let mut buffer = self.buffer.write().unwrap();
        if buffer.len() >= HISTORY_LENGTH {
            buffer.pop_front();
        }
        buffer.push_back(sample);
    }
}

/// Maximum consecutive CPU read failures before permanently disabling CPU monitoring.
/// At the default 5-second refresh interval, 12 failures = 1 minute.
const MAX_CONSECUTIVE_CPU_READ_FAILURES: u32 = 12;

/// Tracks consecutive CPU read failures across all platforms.
static CONSECUTIVE_CPU_READ_FAILURES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Reads the current system-wide CPU usage as a percentage (0.0 to 100.0).
///
/// Tracks consecutive failures and permanently stops attempting reads after
/// [`MAX_CONSECUTIVE_CPU_READ_FAILURES`] consecutive failures (10 minutes
/// at the default 5-second refresh interval). A successful read resets the
/// failure counter.
fn read_cpu_usage() -> Option<f64> {
    use std::sync::atomic::Ordering;

    if CONSECUTIVE_CPU_READ_FAILURES.load(Ordering::Relaxed) >= MAX_CONSECUTIVE_CPU_READ_FAILURES {
        return None;
    }

    let result = read_cpu_usage_platform();

    match result {
        Some(_) => {
            CONSECUTIVE_CPU_READ_FAILURES.store(0, Ordering::Relaxed);
        }
        None => {
            CONSECUTIVE_CPU_READ_FAILURES.fetch_add(1, Ordering::Relaxed);
        }
    }

    result
}

/// Platform-specific CPU usage reading dispatched by target OS.
fn read_cpu_usage_platform() -> Option<f64> {
    #[cfg(target_os = "linux")]
    {
        read_cpu_usage_linux()
    }

    #[cfg(target_os = "windows")]
    {
        read_cpu_usage_windows()
    }

    #[cfg(target_os = "macos")]
    {
        read_cpu_usage_macos()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
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

/// Reads CPU usage on macOS via the Mach `host_statistics` API.
///
/// Computes the delta of per-state CPU ticks between successive calls
/// (user + system + nice = busy, idle = idle) and returns the busy
/// fraction as a percentage.
#[cfg(target_os = "macos")]
fn read_cpu_usage_macos() -> Option<f64> {
    use std::sync::atomic::{AtomicU64, Ordering};

    static PREV_BUSY: AtomicU64 = AtomicU64::new(0);
    static PREV_TOTAL: AtomicU64 = AtomicU64::new(0);

    let mut info = mach_ffi::HostCpuLoadInfo::default();
    let mut count = mach_ffi::HOST_CPU_LOAD_INFO_COUNT;

    // SAFETY: `info` is a stack-allocated, correctly-sized HostCpuLoadInfo.
    // `host_statistics` writes `count` natural_t values into the pointer.
    let kr = unsafe {
        mach_ffi::host_statistics(
            mach_ffi::mach_host_self(),
            mach_ffi::HOST_CPU_LOAD_INFO,
            &mut info as *mut mach_ffi::HostCpuLoadInfo as *mut mach_ffi::NaturalT,
            &mut count,
        )
    };

    if kr != mach_ffi::KERN_SUCCESS {
        return None;
    }

    let user = u64::from(info.cpu_ticks[0]);
    let system = u64::from(info.cpu_ticks[1]);
    let idle = u64::from(info.cpu_ticks[2]);
    let nice = u64::from(info.cpu_ticks[3]);

    let busy = user + system + nice;
    let total = busy + idle;

    let prev_busy = PREV_BUSY.swap(busy, Ordering::Relaxed);
    let prev_total = PREV_TOTAL.swap(total, Ordering::Relaxed);

    // First reading — no previous sample to compare against.
    if prev_total == 0 {
        return None;
    }

    let busy_delta = busy.saturating_sub(prev_busy);
    let total_delta = total.saturating_sub(prev_total);

    if total_delta == 0 {
        return Some(0.0);
    }

    let usage = 100.0 * (busy_delta as f64 / total_delta as f64);
    Some(usage.clamp(0.0, 100.0))
}

/// Maximum consecutive memory read failures before permanently disabling memory monitoring.
/// At the default 5-second refresh interval, 12 failures = 1 minute.
const MAX_CONSECUTIVE_MEMORY_READ_FAILURES: u32 = 12;

/// Tracks consecutive memory read failures across all platforms.
static CONSECUTIVE_MEMORY_READ_FAILURES: std::sync::atomic::AtomicU32 =
    std::sync::atomic::AtomicU32::new(0);

/// Reads the available system memory in megabytes.
///
/// Tracks consecutive failures and permanently stops attempting reads after
/// [`MAX_CONSECUTIVE_MEMORY_READ_FAILURES`] consecutive failures (1 minute
/// at the default 5-second refresh interval). A successful read resets the
/// failure counter.
fn read_available_memory_mb() -> Option<u64> {
    use std::sync::atomic::Ordering;

    if CONSECUTIVE_MEMORY_READ_FAILURES.load(Ordering::Relaxed)
        >= MAX_CONSECUTIVE_MEMORY_READ_FAILURES
    {
        return None;
    }

    let result = read_available_memory_mb_platform();

    match result {
        Some(_) => {
            CONSECUTIVE_MEMORY_READ_FAILURES.store(0, Ordering::Relaxed);
        }
        None => {
            CONSECUTIVE_MEMORY_READ_FAILURES.fetch_add(1, Ordering::Relaxed);
        }
    }

    result
}

/// Platform-specific memory reading dispatched by target OS.
fn read_available_memory_mb_platform() -> Option<u64> {
    #[cfg(target_os = "linux")]
    {
        read_available_memory_linux()
    }

    #[cfg(target_os = "windows")]
    {
        read_available_memory_windows()
    }

    #[cfg(target_os = "macos")]
    {
        read_available_memory_macos()
    }

    #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
    {
        None
    }
}

#[cfg(target_os = "linux")]
fn read_available_memory_linux() -> Option<u64> {
    // Read /proc/meminfo for MemAvailable
    let content = fs::read_to_string("/proc/meminfo").ok()?;

    for line in content.lines() {
        if line.starts_with("MemAvailable:") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if let Some(kb_str) = parts.get(1) {
                if let Ok(kb) = kb_str.parse::<u64>() {
                    return Some(kb / 1024); // Convert KB to MB
                }
            }
        }
    }

    None
}

#[cfg(target_os = "windows")]
fn read_available_memory_windows() -> Option<u64> {
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
        return None;
    }

    Some(mem_info.ullAvailPhys / (1024 * 1024))
}

/// Reads available memory on macOS via the Mach `host_statistics64` API.
///
/// Computes available memory as `(free + inactive + purgeable) * page_size`,
/// which matches what macOS Activity Monitor reports as "Memory Available".
#[cfg(target_os = "macos")]
fn read_available_memory_macos() -> Option<u64> {
    let mut info = mach_ffi::VmStatistics64::default();
    let mut count = mach_ffi::HOST_VM_INFO64_COUNT;

    // SAFETY: `info` is a stack-allocated, correctly-sized VmStatistics64.
    // `host_statistics64` writes `count` natural_t values into the pointer.
    let kr = unsafe {
        mach_ffi::host_statistics64(
            mach_ffi::mach_host_self(),
            mach_ffi::HOST_VM_INFO64,
            &mut info as *mut mach_ffi::VmStatistics64 as *mut mach_ffi::NaturalT,
            &mut count,
        )
    };

    if kr != mach_ffi::KERN_SUCCESS {
        return None;
    }

    // SAFETY: `vm_page_size` returns the kernel page size (always a valid usize).
    let page_size = unsafe { mach_ffi::vm_page_size() } as u64;

    let free = u64::from(info.free_count);
    let inactive = u64::from(info.inactive_count);
    let purgeable = u64::from(info.purgeable_count);

    let available_bytes = (free + inactive + purgeable) * page_size;
    Some(available_bytes / (1024 * 1024))
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
    fn cpu_usage_negative_clamped_to_zero() {
        let usage = CpuUsage::new(-1.0);
        assert_eq!(usage.value(), 0.0);
    }

    #[test]
    fn cpu_usage_large_negative_clamped_to_zero() {
        let usage = CpuUsage::new(-999.0);
        assert_eq!(usage.value(), 0.0);
    }

    #[test]
    fn cpu_usage_over_100_clamped() {
        let usage = CpuUsage::new(101.0);
        assert_eq!(usage.value(), 100.0);
    }

    #[test]
    fn cpu_usage_large_over_100_clamped() {
        let usage = CpuUsage::new(500.0);
        assert_eq!(usage.value(), 100.0);
    }

    #[test]
    fn cpu_usage_infinity_clamped_to_100() {
        let usage = CpuUsage::new(f64::INFINITY);
        assert_eq!(usage.value(), 100.0);
    }

    #[test]
    fn cpu_usage_neg_infinity_clamped_to_zero() {
        let usage = CpuUsage::new(f64::NEG_INFINITY);
        assert_eq!(usage.value(), 0.0);
    }

    #[test]
    fn cpu_usage_boundary_zero() {
        assert_eq!(CpuUsage::new(0.0).value(), 0.0);
    }

    #[test]
    fn cpu_usage_boundary_100() {
        assert_eq!(CpuUsage::new(100.0).value(), 100.0);
    }

    #[test]
    fn system_sample_eq() {
        let t = Instant::now();
        let a = SystemSample {
            timestamp: t,
            cpu: Some(CpuUsage::new(42.5)),
            available_mb: Some(1024),
        };
        let b = SystemSample {
            timestamp: t,
            cpu: Some(CpuUsage::new(42.5)),
            available_mb: Some(1024),
        };
        assert_eq!(a, b);
    }

    #[test]
    fn cpu_memory_history_empty() {
        let history = CpuMemoryHistory {
            samples: Vec::new(),
            refresh_interval: DEFAULT_REFRESH_INTERVAL,
        };
        assert!(history.samples().is_empty());
        assert!(!history.is_cpu_overloaded());
    }

    #[test]
    fn cpu_memory_history_overload_detection() {
        let history = CpuMemoryHistory {
            samples: vec![SystemSample {
                timestamp: Instant::now(),
                cpu: Some(CpuUsage::new(95.0)),
                available_mb: Some(1024),
            }],
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

    // ---- Platform-specific tests exercising real OS APIs ----

    #[test]
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn read_available_memory_mb_platform_returns_value() {
        let mb = read_available_memory_mb_platform();
        assert!(
            mb.is_some(),
            "read_available_memory_mb_platform() should return Some on this OS"
        );
        assert!(
            mb.unwrap() > 0,
            "available memory should be greater than 0 MB"
        );
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn read_cpu_usage_platform_returns_value_on_second_call() {
        // First call primes the static tick counters and may return None.
        let _ = read_cpu_usage_platform();

        // Let OS tick counters advance.
        std::thread::sleep(Duration::from_millis(200));

        let cpu = read_cpu_usage_platform();
        assert!(
            cpu.is_some(),
            "read_cpu_usage_platform() should return Some on the second call"
        );
        let pct = cpu.unwrap();
        assert!(
            (0.0..=100.0).contains(&pct),
            "CPU usage should be in 0..=100, got {pct}"
        );
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn read_available_memory_mb_wrapper_returns_value() {
        let mb = read_available_memory_mb();
        assert!(
            mb.is_some(),
            "read_available_memory_mb() wrapper should return Some on this OS"
        );
        assert!(
            mb.unwrap() > 0,
            "available memory should be greater than 0 MB"
        );
    }

    #[test]
    #[cfg(any(target_os = "linux", target_os = "windows", target_os = "macos"))]
    fn read_cpu_usage_wrapper_returns_value_on_second_call() {
        // First call primes the static tick counters and may return None.
        let _ = read_cpu_usage();

        // Let OS tick counters advance.
        std::thread::sleep(Duration::from_millis(200));

        let cpu = read_cpu_usage();
        assert!(
            cpu.is_some(),
            "read_cpu_usage() wrapper should return Some on the second call"
        );
        let pct = cpu.unwrap();
        assert!(
            (0.0..=100.0).contains(&pct),
            "CPU usage should be in 0..=100, got {pct}"
        );
    }
}
