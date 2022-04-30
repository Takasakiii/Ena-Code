use ecode_core::configs::{
    dirs_and_files::{config_folder, remove_caracteres},
    Config,
};
use std::{path::Path, process::Command};

pub fn launch(profile: &str, url: &str) {
    let config = Config::get_config(false);

    let path = Path::new(&config.profiles_folder);
    let joined_path = path.join(&remove_caracteres(profile, &config));
    let extension_folder = joined_path.join("extensions");
    let configs_folder = config_folder(&config, &joined_path, path);

    let extension_folder = extension_folder.to_str();
    let configs_folder = configs_folder.to_str();

    if let Some(extension_folder) = extension_folder {
        if let Some(configs_folder) = configs_folder {
            let cmd_exec = Command::new(&config.vs_code_path[..])
                .arg("--extensions-dir")
                .arg(extension_folder)
                .arg("--user-data-dir")
                .arg(configs_folder)
                .arg("--open-url")
                .arg(url)
                .output();

            if let Err(why) = cmd_exec {
                println!("Error starting VSCode proccess: {:?}", why)
            }
        } else {
            println!("Error building VSCode launch: extensions folder is null.")
        }
    } else {
        println!("Error building VSCode launch: configs folder is null.")
    }
}
