use std::{env, fs::{self, File}, io::prelude::*, path::{Path, PathBuf}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub profiles_folder: String,
    pub create_new_profile_from: String,
    pub vs_code_path: String
}

impl Config {
    fn new() -> Self {
        let code_path = find_it_to_string(get_bin_or_cmd_name())
                    .unwrap_or("".into());
        Self {
            create_new_profile_from: "Default".into(),
            profiles_folder: format!("{}/vs-code-profiles", get_home_dir()),
            vs_code_path: code_path
        }
    }

    fn get_config_raw() -> Self {
        let mut config = Self::new();

        if let Ok(config_string) = fs::read_to_string(format!("{}/config.yml", get_home_dir())) {
            let config_obj = serde_yaml::from_str::<Config>(&config_string[..]);
            
            if let Ok(config_obj) = config_obj {
                config = config_obj;
            }
        }

        if config.vs_code_path == String::from("") {
            let code_path = find_it_to_string(get_bin_or_cmd_name())
                .expect("Não foi possivel localizar o visual studio code instalado em seu computador");
            config.vs_code_path = code_path;
        }
        config
    }

    fn create_config() {
        match File::create(format!("{}/config.yml", get_home_dir())) {
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
        match Path::new("config.yml").exists() {
            true => Self::get_config_raw(),
            false => {
                Self::create_config();
                Self::get_config_raw()
            }
        }
    }
}

fn find_it<P>(exe_name: P) -> Option<PathBuf>
    where P: AsRef<Path>,
{
    env::var_os("PATH").and_then(|paths| {
        env::split_paths(&paths).filter_map(|dir| {
            let full_path = dir.join(&exe_name);
            if full_path.is_file() {
                Some(full_path)
            } else {
                None
            }
        }).next()
    })
}

fn find_it_to_string<P>(exe_name: P) -> Option<String>
    where P: AsRef<Path>
{
    let result = find_it(exe_name);
    match result {
        None => None,
        Some(data) => {
            match data.to_str() {
                None => None,
                Some(data) => Some(data.into())
            }
        }
    }
}

fn get_bin_or_cmd_name<'a>() -> &'a str {
    if cfg!(target_os = "windows") {
        "code.cmd"
    } else {
        "code"
    }
}

fn get_home_dir() -> String {
    let home = dirs::home_dir()
        .expect("Plataforma não suportada.");
    
    let ena_home = home.join("ena-code");
    
    ena_home
        .to_str()
        .expect("Problema interno")
        .into()
}