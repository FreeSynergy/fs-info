//! Integration tests for fs-info.
//!
//! Tests run on the actual system — no mocking required since all data is read-only.

use fs_info::{
    AlertChecker, AlertThresholds, DiskInfo, FsInfo, MemInfo, MetricsCollector, NetworkInfo,
    SystemInfo, Uptime,
};

// ── MemInfo ───────────────────────────────────────────────────────────────────

#[test]
fn mem_info_has_positive_total() {
    let mem = MemInfo::detect();
    assert!(mem.total_bytes > 0, "total RAM must be > 0");
    assert!(
        mem.used_bytes <= mem.total_bytes,
        "used must not exceed total"
    );
}

#[test]
fn mem_info_used_percent_range() {
    let mem = MemInfo::detect();
    let pct = mem.used_percent();
    assert!(
        (0.0..=100.0).contains(&pct),
        "used_percent must be 0–100, got {pct}"
    );
}

// ── DiskInfo ──────────────────────────────────────────────────────────────────

#[test]
fn disk_info_detects_partitions() {
    let disk = DiskInfo::detect();
    assert!(
        !disk.partitions.is_empty(),
        "at least one partition expected"
    );
}

#[test]
fn disk_partition_percent_range() {
    let disk = DiskInfo::detect();
    for part in &disk.partitions {
        let pct = part.used_percent();
        assert!(
            (0.0..=100.0).contains(&pct),
            "partition {} used_percent={pct} out of range",
            part.mount_point
        );
    }
}

// ── Uptime ────────────────────────────────────────────────────────────────────

#[test]
fn uptime_is_positive() {
    let up = Uptime::detect();
    assert!(up.seconds > 0, "uptime must be > 0");
}

#[test]
fn uptime_display_is_non_empty() {
    let up = Uptime::detect();
    assert!(!up.display().is_empty());
}

// ── NetworkInfo ───────────────────────────────────────────────────────────────

#[test]
fn network_info_detects_interfaces() {
    let net = NetworkInfo::detect();
    // At least the loopback interface must exist.
    assert!(
        !net.interfaces.is_empty(),
        "at least one network interface expected"
    );
}

// ── FsInfo facade ─────────────────────────────────────────────────────────────

#[test]
fn fs_info_memory_returns_data() {
    let info = FsInfo::new();
    let mem = info.memory();
    assert!(mem.total_bytes > 0);
}

#[test]
fn fs_info_disk_returns_data() {
    let info = FsInfo::new();
    let disk = info.disk();
    assert!(!disk.partitions.is_empty());
}

#[test]
fn fs_info_uptime_returns_data() {
    let info = FsInfo::new();
    let up = info.uptime();
    assert!(up.seconds > 0);
}

#[test]
fn fs_info_network_returns_data() {
    let info = FsInfo::new();
    let net = info.network();
    assert!(!net.interfaces.is_empty());
}

// ── MetricsCollector ──────────────────────────────────────────────────────────

#[test]
fn collect_returns_metrics() {
    let info = FsInfo::new();
    let metrics = info.collect();
    assert!(
        !metrics.is_empty(),
        "collect() must return at least one metric"
    );
}

#[test]
fn collect_includes_memory_metrics() {
    let info = FsInfo::new();
    let metrics = info.collect();
    let names: Vec<&str> = metrics.iter().map(|m| m.name.as_str()).collect();
    assert!(
        names.contains(&"memory.total_bytes"),
        "memory.total_bytes missing from metrics"
    );
    assert!(
        names.contains(&"memory.used_percent"),
        "memory.used_percent missing from metrics"
    );
}

#[test]
fn collect_includes_uptime_metric() {
    let info = FsInfo::new();
    let metrics = info.collect();
    let has_uptime = metrics.iter().any(|m| m.name == "system.uptime_seconds");
    assert!(has_uptime, "system.uptime_seconds missing from metrics");
}

// ── AlertChecker ──────────────────────────────────────────────────────────────

#[test]
fn alert_checker_with_high_thresholds_produces_no_alerts() {
    // Set thresholds so high that nothing should trigger on a healthy dev machine.
    let checker = AlertChecker::new(AlertThresholds {
        disk_full_percent: 99.9,
        cpu_hot_celsius: 200.0,
        memory_full_percent: 99.9,
    });
    let alerts = checker.check_once();
    assert!(
        alerts.is_empty(),
        "expected no alerts with very high thresholds, got {alerts:?}"
    );
}

#[test]
fn alert_checker_with_zero_thresholds_produces_alerts() {
    // Set thresholds to 0 so every metric triggers.
    let checker = AlertChecker::new(AlertThresholds {
        disk_full_percent: 0.0,
        cpu_hot_celsius: 0.0,
        memory_full_percent: 0.0,
    });
    let alerts = checker.check_once();
    assert!(!alerts.is_empty(), "expected alerts with zero thresholds");
}
