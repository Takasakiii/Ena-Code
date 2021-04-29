use fs_extra::dir::{CopyOptions, copy};

use crate::{arguments::LaunchOptions, configs::{Config, dirs_and_files}};
use std::{path::{Path, PathBuf}, process::Command};

pub fn launch(args: &LaunchOptions, config: &Config) {
    let path = Path::new(&config.profiles_folder);
    let joined_path = path.join(&remove_caracteres(&args.profile, &config));
    let extension_folder = joined_path.join("extensions");
    let configs_folder = config_folder(&config, &joined_path, &path);

    let extension_folder = extension_folder.to_str();
    let configs_folder = configs_folder.to_str();


    if extension_folder.is_some() && configs_folder.is_some() {
        match &args.base_derive {
            None => copy_profile(args, &config.create_new_profile_from),
            Some(profile_in_args) => copy_profile(args, profile_in_args)
        }

        let path_workflow = match &args.path {
            Some(path) => &path[..],
            None => if config.default_current_folder {
                "."
            } else {
                ""
            }
        };

        if args.verbose {
            println!("DirsFinal: {{ex: {}, cf: {}, pl: {}}}", extension_folder.unwrap(), configs_folder.unwrap(), path_workflow);
        }

        let cmd_exec = Command::new(&config.vs_code_path[..])
            .arg(path_workflow)
            .arg("--extensions-dir")
            .arg(extension_folder.unwrap())
            .arg("--user-data-dir")
            .arg(configs_folder.unwrap())
            .output();

        match cmd_exec {
            Err(why) => println!("Problema ao iniciar o processo do visual studio code: {:?}", why),
            Ok(out) => if args.verbose {
                println!("{:?}", out)
            }
        }

    } else {
        println!("Um problema ao contruir o launch do visual studio code.")
    }
}

fn get_profile_path(profile_name: &String) -> PathBuf {
    dirs_and_files::create_or_get_ena_home_folder()
        .unwrap()
        .join("vs-code-profiles")
        .join(profile_name)
}

fn check_profile_exists(profile_name: &String) -> bool {
    let ena_folder = get_profile_path(profile_name);

    let path = Path::new(&ena_folder);

    path.is_dir()
}

fn create_profile(profile_name: &String, profile_fonte: &String) {
    let dir_destino = get_profile_path(profile_name);
    let dir_origin = get_profile_path(profile_fonte);
    let mut options = CopyOptions::new();
    options.skip_exist = true;
    options.overwrite = false;
    options.copy_inside = true;

    if let Err(why) = copy(&dir_origin, &dir_destino, &options) {
        println!("Não foi possivel derivar do profile: {}, iniciando a partir de um novo.\n{{Origem: {:?}, Destinho: {:?}}}\n\nMotivo: {}", profile_fonte, dir_origin, dir_destino, why);
    }
}

fn copy_profile(args: &LaunchOptions, profile_origin: &String) {
    if args.profile != *profile_origin {
        if !check_profile_exists(&args.profile) && check_profile_exists(profile_origin) {
            create_profile(&args.profile, profile_origin)
        }
    }
}

fn remove_caracteres(path: &String, config: &Config) -> String {
    let mut string_path = path.to_string();
    string_path.retain(|c| !r#"(),".;:'<>/\|?*"#.contains(c));

    if string_path == "" {
        string_path = config.create_new_profile_from.clone();
    }
    string_path
}

fn config_folder(config: &Config, profile_path: &PathBuf, profiles_base_folder: &Path) -> PathBuf {
    if config.shared_profiles_configs {
        let default_profile_folder = profiles_base_folder.join(&config.create_new_profile_from);
        default_profile_folder.join("configs")

    } else {
        profile_path.join("configs")
    }
}
