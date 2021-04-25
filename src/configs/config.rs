use std::{fs::{self, File}, io::prelude::*, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};
use crate::configs::dirs_and_files;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub profiles_folder: String,
    pub create_new_profile_from: String,
    pub vs_code_path: String
}

impl Config {
    fn new() -> Self {
        let bin_name = dirs_and_files::get_bin_or_cmd_name();
        let bin_code_path = dirs_and_files::find_it(bin_name)
            .unwrap_or(PathBuf::default());

        let profiles_dir = dirs_and_files::create_or_get_ena_home_folder()
            .unwrap()
            .join("vs-code-profiles");

        Self {
            create_new_profile_from: "Default".into(),
            profiles_folder: dirs_and_files::path_to_string(profiles_dir),
            vs_code_path: dirs_and_files::path_to_string(bin_code_path)
        }
    }

    fn get_config_raw() -> Self {
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

    fn create_config() {
        let config_file_path = get_ena_config_path();

        match File::create(config_file_path) {
            Ok(mut file) => {
                let obj = Self::new();

                if let Ok(yml) = serde_yaml::to_string(&obj) {
                    if let Err(why) = file.write_all(yml.as_bytes()) {
                        println!("Config não pode ser gravado, {:?}", why);
                    }
                }
            },
            Err(why) => println!("Erro ao criar config.yml, {:?}", why)
        }

    }

    pub fn get_config() -> Self {
        if !Path::new(&get_ena_config_path()).exists() {
            Self::create_config();
        }
        Self::get_config_raw()
    }
}

fn get_ena_config_path() -> PathBuf {
    dirs_and_files::create_or_get_ena_home_folder()
        .unwrap()
        .join("config.yml")
}
