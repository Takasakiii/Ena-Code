use fs_extra::dir::{CopyOptions, copy};

use crate::{arguments::LaunchOptions, configs::{Config, dirs_and_files}};
use std::{path::{Path, PathBuf}, process::Command};

pub fn launch(args: &LaunchOptions, config: &Config) {
    let path = Path::new(&config.profiles_folder);
    let joined_path = path.join(&args.profile);
    let extension_folder = joined_path.join("extensions");
    let configs_folder = joined_path.join("configs");

    let extension_folder = extension_folder.to_str();
    let configs_folder = configs_folder.to_str();


    if extension_folder.is_some() && configs_folder.is_some() {
        match &args.base_derive {
            None => copy_profile(args, &config.create_new_profile_from),
            Some(profile_in_args) => copy_profile(args, profile_in_args)
        }

        if args.verbose {
            println!("DirsFinal: {{ex: {}, cf: {}}}", extension_folder.unwrap(), configs_folder.unwrap());
        }

        let cmd_exec = Command::new(&config.vs_code_path[..])
            .arg(&args.path)
            .arg("--extensions-dir")
            .arg(extension_folder.unwrap())
            .arg("--user-data-dir")
            .arg(configs_folder.unwrap())
            .output();

        match cmd_exec {
            Err(why) => println!("Problema ao iniciar o processo do visual studio code: {:?}", why),
            Ok(out) => println!("{:?}", out)
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

// fn get_profile_args(args: &LaunchOptions) -> Option<String> {
//     if args.has_flag_in_index(0, "-b") && args.exists_flag_in_index(1) {
//         Some(args.get_flag(1))
//     } else {
//         None
//     }
// }

fn copy_profile(args: &LaunchOptions, profile_origin: &String) {
    if args.profile != *profile_origin {
        if !check_profile_exists(&args.profile) && check_profile_exists(profile_origin) {
            create_profile(&args.profile, profile_origin)
        }
    }
}
