use std::path::Path;

use crate::actions::EnableDisable;
use ecode_core::configs::{dirs_and_files, Config};

pub fn shared_profiles_config(enable: &EnableDisable) {
    let mut config = Config::get_config(false);
    config.shared_profiles_configs = enable.to_bool();
    config.save_config();
}

pub fn default_profile(name: &str) {
    if dirs_and_files::check_profile_exists(name) {
        let mut config = Config::get_config(false);
        config.create_new_profile_from = name.to_owned();
        config.save_config();
    } else {
        println!(
            "It was not possible to use the profile {} as default as it does not exist.",
            &name
        );
    }
}

pub fn profiles_folder(path: &str) {
    let path_p = Path::new(path);
    if path_p.is_dir() {
        let mut config = Config::get_config(false);
        config.profiles_folder = path.to_owned();
        config.save_config();
    } else {
        println!("The path {} is not a folder.", path);
    }
}

pub fn use_current_folder(enable: &EnableDisable) {
    let mut config = Config::get_config(false);
    config.default_current_folder = enable.to_bool();
    config.save_config();
}

pub fn vs_code_path(path: &str) {
    let path_p = Path::new(path);
    if path_p.is_file() {
        let mut config = Config::get_config(false);
        config.vs_code_path = path.to_owned();
        config.save_config();
    } else {
        println!("The path {} is not a file.", path);
    }
}
