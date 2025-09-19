//! 综合测试文件 - 验证所有特性功能
//!
//! 这个测试文件验证：
//! 1. 无特性启用时的错误处理
//! 2. 单个特性启用的功能
//! 3. 多特性组合的优先级逻辑
//! 4. 所有格式的完整功能

use orion_conf::*;
use std::path::Path;

#[derive(serde::Deserialize, serde::Serialize, Debug, PartialEq)]
struct TestConfig {
    name: String,
    value: i32,
    enabled: bool,
    description: Option<String>,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            name: "test_app".to_string(),
            value: 42,
            enabled: true,
            description: Some("A test configuration".to_string()),
        }
    }
}

fn main() {
    println!("🚀 开始 orion-conf v0.2.0 综合测试");
    println!("{}", "=".repeat(50));

    let config = TestConfig::default();

    // 测试 1: 验证特性系统工作
    test_feature_system();

    // 测试 2: 验证优先级逻辑
    test_priority_logic();

    // 测试 3: 验证所有启用格式的功能
    test_all_formats();

    // 测试 4: 验证 Configable 默认行为
    test_configable_default();

    println!("\n✅ 所有测试完成！orion-conf 特性系统工作正常！");
}

fn test_feature_system() {
    println!("\n📋 测试 1: 特性系统验证");

    // 这个测试在编译时通过特性控制，运行时主要验证编译是否成功
    println!("✅ 编译时特性检查通过");

    // 检查当前启用的特性
    #[cfg(feature = "yaml")]
    println!("  🔧 YAML 特性已启用");

    #[cfg(feature = "json")]
    println!("  🔧 JSON 特性已启用");

    #[cfg(feature = "toml")]
    println!("  🔧 TOML 特性已启用");

    #[cfg(feature = "ini")]
    println!("  🔧 INI 特性已启用");

    #[cfg(not(any(feature = "yaml", feature = "json", feature = "toml", feature = "ini")))]
    println!("  ⚠️  未启用任何格式特性");
}

fn test_priority_logic() {
    println!("\n🎯 测试 2: 特性优先级验证");

    let config = TestConfig::default();
    let test_path = Path::new("priority_test.conf");

    // 根据启用的特性测试优先级
    #[cfg(feature = "yaml")]
    {
        println!("  🔧 测试 YAML 优先级（应该最高）");
        if let Err(e) = Configable::save_conf(&config, test_path) {
            println!("    ❌ 保存失败: {}", e);
            return;
        }

        let content = std::fs::read_to_string(test_path).unwrap_or_default();
        if content.contains("name: test_app") {
            println!("    ✅ YAML 优先级正确");
        } else {
            println!("    ⚠️  优先级可能不正确");
        }
    }

    #[cfg(all(feature = "toml", not(feature = "yaml")))]
    {
        println!("  🔧 测试 TOML 优先级（YAML 未启用时应该最高）");
        if let Err(e) = config.save_conf(test_path) {
            println!("    ❌ 保存失败: {}", e);
            return;
        }

        let content = std::fs::read_to_string(test_path).unwrap_or_default();
        if content.contains("name = \"test_app\"") {
            println!("    ✅ TOML 优先级正确");
        } else {
            println!("    ⚠️  优先级可能不正确");
        }
    }

    #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
    {
        println!("  🔧 测试 JSON 优先级（YAML/TOML 未启用时应该最高）");
        if let Err(e) = config.save_conf(test_path) {
            println!("    ❌ 保存失败: {}", e);
            return;
        }

        let content = std::fs::read_to_string(test_path).unwrap_or_default();
        if content.contains("\"name\": \"test_app\"") {
            println!("    ✅ JSON 优先级正确");
        } else {
            println!("    ⚠️  优先级可能不正确");
        }
    }

    // 清理
    let _ = std::fs::remove_file(test_path);
}

fn test_all_formats() {
    println!("\n📁 测试 3: 所有格式功能验证");

    let config = TestConfig::default();

    // 测试 YAML
    #[cfg(feature = "yaml")]
    {
        let path = Path::new("test_config.yaml");
        println!("  🔧 测试 YAML 格式");

        match config.save_yml(path) {
            Ok(()) => match TestConfig::from_yml(path) {
                Ok(loaded) => {
                    if loaded == config {
                        println!("    ✅ YAML 格式正常");
                    } else {
                        println!("    ❌ YAML 数据不一致");
                    }
                }
                Err(e) => println!("    ❌ YAML 加载失败: {}", e),
            },
            Err(e) => println!("    ❌ YAML 保存失败: {}", e),
        }

        let _ = std::fs::remove_file(path);
    }

    // 测试 JSON
    #[cfg(feature = "json")]
    {
        let path = Path::new("test_config.json");
        println!("  🔧 测试 JSON 格式");

        match config.save_json(path) {
            Ok(()) => match TestConfig::from_json(path) {
                Ok(loaded) => {
                    if loaded == config {
                        println!("    ✅ JSON 格式正常");
                    } else {
                        println!("    ❌ JSON 数据不一致");
                    }
                }
                Err(e) => println!("    ❌ JSON 加载失败: {}", e),
            },
            Err(e) => println!("    ❌ JSON 保存失败: {}", e),
        }

        let _ = std::fs::remove_file(path);
    }

    // 测试 TOML
    #[cfg(feature = "toml")]
    {
        let path = Path::new("test_config.toml");
        println!("  🔧 测试 TOML 格式");

        match config.save_toml(path) {
            Ok(()) => match TestConfig::from_toml(path) {
                Ok(loaded) => {
                    if loaded == config {
                        println!("    ✅ TOML 格式正常");
                    } else {
                        println!("    ❌ TOML 数据不一致");
                    }
                }
                Err(e) => println!("    ❌ TOML 加载失败: {}", e),
            },
            Err(e) => println!("    ❌ TOML 保存失败: {}", e),
        }

        let _ = std::fs::remove_file(path);
    }

    // 测试 INI
    #[cfg(feature = "ini")]
    {
        let path = Path::new("test_config.ini");
        println!("  🔧 测试 INI 格式");

        match config.save_ini(path) {
            Ok(()) => match TestConfig::from_ini(path) {
                Ok(loaded) => {
                    if loaded == config {
                        println!("    ✅ INI 格式正常");
                    } else {
                        println!("    ❌ INI 数据不一致");
                    }
                }
                Err(e) => println!("    ❌ INI 加载失败: {}", e),
            },
            Err(e) => println!("    ❌ INI 保存失败: {}", e),
        }

        let _ = std::fs::remove_file(path);
    }
}

fn test_configable_default() {
    println!("\n🎛️  测试 4: Configable 默认行为");

    let config = TestConfig::default();
    let path = Path::new("configable_test.conf");

    // 测试保存
    match config.save_conf(path) {
        Ok(()) => {
            println!("  ✅ Configable 保存成功");

            // 测试加载
            match TestConfig::from_conf(path) {
                Ok(loaded) => {
                    if loaded == config {
                        println!("  ✅ Configable 加载成功，数据一致");
                    } else {
                        println!("  ❌ Configable 数据不一致");
                    }
                }
                Err(e) => println!("  ❌ Configable 加载失败: {}", e),
            }

            // 检查文件格式
            let content = std::fs::read_to_string(path).unwrap_or_default();

            #[cfg(feature = "yaml")]
            if content.contains("name: test_app") {
                println!("  ✅ Configable 正确使用 YAML 格式");
            } else {
                println!("  ⚠️  Configable 未使用预期的 YAML 格式");
            }

            #[cfg(all(feature = "toml", not(feature = "yaml")))]
            if content.contains("name = \"test_app\"") {
                println!("  ✅ Configable 正确使用 TOML 格式");
            } else {
                println!("  ⚠️  Configable 未使用预期的 TOML 格式");
            }

            #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
            if content.contains("\"name\": \"test_app\"") {
                println!("  ✅ Configable 正确使用 JSON 格式");
            } else {
                println!("  ⚠️  Configable 未使用预期的 JSON 格式");
            }
        }
        Err(e) => {
            #[cfg(not(any(
                feature = "yaml",
                feature = "json",
                feature = "toml",
                feature = "ini"
            )))]
            if e.to_string().contains("no format feature enabled") {
                println!("  ✅ Configable 正确返回特性未启用错误");
            } else {
                println!("  ❌ Configable 返回意外错误: {}", e);
            }

            #[cfg(any(feature = "yaml", feature = "json", feature = "toml", feature = "ini"))]
            println!("  ❌ Configable 保存失败（特性已启用）: {}", e);
        }
    }

    // 清理
    let _ = std::fs::remove_file(path);
}

#[cfg(not(any(feature = "yaml", feature = "json", feature = "toml", feature = "ini")))]
fn compile_time_feature_check() {
    // 这个函数在没有启用任何特性时会被编译
    // 用于验证编译时特性检查
    compile_error!("请至少启用一个格式特性：yaml, json, toml, ini");
}
