use orion_conf::ConfigIO;
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize)]
struct TestConfig {
    name: String,
    value: i32,
}

fn main() {
    let path = Path::new("test_config.json");

    // 这应该返回错误，因为没有启用任何特性
    match TestConfig::load_conf(path) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {}", e),
    }

    let config = TestConfig {
        name: "test".to_string(),
        value: 42,
    };

    match config.save_conf(path) {
        Ok(_) => println!("Unexpected success"),
        Err(e) => println!("Expected error: {}", e),
    }
}
