use clap::{Parser, Subcommand};

use crate::{config, list, remove};

#[derive(Parser, Debug)]
#[clap(
    name = "Ena-Code-Manager",
    about = "Utility to manage Ena-Code profiles"
)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = "Takasakiii <lucasmc2709@live.com>")]
pub struct LaunchOptions {
    #[clap(subcommand)]
    commands: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Manage Ena-Code profiles
    Profiles {
        #[clap(subcommand)]
        commands: Profiles,
    },
    /// Configure Ena-Code
    Config {
        #[clap(subcommand)]
        configs: EnaConfigs,
    },
}

#[derive(Subcommand, Debug)]
pub enum Profiles {
    /// List Ena-Code profiles
    List,
    /// Remove an Ena-Code profile
    Remove {
        /// Profile name
        name: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum EnaConfigs {
    /// Disable segregation of config by profile, making profiles share the same config.
    SharedProfilesConfigs {
        #[clap(subcommand)]
        enable: EnableDisable,
    },
    /// Define that Ena-Code should always consider that if it doesn't pass the project path, use the current folder.
    UseCurrentFolder {
        #[clap(subcommand)]
        enable: EnableDisable,
    },
    /// Set path to Visual Studio Code executable.
    VsCodePath { path: String },
    /// Profile where Ena-Code uses it as a base for new profiles (except those created with the -b flag).
    DefaultProfile { profile: String },
    /// Location where Ena-Code should use to save profiles.
    ProfilesFolder { path: String },
}

#[derive(Subcommand, Debug)]
pub enum EnableDisable {
    /// Activate the configuration.
    Enable,
    /// Disable the configuration.
    Disable,
}

impl Commands {
    pub fn handle(options: &LaunchOptions) {
        match &options.commands {
            Commands::Profiles { commands } => Profiles::handle(commands),
            Commands::Config { configs } => EnaConfigs::handle(configs),
        }
    }
}

impl EnaConfigs {
    fn handle(config: &Self) {
        match &config {
            Self::DefaultProfile { profile } => config::default_profile(profile),
            Self::ProfilesFolder { path } => config::profiles_folder(path),
            Self::SharedProfilesConfigs { enable } => config::shared_profiles_config(enable),
            Self::UseCurrentFolder { enable } => config::use_current_folder(enable),
            Self::VsCodePath { path } => config::vs_code_path(path),
        }
    }
}

impl EnableDisable {
    pub fn to_bool(&self) -> bool {
        matches!(self, &EnableDisable::Enable)
    }
}

impl Profiles {
    fn handle(options: &Profiles) {
        match &options {
            Profiles::List => list::list_profiles(),
            Profiles::Remove { name } => remove::remove(name),
        }
    }
}

impl LaunchOptions {
    pub fn build() -> Self {
        LaunchOptions::parse()
    }
}
