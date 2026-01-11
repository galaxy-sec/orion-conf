//! 演示环境变量未定义时的警告输出
//!
//! 运行方式：
//! ```bash
//! cargo run --example env_var_warning_demo --features json 2>&1
//! ```
//!
//! 注意：警告信息会输出到 stderr，使用 2>&1 可以同时看到 stdout 和 stderr

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

    println!("=== 环境变量警告演示 ===\n");

    // 创建包含环境变量的 JSON 配置内容
    let json_content = r#"{
    "app_name": "${APP_NAME}",
    "version": "${VERSION}",
    "port": 8080
}"#;

    println!("配置文件内容：");
    println!("{}\n", json_content);

    // 场景 1: 所有环境变量都已定义 - 不应该有警告
    println!("============================================================");
    println!("场景 1: 所有环境变量都已定义");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("APP_NAME", ValueType::from("MyApplication"));
    env_dict.insert("VERSION", ValueType::from("v1.0.0"));

    println!("提供的环境变量:");
    println!("  APP_NAME = MyApplication");
    println!("  VERSION = v1.0.0\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("加载配置...");
    let config: Config = Config::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("✓ 加载成功（应该没有警告输出到 stderr）");
    println!("结果:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {}", config.version);
    println!("  port: {}\n", config.port);

    // 场景 2: 部分环境变量未定义 - 应该有警告
    println!("============================================================");
    println!("场景 2: 部分环境变量未定义");
    println!("============================================================");

    let mut env_dict = EnvDict::new();
    env_dict.insert("APP_NAME", ValueType::from("MyApplication"));
    // VERSION 未定义

    println!("提供的环境变量:");
    println!("  APP_NAME = MyApplication");
    println!("  (VERSION 未提供)\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("加载配置...");
    println!("⚠️  注意观察下方 stderr 输出（应该会显示: vars not value : VERSION）\n");

    let config: Config = Config::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("\n✓ 加载成功（但 VERSION 未被替换）");
    println!("结果:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {} (未定义的变量保持原样)", config.version);
    println!("  port: {}\n", config.port);

    // 场景 3: 所有环境变量都未定义 - 应该有警告
    println!("============================================================");
    println!("场景 3: 所有环境变量都未定义");
    println!("============================================================");

    let env_dict = EnvDict::new(); // 空字典

    println!("提供的环境变量:");
    println!("  (未提供任何环境变量)\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_content).unwrap();

    println!("加载配置...");
    println!("⚠️  注意观察下方 stderr 输出（应该会显示: vars not value : APP_NAME,VERSION）\n");

    let config: Config = Config::env_load_json(temp_file.path(), &env_dict)
        .expect("Failed to load config");

    println!("\n✓ 加载成功（但所有变量都未被替换）");
    println!("结果:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {}", config.version);
    println!("  port: {}\n", config.port);

    // 场景 4: 使用 env_parse_json 从字符串加载（也应该有警告）
    println!("============================================================");
    println!("场景 4: 使用 env_parse_json 从内联字符串加载");
    println!("============================================================");

    let env_dict = EnvDict::new();

    println!("提供的环境变量:");
    println!("  (未提供任何环境变量)\n");

    println!("解析配置...");
    println!("⚠️  注意观察下方 stderr 输出（应该会显示: vars not value : APP_NAME,VERSION）\n");

    let config: Config = Config::env_parse_json(json_content, &env_dict)
        .expect("Failed to parse config");

    println!("\n✓ 解析成功");
    println!("结果:");
    println!("  app_name: {}", config.app_name);
    println!("  version: {}", config.version);
    println!("  port: {}\n", config.port);

    println!("============================================================");
    println!("演示完成！");
    println!("============================================================");
    println!("\n总结：");
    println!("  1. 警告信息格式: 'vars not value : VAR1,VAR2,...'");
    println!("  2. 警告输出到 stderr（使用 eprintln!）");
    println!("  3. 同时也会输出到日志系统（log::warn!，如果配置了日志）");
    println!("  4. 未定义的变量在配置中保持 ${{VAR_NAME}} 格式");
    println!("  5. 加载不会失败，只是会发出警告\n");
    println!("提示：要清楚地看到 stderr 输出，请使用: cargo run --example env_var_warning_demo --features json 2>&1");
}
