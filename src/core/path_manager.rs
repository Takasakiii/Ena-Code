use std::{
    fmt::Display,
    fs::{self, DirEntry, File},
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

impl Default for EnaFolder {
    fn default() -> Self {
        Self::new()
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

pub trait ProfileGetters {
    fn get_name(&self) -> &str;
    fn get_path(&self) -> &PathBuf;
}

pub struct Profile {
    name: String,
    path: PathBuf,
}

impl ProfileGetters for Profile {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

impl From<DirEntry> for Profile {
    fn from(dir: DirEntry) -> Self {
        let name = dir
            .file_name()
            .to_str()
            .expect("Falha ao carregar um profile.")
            .to_owned();
        let path = dir.path();

        Self { name, path }
    }
}

pub trait ProfilesOperations: Display {
    fn get_path(&self) -> &PathBuf;
}

pub struct Profiles {
    path: PathBuf,
    profiles: Vec<Profile>,
}

impl Profiles {
    pub const PROFILE_FOLDER_NAME: &'static str = "vs-code-profiles";

    fn get_path(ena_folder: &impl EnaFolderPath) -> PathBuf {
        let ena_path = ena_folder.get_path();
        ena_path.join(Self::PROFILE_FOLDER_NAME)
    }

    pub fn new(ena_folder: &impl EnaFolderPath) -> Self {
        let path = Self::get_path(ena_folder);

        if !path.exists() {
            fs::create_dir(&path).expect("A pasta que armazena os profiles não pode ser criada.");
        }

        let sub_paths = fs::read_dir(&path)
            .expect("Não foi possivel acessar a pasta dos profiles.")
            .map(|el| Profile::from(el.expect("Problemas para ler os profiles.")))
            .collect::<Vec<_>>();

        Self {
            path,
            profiles: sub_paths,
        }
    }
}

impl ProfilesOperations for Profiles {
    fn get_path(&self) -> &PathBuf {
        &self.path
    }
}

impl Display for Profiles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let values = self
            .profiles
            .iter()
            .fold(String::new(), |acc, new| format!("{}, {}", acc, &new.name));
        write!(f, "{}", values)
    }
}
