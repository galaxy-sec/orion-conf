pub use derive_getters::Getters;
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
use serde::de::DeserializeOwned;
pub use serde_derive::{Deserialize, Serialize};
use std::path::Path;

use crate::error::SerdeResult;

// 原有 trait 定义保持不变以确保向后兼容性
pub trait Persistable<T> {
    fn save_to(&self, path: &Path, name: Option<String>) -> SerdeResult<()>;
    fn load_from(path: &Path) -> SerdeResult<T>;
}

pub trait Configable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_conf(path: &Path) -> SerdeResult<T>;
    fn save_conf(&self, path: &Path) -> SerdeResult<()>;
}

pub trait IniAble<T>
where
    T: DeserializeOwned + serde::Serialize,
{
    fn from_ini(path: &Path) -> SerdeResult<T>;
    fn save_ini(&self, path: &Path) -> SerdeResult<()>;
}

pub trait JsonAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_json(path: &Path) -> SerdeResult<T>;
    fn save_json(&self, path: &Path) -> SerdeResult<()>;
}

pub trait Tomlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_toml(path: &Path) -> SerdeResult<T>;
    fn save_toml(&self, path: &Path) -> SerdeResult<()>;
}

pub trait ValueConfable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_valconf(path: &Path) -> SerdeResult<T>;
    fn save_valconf(&self, path: &Path) -> SerdeResult<()>;
}

pub trait Yamlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_yml(path: &Path) -> SerdeResult<T>;
    fn save_yml(&self, path: &Path) -> SerdeResult<()>;
}

// 新的改进命名 trait - Storage 模式，强调持久化存储的完整功能
/// 文件持久化存储 trait，包含读取和写入操作，继承自 Persistable，保持方法一致
pub trait FileStorage<T>: Persistable<T> {}

/// 配置存储 trait，用于处理配置文件的读取和写入，继承自 Configable，保持方法一致
pub trait ConfigStorage<T>: Configable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

/// INI 格式存储 trait，提供 INI 文件的持久化存储功能
pub trait IniStorage<T>: IniAble<T>
where
    T: DeserializeOwned + serde::Serialize,
{
}

/// JSON 格式存储 trait，提供 JSON 文件的持久化存储功能
pub trait JsonStorage<T>: JsonAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

/// TOML 格式存储 trait，提供 TOML 文件的持久化存储功能
pub trait TomlStorage<T>: Tomlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

/// 值配置存储 trait，提供值配置的持久化存储功能
pub trait ValueStorage<T>: ValueConfable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

/// YAML 格式存储 trait，提供 YAML 文件的持久化存储功能
pub trait YamlStorage<T>: Yamlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

// 为所有原有 trait 实现新的 Storage 命名 trait，确保无缝兼容性
impl<T, U> FileStorage<U> for T where T: Persistable<U> {}

impl<T, U> ConfigStorage<U> for T
where
    T: Configable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

impl<T, U> IniStorage<U> for T
where
    T: IniAble<U>,
    U: DeserializeOwned + serde::Serialize,
{
}

impl<T, U> JsonStorage<U> for T
where
    T: JsonAble<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

impl<T, U> TomlStorage<U> for T
where
    T: Tomlable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

impl<T, U> ValueStorage<U> for T
where
    T: ValueConfable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

impl<T, U> YamlStorage<U> for T
where
    T: Yamlable<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

// 向后兼容性文档：通过 trait 继承实现了兼容性
// 用户可以继续使用旧的 trait 名称，同时也可以使用新的改进命名
// 新的 Storage 命名更准确地表达了持久化存储的完整功能，
// 包括文件读取、写入、格式转换等操作
