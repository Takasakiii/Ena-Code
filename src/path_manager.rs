use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
};

use crate::{
    config::{Config, ConfigManipulation, ConfigReader, DefaultConfig},
    errors::{EResult, EnaError},
    path_finder::PathFinder,
};

pub trait EnaFolderPath {
    fn get_path(&self) -> &PathBuf;
}

pub struct EnaFolder {
    path: PathBuf,
}

impl EnaFolder {
    pub const ENA_FOLDER_NAME: &'static str = ".ena-code";

    fn get_ena_path() -> EResult<PathBuf> {
        let home = dirs::home_dir().ok_or(EnaError::HomeDirNotExists)?;
        Ok(home.join(Self::ENA_FOLDER_NAME))
    }

    pub fn new() -> Self {
        if let Ok(path) = Self::get_ena_path() {
            if !path.exists() {
                fs::create_dir(path.clone()).unwrap_or_else(|_| {
                    panic!("Não foi possivel criar a pasta {}", path.to_str().unwrap())
                })
            }
            Self { path }
        } else {
            panic!("Pasta raiz do usuario pode ser identificada.")
        }
    }
}

impl EnaFolderPath for EnaFolder {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

pub trait ConfigFilePath {
    fn get_path(&self) -> &PathBuf;
    fn get_config(&self) -> &Config;
}

pub struct ConfigFile {
    path: PathBuf,
    config: Config,
}

impl ConfigFile {
    #[cfg(feature = "config_yml")]
    pub const ENA_CONFIG: &'static str = "config.yml";

    #[cfg(feature = "config_json")]
    pub const ENA_CONFIG: &'static str = "config.json";

    #[cfg(target_os = "windows")]
    pub const VS_CODE_NAME: &'static str = "code.cmd";

    #[cfg(not(target_os = "windows"))]
    pub const VS_CODE_NAME: &'static str = "code";

    fn get_config_path(ena_path: &impl EnaFolderPath) -> PathBuf {
        ena_path.get_path().join(Self::ENA_CONFIG)
    }

    pub fn new(ena_path: &impl EnaFolderPath, path_finder: &impl PathFinder) -> EResult<Self> {
        let path = Self::get_config_path(ena_path);
        let config;
        if !path.exists() {
            let default_config =
                DefaultConfig::new(path_finder, Self::VS_CODE_NAME)?.get_config()?;
            let mut raw_config = ConfigReader::new("".to_owned());
            raw_config.set_config(&default_config)?;
            let mut file = File::create(&path)?;
            file.write_all(raw_config.get_raw().as_bytes())?;
            config = default_config;
        } else {
            let mut file = File::create(&path)?;
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            let config_handler = ConfigReader::new(data);
            config = config_handler.get_config()?;
        }

        Ok(Self { path, config })
    }
}

impl ConfigFilePath for ConfigFile {
    fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_path(&self) -> &PathBuf {
        &self.path
    }
}
