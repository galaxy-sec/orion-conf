//! Demonstrates using environment variables with complex values (like database URLs)
//!
//! Usage:
//! ```bash
//! cargo run --example env_var_with_urls --features json 2>&1
//! ```
//!
//! Note: This example works correctly in orion-variate v0.10.8+
//!       Versions v0.10.7 and below had bugs when handling values containing ://

use orion_variate::{EnvDict, ValueType};
use serde::{Deserialize, Serialize};
use std::fs;
use tempfile::NamedTempFile;

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    database_url: String,
    api_endpoint: String,
    timeout_secs: u32,
}

fn main() {
    use orion_conf::traits::EnvJsonLoad;

    println!("=== Environment Variables with Complex URL Values Demo ===\n");

    // Create config with URL-type environment variables
    let json_content = r#"{
    "database_url": "${DATABASE_URL}",
    "api_endpoint": "${API_ENDPOINT}",
    "timeout_secs": 30
}"#;

    println!("Config file content:");
    println!("{}\n", json_content);

    // Scenario 1: Using full URLs with ://
    println!("============================================================");
    println!("Scenario 1: Using database URLs with ://");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DATABASE_URL", ValueType::from("postgresql://user:pass@localhost:5432/mydb"));
    env_dict.insert("API_ENDPOINT", ValueType::from("https://api.example.com/v1"));

    println!("Environment variables:");
    println!("  DATABASE_URL = postgresql://user:pass@localhost:5432/mydb");
    println!("  API_ENDPOINT = https://api.example.com/v1\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("Loading config...");
    let config: DatabaseConfig = DatabaseConfig::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("✓ Loaded successfully (no warnings)");
    println!("Result:");
    println!("  database_url: {}", config.database_url);
    println!("  api_endpoint: {}", config.api_endpoint);
    println!("  timeout_secs: {}\n", config.timeout_secs);

    // Scenario 2: Partial URLs undefined
    println!("============================================================");
    println!("Scenario 2: Partial URLs undefined");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DATABASE_URL", ValueType::from("postgresql://localhost/mydb"));
    // API_ENDPOINT is not defined

    println!("Environment variables:");
    println!("  DATABASE_URL = postgresql://localhost/mydb");
    println!("  (API_ENDPOINT not provided)\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("Loading config...");
    println!("⚠️  Watch for stderr output:\n");

    let config: DatabaseConfig = DatabaseConfig::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("\n✓ Loaded successfully");
    println!("Result:");
    println!("  database_url: {}", config.database_url);
    println!("  api_endpoint: {} (undefined)", config.api_endpoint);
    println!("  timeout_secs: {}\n", config.timeout_secs);

    // Scenario 3: Using various protocol URLs
    println!("============================================================");
    println!("Scenario 3: Using multiple protocol URLs");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DATABASE_URL", ValueType::from("mongodb://admin:password@mongo-server:27017/production"));
    env_dict.insert("API_ENDPOINT", ValueType::from("http://internal-api:8080/graphql"));

    println!("Environment variables:");
    println!("  DATABASE_URL = mongodb://admin:password@mongo-server:27017/production");
    println!("  API_ENDPOINT = http://internal-api:8080/graphql\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    let config: DatabaseConfig = DatabaseConfig::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("✓ Loaded successfully");
    println!("Result:");
    println!("  database_url: {}", config.database_url);
    println!("  api_endpoint: {}", config.api_endpoint);
    println!("  timeout_secs: {}\n", config.timeout_secs);

    println!("============================================================");
    println!("Demo Complete!");
    println!("============================================================");
    println!("\nNotes:");
    println!("  ✓ orion-variate v0.10.8 correctly handles URL values containing ://");
    println!("  ✓ Supports various protocols: postgresql://, https://, mongodb://, http://, etc.");
    println!("  ✓ Undefined URL variables trigger warnings but do not fail");
}
