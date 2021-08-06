mod config;
pub mod errors;
mod launcher;
mod path_finder;
mod path_manager;

pub mod models {
    pub use crate::config::Config;
}

pub mod traits {
    pub use crate::path_manager::{
        ConfigFilePath, EnaFolderPath, ProfileGetters, ProfilesOperations,
    };

    pub use crate::config::ConfigManipulation;
}

pub mod internal {
    pub use crate::config::{ConfigReader, DefaultConfig};
}

pub use path_manager::{ConfigFile, EnaFolder, Profile, Profiles};
