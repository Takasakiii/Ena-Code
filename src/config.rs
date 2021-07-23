use serde::{Deserialize, Serialize};

use crate::{
    errors::{EResult, EnaError},
    path_finder::PathFinder,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub create_new_profile_from: String,
    pub vs_code_path: String,
    #[serde(default = "bool::default")]
    pub default_current_folder: bool,
    #[serde(default = "bool::default")]
    pub shared_profiles_configs: bool,
}

pub trait ConfigManipulation {
    fn get_config(&self) -> EResult<Config>;
    fn set_config(&mut self, new_data: &Config) -> EResult<()>;
    fn get_raw(self) -> String;
}

pub struct DefaultConfig(Config);

impl DefaultConfig {
    pub fn new(path_finder: &impl PathFinder) -> EResult<Self> {
        let vs_code_path = path_finder
            .get_path()
            .ok_or(EnaError::VsCodeNotFound)?
            .to_str()
            .ok_or(EnaError::PathToStrNone)?
            .to_owned();

        let config = Self(Config {
            vs_code_path,
            create_new_profile_from: "Default".to_owned(),
            default_current_folder: false,
            shared_profiles_configs: false,
        });

        Ok(config)
    }
}

impl ConfigManipulation for DefaultConfig {
    fn get_config(&self) -> EResult<Config> {
        Ok(self.0.clone())
    }

    fn get_raw(self) -> String {
        format!("{:?}", self.0)
    }

    fn set_config(&mut self, new_data: &Config) -> EResult<()> {
        self.0 = new_data.clone();
        Ok(())
    }
}

pub struct ConfigReader {
    data: String,
}

impl ConfigReader {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

#[cfg(feature = "config_yml")]
impl ConfigManipulation for ConfigReader {
    fn get_config(&self) -> EResult<Config> {
        let deserialized_data = serde_yaml::from_str::<Config>(&self.data)?;
        Ok(deserialized_data)
    }

    fn set_config(&mut self, new_data: &Config) -> EResult<()> {
        let new_str = serde_yaml::to_string(new_data)?;
        self.data = new_str;
        Ok(())
    }

    fn get_raw(self) -> String {
        self.data
    }
}

#[cfg(feature = "config_json")]
impl ConfigManipulation for ConfigReader {
    fn get_config(&self) -> EResult<Config> {
        let deserialized_data = serde_json::from_str::<Config>(&self.data)?;
        Ok(deserialized_data)
    }

    fn set_config(&mut self, new_data: &Config) -> EResult<()> {
        let new_str = serde_json::to_string(new_data)?;
        self.data = new_str;
        Ok(())
    }

    fn get_raw(self) -> String {
        self.data
    }
}
