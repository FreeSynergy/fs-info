// bus_publisher.rs — publishes system::health::degraded / restored events to the bus.
//
// The `AlertPublisher` runs a periodic check loop. When a threshold is crossed
// it publishes `system::health::degraded` (+ a detail topic). When the system
// recovers from a previously active alert it publishes `system::health::restored`.

use std::{collections::HashSet, sync::Arc, time::Duration};

use fs_bus::{
    topics::{SYSTEM_HEALTH_DEGRADED, SYSTEM_HEALTH_RESTORED},
    BusMessage, Event, MessageBus,
};
use tracing::{info, warn};

use crate::alert::{AlertChecker, AlertThresholds, SysInfoAlert};

// ── AlertKey ──────────────────────────────────────────────────────────────────

/// Stable key that identifies a specific alert so we can detect recovery.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum AlertKey {
    Disk(String),   // mount point
    CpuHot(String), // sensor label
    Memory,
    #[cfg(feature = "smart")]
    Smart(String), // device path
}

impl AlertKey {
    fn from_alert(alert: &SysInfoAlert) -> Self {
        match alert {
            SysInfoAlert::DiskFull { mount_point, .. } => Self::Disk(mount_point.clone()),
            SysInfoAlert::CpuHot { sensor, .. } => Self::CpuHot(sensor.clone()),
            SysInfoAlert::MemoryFull { .. } => Self::Memory,
            #[cfg(feature = "smart")]
            SysInfoAlert::SmartError { device, .. } => Self::Smart(device.clone()),
        }
    }
}

// ── AlertPublisher ────────────────────────────────────────────────────────────

/// Runs a polling loop: checks thresholds and publishes health events to the bus.
pub struct AlertPublisher {
    checker: AlertChecker,
    bus: Arc<MessageBus>,
    interval: Duration,
}

impl AlertPublisher {
    /// Create a publisher with the given thresholds and check interval.
    #[must_use]
    pub fn new(thresholds: AlertThresholds, bus: Arc<MessageBus>, interval: Duration) -> Self {
        Self {
            checker: AlertChecker::new(thresholds),
            bus,
            interval,
        }
    }

    /// Create a publisher with default thresholds and a 60-second interval.
    #[must_use]
    pub fn with_defaults(bus: Arc<MessageBus>) -> Self {
        Self::new(AlertThresholds::default(), bus, Duration::from_secs(60))
    }

    /// Run the alert loop until the process is stopped.
    pub async fn run(self) {
        let mut active: HashSet<AlertKey> = HashSet::new();

        loop {
            let current_alerts = self.checker.check_once();
            let current_keys: HashSet<AlertKey> =
                current_alerts.iter().map(AlertKey::from_alert).collect();

            // Publish new alerts (threshold newly crossed).
            for alert in &current_alerts {
                let key = AlertKey::from_alert(alert);
                if !active.contains(&key) {
                    self.publish_degraded(alert).await;
                }
            }

            // Publish recoveries (previously active, now below threshold).
            for key in &active {
                if !current_keys.contains(key) {
                    self.publish_restored(key).await;
                }
            }

            active = current_keys;
            tokio::time::sleep(self.interval).await;
        }
    }

    async fn publish_degraded(&self, alert: &SysInfoAlert) {
        let desc = alert.description();
        let ev = match Event::new(SYSTEM_HEALTH_DEGRADED, "fs-info", alert) {
            Ok(e) => e,
            Err(e) => {
                warn!("failed to build degraded event: {e}");
                return;
            }
        };
        self.bus.publish(BusMessage::fire(ev)).await;
        info!("health degraded: {desc}");
    }

    async fn publish_restored(&self, key: &AlertKey) {
        // Build a minimal payload that describes what recovered.
        let component = match key {
            AlertKey::Disk(mp) => format!("disk:{mp}"),
            AlertKey::CpuHot(sensor) => format!("cpu:{sensor}"),
            AlertKey::Memory => "memory".to_owned(),
            #[cfg(feature = "smart")]
            AlertKey::Smart(dev) => format!("smart:{dev}"),
        };
        let payload = serde_json::json!({ "component": component });
        let ev = match Event::new(SYSTEM_HEALTH_RESTORED, "fs-info", &payload) {
            Ok(e) => e,
            Err(e) => {
                warn!("failed to build restored event: {e}");
                return;
            }
        };
        self.bus.publish(BusMessage::fire(ev)).await;
        info!("health restored: {component}");
    }
}
