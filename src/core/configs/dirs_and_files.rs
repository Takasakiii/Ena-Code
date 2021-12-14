use std::{
    env, fs,
    path::{Path, PathBuf},
};

use fs_extra::dir::{copy, CopyOptions};

use super::Config;

pub fn path_to_string(path: PathBuf) -> String {
    path.to_str().unwrap_or("").to_string()
}

pub fn find_it<P>(exe_name: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths)
            .filter_map(|dir| {
                let full_path = dir.join(&exe_name);
                if full_path.is_file() {
                    Some(full_path)
                } else {
                    None
                }
            })
            .next()
    })
}

pub fn get_bin_or_cmd_name<'a>() -> &'a str {
    if cfg!(target_os = "windows") {
        "code.cmd"
    } else {
        "code"
    }
}

pub fn get_home_dir() -> Result<PathBuf, String> {
    let home_dir = dirs::home_dir();
    match home_dir {
        None => Err("Não foi possível localizar a pasta home do usuário.".into()),
        Some(data) => Ok(data),
    }
}

pub fn create_or_get_ena_home_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut home_folder = get_home_dir()?;

    home_folder = home_folder.join(".ena-code");
    let path_ena_code_folder = Path::new(&home_folder);
    if !path_ena_code_folder.is_dir() {
        fs::create_dir(&path_ena_code_folder)?;
    }
    Ok(home_folder)
}

pub fn get_profiles_folder_path() -> PathBuf {
    let config = Config::get_config(false);
    create_or_get_ena_home_folder()
        .unwrap()
        .join(config.profiles_folder)
}

pub fn get_profile_path(profile_name: &str) -> PathBuf {
    get_profiles_folder_path().join(profile_name)
}

pub fn check_profile_exists(profile_name: &str) -> bool {
    let ena_folder = get_profile_path(profile_name);

    let path = Path::new(&ena_folder);

    path.is_dir()
}

pub fn create_profile(profile_name: &str, profile_fonte: &str) {
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

pub fn copy_profile(profile: &str, profile_origin: &str) {
    if profile != profile_origin
        && !check_profile_exists(profile)
        && check_profile_exists(profile_origin)
    {
        create_profile(profile, profile_origin)
    }
}

pub fn remove_caracteres(path: &str, config: &Config) -> String {
    let mut string_path = path.to_string();
    string_path.retain(|c| !r#"(),".;:'<>/\|?*"#.contains(c));

    if string_path.is_empty() {
        string_path = config.create_new_profile_from.clone();
    }
    string_path
}

pub fn config_folder(config: &Config, profile_path: &Path, profiles_base_folder: &Path) -> PathBuf {
    if config.shared_profiles_configs {
        let default_profile_folder = profiles_base_folder.join(&config.create_new_profile_from);
        default_profile_folder.join("configs")
    } else {
        profile_path.join("configs")
    }
}
