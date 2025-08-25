pub use derive_getters::Getters;
use orion_error::{ContextRecord, OperationContext};
pub use orion_error::{ErrorOwe, ErrorWith, StructError, UvsConfFrom};
use serde::de::DeserializeOwned;
pub use serde_derive::{Deserialize, Serialize};
use std::{fs, path::Path};

use crate::{error::SerdeResult, traits::Configable};

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

pub trait IniAble<T>
where
    T: DeserializeOwned + serde::Serialize,
{
    fn from_ini(path: &Path) -> SerdeResult<T>;
    fn save_ini(&self, path: &Path) -> SerdeResult<()>;
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

pub trait JsonAble<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_json(path: &Path) -> SerdeResult<T>;
    fn save_json(&self, path: &Path) -> SerdeResult<()>;
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

pub trait Tomlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_toml(path: &Path) -> SerdeResult<T>;
    fn save_toml(&self, path: &Path) -> SerdeResult<()>;
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

pub trait ValueConfable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_valconf(path: &Path) -> SerdeResult<T>;
    fn save_valconf(&self, path: &Path) -> SerdeResult<()>;
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

pub trait Yamlable<T>
where
    T: serde::de::DeserializeOwned + serde::Serialize,
{
    fn from_yml(path: &Path) -> SerdeResult<T>;
    fn save_yml(&self, path: &Path) -> SerdeResult<()>;
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
