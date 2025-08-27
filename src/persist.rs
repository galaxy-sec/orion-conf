pub use derive_getters::Getters;
use orion_error::{ContextRecord, OperationContext};
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
use serde::de::DeserializeOwned;
pub use serde_derive::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{
    IniAble, JsonAble, JsonStorageExt, StorageLoadEvent, TomlStorageExt, Tomlable, ValueConfable,
    YamlStorageExt, Yamlable, error::SerdeResult, traits::Configable,
};

impl<T> Configable<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_conf(path: &Path) -> SerdeResult<T> {
        T::from_yml(path)
    }
    fn save_conf(&self, path: &Path) -> SerdeResult<()> {
        self.save_yml(path)
    }
}

impl<T> IniAble<T> for T
where
    T: DeserializeOwned + serde::Serialize,
{
    fn from_ini(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from ini").with_exit_log();
        ctx.record("from path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        let loaded: T = serde_ini::de::from_str(file_content.as_str())
            .owe_res()
            .with(&ctx)?;
        ctx.mark_suc();
        Ok(loaded)
    }
    fn save_ini(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("load conf spec");
        ctx.record("from path", path);
        let data_content = serde_ini::ser::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;
        Ok(())
    }
}

impl<T> JsonAble<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_json(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from json").with_exit_log();
        ctx.record("from path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        let loaded: T = serde_json::from_str(file_content.as_str())
            .owe_res()
            .with(&ctx)?;
        ctx.mark_suc();
        Ok(loaded)
    }
    fn save_json(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("save json").with_exit_log();
        ctx.record("from path", path);
        let data_content = serde_json::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;
        ctx.mark_suc();
        Ok(())
    }
}

impl<T> JsonStorageExt<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize + StorageLoadEvent,
{
    fn from_json(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from json").with_exit_log();
        ctx.record("from path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        let mut loaded: T = serde_json::from_str(file_content.as_str())
            .owe_res()
            .with(&ctx)?;
        ctx.mark_suc();
        loaded.loaded_event_do();
        Ok(loaded)
    }
    fn save_json(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("save json").with_exit_log();
        ctx.record("from path", path);
        let data_content = serde_json::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;
        ctx.mark_suc();
        Ok(())
    }
}

impl<T> Tomlable<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_toml(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from toml").with_exit_log();
        ctx.record("from path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        let loaded: T = toml::from_str(file_content.as_str()).owe_res().with(&ctx)?;
        ctx.mark_suc();
        Ok(loaded)
    }
    fn save_toml(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("save object to toml").with_exit_log();
        ctx.record("from path", path);
        let data_content = toml::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;
        ctx.mark_suc();
        Ok(())
    }
}

impl<T> TomlStorageExt<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize + StorageLoadEvent,
{
    fn from_toml(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from toml").with_exit_log();
        ctx.record("from path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        let mut loaded: T = toml::from_str(file_content.as_str()).owe_res().with(&ctx)?;
        ctx.mark_suc();
        loaded.loaded_event_do();
        Ok(loaded)
    }
    fn save_toml(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("save object to toml").with_exit_log();
        ctx.record("from path", path);
        let data_content = toml::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;

        ctx.mark_suc();
        Ok(())
    }
}

impl<T> ValueConfable<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_valconf(path: &Path) -> SerdeResult<T> {
        T::from_yml(path)
    }
    fn save_valconf(&self, path: &Path) -> SerdeResult<()> {
        T::save_yml(self, path)
    }
}

impl<T> Yamlable<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_yml(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from yml").with_exit_log();
        ctx.record("path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        //let loaded: T = toml::from_str(file_content.as_str()).owe_res().with(&ctx)?;
        let loaded: T = serde_yaml::from_str(file_content.as_str())
            .owe_res()
            .with(&ctx)?;
        ctx.mark_suc();
        Ok(loaded)
    }
    fn save_yml(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("save object fo yml").with_exit_log();
        ctx.record("path", path);
        let data_content = serde_yaml::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;
        ctx.mark_suc();
        Ok(())
    }
}

impl<T> YamlStorageExt<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize + StorageLoadEvent,
{
    fn from_yml(path: &Path) -> SerdeResult<T> {
        let mut ctx = OperationContext::want("load object from yml").with_exit_log();
        ctx.record("path", path);
        let file_content = fs::read_to_string(path).owe_res().with(&ctx)?;
        //let loaded: T = toml::from_str(file_content.as_str()).owe_res().with(&ctx)?;
        let mut loaded: T = serde_yaml::from_str(file_content.as_str())
            .owe_res()
            .with(&ctx)?;
        loaded.loaded_event_do();
        ctx.mark_suc();
        Ok(loaded)
    }
    fn save_yml(&self, path: &Path) -> SerdeResult<()> {
        let mut ctx = OperationContext::want("save object fo yml").with_exit_log();
        ctx.record("path", path);
        let data_content = serde_yaml::to_string(self).owe_data().with(&ctx)?;
        fs::write(path, data_content).owe_res().with(&ctx)?;
        ctx.mark_suc();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::NamedTempFile;

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
        let loaded_config = TestConfig::from_conf(path).expect("Failed to load config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证委托给 YAML 的行为
        let yaml_content = std::fs::read_to_string(path).unwrap();
        assert!(yaml_content.contains("name: test_app"));
        assert!(yaml_content.contains("version: 1"));
        assert!(yaml_content.contains("true"));
    }

    #[test]
    fn test_configable_file_not_found() {
        let non_existent_path = Path::new("non_existent_config.yml");
        let result = TestConfig::from_conf(non_existent_path);
        assert!(result.is_err());
    }

    // 测试用例 2: IniAble trait INI 格式测试
    #[test]
    fn test_ini_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_ini(path).expect("Failed to save INI config");

        // 测试加载
        let loaded_config = TestConfig::from_ini(path).expect("Failed to load INI config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 INI 格式
        let ini_content = std::fs::read_to_string(path).unwrap();
        assert!(ini_content.contains("[nested_config]"));
        assert!(ini_content.contains("retry_count=3"));
    }

    #[test]
    fn test_ini_able_invalid_format() {
        let invalid_ini_content = r#"
    [invalid]
    this_is_not_valid_ini_data
    "#;
        let temp_file = create_test_file_with_content(invalid_ini_content, "ini");
        let result = TestConfig::from_ini(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 3: JsonAble trait JSON 格式测试
    #[test]
    fn test_json_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_json(path).expect("Failed to save JSON config");

        // 测试加载
        let loaded_config = TestConfig::from_json(path).expect("Failed to load JSON config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 JSON 格式
        let json_content = std::fs::read_to_string(path).unwrap();
        let parsed_json: serde_json::Value = serde_json::from_str(&json_content).unwrap();
        assert_eq!(parsed_json["name"], "test_app");
        assert_eq!(parsed_json["version"], 1);
    }

    #[test]
    fn test_json_able_malformed_json() {
        let malformed_json = r#"{"name": "test", "version": "invalid_number"}"#;
        let temp_file = create_test_file_with_content(malformed_json, "json");
        let result = TestConfig::from_json(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 4: Tomlable trait TOML 格式测试
    #[test]
    fn test_toml_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_toml(path).expect("Failed to save TOML config");

        // 测试加载
        let loaded_config = TestConfig::from_toml(path).expect("Failed to load TOML config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 TOML 格式
        let toml_content = std::fs::read_to_string(path).unwrap();
        let parsed_toml: toml::Value = toml::from_str(&toml_content).unwrap();
        assert_eq!(parsed_toml["name"].as_str(), Some("test_app"));
        assert_eq!(parsed_toml["version"].as_integer(), Some(1));
    }

    #[test]
    fn test_toml_able_invalid_toml() {
        let invalid_toml = r#"
    name = "test"
    version = "invalid_number"
    "#;
        let temp_file = create_test_file_with_content(invalid_toml, "toml");
        let result = TestConfig::from_toml(temp_file.path());
        assert!(result.is_err());
    }

    // 测试用例 5: Yamlable trait YAML 格式测试
    #[test]
    fn test_yaml_able_save_and_load() {
        let config = create_test_config();
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();

        // 测试保存
        config.save_yml(path).expect("Failed to save YAML config");

        // 测试加载
        let loaded_config = TestConfig::from_yml(path).expect("Failed to load YAML config");

        // 验证数据一致性
        assert_eq!(config, loaded_config);

        // 验证 YAML 格式
        let yaml_content = std::fs::read_to_string(path).unwrap();
        let parsed_yaml: serde_yaml::Value = serde_yaml::from_str(&yaml_content).unwrap();
        assert_eq!(parsed_yaml["name"].as_str(), Some("test_app"));
        assert_eq!(parsed_yaml["version"].as_u64(), Some(1));
    }

    #[test]
    fn test_yaml_able_malformed_yaml() {
        let malformed_yaml = r#"
    name: "test"
    version: "invalid_number"
    nested_config:
      retry_count: "invalid_retry"
    "#;
        let temp_file = create_test_file_with_content(malformed_yaml, "yml");
        let result = TestConfig::from_yml(temp_file.path());
        assert!(result.is_err());
    }
}
