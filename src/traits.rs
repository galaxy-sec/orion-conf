pub use derive_getters::Getters;
pub use orion_error::{ErrorOwe, ErrorWith, StructError, ToStructError, UvsConfFrom};
pub use serde_derive::{Deserialize, Serialize};
use std::path::Path;

use crate::error::SerdeResult;

// 核心持久化 trait - 不依赖任何特定格式
pub trait Persistable<T> {
    fn save_to(&self, path: &Path, name: Option<String>) -> SerdeResult<()>;
    fn load_from(path: &Path) -> SerdeResult<T>;
}

// 通用配置 trait - 默认行为基于可用特性
pub trait Configable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_conf(path: &Path) -> SerdeResult<T>;
    fn save_conf(&self, path: &Path) -> SerdeResult<()>;
}

// 格式特定的 trait - 每个都使用条件编译

#[cfg(feature = "ini")]
pub trait IniAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_ini(path: &Path) -> SerdeResult<T>;
    fn save_ini(&self, path: &Path) -> SerdeResult<()>;
}

#[cfg(feature = "json")]
pub trait JsonAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_json(path: &Path) -> SerdeResult<T>;
    fn save_json(&self, path: &Path) -> SerdeResult<()>;
}

#[cfg(feature = "toml")]
pub trait Tomlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_toml(path: &Path) -> SerdeResult<T>;
    fn save_toml(&self, path: &Path) -> SerdeResult<()>;
}

#[cfg(feature = "toml")]
pub trait ValueConfable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_valconf(path: &Path) -> SerdeResult<T>;
    fn save_valconf(&self, path: &Path) -> SerdeResult<()>;
}

#[cfg(feature = "yaml")]
pub trait Yamlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_yml(path: &Path) -> SerdeResult<T>;
    fn save_yml(&self, path: &Path) -> SerdeResult<()>;
}

// Storage trait 别名 - 继承自对应的格式 trait

pub trait FileStorage<T>: Persistable<T> {}

pub trait StorageLoadEvent {
    fn loaded_event_do(&mut self) {}
}

pub trait ConfigStorage<T>: Configable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "ini")]
pub trait IniStorage<T>: IniAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "json")]
pub trait JsonStorage<T>: JsonAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
pub trait TomlStorage<T>: Tomlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
pub trait ValueStorage<T>: ValueConfable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "yaml")]
pub trait YamlStorage<T>: Yamlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

// 扩展 trait - 包含加载事件处理

// StorageExt traits removed to avoid method conflicts with base traits
// Use the base traits (JsonAble, Tomlable, Yamlable) for basic functionality

// 为所有原有 trait 实现新的 Storage 命名 trait，确保兼容性

impl<T, U> FileStorage<U> for T where T: Persistable<U> {}

impl<T, U> ConfigStorage<U> for T
where
    T: Configable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "ini")]
impl<T, U> IniStorage<U> for T
where
    T: IniAble<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "json")]
impl<T, U> JsonStorage<U> for T
where
    T: JsonAble<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
impl<T, U> TomlStorage<U> for T
where
    T: Tomlable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
impl<T, U> ValueStorage<U> for T
where
    T: ValueConfable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "yaml")]
impl<T, U> YamlStorage<U> for T
where
    T: Yamlable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

// 默认实现选择逻辑 - 基于特性优先级

impl<T> Configable<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[cfg(feature = "yaml")]
    fn from_conf(path: &Path) -> SerdeResult<T> {
        T::from_yml(path)
    }

    #[cfg(all(feature = "toml", not(feature = "yaml")))]
    fn from_conf(path: &Path) -> SerdeResult<T> {
        T::from_toml(path)
    }

    #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
    fn from_conf(path: &Path) -> SerdeResult<T> {
        T::from_json(path)
    }

    #[cfg(all(
        feature = "ini",
        not(any(feature = "yaml", feature = "toml", feature = "json"))
    ))]
    fn from_conf(path: &Path) -> SerdeResult<T> {
        T::from_ini(path)
    }

    #[cfg(not(any(feature = "yaml", feature = "toml", feature = "json", feature = "ini")))]
    fn from_conf(_path: &Path) -> SerdeResult<T> {
        use crate::error::StorageReason;
        Err(StorageReason::NoFormatEnabled.to_err())
    }

    #[cfg(feature = "yaml")]
    fn save_conf(&self, path: &Path) -> SerdeResult<()> {
        self.save_yml(path)
    }

    #[cfg(all(feature = "toml", not(feature = "yaml")))]
    fn save_conf(&self, path: &Path) -> SerdeResult<()> {
        self.save_toml(path)
    }

    #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
    fn save_conf(&self, path: &Path) -> SerdeResult<()> {
        self.save_json(path)
    }

    #[cfg(all(
        feature = "ini",
        not(any(feature = "yaml", feature = "toml", feature = "json"))
    ))]
    fn save_conf(&self, path: &Path) -> SerdeResult<()> {
        self.save_ini(path)
    }

    #[cfg(not(any(feature = "yaml", feature = "toml", feature = "json", feature = "ini")))]
    fn save_conf(&self, _path: &Path) -> SerdeResult<()> {
        use crate::error::StorageReason;
        Err(StorageReason::NoFormatEnabled.to_err())
    }
}

// ValueConfable 的默认实现 - 委托给 YAML 或 TOML

#[cfg(feature = "toml")]
impl<T> ValueConfable<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[cfg(feature = "yaml")]
    fn from_valconf(path: &Path) -> SerdeResult<T> {
        T::from_yml(path)
    }

    #[cfg(not(feature = "yaml"))]
    fn from_valconf(path: &Path) -> SerdeResult<T> {
        T::from_toml(path)
    }

    #[cfg(feature = "yaml")]
    fn save_valconf(&self, path: &Path) -> SerdeResult<()> {
        self.save_yml(path)
    }

    #[cfg(not(feature = "yaml"))]
    fn save_valconf(&self, path: &Path) -> SerdeResult<()> {
        self.save_toml(path)
    }
}
