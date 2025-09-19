use orion_conf::{Configable, JsonAble};
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
    enabled: bool,
}

fn main() {
    let path = Path::new("test_config.json");

    // 创建测试配置
    let config = TestConfig {
        name: "test_app".to_string(),
        value: 42,
        enabled: true,
    };

    // 测试 JSON 格式保存
    match config.save_json(path) {
        Ok(()) => println!("Successfully saved JSON config"),
        Err(e) => println!("Failed to save JSON config: {}", e),
    }

    // 测试 JSON 格式加载
    match TestConfig::from_json(path) {
        Ok(loaded_config) => {
            println!("Successfully loaded JSON config: {:?}", loaded_config);
            assert_eq!(config, loaded_config);
            println!("JSON round-trip test passed!");
        }
        Err(e) => println!("Failed to load JSON config: {}", e),
    }

    // 测试 Configable 默认行为（应该使用 JSON，因为只启用了 json 特性）
    match TestConfig::from_conf(path) {
        Ok(loaded_config) => {
            println!(
                "Successfully loaded config using Configable: {:?}",
                loaded_config
            );
            assert_eq!(config, loaded_config);
            println!("Configable with JSON feature test passed!");
        }
        Err(e) => println!("Failed to load config using Configable: {}", e),
    }

    // 清理测试文件
    let _ = std::fs::remove_file(path);
}
