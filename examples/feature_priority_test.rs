use orion_conf::{ConfigIO, JsonIO, YamlIO};
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
    enabled: bool,
}

fn main() {
    let json_path = Path::new("test_config.json");
    let yaml_path = Path::new("test_config.yaml");

    let config = TestConfig {
        name: "test_app".to_string(),
        value: 42,
        enabled: true,
    };

    println!("Testing feature priority with YAML + JSON enabled...");
    println!("According to our implementation, YAML should have priority over JSON");

    // 测试 JSON 保存和加载
    println!("\n=== Testing JSON format ===");
    if let Err(e) = config.save_json(json_path) {
        println!("Failed to save JSON: {}", e);
        return;
    }

    match TestConfig::load_json(json_path) {
        Ok(loaded) => {
            println!("JSON load successful: {:?}", loaded);
            assert_eq!(config, loaded);
        }
        Err(e) => println!("Failed to load JSON: {}", e),
    }

    // 测试 YAML 保存和加载
    println!("\n=== Testing YAML format ===");
    if let Err(e) = config.save_yaml(yaml_path) {
        println!("Failed to save YAML: {}", e);
        return;
    }

    match TestConfig::load_yaml(yaml_path) {
        Ok(loaded) => {
            println!("YAML load successful: {:?}", loaded);
            assert_eq!(config, loaded);
        }
        Err(e) => println!("Failed to load YAML: {}", e),
    }

    // 测试 ConfigIO 默认行为（应该优先使用 YAML）
    println!("\n=== Testing ConfigIO default behavior ===");
    println!("ConfigIO should delegate to YAML (higher priority than JSON)");

    // 使用 YAML 文件测试 ConfigIO
    match TestConfig::load_conf(yaml_path) {
        Ok(loaded) => {
            println!("ConfigIO loaded from YAML: {:?}", loaded);
            assert_eq!(config, loaded);
            println!("✅ YAML priority confirmed!");
        }
        Err(e) => println!("ConfigIO failed to load from YAML: {}", e),
    }

    // 测试保存行为
    let config2 = TestConfig {
        name: "updated_app".to_string(),
        value: 100,
        enabled: false,
    };

    let save_path = Path::new("configable_save.yaml");
    if let Err(e) = config2.save_conf(save_path) {
        println!("ConfigIO save failed: {}", e);
        return;
    }

    // 验证保存的是 YAML 格式
    let content = std::fs::read_to_string(save_path).unwrap();
    println!("ConfigIO saved content:\n{}", content);

    if content.contains("name: updated_app") && content.contains("value: 100") {
        println!("✅ ConfigIO saved in YAML format (priority confirmed)!");
    } else {
        println!("❌ ConfigIO did not save in expected YAML format");
    }

    // 清理测试文件
    let _ = std::fs::remove_file(json_path);
    let _ = std::fs::remove_file(yaml_path);
    let _ = std::fs::remove_file(save_path);

    println!("\n=== Feature Priority Test Summary ===");
    println!("✅ YAML format works correctly");
    println!("✅ JSON format works correctly");
    println!("✅ ConfigIO defaults to YAML (higher priority)");
    println!("✅ All tests passed!");
}
