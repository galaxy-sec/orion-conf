//! 演示使用包含复杂值（如数据库 URL）的环境变量
//!
//! 运行方式：
//! ```bash
//! cargo run --example env_var_with_urls --features json 2>&1
//! ```
//!
//! 注意：此示例在 orion-variate v0.10.8+ 中正常工作
//!       v0.10.7 及以下版本在处理包含 :// 的值时存在 bug

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

    println!("=== 使用复杂 URL 值的环境变量演示 ===\n");

    // 创建包含 URL 类型环境变量的配置
    let json_content = r#"{
    "database_url": "${DATABASE_URL}",
    "api_endpoint": "${API_ENDPOINT}",
    "timeout_secs": 30
}"#;

    println!("配置文件内容：");
    println!("{}\n", json_content);

    // 场景 1: 使用包含 :// 的完整 URL
    println!("============================================================");
    println!("场景 1: 使用包含 :// 的数据库 URL");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DATABASE_URL", ValueType::from("postgresql://user:pass@localhost:5432/mydb"));
    env_dict.insert("API_ENDPOINT", ValueType::from("https://api.example.com/v1"));

    println!("环境变量:");
    println!("  DATABASE_URL = postgresql://user:pass@localhost:5432/mydb");
    println!("  API_ENDPOINT = https://api.example.com/v1\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("加载配置...");
    let config: DatabaseConfig = DatabaseConfig::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("✓ 加载成功（无警告）");
    println!("结果:");
    println!("  database_url: {}", config.database_url);
    println!("  api_endpoint: {}", config.api_endpoint);
    println!("  timeout_secs: {}\n", config.timeout_secs);

    // 场景 2: 部分 URL 未定义
    println!("============================================================");
    println!("场景 2: 部分 URL 未定义");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DATABASE_URL", ValueType::from("postgresql://localhost/mydb"));
    // API_ENDPOINT 未定义

    println!("环境变量:");
    println!("  DATABASE_URL = postgresql://localhost/mydb");
    println!("  (API_ENDPOINT 未提供)\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("加载配置...");
    println!("⚠️  注意 stderr 输出:\n");

    let config: DatabaseConfig = DatabaseConfig::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("\n✓ 加载成功");
    println!("结果:");
    println!("  database_url: {}", config.database_url);
    println!("  api_endpoint: {} (未定义)", config.api_endpoint);
    println!("  timeout_secs: {}\n", config.timeout_secs);

    // 场景 3: 使用不同协议的 URL
    println!("============================================================");
    println!("场景 3: 使用多种协议的 URL");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("DATABASE_URL", ValueType::from("mongodb://admin:password@mongo-server:27017/production"));
    env_dict.insert("API_ENDPOINT", ValueType::from("http://internal-api:8080/graphql"));

    println!("环境变量:");
    println!("  DATABASE_URL = mongodb://admin:password@mongo-server:27017/production");
    println!("  API_ENDPOINT = http://internal-api:8080/graphql\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    let config: DatabaseConfig = DatabaseConfig::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("✓ 加载成功");
    println!("结果:");
    println!("  database_url: {}", config.database_url);
    println!("  api_endpoint: {}", config.api_endpoint);
    println!("  timeout_secs: {}\n", config.timeout_secs);

    println!("============================================================");
    println!("演示完成！");
    println!("============================================================");
    println!("\n说明：");
    println!("  ✓ orion-variate v0.10.8 正确处理包含 :// 的 URL 值");
    println!("  ✓ 支持各种协议: postgresql://, https://, mongodb://, http:// 等");
    println!("  ✓ 未定义的 URL 变量会触发警告但不会失败");
}
