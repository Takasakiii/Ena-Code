use std::{
    env, fs,
    path::{Path, PathBuf},
};

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

fn get_home_dir() -> Result<PathBuf, String> {
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
