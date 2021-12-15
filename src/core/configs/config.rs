use crate::configs::dirs_and_files;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File, OpenOptions},
    io::prelude::*,
    path::{Path, PathBuf},
};

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

impl Config {
    fn new() -> Self {
        let bin_name = dirs_and_files::get_bin_or_cmd_name();
        let bin_code_path = dirs_and_files::find_it(bin_name).unwrap_or_default();

        let profiles_dir = dirs_and_files::create_or_get_ena_home_folder()
            .unwrap()
            .join("vs-code-profiles");

        Self {
            create_new_profile_from: "Default".into(),
            profiles_folder: dirs_and_files::path_to_string(profiles_dir),
            vs_code_path: dirs_and_files::path_to_string(bin_code_path),
            default_current_folder: false,
            shared_profiles_configs: false,
        }
    }

    pub fn get_config_raw() -> Self {
        let mut config = Self::new();

        let config_file_path = get_ena_config_path();

        if let Ok(config_string) = fs::read_to_string(config_file_path) {
            let config_obj = serde_yaml::from_str::<Config>(&config_string[..]);

            if let Ok(config_obj) = config_obj {
                config = config_obj;
            }
        }

        config
    }

    pub fn create_config() {
        let config_file_path = get_ena_config_path();

        match File::create(config_file_path) {
            Ok(mut file) => {
                let obj = Self::new();

                if let Ok(yml) = serde_yaml::to_string(&obj) {
                    if let Err(why) = file.write_all(yml.as_bytes()) {
                        println!("Config couldn't be written, {:?}", why);
                    }
                }
            }
            Err(why) => println!("Error creating config.yml, {:?}", why),
        }
    }

    pub fn get_config(verbose: bool) -> Self {
        if !Path::new(&get_ena_config_path()).exists() {
            Self::create_config();
        }
        let config = Self::get_config_raw();
        if verbose {
            println!("{:?}", &config)
        }
        config
    }

    pub fn save_config(&self) {
        let ena_config_path = get_ena_config_path();

        let string = serde_yaml::to_string(self).unwrap();
        let mut file = OpenOptions::new()
            .read(false)
            .write(true)
            .append(false)
            .create(true)
            .open(ena_config_path)
            .unwrap();

        let string_encoded = string.as_bytes();
        file.write_all(string_encoded).unwrap();
    }
}

fn get_ena_config_path() -> PathBuf {
    dirs_and_files::create_or_get_ena_home_folder()
        .unwrap()
        .join("config.yml")
}
