use clap::{Parser, Subcommand};

use crate::{list, remove};

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

impl Commands {
    pub fn handle(options: &LaunchOptions) {
        match &options.commands {
            Commands::Profiles { commands } => Profiles::handle(commands),
        }
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
