use crate::arguments::LaunchOptions;

use ecode_core::configs::{
    dirs_and_files::{check_profile_exists, config_folder, copy_profile, remove_caracteres},
    Config,
};
use std::{path::Path, process::Command};

pub fn launch(args: &LaunchOptions, config: &Config) {
    let path = Path::new(&config.profiles_folder);
    let joined_path = path.join(&remove_caracteres(&args.profile, config));
    let extension_folder = joined_path.join("extensions");
    let configs_folder = config_folder(config, &joined_path, path);

    let extension_folder = extension_folder.to_str();
    let configs_folder = configs_folder.to_str();

    if extension_folder.is_some() && configs_folder.is_some() {
        match &args.base_derive {
            None => {
                if !check_profile_exists(&args.profile) {
                    let user_response = scanln::scanln!(
                        "The profile {} doesn't exist. Do you want to create it? [y/N]: ",
                        &args.profile
                    );

                    if user_response.ne("y") {
                        return;
                    }
                }
                copy_profile(&args.profile, &config.create_new_profile_from)
            }
            Some(profile_in_args) => copy_profile(&args.profile, profile_in_args),
        }

        let path_workflow = match &args.path {
            Some(path) => &path[..],
            None => {
                if config.default_current_folder {
                    "."
                } else {
                    ""
                }
            }
        };

        if args.verbose {
            println!(
                "DirsFinal: {{ex: {}, cf: {}, pl: {}}}",
                extension_folder.unwrap(),
                configs_folder.unwrap(),
                path_workflow
            );
        }

        let cmd_exec = Command::new(&config.vs_code_path[..])
            .arg(path_workflow)
            .arg("--extensions-dir")
            .arg(extension_folder.unwrap())
            .arg("--user-data-dir")
            .arg(configs_folder.unwrap())
            .output();

        match cmd_exec {
            Err(why) => println!(
                "Error starting VSCode proccess: {:?}",
                why1
            ),
            Ok(out) => {
                if args.verbose {
                    println!("{:?}", out)
                }
            }
        }
    } else {
        println!("Error building VSCode launch: extensions folder or configs folder is null.")
    }
}
