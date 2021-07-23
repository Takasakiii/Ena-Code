use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::errors::{EResult, EnaError};

trait EnaFolderPath {
    fn get_path<'a>(&'a self) -> &'a PathBuf;
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
    fn get_path<'a>(&'a self) -> &'a PathBuf {
        &self.path
    }
}
