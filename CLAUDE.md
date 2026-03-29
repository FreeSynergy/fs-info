# CLAUDE.md – fs-info

## What is this?

FreeSynergy System Info — detects and reports CPU, memory, disk, network, thermal, and uptime
information. Used by Store, Desktop widgets, Managers, and other programs.

## Rules

- Language in files: **English** (comments, code, variable names)
- Language in chat: **German**
- OOP everywhere: traits over match blocks, types carry their own behavior
- No CHANGELOG.md
- After every feature: commit directly

## Quality Gates (before every commit)

```
cargo clippy --all-targets -- -D warnings
cargo fmt --check
cargo test
```

Every lib.rs / main.rs must have:
```rust
#![deny(clippy::all, clippy::pedantic, warnings)]
```

## Architecture

- `SystemInfo` — **trait**: unified interface for all system info queries
- `FsInfo` — facade implementing `SystemInfo` + `MetricsCollector`
- `MetricsCollector` — **trait**: `collect() → Vec<Metric>`
- `Metric` — named numeric value with unit
- `CpuInfo` — CPU usage, load average, core count
- `MemInfo` — RAM and swap usage
- `DiskInfo` — partition list with used/free bytes
- `NetworkInfo` — network interface statistics
- `Uptime` — system uptime
- `ThermalInfo` — CPU temperature sensors
- `OsInfo` — OS type, version, architecture, kernel, hostname
- `DetectedFeatures` — which system features are present (systemd, Podman, …)
- `SysInfoCache` — persistent cache (~/.config/fsn/sysinfo.toml, TTL 24h)
- `AlertChecker` — compares live metrics against configurable thresholds

## Features

- `smart` — enable SMART disk health queries via `smartctl`

## Notes

- Whether this becomes a Daemon, Bus-subscriber, or pure Library is still open (see G8)
- Extracted from `fs-libs/fs-sysinfo`
