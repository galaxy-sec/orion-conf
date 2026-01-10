pub use derive_getters::Getters;
pub use orion_error::{ErrorOwe, ErrorWith, StructError, ToStructError, UvsConfFrom};
use orion_variate::EnvDict;
pub use serde_derive::{Deserialize, Serialize};
use std::path::Path;

use crate::error::OrionConfResult;

// 核心持久化 trait - 不依赖任何特定格式
pub trait FilePersist<T> {
    fn save_to(&self, path: &Path, name: Option<String>) -> OrionConfResult<()>;
    fn load_from(path: &Path) -> OrionConfResult<T>;
}

// 通用配置 trait - 默认行为基于可用特性
pub trait ConfigIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_conf(path: &Path) -> OrionConfResult<T>;
    fn save_conf(&self, path: &Path) -> OrionConfResult<()>;
}

// 通用配置 trait - 默认行为基于可用特性
pub trait EnvLoadable<T>
where
    T: serde::de::DeserializeOwned,
{
    // 推荐方法名
    fn env_load_conf(path: &Path, dict: &EnvDict) -> OrionConfResult<T>;
}

// 格式特定的 trait - 每个都使用条件编译

#[cfg(feature = "ini")]
pub trait IniIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_ini(path: &Path) -> OrionConfResult<T>;
    fn save_ini(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "ini")]
pub trait EnvIniLoad<T>
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_ini(path: &Path, dict: &EnvDict) -> OrionConfResult<T>;
    fn env_parse_ini(content: &str, dict: &EnvDict) -> OrionConfResult<T>;
}

#[cfg(feature = "json")]
pub trait JsonIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_json(path: &Path) -> OrionConfResult<T>;
    fn save_json(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "json")]
pub trait EnvJsonLoad<T>
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_json(path: &Path, dict: &EnvDict) -> OrionConfResult<T>;
    fn env_parse_json(content: &str, dict: &EnvDict) -> OrionConfResult<T>;
}

#[cfg(feature = "toml")]
pub trait TomlIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_toml(path: &Path) -> OrionConfResult<T>;
    fn save_toml(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "toml")]
pub trait EnvTomlLoad<T>
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_toml(path: &Path, dict: &EnvDict) -> OrionConfResult<T>;
    fn env_parse_toml(content: &str, dict: &EnvDict) -> OrionConfResult<T>;
}

#[cfg(feature = "toml")]
pub trait TextConfigIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_valconf(path: &Path) -> OrionConfResult<T>;
    fn save_valconf(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "yaml")]
pub trait YamlIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn load_yaml(path: &Path) -> OrionConfResult<T>;
    fn save_yaml(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "yaml")]
pub trait EnvYamlLoad<T>
where
    T: serde::de::DeserializeOwned,
{
    fn env_load_yaml(path: &Path, dict: &EnvDict) -> OrionConfResult<T>;
    fn env_parse_yaml(content: &str, dict: &EnvDict) -> OrionConfResult<T>;
}

pub trait LoadHook {
    fn loaded_event_do(&mut self) {}
}

// 默认实现选择逻辑 - 基于特性优先级
impl<T> ConfigIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[cfg(feature = "yaml")]
    fn load_conf(path: &Path) -> OrionConfResult<T> {
        T::load_yaml(path)
    }

    #[cfg(all(feature = "toml", not(feature = "yaml")))]
    fn load_conf(path: &Path) -> OrionConfResult<T> {
        T::load_toml(path)
    }

    #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
    fn load_conf(path: &Path) -> OrionConfResult<T> {
        T::load_json(path)
    }

    #[cfg(all(
        feature = "ini",
        not(any(feature = "yaml", feature = "toml", feature = "json"))
    ))]
    fn load_conf(path: &Path) -> OrionConfResult<T> {
        T::load_ini(path)
    }

    #[cfg(not(any(feature = "yaml", feature = "toml", feature = "json", feature = "ini")))]
    fn load_conf(_path: &Path) -> OrionConfResult<T> {
        use crate::error::ConfIOReason;
        Err(ConfIOReason::NoFormatEnabled.to_err())
    }

    #[cfg(feature = "yaml")]
    fn save_conf(&self, path: &Path) -> OrionConfResult<()> {
        self.save_yaml(path)
    }

    #[cfg(all(feature = "toml", not(feature = "yaml")))]
    fn save_conf(&self, path: &Path) -> OrionConfResult<()> {
        self.save_toml(path)
    }

    #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
    fn save_conf(&self, path: &Path) -> OrionConfResult<()> {
        self.save_json(path)
    }

    #[cfg(all(
        feature = "ini",
        not(any(feature = "yaml", feature = "toml", feature = "json"))
    ))]
    fn save_conf(&self, path: &Path) -> OrionConfResult<()> {
        self.save_ini(path)
    }

    #[cfg(not(any(feature = "yaml", feature = "toml", feature = "json", feature = "ini")))]
    fn save_conf(&self, _path: &Path) -> OrionConfResult<()> {
        use crate::error::ConfIOReason;
        Err(ConfIOReason::NoFormatEnabled.to_err())
    }
}

// TextConfigIO 的默认实现 - 委托给 YAML 或 TOML
#[cfg(feature = "toml")]
impl<T> TextConfigIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[cfg(feature = "yaml")]
    fn load_valconf(path: &Path) -> OrionConfResult<T> {
        T::load_yaml(path)
    }

    #[cfg(not(feature = "yaml"))]
    fn load_valconf(path: &Path) -> OrionConfResult<T> {
        T::load_toml(path)
    }

    #[cfg(feature = "yaml")]
    fn save_valconf(&self, path: &Path) -> OrionConfResult<()> {
        self.save_yaml(path)
    }

    #[cfg(not(feature = "yaml"))]
    fn save_valconf(&self, path: &Path) -> OrionConfResult<()> {
        self.save_toml(path)
    }
}
