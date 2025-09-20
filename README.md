# orion-conf
[![CI](https://github.com/galaxy-sec/orion-conf/workflows/CI/badge.svg)](https://github.com/galaxy-sec/orion-conf/actions)
[![Coverage Status](https://codecov.io/gh/galaxy-sec/orion-conf/branch/main/graph/badge.svg)](https://codecov.io/gh/galaxy-sec/orion-conf)
[![crates.io](https://img.shields.io/crates/v/orion-conf.svg)](https://crates.io/crates/orion-conf)

Lightweight, feature‑gated config IO for Rust. Enable only the formats you need.

Quick Start

```rust
use orion_conf::{ConfigIO, YamlIO};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct AppCfg { name: String, value: i32 }

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cargo run --features yaml
    let cfg = AppCfg { name: "demo".into(), value: 7 };
    let path = std::path::Path::new("app.yaml");

    cfg.save_conf(path)?;                 // delegates by feature priority (YAML > TOML > JSON > INI)
    let loaded = AppCfg::load_conf(path)?; // same priority for loading

    // Or call the format explicitly
    cfg.save_yaml(path)?;
    let _ = AppCfg::load_yaml(path)?;
    Ok(())
}
```

Features

- Formats (opt‑in): `yaml`, `toml`, `json`, `ini`
- Combined: `formats` (all), `full` (all)
- Default features: empty

Key Traits (0.3+)

- `ConfigIO`: unified read/write with feature‑based priority
- `YamlIO`/`TomlIO`/`JsonIO`/`IniIO`: explicit format IO

Examples

- JSON only: `cargo run --example json_feature_test --features json`
- YAML+JSON priority: `cargo run --example feature_priority_test --features yaml,json`

Migration

- See MIGRATION.md for 0.2.x → 0.3.0 changes and API replacements.
