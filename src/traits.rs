pub use derive_getters::Getters;
pub use orion_error::{ErrorOwe, ErrorWith, StructError, ToStructError, UvsConfFrom};
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
    #[deprecated(since = "0.3.0", note = "Use load_conf() instead")]
    fn from_conf(path: &Path) -> OrionConfResult<T>;
    // 推荐方法名
    fn load_conf(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            T::from_conf(path)
        }
    }
    fn save_conf(&self, path: &Path) -> OrionConfResult<()>;
}

// 格式特定的 trait - 每个都使用条件编译

#[cfg(feature = "ini")]
pub trait IniIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[deprecated(since = "0.3.0", note = "Use load_ini() instead")]
    fn from_ini(path: &Path) -> OrionConfResult<T>;
    fn load_ini(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            T::from_ini(path)
        }
    }
    fn save_ini(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "json")]
pub trait JsonIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[deprecated(since = "0.3.0", note = "Use load_json() instead")]
    fn from_json(path: &Path) -> OrionConfResult<T>;
    fn load_json(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            T::from_json(path)
        }
    }
    fn save_json(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "toml")]
pub trait TomlIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[deprecated(since = "0.3.0", note = "Use load_toml() instead")]
    fn from_toml(path: &Path) -> OrionConfResult<T>;
    fn load_toml(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            T::from_toml(path)
        }
    }
    fn save_toml(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "toml")]
pub trait TextConfigIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[deprecated(since = "0.3.0", note = "Use load_valconf() instead")]
    fn from_valconf(path: &Path) -> OrionConfResult<T>;
    fn load_valconf(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            T::from_valconf(path)
        }
    }
    fn save_valconf(&self, path: &Path) -> OrionConfResult<()>;
}

#[cfg(feature = "yaml")]
pub trait YamlIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    // 新的统一方法名（推荐）
    #[deprecated(since = "0.3.0", note = "Use load_yaml() instead")]
    fn from_yaml(path: &Path) -> OrionConfResult<T>;
    fn load_yaml(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            Self::from_yaml(path)
        }
    }
    fn save_yaml(&self, path: &Path) -> OrionConfResult<()>;

    // 旧方法名，保留别名并标记为弃用
    #[deprecated(since = "0.3.0", note = "Use load_yaml() instead")]
    fn from_yml(path: &Path) -> OrionConfResult<T> {
        #[allow(deprecated)]
        {
            Self::from_yaml(path)
        }
    }
    #[deprecated(since = "0.3.0", note = "Use save_yaml() instead")]
    fn save_yml(&self, path: &Path) -> OrionConfResult<()> {
        self.save_yaml(path)
    }
}

// Storage trait 别名（计划弃用）
#[deprecated(
    since = "0.3.0",
    note = "Use the base traits (FilePersist/ConfigIO/IniIO/JsonIO/TomlIO/TextConfigIO/YamlIO) directly"
)]
pub trait FileStorage<T>: FilePersist<T> {}

pub trait LoadHook {
    fn loaded_event_do(&mut self) {}
}
#[deprecated(since = "0.3.0", note = "Use LoadHook instead")]
pub use LoadHook as StorageLoadEvent;

#[deprecated(since = "0.3.0", note = "Use ConfigIO instead")]
pub trait ConfigStorage<T>: ConfigIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "ini")]
#[deprecated(since = "0.3.0", note = "Use IniIO instead")]
pub trait IniStorage<T>: IniIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "json")]
#[deprecated(since = "0.3.0", note = "Use JsonIO instead")]
pub trait JsonStorage<T>: JsonIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
#[deprecated(since = "0.3.0", note = "Use TomlIO instead")]
pub trait TomlStorage<T>: TomlIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
#[deprecated(since = "0.3.0", note = "Use TextConfigIO instead")]
pub trait ValueStorage<T>: TextConfigIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "yaml")]
#[deprecated(since = "0.3.0", note = "Use YamlIO instead")]
pub trait YamlStorage<T>: YamlIO<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
}

// 兼容旧名：在模块内导出旧名为新名的别名，并标记为弃用
#[deprecated(since = "0.3.0", note = "Use ConfigIO instead")]
pub use ConfigIO as Configable;
#[deprecated(since = "0.3.0", note = "Use FilePersist instead")]
pub use FilePersist as Persistable;
#[cfg(feature = "ini")]
#[deprecated(since = "0.3.0", note = "Use IniIO instead")]
pub use IniIO as IniAble;
#[cfg(feature = "json")]
#[deprecated(since = "0.3.0", note = "Use JsonIO instead")]
pub use JsonIO as JsonAble;
#[cfg(feature = "toml")]
#[deprecated(since = "0.3.0", note = "Use TextConfigIO instead")]
pub use TextConfigIO as ValueConfable;
#[cfg(feature = "toml")]
#[deprecated(since = "0.3.0", note = "Use TomlIO instead")]
pub use TomlIO as Tomlable;
#[cfg(feature = "yaml")]
#[deprecated(since = "0.3.0", note = "Use YamlIO instead")]
pub use YamlIO as Yamlable;

// 为所有原有 Storage* trait 实现 blanket impl，维持兼容性
#[allow(deprecated)]
impl<T, U> FileStorage<U> for T where T: FilePersist<U> {}

#[allow(deprecated)]
impl<T, U> ConfigStorage<U> for T
where
    T: ConfigIO<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "ini")]
#[allow(deprecated)]
impl<T, U> IniStorage<U> for T
where
    T: IniIO<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "json")]
#[allow(deprecated)]
impl<T, U> JsonStorage<U> for T
where
    T: JsonIO<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
#[allow(deprecated)]
impl<T, U> TomlStorage<U> for T
where
    T: TomlIO<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "toml")]
#[allow(deprecated)]
impl<T, U> ValueStorage<U> for T
where
    T: TextConfigIO<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

#[cfg(feature = "yaml")]
#[allow(deprecated)]
impl<T, U> YamlStorage<U> for T
where
    T: YamlIO<U>,
    U: serde::de::DeserializeOwned + serde::Serialize,
{
}

// 默认实现选择逻辑 - 基于特性优先级
impl<T> ConfigIO<T> for T
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    #[cfg(feature = "yaml")]
    fn from_conf(path: &Path) -> OrionConfResult<T> {
        T::load_yaml(path)
    }

    #[cfg(all(feature = "toml", not(feature = "yaml")))]
    fn from_conf(path: &Path) -> OrionConfResult<T> {
        T::load_toml(path)
    }

    #[cfg(all(feature = "json", not(any(feature = "yaml", feature = "toml"))))]
    fn from_conf(path: &Path) -> OrionConfResult<T> {
        T::load_json(path)
    }

    #[cfg(all(
        feature = "ini",
        not(any(feature = "yaml", feature = "toml", feature = "json"))
    ))]
    fn from_conf(path: &Path) -> OrionConfResult<T> {
        T::load_ini(path)
    }

    #[cfg(not(any(feature = "yaml", feature = "toml", feature = "json", feature = "ini")))]
    fn from_conf(_path: &Path) -> OrionConfResult<T> {
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
    fn from_valconf(path: &Path) -> OrionConfResult<T> {
        T::load_yaml(path)
    }

    #[cfg(not(feature = "yaml"))]
    fn from_valconf(path: &Path) -> OrionConfResult<T> {
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
