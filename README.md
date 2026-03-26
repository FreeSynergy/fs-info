# fs-info

System information detection and alerting for FreeSynergy.

## What it does

- **Static data** (cached 24h): OS type, version, architecture, kernel, hostname, detected features
- **Dynamic data** (on demand): disk partitions, RAM/swap usage, CPU temperature sensors
- **Alerting**: compare live metrics against configurable thresholds
- **Optional** (`feature = "smart"`): SMART disk health via `smartctl`

## Usage

```rust
use fs_info::{OsInfo, MemInfo, DiskInfo, SysInfoCache};

// Static OS info (cached)
let cache = SysInfoCache::default_path();
let (os, features) = cache.get_or_detect();
println!("{} {} ({})", os.os_type, os.version, os.arch);

// Dynamic memory info
let mem = MemInfo::detect();
println!("RAM: {:.1}% used", mem.used_percent());

// Disk partitions
let disks = DiskInfo::detect();
for p in &disks.partitions {
    println!("{}: {:.1}% used", p.mount_point, p.used_percent());
}
```

## Build

```sh
cargo build
cargo test
cargo clippy -- -D warnings
```

## License

MIT — see [LICENSE](LICENSE)
