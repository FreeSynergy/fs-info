//! System uptime.

use serde::{Deserialize, Serialize};
use sysinfo::System;

/// Current system uptime.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Uptime {
    /// Total uptime in seconds.
    pub seconds: u64,
}

impl Uptime {
    /// Read current uptime from the OS.
    pub fn detect() -> Self {
        Self {
            seconds: System::uptime(),
        }
    }

    /// Uptime broken down into days, hours, minutes, and remaining seconds.
    #[must_use]
    pub fn components(&self) -> (u64, u64, u64, u64) {
        let days = self.seconds / 86_400;
        let hours = (self.seconds % 86_400) / 3_600;
        let minutes = (self.seconds % 3_600) / 60;
        let secs = self.seconds % 60;
        (days, hours, minutes, secs)
    }

    /// Human-readable uptime string, e.g. `"3d 4h 12m"`.
    #[must_use]
    pub fn display(&self) -> String {
        let (d, h, m, _) = self.components();
        if d > 0 {
            format!("{d}d {h}h {m}m")
        } else if h > 0 {
            format!("{h}h {m}m")
        } else {
            format!("{m}m")
        }
    }
}
