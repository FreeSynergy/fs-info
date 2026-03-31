// cli.rs — CLI for fs-info.

use clap::{Parser, Subcommand};

/// `FreeSynergy` system information CLI.
#[derive(Parser)]
#[command(
    name = "fs-info",
    version,
    about = "Query system info (CPU, memory, disk)"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Run as daemon (gRPC server + health alert publisher).
    Daemon,
    /// Print full system overview (OS, kernel, hostname, uptime).
    System,
    /// Print current CPU usage and load averages.
    Cpu,
    /// Print current memory (RAM + swap) usage.
    Memory,
    /// Print disk partition list with usage.
    Disk,
    /// Run one alert check cycle and print any triggered alerts.
    Alerts {
        /// Repeat checks every N seconds until Ctrl-C.
        #[arg(short, long, value_name = "SECONDS")]
        monitor: Option<u64>,
    },
}
