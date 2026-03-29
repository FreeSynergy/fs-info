//! Network interface statistics (on demand).

use serde::{Deserialize, Serialize};
use sysinfo::Networks;

/// Statistics for a single network interface.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    /// Interface name, e.g. `"eth0"` or `"lo"`.
    pub name: String,
    /// Total bytes received since boot.
    pub received_bytes: u64,
    /// Total bytes transmitted since boot.
    pub transmitted_bytes: u64,
    /// Receive errors since boot.
    pub receive_errors: u64,
    /// Transmit errors since boot.
    pub transmit_errors: u64,
}

impl NetworkInterface {
    /// Total traffic (received + transmitted) in bytes.
    #[must_use]
    pub fn total_bytes(&self) -> u64 {
        self.received_bytes.saturating_add(self.transmitted_bytes)
    }
}

/// Snapshot of all network interface statistics.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInfo {
    /// All detected network interfaces.
    pub interfaces: Vec<NetworkInterface>,
}

impl NetworkInfo {
    /// Read current network statistics from the OS.
    pub fn detect() -> Self {
        let networks = Networks::new_with_refreshed_list();
        let interfaces = networks
            .iter()
            .map(|(name, data)| NetworkInterface {
                name: name.clone(),
                received_bytes: data.total_received(),
                transmitted_bytes: data.total_transmitted(),
                receive_errors: data.total_errors_on_received(),
                transmit_errors: data.total_errors_on_transmitted(),
            })
            .collect();
        NetworkInfo { interfaces }
    }

    /// Total bytes received across all interfaces.
    #[must_use]
    pub fn total_received_bytes(&self) -> u64 {
        self.interfaces
            .iter()
            .map(|i| i.received_bytes)
            .fold(0u64, u64::saturating_add)
    }

    /// Total bytes transmitted across all interfaces.
    #[must_use]
    pub fn total_transmitted_bytes(&self) -> u64 {
        self.interfaces
            .iter()
            .map(|i| i.transmitted_bytes)
            .fold(0u64, u64::saturating_add)
    }
}
