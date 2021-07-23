use serde::{Deserialize, Serialize};

use crate::errors::EResult;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub profiles_folder: String,
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
