#![deny(clippy::all, clippy::pedantic, warnings)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::must_use_candidate)]
#![allow(clippy::doc_markdown)]
//! `fs-info` — System information detection and alerting for FreeSynergy.
//!
//! # Entry point
//!
//! [`FsInfo`] is the facade: create one instance and call the [`SystemInfo`]
//! methods to get fresh data. Use [`MetricsCollector::collect`] for a flat
//! list of all metrics.
//!
//! # Static data (cached 24 h)
//! - [`OsInfo`]           — OS type, version, architecture, kernel, hostname
//! - [`DetectedFeatures`] — which system features are present (systemd, Podman, …)
//! - [`SysInfoCache`]     — read/write `~/.config/fsn/sysinfo.toml`
//!
//! # Dynamic data (on demand via [`SystemInfo`])
//! - [`CpuInfo`]      — CPU usage, load average, core count
//! - [`MemInfo`]      — RAM and swap usage
//! - [`DiskInfo`]     — partition list with used / free bytes
//! - [`NetworkInfo`]  — network interface statistics
//! - [`ThermalInfo`]  — CPU temperature sensors
//! - [`Uptime`]       — system uptime
//!
//! # Metrics
//! - [`MetricsCollector`] — trait: `collect() → Vec<Metric>`
//! - [`Metric`]           — named numeric value with unit
//!
//! # Alerting
//! - [`AlertChecker`]    — compares live metrics against [`AlertThresholds`]
//! - [`SysInfoAlert`]    — returned alerts carry the correct bus topic
//!
//! # Optional (feature = `"smart"`)
//! - [`SmartInfo`]   — SMART disk health via `smartctl`
//!
//! # Example
//!
//! ```no_run
//! use fs_info::{FsInfo, SystemInfo, MetricsCollector};
//!
//! let info = FsInfo::new();
//! let mem = info.memory();
//! println!("RAM: {:.1}% used", mem.used_percent());
//!
//! let metrics = info.collect();
//! println!("{} metrics collected", metrics.len());
//! ```

pub mod alert;
pub mod bus_publisher;
pub mod cache;
pub mod cli;
pub mod cpu;
pub mod disk;
pub mod facade;
pub mod features;
pub mod grpc;
pub mod mem;
pub mod metric;
pub mod network;
pub mod os;
pub mod thermal;
pub mod uptime;

#[cfg(feature = "smart")]
pub mod smart;

pub use alert::{AlertChecker, AlertThresholds, SysInfoAlert};
pub use cache::SysInfoCache;
pub use cpu::{CpuInfo, LoadAverage};
pub use disk::{DiskInfo, Partition};
pub use facade::{FsInfo, SystemInfo};
pub use features::{DetectedFeatures, Feature, FeatureDetect};
pub use mem::MemInfo;
pub use metric::{Metric, MetricsCollector};
pub use network::{NetworkInfo, NetworkInterface};
pub use os::{OsInfo, OsType};
pub use thermal::{CpuTemp, ThermalInfo};
pub use uptime::Uptime;

#[cfg(feature = "smart")]
pub use smart::{DriveSmartStatus, SmartInfo};
