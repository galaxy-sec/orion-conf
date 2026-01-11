//! 演示环境变量未定义时的警告输出（简化版）
//!
//! 运行方式：
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

    println!("=== 环境变量警告功能演示 ===\n");

    // 场景 1: 变量已定义 - 不会有警告
    println!("场景 1: 环境变量已定义");
    println!("----------------------------");

    let json_with_var = r#"{"name":"${APP_NAME}"}"#;
    let mut env_dict = EnvDict::new();
    env_dict.insert("APP_NAME", ValueType::from("TestApp"));

    println!("配置: {}", json_with_var);
    println!("环境变量: APP_NAME = TestApp\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_with_var).unwrap();

    match SimpleConfig::env_load_json(temp_file.path(), &env_dict) {
        Ok(config) => {
            println!("✓ 加载成功，name = {}", config.name);
            println!("  (没有警告)\n");
        }
        Err(e) => println!("✗ 加载失败: {}\n", e),
    }

    // 场景 2: 变量未定义 - 会有警告输出到 stderr
    println!("场景 2: 环境变量未定义");
    println!("----------------------------");

    let env_dict = EnvDict::new(); // 空字典

    println!("配置: {}", json_with_var);
    println!("环境变量: (未提供)\n");
    println!("⚠️  注意下方 stderr 输出:\n");

    let temp_file = NamedTempFile::new().unwrap();
    fs::write(temp_file.path(), json_with_var).unwrap();

    match SimpleConfig::env_load_json(temp_file.path(), &env_dict) {
        Ok(config) => {
            println!("\n✓ 加载成功，name = {}", config.name);
            println!("  (未定义的变量保持为 ${{APP_NAME}})\n");
        }
        Err(e) => println!("\n✗ 加载失败: {}\n", e),
    }

    // 场景 3: 使用 env_parse_json
    println!("场景 3: 使用 env_parse_json 解析字符串");
    println!("----------------------------");

    let env_dict = EnvDict::new();

    println!("配置字符串: {}", json_with_var);
    println!("环境变量: (未提供)\n");
    println!("⚠️  注意下方 stderr 输出:\n");

    match SimpleConfig::env_parse_json(json_with_var, &env_dict) {
        Ok(config) => {
            println!("\n✓ 解析成功，name = {}", config.name);
            println!("  (未定义的变量保持为 ${{APP_NAME}})\n");
        }
        Err(e) => println!("\n✗ 解析失败: {}\n", e),
    }

    println!("===演示完成 ===\n");
    println!("总结:");
    println!("  - env_load_file 和 parse_env_string 会自动检测未定义的环境变量");
    println!("  - 警告格式: 'vars not value : VAR1,VAR2,...'");
    println!("  - 警告输出到 stderr（eprintln!）和日志（log::warn!）");
    println!("  - 未定义的变量会保持 ${{VAR_NAME}} 格式");
}
