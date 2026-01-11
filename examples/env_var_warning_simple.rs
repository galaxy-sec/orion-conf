//! Demonstrates warning output for undefined environment variables (simplified version)
//!
//! Usage:
//! ```bash
//! cargo run --example env_var_warning_simple --features json 2>&1
//! ```

use orion_variate::{EnvDict, ValueType};
use serde::{Deserialize, Serialize};
use std::fs;
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize)]
struct SimpleConfig {
    name: String,
}

fn main() {
    use orion_conf::traits::EnvJsonLoad;

    println!("=== Environment Variable Warning Feature Demo ===\n");

    // Scenario 1: Variable defined - no warning
    println!("Scenario 1: Environment variable defined");
    println!("----------------------------");

    let json_with_var = r#"{"name":"${APP_NAME}"}"#;
    let mut env_dict = EnvDict::new();
    env_dict.insert("APP_NAME", ValueType::from("TestApp"));

    println!("Config: {}", json_with_var);
    println!("Environment variable: APP_NAME = TestApp\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_with_var).unwrap();

    match SimpleConfig::env_load_json(temp_file.path(), &env_dict) {
        Ok(config) => {
            println!("✓ Loaded successfully, name = {}", config.name);
            println!("  (no warnings)\n");
        }
        Err(e) => println!("✗ Load failed: {}\n", e),
    }

    // Scenario 2: Variable undefined - warning will be output to stderr
    println!("Scenario 2: Environment variable undefined");
    println!("----------------------------");

    let env_dict = EnvDict::new(); // Empty dict

    println!("Config: {}", json_with_var);
    println!("Environment variable: (not provided)\n");
    println!("⚠️  Watch for stderr output below:\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_with_var).unwrap();

    match SimpleConfig::env_load_json(temp_file.path(), &env_dict) {
        Ok(config) => {
            println!("\n✓ Loaded successfully, name = {}", config.name);
            println!("  (undefined variable kept as ${{APP_NAME}})\n");
        }
        Err(e) => println!("\n✗ Load failed: {}\n", e),
    }

    // Scenario 3: Using env_parse_json
    println!("Scenario 3: Using env_parse_json to parse string");
    println!("----------------------------");

    let env_dict = EnvDict::new();

    println!("Config string: {}", json_with_var);
    println!("Environment variable: (not provided)\n");
    println!("⚠️  Watch for stderr output below:\n");

    match SimpleConfig::env_parse_json(json_with_var, &env_dict) {
        Ok(config) => {
            println!("\n✓ Parsed successfully, name = {}", config.name);
            println!("  (undefined variable kept as ${{APP_NAME}})\n");
        }
        Err(e) => println!("\n✗ Parse failed: {}\n", e),
    }

    println!("=== Demo Complete ===\n");
    println!("Summary:");
    println!("  - env_load_file and parse_env_string automatically detect undefined environment variables");
    println!("  - Warning format: 'vars not value : VAR1,VAR2,...'");
    println!("  - Warnings output to stderr (eprintln!) and log (log::warn!)");
    println!("  - Undefined variables kept as ${{VAR_NAME}} format");
}
