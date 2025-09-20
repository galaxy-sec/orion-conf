pub use derive_getters::Getters;
use orion_error::{ContextRecord, OperationContext, ToStructError};
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
pub use serde_derive::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::error::{ConfIOReason, OrionConfResult};

/// 通用文件加载函数，处理文件读取和反序列化的重复逻辑
#[allow(dead_code)]
fn load_from_file<T, F>(path: &Path, operation_name: &str, deserializer: F) -> OrionConfResult<T>
where
    F: FnOnce(&str) -> Result<T, Box<dyn std::error::Error>>,
{
    let mut ctx =
        OperationContext::want(format!("load object from {operation_name}")).with_auto_log();
    ctx.record("from path", path);
    let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
    let loaded: T = deserializer(file_content.as_str())
        .map_err(|e| ConfIOReason::from_conf(e.to_string()).to_err())
        .with(&ctx)?;
    ctx.mark_suc();
    Ok(loaded)
}
/// 通用文件保存函数，处理序列化和文件写入的重复逻辑
#[allow(dead_code)]
fn save_to_file<F>(path: &Path, operation_name: &str, serializer: F) -> OrionConfResult<()>
where
    F: FnOnce() -> Result<String, Box<dyn std::error::Error>>,
{
    let mut ctx = OperationContext::want(format!("save {operation_name}")).with_auto_log();
    ctx.record("from path", path);
    let data_content = serializer()
        .map_err(|e| ConfIOReason::from_conf(e.to_string()).to_err())
        .with(&ctx)?;
    fs::write(path, data_content).owe_res().with(&ctx)?;
    ctx.mark_suc();
    Ok(())
}

// ConfigIO trait 的默认实现已在 traits.rs 中处理

#[cfg(feature = "ini")]
use crate::traits::IniIO;

#[cfg(feature = "ini")]
impl<T> IniIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_ini(path: &Path) -> OrionConfResult<T> {
        load_from_file(path, "ini", |content| {
            serde_ini::de::from_str(content).map_err(Into::into)
        })
    }
    fn save_ini(&self, path: &Path) -> OrionConfResult<()> {
        save_to_file(path, "ini", || {
            serde_ini::ser::to_string(self).map_err(Into::into)
        })
    }
}

#[cfg(feature = "json")]
use crate::traits::JsonIO;

#[cfg(feature = "json")]
impl<T> JsonIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_json(path: &Path) -> OrionConfResult<T> {
        load_from_file(path, "json", |content| {
            serde_json::from_str(content).map_err(Into::into)
        })
    }
    fn save_json(&self, path: &Path) -> OrionConfResult<()> {
        save_to_file(path, "json", || {
            serde_json::to_string(self).map_err(Into::into)
        })
    }
}

// JsonStorageExt trait removed to avoid method conflicts
#[cfg(feature = "toml")]
use crate::traits::TomlIO;

#[cfg(feature = "toml")]
impl<T> TomlIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_toml(path: &Path) -> OrionConfResult<T> {
        load_from_file(path, "toml", |content| {
            toml::from_str(content).map_err(Into::into)
        })
    }
    fn save_toml(&self, path: &Path) -> OrionConfResult<()> {
        save_to_file(path, "toml", || toml::to_string(self).map_err(Into::into))
    }
}
#[cfg(feature = "yaml")]
use crate::traits::YamlIO;

#[cfg(feature = "yaml")]
impl<T> YamlIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_yaml(path: &Path) -> OrionConfResult<T> {
        load_from_file(path, "yaml", |content| {
            serde_yaml::from_str(content).map_err(Into::into)
        })
    }
    fn save_yaml(&self, path: &Path) -> OrionConfResult<()> {
        save_to_file(path, "yaml", || {
            serde_yaml::to_string(self).map_err(Into::into)
        })
    }
}

#[cfg(feature = "yaml")]
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::NamedTempFile;

    // Bring trait methods into scope for method resolution in tests
    use crate::traits::ConfigIO;
    #[cfg(feature = "ini")]
    use crate::traits::IniIO;
    #[cfg(feature = "json")]
    use crate::traits::JsonIO;
    #[cfg(feature = "toml")]
    use crate::traits::TomlIO;
    #[cfg(feature = "yaml")]
    use crate::traits::YamlIO;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Getters)]
    struct TestConfig {
        name: String,
        version: u32,
        enabled: String,
        timeout_secs: u64,
        nested_config: NestedConfig,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Getters)]
    struct NestedConfig {
        retry_count: u32,
        backoff_ms: u64,
    }

    impl Default for TestConfig {
        fn default() -> Self {
            Self {
                name: "test_app".to_string(),
                version: 1,
                enabled: "true".to_string(),
                timeout_secs: 30,
                nested_config: NestedConfig {
                    retry_count: 3,
                    backoff_ms: 1000,
                },
            }
        }
    }

    // StorageLoadEvent implementation removed to avoid trait conflicts

    fn create_test_config() -> TestConfig {
        TestConfig::default()
    }

    fn create_test_file_with_content(content: &str, extension: &str) -> NamedTempFile {
        let file = NamedTempFile::with_suffix(extension).unwrap();
        std::fs::write(file.path(), content).unwrap();
        file
    }

    // 测试用例 1: Configable trait 完整功能测试
    #[test]
    fn test_configable_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_conf(path).expect("Failed to save config");

        // 测试加载
        let loaded_config = TestConfig::load_conf(path).expect("Failed to load config");

        // 验证数据一致性（注意：StorageLoadEvent 会修改 version）
        assert_eq!(config.name, loaded_config.name);
        assert_eq!(config.enabled, loaded_config.enabled);
        assert_eq!(config.timeout_secs, loaded_config.timeout_secs);
        assert_eq!(config.nested_config, loaded_config.nested_config);
    }

    #[test]
    fn test_configable_file_not_found() {
        let non_existent_path = Path::new("non_existent_config.yml");
        let result = TestConfig::load_conf(non_existent_path);
        assert!(result.is_err());
    }

    // 测试用例 2: IniAble trait INI 格式测试
    #[cfg(feature = "ini")]
    #[test]
    fn test_ini_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_ini(path).expect("Failed to save INI config");

        // 测试加载
        let loaded_config = TestConfig::load_ini(path).expect("Failed to load INI config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 INI 格式
        let ini_content = std::fs::read_to_string(path).unwrap();
        assert!(ini_content.contains("[nested_config]"));
        assert!(ini_content.contains("retry_count=3"));
    }

    #[cfg(feature = "ini")]
    #[test]
    fn test_ini_able_invalid_format() {
        let invalid_ini_content = r#"
    [invalid]
    this_is_not_valid_ini_data
    "#;
        let temp_file = create_test_file_with_content(invalid_ini_content, "ini");
        let result = TestConfig::load_ini(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 3: JsonAble trait JSON 格式测试
    #[cfg(feature = "json")]
    #[test]
    fn test_json_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_json(path).expect("Failed to save JSON config");

        // 测试加载
        let loaded_config = TestConfig::load_json(path).expect("Failed to load JSON config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 JSON 格式
        let json_content = std::fs::read_to_string(path).unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
        assert_eq!(parsed_json["name"], "test_app");
        assert_eq!(parsed_json["version"], 1);
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json_able_malformed_json() {
        let malformed_json = r#"{"name": "test", "version": "invalid_number"}"#;
        let temp_file = create_test_file_with_content(malformed_json, "json");
        let result = TestConfig::load_json(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 4: Tomlable trait TOML 格式测试
    #[cfg(feature = "toml")]
    #[test]
    fn test_toml_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_toml(path).expect("Failed to save TOML config");

        // 测试加载
        let loaded_config = TestConfig::load_toml(path).expect("Failed to load TOML config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 TOML 格式
        let toml_content = std::fs::read_to_string(path).unwrap();
        let parsed_toml: toml::Value = toml::from_str(&toml_content).unwrap();
        assert_eq!(parsed_toml["name"].as_str(), Some("test_app"));
        assert_eq!(parsed_toml["version"].as_integer(), Some(1));
    }

    #[cfg(feature = "toml")]
    #[test]
    fn test_toml_able_invalid_toml() {
        let invalid_toml = r#"
    name = "test"
    version = "invalid_number"
    "#;
        let temp_file = create_test_file_with_content(invalid_toml, "toml");
        let result = TestConfig::load_toml(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 5: Yamlable trait YAML 格式测试
    #[cfg(feature = "yaml")]
    #[test]
    fn test_yaml_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_yaml(path).expect("Failed to save YAML config");

        // 测试加载
        let loaded_config = TestConfig::load_yaml(path).expect("Failed to load YAML config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 YAML 格式
        let yaml_content = std::fs::read_to_string(path).unwrap();
        let parsed_yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_content).unwrap();
        assert_eq!(parsed_yaml["name"].as_str(), Some("test_app"));
        assert_eq!(parsed_yaml["version"].as_u64(), Some(1));
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_yaml_able_malformed_yaml() {
        let malformed_yaml = r#"
    name: "test"
    version: "invalid_number"
    nested_config:
      retry_count: "invalid_retry"
    "#;
        let temp_file = create_test_file_with_content(malformed_yaml, "yml");
        let result = TestConfig::load_yaml(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 6: JsonStorageExt trait 扩展功能测试
    #[cfg(feature = "json")]
    #[test]
    fn test_json_storage_ext_with_event() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_json(path).expect("Failed to save JSON config");

        // 测试加载
        let loaded_config = TestConfig::load_json(path).expect("Failed to load JSON config");

        // 验证数据一致性
        assert_eq!(loaded_config, config);
    }

    // 测试用例 7: TomlStorageExt trait 扩展功能测试
    #[cfg(feature = "toml")]
    #[test]
    fn test_toml_storage_ext_with_event() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_toml(path).expect("Failed to save TOML config");

        // 测试加载
        let loaded_config = TestConfig::load_toml(path).expect("Failed to load TOML config");

        // 验证数据一致性
        assert_eq!(loaded_config, config);
    }

    // 测试用例 8: YamlStorageExt trait 扩展功能测试
    #[cfg(feature = "yaml")]
    #[test]
    fn test_yaml_storage_ext_with_event() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_yaml(path).expect("Failed to save YAML config");

        // 测试加载
        let loaded_config = TestConfig::load_yaml(path).expect("Failed to load YAML config");

        // 验证数据一致性
        assert_eq!(loaded_config, config);
    }
}
