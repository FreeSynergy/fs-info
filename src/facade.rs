//! `SystemInfo` trait and `FsInfo` facade — unified system-info entry point.

use crate::{
    cpu::CpuInfo,
    disk::DiskInfo,
    mem::MemInfo,
    metric::{Metric, MetricsCollector},
    network::NetworkInfo,
    os::OsInfo,
    thermal::ThermalInfo,
    uptime::Uptime,
};

// ── SystemInfo trait ──────────────────────────────────────────────────────────

/// Unified interface for system information queries.
///
/// Concrete implementation: [`FsInfo`].
/// Consumer code depends on this trait only.
pub trait SystemInfo {
    /// Current CPU usage and load average.
    fn cpu(&self) -> CpuInfo;

    /// Current memory (RAM + swap) usage.
    fn memory(&self) -> MemInfo;

    /// Current disk partition usage.
    fn disk(&self) -> DiskInfo;

    /// Current network interface statistics.
    fn network(&self) -> NetworkInfo;

    /// Current system uptime.
    fn uptime(&self) -> Uptime;

    /// Static OS information (version, architecture, hostname).
    fn os(&self) -> OsInfo;

    /// CPU temperature readings.
    fn thermal(&self) -> ThermalInfo;
}

// ── FsInfo facade ─────────────────────────────────────────────────────────────

/// Facade that implements [`SystemInfo`] and [`MetricsCollector`].
///
/// All queries delegate to the per-subsystem detection functions.
/// Create one instance and call the methods you need — each call reads
/// fresh data from the OS.
///
/// ```no_run
/// use fs_info::{FsInfo, SystemInfo};
///
/// let info = FsInfo::new();
/// let mem = info.memory();
/// println!("RAM used: {:.1}%", mem.used_percent());
/// ```
pub struct FsInfo;

impl FsInfo {
    pub fn new() -> Self {
        Self
    }
}

impl Default for FsInfo {
    fn default() -> Self {
        Self::new()
    }
}

impl SystemInfo for FsInfo {
    fn cpu(&self) -> CpuInfo {
        CpuInfo::detect()
    }

    fn memory(&self) -> MemInfo {
        MemInfo::detect()
    }

    fn disk(&self) -> DiskInfo {
        DiskInfo::detect()
    }

    fn network(&self) -> NetworkInfo {
        NetworkInfo::detect()
    }

    fn uptime(&self) -> Uptime {
        Uptime::detect()
    }

    fn os(&self) -> OsInfo {
        OsInfo::detect()
    }

    fn thermal(&self) -> ThermalInfo {
        ThermalInfo::detect()
    }
}

impl MetricsCollector for FsInfo {
    #[allow(clippy::cast_precision_loss)]
    fn collect(&self) -> Vec<Metric> {
        let mut metrics = Vec::new();

        // ── Memory ────────────────────────────────────────────────────────────
        let mem = self.memory();
        metrics.push(Metric::new(
            "memory.total_bytes",
            mem.total_bytes as f64,
            "bytes",
        ));
        metrics.push(Metric::new(
            "memory.used_bytes",
            mem.used_bytes as f64,
            "bytes",
        ));
        metrics.push(Metric::new("memory.used_percent", mem.used_percent(), "%"));
        metrics.push(Metric::new(
            "memory.swap_total_bytes",
            mem.swap_total_bytes as f64,
            "bytes",
        ));
        metrics.push(Metric::new(
            "memory.swap_used_bytes",
            mem.swap_used_bytes as f64,
            "bytes",
        ));

        // ── Disk ──────────────────────────────────────────────────────────────
        let disk = self.disk();
        for part in &disk.partitions {
            let safe = part
                .mount_point
                .replace('/', "_")
                .trim_matches('_')
                .to_owned();
            let prefix = if safe.is_empty() {
                "root".to_owned()
            } else {
                safe
            };
            metrics.push(Metric::new(
                format!("disk.{prefix}.used_bytes"),
                part.used_bytes() as f64,
                "bytes",
            ));
            metrics.push(Metric::new(
                format!("disk.{prefix}.available_bytes"),
                part.available_bytes as f64,
                "bytes",
            ));
            metrics.push(Metric::new(
                format!("disk.{prefix}.used_percent"),
                part.used_percent(),
                "%",
            ));
        }

        // ── Network ───────────────────────────────────────────────────────────
        let net = self.network();
        for iface in &net.interfaces {
            metrics.push(Metric::new(
                format!("network.{}.received_bytes", iface.name),
                iface.received_bytes as f64,
                "bytes",
            ));
            metrics.push(Metric::new(
                format!("network.{}.transmitted_bytes", iface.name),
                iface.transmitted_bytes as f64,
                "bytes",
            ));
        }

        // ── Uptime ────────────────────────────────────────────────────────────
        let up = self.uptime();
        metrics.push(Metric::new("system.uptime_seconds", up.seconds as f64, "s"));

        // ── Thermal ───────────────────────────────────────────────────────────
        let thermal = self.thermal();
        for sensor in &thermal.sensors {
            let safe = sensor.label.replace(' ', "_").to_lowercase();
            metrics.push(Metric::new(
                format!("thermal.{safe}.temp_celsius"),
                f64::from(sensor.temp_celsius),
                "°C",
            ));
        }

        metrics
    }
}
