pub use derive_getters::Getters;
use orion_error::{ContextRecord, OperationContext, ToStructError};
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
use orion_variate::EnvChecker;
#[allow(unused_imports)]
use orion_variate::{EnvDict, EnvEvaluable};
pub use serde_derive::{Deserialize, Serialize};
use std::{fmt::Display, fs, path::Path};

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

/// Helper to unify env string evaluation + deserialization across formats
#[allow(dead_code)]
fn parse_env_string<T, F, E>(
    operation_name: &str,
    content: &str,
    dict: &EnvDict,
    deserializer: F,
) -> OrionConfResult<T>
where
    F: FnOnce(&str) -> Result<T, E>,
    E: Display,
{
    let mut ctx = OperationContext::want(format!("load object from {operation_name} env string"))
        .with_auto_log();
    ctx.record("source", "inline content");
    let evaluated = content.to_string().env_eval(dict);
    if evaluated.needs_env_eval() {
        let msg = format!("vars not value : {}", evaluated.list_env_vars().join(","));
        eprintln!("{}", msg);
        log::warn!(target : "conf", "{}", msg);
    }
    let loaded = deserializer(&evaluated)
        .map_err(|e| ConfIOReason::from_conf(e.to_string()).to_err())
        .with(&ctx)?;
    ctx.mark_suc();
    Ok(loaded)
}

/// Helper to unify file loading + env evaluation + deserialization across formats
#[allow(dead_code)]
fn env_load_file<T, F, E>(
    path: &Path,
    operation_name: &str,
    dict: &EnvDict,
    deserializer: F,
) -> OrionConfResult<T>
where
    F: FnOnce(&str) -> Result<T, E>,
    E: Display,
{
    let mut ctx = OperationContext::want(format!("load object from {operation_name} file with env"))
        .with_auto_log();
    ctx.record("from path", path);

    let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
    let evaluated = file_content.env_eval(dict);

    if evaluated.needs_env_eval() {
        let msg = format!("vars not value : {}", evaluated.list_env_vars().join(","));
        eprintln!("{}", msg);
        log::warn!(target: "conf", "{}", msg);
    }

    let loaded = deserializer(&evaluated)
        .map_err(|e| ConfIOReason::from_conf(e.to_string()).to_err())
        .with(&ctx)?;

    ctx.mark_suc();
    Ok(loaded)
}

// Default implementation of ConfigIO trait is handled in traits.rs

#[cfg(feature = "ini")]
use crate::traits::IniIO;

#[cfg(feature = "ini")]
use crate::traits::EnvIniLoad;

#[cfg(feature = "ini")]
impl<T> IniIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_ini(path: &Path) -> OrionConfResult<T> {
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

#[cfg(feature = "ini")]
impl<T> EnvIniLoad<T> for T
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_ini(path: &Path, dict: &EnvDict) -> OrionConfResult<T> {
        env_load_file(path, "ini", dict, |evaluated| {
            serde_ini::de::from_str(evaluated)
        })
    }

    fn env_parse_ini(content: &str, dict: &EnvDict) -> OrionConfResult<T> {
        parse_env_string("ini", content, dict, |evaluated| {
            serde_ini::de::from_str(evaluated)
        })
    }
}

#[cfg(feature = "json")]
use crate::traits::JsonIO;

#[cfg(feature = "json")]
use crate::traits::EnvJsonLoad;

#[cfg(feature = "json")]
impl<T> JsonIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_json(path: &Path) -> OrionConfResult<T> {
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

#[cfg(feature = "json")]
impl<T> EnvJsonLoad<T> for T
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_json(path: &Path, dict: &EnvDict) -> OrionConfResult<T> {
        env_load_file(path, "json", dict, |evaluated| {
            serde_json::from_str(evaluated)
        })
    }

    fn env_parse_json(content: &str, dict: &EnvDict) -> OrionConfResult<T> {
        parse_env_string("json", content, dict, |evaluated| {
            serde_json::from_str(evaluated)
        })
    }
}

// JsonStorageExt trait removed to avoid method conflicts
#[cfg(feature = "toml")]
use crate::traits::TomlIO;

#[cfg(feature = "toml")]
use crate::traits::EnvTomlLoad;

#[cfg(feature = "toml")]
impl<T> TomlIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_toml(path: &Path) -> OrionConfResult<T> {
        load_from_file(path, "toml", |content| {
            toml::from_str(content).map_err(Into::into)
        })
    }
    fn save_toml(&self, path: &Path) -> OrionConfResult<()> {
        save_to_file(path, "toml", || toml::to_string(self).map_err(Into::into))
    }
}

#[cfg(feature = "toml")]
impl<T> EnvTomlLoad<T> for T
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_toml(path: &Path, dict: &EnvDict) -> OrionConfResult<T> {
        env_load_file(path, "toml", dict, |evaluated| toml::from_str(evaluated))
    }

    fn env_parse_toml(content: &str, dict: &EnvDict) -> OrionConfResult<T> {
        parse_env_string("toml", content, dict, |evaluated| toml::from_str(evaluated))
    }
}

#[cfg(feature = "yaml")]
use crate::traits::YamlIO;

#[cfg(feature = "yaml")]
use crate::traits::EnvYamlLoad;

#[cfg(feature = "yaml")]
impl<T> YamlIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_yaml(path: &Path) -> OrionConfResult<T> {
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
impl<T> EnvYamlLoad<T> for T
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_yaml(path: &Path, dict: &EnvDict) -> OrionConfResult<T> {
        env_load_file(path, "yaml", dict, |evaluated| {
            serde_yaml::from_str(evaluated)
        })
    }

    fn env_parse_yaml(content: &str, dict: &EnvDict) -> OrionConfResult<T> {
        parse_env_string("yaml", content, dict, |evaluated| {
            serde_yaml::from_str(evaluated)
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

    // 测试用例 9: env_load_ini 环境变量替换测试
    #[cfg(feature = "ini")]
    #[test]
    fn test_env_load_ini() {
        use crate::traits::EnvIniLoad;
        use orion_variate::{EnvDict, ValueType};

        let ini_content = r#"
name = ${APP_NAME}
version = 2
enabled = ${ENABLED}
timeout_secs = 60

[nested_config]
retry_count = 5
backoff_ms = 2000
"#;
        let temp_file = create_test_file_with_content(ini_content, ".ini");

        // 创建环境变量字典
        let mut env_dict = EnvDict::new();
        env_dict.insert("APP_NAME", ValueType::from("production_app"));
        env_dict.insert("ENABLED", ValueType::from("false"));

        // 测试加载
        let loaded_config = TestConfig::env_load_ini(temp_file.path(), &env_dict)
            .expect("Failed to load INI with env vars");

        // 验证环境变量被正确替换
        assert_eq!(loaded_config.name, "production_app");
        assert_eq!(loaded_config.version, 2);
        assert_eq!(loaded_config.enabled, "false");
        assert_eq!(loaded_config.timeout_secs, 60);
        assert_eq!(loaded_config.nested_config.retry_count, 5);
        assert_eq!(loaded_config.nested_config.backoff_ms, 2000);
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Getters)]
    struct SingleFieldConfig {
        value: String,
    }

    // 测试用例 10: env_load_json 环境变量替换测试
    #[cfg(feature = "json")]
    #[test]
    fn test_env_load_json() {
        use crate::traits::JsonIO;
        use orion_variate::{EnvDict, ValueType};

        // 创建一个单字段配置
        let config = SingleFieldConfig {
            value: "placeholder".to_string(),
        };
        let temp_file = NamedTempFile::new().unwrap();
        config.save_json(temp_file.path()).unwrap();

        // 替换为环境变量模板
        let mut json_content = std::fs::read_to_string(temp_file.path()).unwrap();
        json_content = json_content.replace("placeholder", "${MY_VALUE}");
        std::fs::write(temp_file.path(), &json_content).unwrap();

        // 创建环境变量字典
        let mut env_dict = EnvDict::new();
        env_dict.insert("MY_VALUE", ValueType::from("production_value"));

        // 使用 env_load_json 加载
        use crate::traits::EnvJsonLoad;
        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_load_json(temp_file.path(), &env_dict)
                .expect("Failed to load JSON with env vars");

        // 验证环境变量被正确替换
        assert_eq!(loaded_config.value, "production_value");
    }

    // 测试用例 11: env_load_toml 环境变量替换测试
    #[cfg(feature = "toml")]
    #[test]
    fn test_env_load_toml() {
        use crate::traits::EnvTomlLoad;
        use orion_variate::{EnvDict, ValueType};

        let toml_content = r#"
name = "${APP_NAME}"
version = 2
enabled = "${ENABLED}"
timeout_secs = 60

[nested_config]
retry_count = 5
backoff_ms = 2000
"#;
        let temp_file = create_test_file_with_content(toml_content, ".toml");

        // 创建环境变量字典
        let mut env_dict = EnvDict::new();
        env_dict.insert("APP_NAME", ValueType::from("production_app"));
        env_dict.insert("ENABLED", ValueType::from("false"));

        // 测试加载
        let loaded_config = TestConfig::env_load_toml(temp_file.path(), &env_dict)
            .expect("Failed to load TOML with env vars");

        // 验证环境变量被正确替换
        assert_eq!(loaded_config.name, "production_app");
        assert_eq!(loaded_config.version, 2);
        assert_eq!(loaded_config.enabled, "false");
        assert_eq!(loaded_config.timeout_secs, 60);
        assert_eq!(loaded_config.nested_config.retry_count, 5);
        assert_eq!(loaded_config.nested_config.backoff_ms, 2000);
    }

    // 测试用例 12: env_load_yaml 环境变量替换测试
    #[cfg(feature = "yaml")]
    #[test]
    fn test_env_load_yaml() {
        use crate::traits::YamlIO;
        use orion_variate::{EnvDict, ValueType};

        // 创建一个单字段配置
        let config = SingleFieldConfig {
            value: "placeholder".to_string(),
        };
        let temp_file = NamedTempFile::new().unwrap();
        config.save_yaml(temp_file.path()).unwrap();

        // 替换为环境变量模板
        let mut yaml_content = std::fs::read_to_string(temp_file.path()).unwrap();
        yaml_content = yaml_content.replace("placeholder", "${MY_VALUE}");
        std::fs::write(temp_file.path(), &yaml_content).unwrap();

        // 创建环境变量字典
        let mut env_dict = EnvDict::new();
        env_dict.insert("MY_VALUE", ValueType::from("production_value"));

        // 使用 env_load_yaml 加载
        use crate::traits::EnvYamlLoad;
        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_load_yaml(temp_file.path(), &env_dict)
                .expect("Failed to load YAML with env vars");

        // 验证环境变量被正确替换
        assert_eq!(loaded_config.value, "production_value");
    }

    // 测试用例 13: env_load_json 未定义环境变量测试
    #[cfg(feature = "json")]
    #[test]
    fn test_env_load_json_undefined_var() {
        use crate::traits::JsonIO;
        use orion_variate::EnvDict;

        // 创建一个单字段配置
        let config = SingleFieldConfig {
            value: "placeholder".to_string(),
        };
        let temp_file = NamedTempFile::new().unwrap();
        config.save_json(temp_file.path()).unwrap();

        // 替换为未定义的环境变量
        let mut json_content = std::fs::read_to_string(temp_file.path()).unwrap();
        json_content = json_content.replace("placeholder", "${UNDEFINED_VAR}");
        std::fs::write(temp_file.path(), &json_content).unwrap();

        // 创建空的环境变量字典
        let env_dict = EnvDict::new();

        // 使用 env_load_json 加载（未定义的变量应该保持原样）
        use crate::traits::EnvJsonLoad;
        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_load_json(temp_file.path(), &env_dict)
                .expect("Failed to load JSON with undefined var");

        // 未定义的变量应该保持原样
        assert_eq!(loaded_config.value, "${UNDEFINED_VAR}");
    }

    #[cfg(feature = "ini")]
    #[test]
    fn test_env_parse_ini_inline_content() {
        use crate::traits::EnvIniLoad;
        use orion_variate::{EnvDict, ValueType};

        let ini_content = "value = ${MY_VALUE}";
        let mut env_dict = EnvDict::new();
        env_dict.insert("MY_VALUE", ValueType::from("inline_value"));

        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_parse_ini(ini_content, &env_dict)
                .expect("Failed to load inline INI content");

        assert_eq!(loaded_config.value, "inline_value");
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_env_parse_json_inline_content() {
        use crate::traits::EnvJsonLoad;
        use orion_variate::{EnvDict, ValueType};

        let json_content = r#"{"value":"${MY_VALUE}"}"#;
        let mut env_dict = EnvDict::new();
        env_dict.insert("MY_VALUE", ValueType::from("inline_json"));

        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_parse_json(json_content, &env_dict)
                .expect("Failed to load inline JSON content");

        assert_eq!(loaded_config.value, "inline_json");
    }

    #[cfg(feature = "toml")]
    #[test]
    fn test_env_parse_toml_inline_content() {
        use crate::traits::EnvTomlLoad;
        use orion_variate::{EnvDict, ValueType};

        let toml_content = "value = \"${MY_VALUE}\"";
        let mut env_dict = EnvDict::new();
        env_dict.insert("MY_VALUE", ValueType::from("inline_toml"));

        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_parse_toml(toml_content, &env_dict)
                .expect("Failed to load inline TOML content");

        assert_eq!(loaded_config.value, "inline_toml");
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn test_env_parse_yaml_inline_content() {
        use crate::traits::EnvYamlLoad;
        use orion_variate::{EnvDict, ValueType};

        let yaml_content = "value: ${MY_VALUE}";
        let mut env_dict = EnvDict::new();
        env_dict.insert("MY_VALUE", ValueType::from("inline_yaml"));

        let loaded_config: SingleFieldConfig =
            SingleFieldConfig::env_parse_yaml(yaml_content, &env_dict)
                .expect("Failed to load inline YAML content");

        assert_eq!(loaded_config.value, "inline_yaml");
    }
}
