# CLAUDE.md – fs-info

## What is this?

FreeSynergy System Info — detects and reports CPU, memory, disk, thermal, and system feature
information. Used by Store, Desktop widgets, Managers, and other programs.

## Rules

- Language in files: **English** (comments, code, variable names)
- Language in chat: **German**
- OOP everywhere: traits over match blocks, types carry their own behavior
- No CHANGELOG.md
- After every feature: commit directly

## Quality Gates (before every commit)

```
1. Design Pattern (Traits, Object hierarchy)
2. Structs + Traits — no impl code yet
3. cargo check
4. Impl (OOP)
5. cargo clippy --all-targets -- -D warnings
6. cargo fmt --check
7. Unit tests (min. 1 per public module)
8. cargo test
9. commit + push
```

Every lib.rs / main.rs must have:
```rust
#![deny(clippy::all, clippy::pedantic)]
#![deny(warnings)]
```

## Architecture

- `OsInfo`           — OS type, version, architecture, kernel, hostname
- `DetectedFeatures` — which system features are present (systemd, Podman, …)
- `SysInfoCache`     — persistent cache (~/.config/fsn/sysinfo.toml, TTL 24h)
- `DiskInfo`         — partition list with used/free bytes
- `MemInfo`          — RAM and swap usage
- `ThermalInfo`      — CPU temperature sensors
- `AlertChecker`     — compares live metrics against configurable thresholds

## Features

- `smart` — enable SMART disk health queries via `smartctl`

## Notes

- Whether this becomes a Daemon, Bus-subscriber, or pure Library is still open (see G8)
- Extracted from `fs-libs/fs-sysinfo`
