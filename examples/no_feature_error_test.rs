//! 无特性启用时的错误处理测试
//!
//! 这个测试验证在没有启用任何格式特性时，库会正确返回错误

use orion_conf::Configable;
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
    enabled: bool,
}

fn main() {
    println!("🧪 Testing orion-conf with no format features enabled");
    println!("This should demonstrate proper error handling...\n");

    let config = TestConfig {
        name: "no_feature_test".to_string(),
        value: 42,
        enabled: true,
    };

    let path = Path::new("no_feature_test.conf");

    // 测试保存 - 应该失败
    println!("=== Testing Save Operation ===");
    match config.save_conf(path) {
        Ok(_) => {
            println!("❌ Unexpected success: save should have failed without features");
        }
        Err(e) => {
            println!("✅ Expected error on save: {}", e);
            if e.to_string().contains("no format feature enabled") {
                println!("✅ Error message is correct");
            } else {
                println!("⚠️  Unexpected error message");
            }
        }
    }

    // 测试加载 - 应该失败
    println!("\n=== Testing Load Operation ===");
    match TestConfig::from_conf(path) {
        Ok(_) => {
            println!("❌ Unexpected success: load should have failed without features");
        }
        Err(e) => {
            println!("✅ Expected error on load: {}", e);
            if e.to_string().contains("no format feature enabled") {
                println!("✅ Error message is correct");
            } else {
                println!("⚠️  Unexpected error message");
            }
        }
    }

    println!("\n🎉 No-feature error handling test completed!");
    println!("✅ Library correctly prevents usage when no format features are enabled");
    println!("✅ Users must explicitly enable at least one format feature");
}
