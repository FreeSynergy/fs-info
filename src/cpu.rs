//! CPU load and core information (on demand).

use serde::{Deserialize, Serialize};
use sysinfo::{CpuRefreshKind, RefreshKind, System};

/// Load average values (1, 5, 15 minutes).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadAverage {
    /// 1-minute load average.
    pub one: f64,
    /// 5-minute load average.
    pub five: f64,
    /// 15-minute load average.
    pub fifteen: f64,
}

/// CPU usage and core information snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    /// Number of logical CPU cores.
    pub core_count: usize,
    /// Overall CPU usage as a percentage (0–100), averaged across all cores.
    pub usage_percent: f32,
    /// Per-core usage percentages.
    pub per_core_percent: Vec<f32>,
    /// Load averages (1, 5, 15 min). Returns zeros on Windows.
    pub load_average: LoadAverage,
    /// CPU brand string, e.g. `"Intel(R) Core(TM) i7-10750H"`.
    pub brand: String,
}

impl CpuInfo {
    /// Read current CPU usage from the OS.
    ///
    /// Note: `sysinfo` requires a small delay between refresh calls to compute
    /// meaningful usage percentages. For snapshot use, the values reflect the
    /// usage since the last `sysinfo` refresh.
    pub fn detect() -> Self {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
        );
        // A second refresh is required for accurate delta-based CPU usage.
        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
        sys.refresh_cpu_all();

        let cpus = sys.cpus();
        let core_count = cpus.len();
        #[allow(clippy::cast_precision_loss)]
        let usage_percent = if core_count == 0 {
            0.0
        } else {
            cpus.iter().map(sysinfo::Cpu::cpu_usage).sum::<f32>() / core_count as f32
        };
        let per_core_percent = cpus.iter().map(sysinfo::Cpu::cpu_usage).collect();
        let brand = cpus
            .first()
            .map(|c| c.brand().to_owned())
            .unwrap_or_default();

        let la = System::load_average();
        let load_average = LoadAverage {
            one: la.one,
            five: la.five,
            fifteen: la.fifteen,
        };

        CpuInfo {
            core_count,
            usage_percent,
            per_core_percent,
            load_average,
            brand,
        }
    }
}
