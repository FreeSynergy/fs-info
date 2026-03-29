//! `Metric` type and `MetricsCollector` trait.

use serde::{Deserialize, Serialize};

// ── Metric ────────────────────────────────────────────────────────────────────

/// A single named numeric measurement with a unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    /// Metric name, e.g. `"cpu.usage_percent"`.
    pub name: String,
    /// Numeric value.
    pub value: f64,
    /// Unit label, e.g. `"%"`, `"bytes"`, `"°C"`, `"s"`.
    pub unit: String,
}

impl Metric {
    pub fn new(name: impl Into<String>, value: f64, unit: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value,
            unit: unit.into(),
        }
    }
}

// ── MetricsCollector ──────────────────────────────────────────────────────────

/// Collects a snapshot of system metrics as a flat list of [`Metric`] values.
///
/// Implementations: [`crate::FsInfo`].
pub trait MetricsCollector {
    /// Collect all available metrics in a single pass.
    fn collect(&self) -> Vec<Metric>;
}
