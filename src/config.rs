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

pub struct ConfigYML {
    data: String,
}

impl ConfigYML {
    pub fn new(data: String) -> Self {
        Self { data }
    }
}

impl ConfigManipulation for ConfigYML {
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
