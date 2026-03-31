#![deny(clippy::all, clippy::pedantic, warnings)]
//! `fs-info` — system information daemon for `FreeSynergy`.
//!
//! Starts a gRPC server (tonic) and runs a health-alert publisher that
//! periodically checks system thresholds and emits bus events.
//!
//! # Environment variables
//!
//! | Variable          | Default                |
//! |-------------------|------------------------|
//! | `FS_GRPC_PORT`    | `50062`                |

use std::{net::SocketAddr, sync::Arc, time::Duration};

use clap::Parser as _;
use tonic::transport::Server;
use tracing::info;
use tracing_subscriber::{fmt, EnvFilter};

use fs_info::{
    bus_publisher::AlertPublisher,
    cli::{Cli, Command},
    facade::{FsInfo, SystemInfo},
    grpc::{GrpcInfo, InfoServiceServer},
    AlertChecker,
};

// ── Config ────────────────────────────────────────────────────────────────────

struct Config {
    grpc_addr: SocketAddr,
}

impl Config {
    fn from_env() -> Self {
        let grpc_port: u16 = std::env::var("FS_GRPC_PORT")
            .ok()
            .and_then(|p| p.parse().ok())
            .unwrap_or(50_062);
        Self {
            grpc_addr: SocketAddr::from(([0, 0, 0, 0], grpc_port)),
        }
    }
}

// ── Entry point ───────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();

    let args = Cli::parse();
    let cfg = Config::from_env();

    match args.command {
        Command::Daemon => run_daemon(cfg).await?,
        ref cmd => run_cli(cmd),
    }
    Ok(())
}

// ── Daemon ────────────────────────────────────────────────────────────────────

async fn run_daemon(cfg: Config) -> Result<(), Box<dyn std::error::Error>> {
    let bus = Arc::new(fs_bus::MessageBus::new());

    // Spawn the health-alert publisher in the background.
    let publisher = AlertPublisher::with_defaults(Arc::clone(&bus));
    tokio::spawn(async move {
        publisher.run().await;
    });

    info!("gRPC listening on {}", cfg.grpc_addr);
    Server::builder()
        .add_service(InfoServiceServer::new(GrpcInfo::new()))
        .serve(cfg.grpc_addr)
        .await?;

    Ok(())
}

// ── CLI ───────────────────────────────────────────────────────────────────────

fn run_cli(cmd: &Command) {
    let info = FsInfo::new();

    match cmd {
        Command::Daemon => unreachable!(),
        Command::System => {
            let os = info.os();
            let up = info.uptime();
            let cpu = info.cpu();
            println!("Hostname : {}", os.hostname);
            println!("OS       : {} {}", os.os_type.label(), os.version);
            println!("Kernel   : {}", os.kernel);
            println!("Arch     : {}", os.arch);
            println!("Uptime   : {}", up.display());
            println!("CPU      : {} ({} cores)", cpu.brand, cpu.core_count);
        }
        Command::Cpu => {
            let cpu = info.cpu();
            println!(
                "Usage: {:.1}%  Cores: {}  Load: {:.2} / {:.2} / {:.2}",
                cpu.usage_percent,
                cpu.core_count,
                cpu.load_average.one,
                cpu.load_average.five,
                cpu.load_average.fifteen,
            );
        }
        Command::Memory => {
            let mem = info.memory();
            println!(
                "RAM  : {:.1} % used  ({} / {} MB)",
                mem.used_percent(),
                mem.used_bytes / 1_048_576,
                mem.total_bytes / 1_048_576,
            );
            println!(
                "Swap : {}/{} MB",
                mem.swap_used_bytes / 1_048_576,
                mem.swap_total_bytes / 1_048_576,
            );
        }
        Command::Disk => {
            let disk = info.disk();
            for p in &disk.partitions {
                println!(
                    "{:20}  {:5.1}%  ({} / {} GB)",
                    p.mount_point,
                    p.used_percent(),
                    p.used_bytes() / 1_073_741_824,
                    p.total_bytes / 1_073_741_824,
                );
            }
        }
        Command::Alerts { monitor } => {
            let checker = AlertChecker::with_defaults();
            loop {
                let alerts = checker.check_once();
                if alerts.is_empty() {
                    println!("No alerts.");
                } else {
                    for a in &alerts {
                        println!("[{}] {}", a.bus_topic(), a.description());
                    }
                }
                match monitor {
                    Some(secs) => {
                        std::thread::sleep(Duration::from_secs(*secs));
                    }
                    None => break,
                }
            }
        }
    }
}
