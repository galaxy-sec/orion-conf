//! Demonstrates warning output for undefined environment variables
//!
//! Usage:
//! ```bash
//! cargo run --example env_var_warning_demo --features json 2>&1
//! ```
//!
//! Note: Warning messages are output to stderr, use 2>&1 to see both stdout and stderr

use orion_variate::{EnvDict, ValueType};
use serde::{Deserialize, Serialize};
use std::fs;
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    app_name: String,
    version: String,
    port: u32,
}

fn main() {
    use orion_conf::traits::EnvJsonLoad;

    println!("=== Environment Variable Warning Demo ===\n");

    // Create JSON config content with environment variables
    let json_content = r#"{
    "app_name": "${APP_NAME}",
    "version": "${VERSION}",
    "port": 8080
}"#;

    println!("Config file content:");
    println!("{}\n", json_content);

    // Scenario 1: All environment variables defined - no warnings
    println!("============================================================");
    println!("Scenario 1: All environment variables defined");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("APP_NAME", ValueType::from("MyApplication"));
    env_dict.insert("VERSION", ValueType::from("v1.0.0"));

    println!("Provided environment variables:");
    println!("  APP_NAME = MyApplication");
    println!("  VERSION = v1.0.0\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("Loading config...");
    let config: Config = Config::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("✓ Loaded successfully (no warnings to stderr)");
    println!("Result:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {}", config.version);
    println!("  port: {}\n", config.port);

    // Scenario 2: Partial environment variables undefined - warnings expected
    println!("============================================================");
    println!("Scenario 2: Partial environment variables undefined");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("APP_NAME", ValueType::from("MyApplication"));
    // VERSION is not defined

    println!("Provided environment variables:");
    println!("  APP_NAME = MyApplication");
    println!("  (VERSION not provided)\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("Loading config...");
    println!("⚠️  Watch for stderr output below (should display: vars not value : VERSION)\n");

    let config: Config = Config::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("\n✓ Loaded successfully (but VERSION was not replaced)");
    println!("Result:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {} (undefined variable kept as-is)", config.version);
    println!("  port: {}\n", config.port);

    // Scenario 3: All environment variables undefined - warnings expected
    println!("============================================================");
    println!("Scenario 3: All environment variables undefined");
    println!("============================================================");

    let env_dict = EnvDict::new(); // Empty dict

    println!("Provided environment variables:");
    println!("  (no environment variables provided)\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("Loading config...");
    println!("⚠️  Watch for stderr output below (should display: vars not value : APP_NAME,VERSION)\n");

    let config: Config = Config::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("\n✓ Loaded successfully (but all variables were not replaced)");
    println!("Result:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {}", config.version);
    println!("  port: {}\n", config.port);

    // Scenario 4: Using env_parse_json to load from inline string (also with warnings)
    println!("============================================================");
    println!("Scenario 4: Using env_parse_json to load from inline string");
    println!("============================================================");

    let env_dict = EnvDict::new();

    println!("Provided environment variables:");
    println!("  (no environment variables provided)\n");

    println!("Parsing config...");
    println!("⚠️  Watch for stderr output below (should display: vars not value : APP_NAME,VERSION)\n");

    let config: Config = Config::env_parse_json(json_content, &env_dict)
        .expect("Failed to parse config");

    println!("\n✓ Parsed successfully");
    println!("Result:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {}", config.version);
    println!("  port: {}\n", config.port);

    println!("============================================================");
    println!("Demo Complete!");
    println!("============================================================");
    println!("\nSummary:");
    println!("  1. Warning message format: 'vars not value : VAR1,VAR2,...'");
    println!("  2. Warnings output to stderr (using eprintln!)");
    println!("  3. Also output to logging system (log::warn!, if configured)");
    println!("  4. Undefined variables kept as ${{VAR_NAME}} format");
    println!("  5. Loading does not fail, only warns\n");
    println!("Tip: To clearly see stderr output, use: cargo run --example env_var_warning_demo --features json 2>&1");
}
