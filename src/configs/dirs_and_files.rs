use super::Enarc;
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

fn read_enarc(path_mod: &mut PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(p) = check_and_get_enarc() {
        let raw_enarc = fs::read_to_string(p)?;
        let enarc = Enarc::from_string(raw_enarc)?;
        *path_mod = PathBuf::from(enarc.ena_home_path);
    }
    Ok(())
}

pub fn create_or_get_ena_home_folder() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut home_folder = get_home_dir()?;

    if let Err(why) = read_enarc(&mut home_folder) {
        println!("Houve um erro ao ler o .enarc\n\nMotivo: {:?}", why);
    }
    home_folder = home_folder.join(".ena-code");
    let path_ena_code_folder = Path::new(&home_folder);
    if !path_ena_code_folder.is_dir() {
        fs::create_dir(&path_ena_code_folder)?;
    }
    Ok(home_folder)
}

fn check_and_get_enarc() -> Result<PathBuf, String> {
    let home_dir = get_home_dir()?;
    let enarc_file = home_dir.join(".enarc");

    let path = Path::new(&enarc_file);
    if path.is_file() {
        Ok(enarc_file)
    } else {
        Err("O arquivo enarc não existe".into())
    }
}
